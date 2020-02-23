{ config, pkgs, lib, ... }:
with lib;
let
  makeBackup = pkgs.writeScript "make-backup" ''
    #!${pkgs.bash}/bin/bash

    set -e
    umask 0077

    DBDUMPDIR=`mktemp -d`
    pushd $DBDUMPDIR

    ${config.services.mysql.package}/bin/mysqldump --single-transaction flarum > flarum.sql
    ${pkgs.sudo}/bin/sudo -u mattermost ${config.services.postgresql.package}/bin/pg_dump mattermost > mattermost.sql

    ${pkgs.gnutar}/bin/tar cf - --exclude "/var/lib/gitea/repositories/*/*.git/archives" /etc/nixos /var/lib/gitea flarum.sql mattermost.sql | \
        ${pkgs.bzip2}/bin/bzip2 | \
        ${pkgs.gnupg}/bin/gpg --symmetric --batch --passphrase-file /etc/nixos/secret/backup-passphrase | \
        ${pkgs.rclone}/bin/rclone rcat --config /etc/nixos/secret/rclone.conf dropbox:backup-`date +%F`.tar.bz2.gpg

    popd
    rm -rf $DBDUMPDIR

    echo Backup done
  '';
  cfg = config.services.mlabs-backup;
in
{
  options.services.mlabs-backup = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable backups";
    };
  };

  config = mkIf cfg.enable {
    systemd.services.mlabs-backup = {
      description = "M-Labs backup";
      serviceConfig = {
        Type = "oneshot";
        User = "root";
        Group = "root";
        ExecStart = "${makeBackup}";
      };
    };

    systemd.timers.mlabs-backup = {
      description = "M-Labs backup";
      wantedBy = [ "timers.target" ];
      timerConfig.OnCalendar = "weekly";
    };
  };
}
