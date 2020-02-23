{ config, pkgs }:

let
  pkg = pkgs.callPackage ./pkg.nix {};
in {
  type = "normal";
  pythonPackages = self: [ pkg ];
  module = "mattermostgithub:app";
  env = [
    "MGI_CONFIG_FILE=${./../secret/mattermost-github-integration.py}"
  ];
  socket = "${config.services.uwsgi.runDir}/uwsgi.sock";
  # allow access from nginx
  chmod-socket = 666;
}
