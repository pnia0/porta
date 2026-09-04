{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy

            libxkbcommon
            libGL
            vulkan-loader
            wayland
          ];
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libGL
              pkgs.vulkan-loader
            ]}"
          '';
          PORTA_CONFIG = "${./fixtures/config.toml}";
        };

        packages.default = pkgs.callPackage ./nix/package.nix { };
      }
    ) // {
      homeManagerModules.default = { lib, pkgs, ... }: {
        imports = [ ./nix/hm-module.nix ];
        programs.porta.package = lib.mkDefault self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      };
    };
}
