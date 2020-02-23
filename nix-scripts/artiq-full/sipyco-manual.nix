{ stdenv, lib, python3Packages, texlive, texinfo, sipyco }:

let
  version = sipyco.version;

  isLatexPdfTarget = target: builtins.match "latexpdf.*" target != null;

  latex = texlive.combine {
    inherit (texlive)
      scheme-basic latexmk cmap collection-fontsrecommended fncychap
      titlesec tabulary varwidth framed fancyvrb float wrapfig parskip
      upquote capt-of needspace etoolbox;
  };

  sipyco-manual = target: stdenv.mkDerivation rec {
    name = "sipyco-manual-${target}-${version}";
    inherit version;

    src = sipyco.src;
    buildInputs = [
      python3Packages.sphinx python3Packages.sphinx_rtd_theme
      python3Packages.sphinx-argparse sipyco
    ] ++
      lib.optional (isLatexPdfTarget target) latex ++
      lib.optional (target == "texinfo") texinfo;

    preBuild = ''
        export SOURCE_DATE_EPOCH=`cat TIMESTAMP`
        cd doc
      '';
    makeFlags = [ target ];

    installPhase =
      let
        dest = "$out/share/doc/sipyco-manual";
      in
        if isLatexPdfTarget target
        then ''
            mkdir -p ${dest}
            cp _build/latex/SiPyCo.pdf ${dest}/

            mkdir -p $out/nix-support/
            echo doc-pdf manual ${dest} SiPyCo.pdf >> $out/nix-support/hydra-build-products
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
  builtins.listToAttrs (map (target: { name = "sipyco-manual-${target}"; value = sipyco-manual target; }) targets)
