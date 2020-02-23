# We need to pass the whole source to conda for the git variables to work.
# recipe must be a string pointing to a path within the source.

{ pkgs }:
{ name, src, recipe ? "fake-conda"}:

let
  condaBuilderEnv = import ./conda-builder-env.nix { inherit pkgs; };
in pkgs.stdenv.mkDerivation {
  inherit name src;
  buildCommand =
    ''
    HOME=`pwd`
    mkdir $out
    ${condaBuilderEnv}/bin/conda-builder-env -c "PYTHON=python conda build --no-anaconda-upload --no-test --output-folder $out $src/${recipe}"

    mkdir -p $out/nix-support
    echo file conda $out/noarch/*.tar.bz2 >> $out/nix-support/hydra-build-products
    '';
}
