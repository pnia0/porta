{ config, lib, pkgs, ... }:
let
  cfg = config.programs.porta;
  aliases = cfg.settings.aliases or { };
  tomlFormat = pkgs.formats.toml { };
  configFile = tomlFormat.generate "config.toml" aliases;
in
{
  options.programs.porta = {
    enable = lib.mkEnableOption "porta";

    package = lib.mkOption {
      type = lib.types.package;
      description = "porta package to install";
    };

    settings = lib.mkOption {
      type = lib.types.attrsOf lib.types.anything;
      default = { };
      description = "Configuration written to ~/.config/porta/config.toml";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];
    xdg.configFile."porta/config.toml".source = configFile;
  };
}
