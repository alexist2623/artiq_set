{ config, pkgs, lib, ... }:
with lib;
let
  homu = pkgs.callPackage ./pkg.nix {};
  cfg = config.services.homu;
in

{
  options.services.homu = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable the bot";
    };
    dbDir = mkOption {
      type = types.str;
      default = "/var/db/homu";
      description = "Path to the database file (use the same path in config.toml)";
    };
    config = mkOption {
      description = "Location of config.toml";
      type = types.str;
    };
  };

  config = mkIf cfg.enable {
    users.users.homu = {
      group = "homu";
      home = cfg.dbDir;
      createHome = true;
    };
    users.groups.homu = {};

    systemd.services.homu = {
      description = "Homu bot";
      wantedBy = [ "multi-user.target" ];
      after    = [ "network.target" ];
      serviceConfig = {
        Type = "simple";
        ExecStart = "${homu}/bin/homu -c ${cfg.config}";

        Restart = "always";
        RestartSec = "5sec";

        User = "homu";
        Group = "homu";
      };
    };
  };

}
  
