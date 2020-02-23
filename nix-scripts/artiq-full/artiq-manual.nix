{ stdenv, lib, fetchgit, git, python3Packages, texlive, texinfo, sphinxcontrib-wavedrom }:

let
  artiqVersion = import <artiq-fast/pkgs/artiq-version.nix> { inherit stdenv fetchgit git; };

  isLatexPdfTarget = target: builtins.match "latexpdf.*" target != null;

  latex = texlive.combine {
    inherit (texlive)
      scheme-basic latexmk cmap collection-fontsrecommended fncychap
      titlesec tabulary varwidth framed fancyvrb float wrapfig parskip
      upquote capt-of needspace etoolbox;
  };

  artiq-manual = target: stdenv.mkDerivation rec {
    name = "artiq-manual-${target}-${version}";
    version = artiqVersion;

    src = import <artiq-fast/pkgs/artiq-src.nix> { inherit fetchgit; };
    buildInputs = [
      python3Packages.sphinx python3Packages.sphinx_rtd_theme
      python3Packages.sphinx-argparse sphinxcontrib-wavedrom
    ] ++
      lib.optional (isLatexPdfTarget target) latex ++
      lib.optional (target == "texinfo") texinfo;

    preBuild = ''
        export VERSIONEER_OVERRIDE=${artiqVersion}
        export SOURCE_DATE_EPOCH=${import <artiq-fast/pkgs/artiq-timestamp.nix> { inherit stdenv fetchgit git; }}
        cd doc/manual
      '';
    makeFlags = [ target ];

    installPhase =
      let
        dest = "$out/share/doc/artiq-manual";
      in
        if isLatexPdfTarget target
        then ''
            mkdir -p ${dest}
            cp _build/latex/ARTIQ.pdf ${dest}/

            mkdir -p $out/nix-support/
            echo doc-pdf manual ${dest} ARTIQ.pdf >> $out/nix-support/hydra-build-products
          ''
        else ''
            mkdir -p ${dest}
            cp -r _build/${target} ${dest}/

            mkdir -p $out/nix-support/
            echo doc manual ${dest}/${target} index.html >> $out/nix-support/hydra-build-products
          '';
  };

  targets = [ "html" "latexpdf" ];
in
  builtins.listToAttrs (map (target: { name = "artiq-manual-${target}"; value = artiq-manual target; }) targets)
