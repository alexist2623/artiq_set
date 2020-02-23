{ stdenv, fetchgit, git, rustc }:
stdenv.mkDerivation rec {
  name = "libartiq-support-${version}";
  version = import ./artiq-version.nix { inherit stdenv fetchgit git; };
  src = import ./artiq-src.nix { inherit fetchgit; };
  phases = [ "buildPhase" ];
  # keep in sync with artiq/test/lit/lit.cfg or remove build from the latter once we don't use buildbot/conda anymore
  buildPhase =
  ''
  mkdir $out
  ${rustc}/bin/rustc ${src}/artiq/test/libartiq_support/lib.rs --out-dir $out -Cpanic=unwind -g
  '';
}
