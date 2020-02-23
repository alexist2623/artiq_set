{ python3Packages, python3, fetchFromGitHub, git, openssh }:

let
  uritemplate_0_2_0 = python3Packages.github3_py.overrideAttrs(oa: rec {
  	version = "0.2.0";
  	src = python3Packages.fetchPypi {
      pname = "uritemplate.py";
      inherit version;
      sha256 = "1pfk04pmnysz0383lwzgig8zqlwiv2n4pmq51f0mc60zz1jimq4g";
    };
  });
  github3_py_0_9_6 = python3Packages.github3_py.overrideAttrs(oa: rec {
    version = "0.9.6";
    src = python3Packages.fetchPypi {
      pname = "github3.py";
      inherit version;
      sha256 = "1i8xnh586z4kka7pjl7cy08fmzjs14c8jdp8ykb9jjpzsy2xncdq";
    };
    propagatedBuildInputs = [ python3Packages.requests uritemplate_0_2_0 ];
  });
in
  python3Packages.buildPythonApplication {
    name = "homu";
    src = fetchFromGitHub {
      owner = "servo";
      repo = "homu";
      rev = "2ea53e76ebac3e5fa11bc39054b3cd4c42eff607";
      sha256 = "1ih7s8zfbpq0qb9vqbxzr0r4s9ff52l4ipr916kwbck3ygliq3r9";
    };
    patches = [ ./patch-cache-directory.patch ./disable-ssh-host-keycheck.patch ];
    postInstall = "chmod 755 $out/${python3.sitePackages}/homu/git_helper.py";
    propagatedBuildInputs = [ github3_py_0_9_6 git openssh ] ++ (with python3Packages; [ toml jinja2 requests bottle waitress retrying ]);
    checkPhase = "python -m unittest discover tests -v";
  }
