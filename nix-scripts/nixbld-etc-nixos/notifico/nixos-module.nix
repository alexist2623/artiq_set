{ config, pkgs, lib, ... }:
with lib;
let
  notifico = (pkgs.callPackage ./pkg.nix {})
    .overrideAttrs (attrs: {
      buildInputs = attrs.buildInputs ++ [ pkgs.makeWrapper ];
      # Extend the module path so that local_config.py can be found
      postInstall = ''
        ${attrs.postInstall}

        wrapProgram $out/bin/notifico \
          --set PYTHONPATH "$${PYTHONPATH}:${cfg.dbDir}"
      '';
    });
  cfg = config.services.notifico;
in

{
  options.services.notifico = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable the commit notification service";
    };
    enableLocalRedis = mkOption {
      type = types.bool;
      default = true;
      description = "Enable a local Redis server";
    };
    dbDir = mkOption {
      type = types.str;
      default = "/var/db/notifico";
      description = "Home directory and location of the database file";
    };
    config = mkOption {
      description = "Path to local_config.py, https://github.com/notifico/notifico/raw/master/notifico/config.py";
      type = types.str;
    };
  };

  config = mkIf cfg.enable {
    users.users.notifico = {
      group = "notifico";
      home = cfg.dbDir;
      createHome = true;
    };
    users.groups.notifico = {};

    services.redis = mkIf cfg.enableLocalRedis {
      enable = true;
      bind = "127.0.0.1";
    };

    systemd.services =
      let
        User = "notifico";
        Group = "notifico";
        WorkingDirectory = "${cfg.dbDir}";
        ExecStartPre = [
          "${pkgs.coreutils}/bin/rm -f local_config.pyc"
          "${pkgs.coreutils}/bin/ln -sf ${cfg.config} local_config.py"
        ];

        notifico-init = {
          description = "Notifico initialization";
          serviceConfig = {
            inherit User Group WorkingDirectory ExecStartPre;
            Type = "oneshot";
            ExecStart = "${notifico}/bin/notifico init";
          };
        };
        notificoService = component: {
          description = "Notifico ${component}";
          wantedBy = [ "multi-user.target" ];
          after    = [ "network.target" "notifico-init.service" ];
          requires = [ "notifico-init.service" ];
          serviceConfig = {
            inherit User Group WorkingDirectory ExecStartPre;
            Type = "simple";
            ExecStart = "${notifico}/bin/notifico ${component}";

            Restart = "always";
            RestartSec = "5sec";
          };
        };
      in {
        inherit notifico-init;
        notifico-www = notificoService "www";
        notifico-worker = notificoService "worker";
        notifico-bots = notificoService "bots";
      };
  };
}
