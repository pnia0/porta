{
  lib,
  rustPlatform,
  pkg-config,
  makeWrapper,

  libxkbcommon,
  libGL,
  vulkan-loader,
  wayland,
}:
let
  runtimeLibs = [
    libxkbcommon
    libGL
    vulkan-loader
    wayland
  ];
in
  rustPlatform.buildRustPackage {
    pname = "porta";
    version = "0.1.0";

    src = ../.;
    cargoLock.lockFile = ../Cargo.lock;

    nativeBuildInputs = [
      pkg-config
      makeWrapper
    ];

    buildInputs = runtimeLibs;

    postInstall = ''
      wrapProgram "$out/bin/porta" \
        --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath runtimeLibs}"
    '';

    meta = with lib; {
      description = "Desktop rancher app";
      mainProgram = "porta";
    };
  }
