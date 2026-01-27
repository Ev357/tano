{self, ...}: {
  lib,
  pkgs,
  config,
  ...
}: let
  cfg = config.programs.tano;

  localProviderType = lib.types.submodule {
    options = {
      type = lib.mkOption {
        type = lib.types.enum ["local"];
        description = "Local provider type";
      };

      path = lib.mkOption {
        type = lib.types.str;
        description = "Path to local music directory";
      };
    };
  };

  providerType = lib.types.oneOf [localProviderType];

  settingsType = lib.types.submodule {
    options = {
      providers = lib.mkOption {
        type = lib.types.listOf providerType;
        default = [
          {
            type = "local";
            path = "~/Music";
          }
        ];
        description = "List of music providers";
      };
    };
  };
in {
  options.programs.tano = {
    enable = lib.mkEnableOption "tano";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.template-manager;
      description = "The package to use";
    };

    settings = lib.mkOption {
      type = settingsType;
      default = {};
      description = "Settings";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [cfg.package];

    xdg.configFile."tano/config.toml" = lib.mkIf (cfg.settings != {}) {
      source = (pkgs.formats.toml {}).generate "config" cfg.settings;
    };
  };
}
