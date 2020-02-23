{ pkgs }:

rec {
  wavedrom = pkgs.python3Packages.buildPythonPackage rec {
    pname = "wavedrom";
    version = "0.1";

    src = pkgs.python3Packages.fetchPypi {
      inherit pname version;
      sha256 = "006w683zlmmwcw5xz1n5dwg34ims5jg3gl2700ql4wr0myjz6710";
    };

    buildInputs = [ pkgs.python3Packages.setuptools_scm ];
    propagatedBuildInputs = with pkgs.python3Packages; [ svgwrite attrdict ];
    doCheck = false;

    meta = with pkgs.stdenv.lib; {
      description = "WaveDrom compatible Python module and command line";
      homepage    = "https://pypi.org/project/wavedrom/";
      license     = licenses.mit;
    };
  };

  sphinxcontrib-wavedrom = pkgs.python3Packages.buildPythonPackage rec {
    pname = "sphinxcontrib-wavedrom";
    version = "2.0.0";

    src = pkgs.python3Packages.fetchPypi {
      inherit pname version;
      sha256 = "0nk36zqq5ipxqx9izz2iazb3iraasanv3nm05bjr21gw42zgkz22";
    };

    buildInputs = [ pkgs.python3Packages.setuptools_scm ];
    propagatedBuildInputs = [ wavedrom ] ++ (with pkgs.python3Packages; [ sphinx xcffib cairosvg ]);
    doCheck = false;

    meta = with pkgs.stdenv.lib; {
      description = "A Sphinx extension that allows including WaveDrom diagrams";
      homepage    = "https://pypi.org/project/sphinxcontrib-wavedrom/";
      license     = licenses.mit;
    };
  };
}
