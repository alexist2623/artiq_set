{ pkgs, sipyco, asyncserial, artiq }:
let
  condaBuild = import ./fast/conda-build.nix { inherit pkgs; };
  condaFakeSource = import ./fast/conda-fake-source.nix { inherit pkgs; };
  dualPackage = (
    { name, version, src, pythonOptions ? {}, condaOptions ? {}}:
      {
        "${name}" = pkgs.python3Packages.buildPythonPackage ({
          inherit version;
          name = "${name}-${version}";
          inherit src;
        } // pythonOptions);
        "${name}-manual-html" = pkgs.stdenv.mkDerivation {
          name = "${name}-manual-html-${version}";
          inherit version src;
          buildInputs = (with pkgs.python3Packages; [ sphinx sphinx_rtd_theme sphinx-argparse ]) ++ [ artiq ];
          preBuild = ''
            export SOURCE_DATE_EPOCH=${import ./fast/pkgs/artiq-timestamp.nix { inherit (pkgs) stdenv fetchgit git; }}
            cd doc
          '';
          makeFlags = [ "html" ];
          installPhase =
            let
              dest = "$out/share/doc/${name}-manual";
            in
              ''
              mkdir -p ${dest}
              cp -r _build/html ${dest}/

              mkdir -p $out/nix-support/
              echo doc manual ${dest}/html index.html >> $out/nix-support/hydra-build-products
              '';
        };
        "conda-${name}" = condaBuild {
          name = "conda-${name}";
          src = condaFakeSource ({
            inherit name version src;
          } // condaOptions);
        };
      }
    );
  # https://github.com/m-labs/artiq/issues/23
  hidapi = pkgs.hidapi.overrideAttrs (oa: {
      src = pkgs.fetchFromGitHub {
        owner = "signal11";
        repo = "hidapi";
        rev = "a6a622ffb680c55da0de787ff93b80280498330f";
        sha256 = "17n7c4v3jjrnzqwxpflggxjn6vkzscb32k4kmxqjbfvjqnx7qp7j";
      };
    });
in
  (dualPackage {
    name = "korad_ka3005p";
    version = "1.1";
    src = pkgs.fetchFromGitHub {
      owner = "m-labs";
      repo = "korad_ka3005p";
      rev = "a0cfaa5792a211e166d224314c4d0be4881b9b8d";
      sha256 = "1bxzyjyvdhsbm9hj7ypf0vgkd1lvc340bb6lx3wchvh30n7bv9gv";
    };
    pythonOptions = { propagatedBuildInputs = [ sipyco asyncserial ]; };
    condaOptions = { dependencies = [ "sipyco" "asyncserial" ]; };
  }) // (dualPackage {
    name = "novatech409b";
    version = "1.1";
    src = pkgs.fetchFromGitHub {
      owner = "m-labs";
      repo = "novatech409b";
      rev = "8740b3e7b254e03395135e6bc128bbaca70d4fbb";
      sha256 = "0mwm434y83y8jb30fpz69z6z3b6sxbc8dv3nw0hq4wc7iginx89d";
    };
    pythonOptions = { propagatedBuildInputs = [ sipyco asyncserial ]; };
    condaOptions = { dependencies = [ "sipyco" "asyncserial" ]; };
  }) // (dualPackage {
    name = "lda";
    version = "1.1";
    src = pkgs.fetchFromGitHub {
      owner = "m-labs";
      repo = "lda";
      rev = "6138a94a1116c8f7b40b8bd8bb161f847065aab6";
      sha256 = "1009k9pq8wx5zxrljkxr1g95g8q979i7mq3csksdkd3d0v2jvqif";
    };
    pythonOptions = {
      propagatedBuildInputs = [ sipyco ];
      postPatch = ''
        substituteInPlace lda/hidapi.py \
        --replace "hidapi_lib_path = None"\
                  "hidapi_lib_path = '${hidapi}/lib/libhidapi-libusb.so.0'"
      '';
    };
    condaOptions = { dependencies = [ "sipyco" ]; };
  }) // (dualPackage {
    name = "thorlabs_tcube";
    version = "1.1";
    src = pkgs.fetchFromGitHub {
      owner = "m-labs";
      repo = "thorlabs_tcube";
      rev = "8b85292d76a69ae72ba8da32b894c87c794574ba";
      sha256 = "09cy9nhydcwdib21wb0qg1cinvibfbszwgphrmf2ajw5kqpr1d6a";
    };
    pythonOptions = { propagatedBuildInputs = [ sipyco asyncserial ]; };
    condaOptions = { dependencies = [ "sipyco" "asyncserial" ]; };
  }) // (dualPackage {
    name = "newfocus8742";
    version = "0.2";
    src = pkgs.fetchFromGitHub {
      owner = "quartiq";
      repo = "newfocus8742";
      rev = "9f6092b724b33b934aa4d3a1d6a20c295cd1d02d";
      sha256 = "0qf05ghylnqf3l5vjx5dc748wi84xn6p6lb6f9r8p6f1z7z67fb8";
    };
    pythonOptions = {
      propagatedBuildInputs = [ sipyco pkgs.python3Packages.pyusb ];
      # no unit tests so do a simple smoke test
      checkPhase = "python -m newfocus8742.aqctl_newfocus8742 -h";
    };
    condaOptions = { dependencies = [ "sipyco" ]; };
  }) // (dualPackage {
    name = "hut2";
    version = "0.2";
    src = pkgs.fetchFromGitHub {
      owner = "quartiq";
      repo = "hut2";
      rev = "68369d5d63d233827840a9a752d90454a4e03baa";
      sha256 = "0r832c0icz8v3w27ci13024bqfslj1gx6dwhjv11ksw229xdcghd";
    };
    pythonOptions = {
      propagatedBuildInputs = [ sipyco ];
      # no unit tests without hardware so do a simple smoke test
      checkPhase = "python -m hut2.aqctl_hut2 -h";
    };
    condaOptions = { dependencies = [ "sipyco" ]; };
  }) // rec {
    toptica-lasersdk = pkgs.python3Packages.buildPythonPackage rec {
      version = "2.0.0";
      name = "toptica-lasersdk-${version}";
      format = "wheel";
      src = pkgs.fetchurl {
        url = "https://files.pythonhosted.org/packages/6b/e2/5c98407215884c2570453a78bc0d6f0bbe619f06593847ccd6a2f1d3fe59/toptica_lasersdk-2.0.0-py3-none-any.whl";
        sha256 = "1k5d9ah8qzp75hh63nh9l5dk808v9ybpmzlhrdc3sxmas3ajv8s7";
      };
      propagatedBuildInputs = [ pkgs.python3Packages.pyserial ];
    };
    toptica-lasersdk-artiq = pkgs.python3Packages.buildPythonPackage rec {
      version = "0.2";
      name = "toptica-lasersdk-artiq-${version}";
      src = pkgs.fetchFromGitHub {
        owner = "quartiq";
        repo = "lasersdk-artiq";
        rev = "901dec13a1bf9429ce7ab49be34b03d1c49b8a9f";
        sha256 = "0lqxvgvpgrpw1kzhg5axnfb40ils2vdk75r43hqmk2lfz4sydwb2";
      };
      postPatch = ''
        substituteInPlace lasersdk_artiq/aqctl_laser.py \
          --replace "toptica.lasersdk.async.client" \
                    "toptica.lasersdk.asyncio.client"
        substituteInPlace lasersdk_artiq/test.py \
          --replace "toptica.lasersdk.async.client" \
                    "toptica.lasersdk.asyncio.client"
      '';
      propagatedBuildInputs = [ sipyco toptica-lasersdk ];
    };
    conda-toptica-lasersdk-artiq = condaBuild {
      name = "conda-toptica-lasersdk-artiq";
      src = condaFakeSource {
        name = "toptica-lasersdk-artiq";
        inherit (toptica-lasersdk-artiq) version src;
        dependencies = [ "sipyco" "lasersdk =1.3.1" ];
      };
    };
  } // (dualPackage {
    name = "highfinesse-net";
    version = "0.2";
    src = pkgs.fetchFromGitHub {
      owner = "quartiq";
      repo = "highfinesse-net";
      rev = "a9cc049c9846845d2b2d8662266ec11fe770abee";
      sha256 = "01mk4gf6rk3jqpz4y7m35vawjybvyp26bizz5a4ygkb8dq5l51g4";
    };
    pythonOptions = {
      propagatedBuildInputs = [ sipyco ];
      # no unit tests without hardware so do a simple smoke test
      checkPhase = "python -m highfinesse_net.aqctl_highfinesse_net -h";
    };
    condaOptions = { dependencies = [ "sipyco" ]; };
  }) // rec {
    artiq-comtools = pkgs.python3Packages.buildPythonPackage rec {
      name = "artiq-comtools-${version}";
      version = "1.0";
      src = pkgs.fetchFromGitHub {
        owner = "m-labs";
        repo = "artiq-comtools";
        rev = "398f6becdaf89ec2e115106778467f9a766a2007";
        sha256 = "02i2a1mmzc7jixq4nkbmi1a4c0gk6kmm8bv1dxrka7m56nb9sk9w";
      };
      propagatedBuildInputs = [ sipyco pkgs.python3Packages.numpy pkgs.python3Packages.aiohttp ];
    };
    conda-artiq-comtools = condaBuild {
      name = "conda-artiq-comtools";
      src = condaFakeSource {
        name = "artiq-comtools";
        inherit (artiq-comtools) version src;
        dependencies = [ "sipyco" "numpy" "aiohttp >=3" ];
      };
    };
  }
