{ pkgs ? import <nixpkgs> {},
  zc706-nix ? import <zc706> { mozillaOverlay = import <mozillaOverlay>; },
}:
let
  addBuildProducts = drv: drv.overrideAttrs (oldAttrs: {
      installPhase = ''
        ${oldAttrs.installPhase}

        mkdir -p $out/nix-support
        for f in $out/*.elf ; do
          echo file binary-dist $f >> $out/nix-support/hydra-build-products
        done
      '';
    });
in
builtins.mapAttrs (name: drv:
  pkgs.lib.hydraJob (
    addBuildProducts drv
  )
) zc706-nix.zc706
