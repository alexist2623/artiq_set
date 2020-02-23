{ stdenv, git, fetchgit }:
let
  artiq-timestamp = stdenv.mkDerivation {
    name = "artiq-timestamp";
    src = import ./artiq-src.nix { inherit fetchgit; };
    buildPhase = ''
      TIMESTAMP=`${git}/bin/git log -1 --format=%ct`
    '';
    installPhase = ''
      echo -n $TIMESTAMP > $out
    '';
  };
in
  builtins.readFile artiq-timestamp
