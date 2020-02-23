# Install ISE in /opt and add to /etc/nixos/configuration.nix:
#  nix.sandboxPaths = ["/opt"];

{ pkgs, isePath ? "/opt/Xilinx/14.7/ISE_DS" }:

let
  makeXilinxEnv = name: pkgs.buildFHSUserEnv {
    inherit name;
    targetPkgs = pkgs: (
      with pkgs; [
        ncurses5
        zlib
        libuuid
        xorg.libSM
        xorg.libICE
        xorg.libXrender
        xorg.libX11
        xorg.libXext
        xorg.libXtst
        xorg.libXi
      ]
    );
    profile = 
      ''
      source ${isePath}/common/.settings64.sh ${isePath}/common
      source ${isePath}/ISE/.settings64.sh ${isePath}/ISE
      '';
    runScript = name;
  };
in
  pkgs.lib.attrsets.genAttrs ["xst" "ngdbuild" "cpldfit" "taengine" "hprep6"] makeXilinxEnv
