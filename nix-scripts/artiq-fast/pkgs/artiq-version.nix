{ stdenv, git, fetchgit }:
let
  artiq-version = stdenv.mkDerivation {
    name = "artiq-version";
    src = import ./artiq-src.nix { inherit fetchgit; };
    # keep in sync with ../../artiq-fast.nix
    buildPhase = ''
      REV=`${git}/bin/git rev-parse HEAD`
      MAJOR_VERSION=`cat MAJOR_VERSION`
      if [ -e BETA ]; then
        SUFFIX=".beta"
      else
        SUFFIX=""
      fi
      COMMIT_COUNT=`${git}/bin/git rev-list --count HEAD`
    '';
    installPhase = ''
      echo -n $MAJOR_VERSION.$COMMIT_COUNT.`cut -c1-8 <<< $REV`$SUFFIX > $out
    '';
  };
in
  builtins.readFile artiq-version
