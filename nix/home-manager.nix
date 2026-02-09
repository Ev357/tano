{self, ...}: {
  lib,
  pkgs,
  config,
  ...
}: let
  cfg = config.programs.tano;

  tomlFormat = pkgs.formats.toml {};
in {
  options.programs.tano = {
    enable = lib.mkEnableOption "tano";

    package = lib.mkPackageOption self.packages.${pkgs.stdenv.hostPlatform.system} "tano" {nullable = true;};

    settings = lib.mkOption {
      type = tomlFormat.type;
      default = {};
      example =
        lib.literalExpression
        # nix
        ''
          {
            providers = [
              {
                type = "local";
                path = "~/Music";
              }
            ];
          }
        '';
      description = ''
        Configuration written to
        {file}`$XDG_CONFIG_HOME/tano/config.toml`.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [cfg.package];

    xdg.configFile."tano/config.toml" = lib.mkIf (cfg.settings != {}) {
      source = tomlFormat.generate "tano-settings" cfg.settings;
    };
  };
}
