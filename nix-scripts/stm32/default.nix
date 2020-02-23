{ # Use master branch of the overlay by default
  mozillaOverlay ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz),
  rustManifest ? builtins.fetchurl "https://static.rust-lang.org/dist/channel-rust-nightly.toml"
}:

let
  pkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rustPlatform = pkgs.recurseIntoAttrs (pkgs.callPackage ./rustPlatform.nix {
    inherit rustManifest;
  });
  buildStm32Firmware = { name, src }:
    let
      cargoSha256Drv = pkgs.runCommand "${name}-cargosha256" { } ''cp "${src}/cargosha256.nix" $out'';
    in
      rustPlatform.buildRustPackage rec {
        inherit name;
        version = "0.0.0";

        inherit src;
        cargoSha256 = (import cargoSha256Drv);

        buildPhase = ''
          export CARGO_HOME=$(mktemp -d cargo-home.XXX)
          cargo build --release
        '';

        doCheck = false;
        installPhase = ''
          mkdir -p $out $out/nix-support
          cp target/thumbv7em-none-eabihf/release/${name} $out/${name}.elf
          echo file binary-dist $out/${name}.elf >> $out/nix-support/hydra-build-products
        '';
      };
in
  {
    stabilizer = buildStm32Firmware {
      name = "stabilizer";
      src = <stabilizerSrc>;
    };
    thermostat = buildStm32Firmware {
      name = "thermostat";
      src = <thermostatSrc>;
    };
  }
