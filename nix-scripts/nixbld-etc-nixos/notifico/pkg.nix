{ python2Packages, python2, fetchFromGitHub, fetchurl }:

let
  Flask-Gravatar = python2Packages.buildPythonPackage {
    name = "Flask-Gravatar";
    src = python2Packages.fetchPypi {
      pname = "Flask-Gravatar";
      version = "0.5.0";
      sha256 = "1qb2ylirjajdqsmldhwfdhf8i86k7vlh3y4gnqfqj4n6q8qmyrk0";
    };
    propagatedBuildInputs = with python2Packages; [
      pytestrunner
      flask
    ];
    checkInputs = with python2Packages; [
      check-manifest
      coverage
      isort
      pydocstyle
      pytestcache
      pytestcov
      pytestpep8
      pytest
      pygments
    ];
  };
  utopia = python2Packages.buildPythonPackage {
    name = "utopia";
    src = fetchFromGitHub {
      owner = "notifico";
      repo = "utopia";
      rev = "70293ed5e1ca55232e0fae71061e7e9b9b29be6f";
      sha256 = "11cnh9l4d9jlhafnfis9si6kgk9zsdd5439qnhxh6dca3x4a986q";
    };
    propagatedBuildInputs = with python2Packages; [
      gevent
      blinker
    ];
    doCheck = false;
  };
  Flask-WTF = python2Packages.flask_wtf.overrideAttrs(oa: rec {
  	version = "0.8.4";
  	src = python2Packages.fetchPypi {
      pname = "Flask-WTF";
      inherit version;
      sha256 = "1khbwmlrcnk9f46f7kf531n06pkyfs6nc8fk273js9mj2igngg2y";
    };
  });
  Flask-XML-RPC = python2Packages.flask_wtf.overrideAttrs(oa: rec {
  	version = "0.1.2";
  	src = python2Packages.fetchPypi {
      pname = "Flask-XML-RPC";
      inherit version;
      sha256 = "1dwalj7pc5iid9l1k50q5mllirnn9f5s7jq54a66x48a4j179p2a";
    };
  });
in
  python2Packages.buildPythonApplication {
    name = "notifico";
    src = fetchFromGitHub {
      owner = "notifico";
      repo = "notifico";
      rev = "6af849e4c75dff4d740051676f5a2093a44efcee";
      sha256 = "18jifqdvjy4x5s1bh7vx501pin52g4n3hhw1z4m2c0h512z4spdr";
    };
    patches = [
      (fetchurl {
        url = https://github.com/whitequark/notifico/commit/22b582fad6cb97af6f7437e8462d720ddacc42ef.patch;
        sha256 = "0w8i8hf1r8b0p1y1zn9vyvnyi20qp120aiyalqymhsxsh17mma52";
      })
    ];
    propagatedBuildInputs = with python2Packages; [
      flask
      Flask-WTF
      Flask-Gravatar
      flask_sqlalchemy
      Flask-XML-RPC
      flask_mail
      flask-caching
      Fabric
      sqlalchemy
      utopia
      gevent
      oauth2
      redis
      gunicorn
      requests
      PyGithub
      xmltodict
      unidecode
      raven
      blinker
      docopt
      celery
    ];
    postInstall = ''
      mkdir $out/bin
      cat << EOF > $out/bin/notifico
      #!${python2}/bin/python
      import sys
      from notifico.__main__ import main

      sys.exit(main(sys.argv))
      EOF
      chmod +x $out/bin/notifico
    '';
  }
