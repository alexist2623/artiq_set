{ pkgs }:

with pkgs;

let
  condaDeps = [ stdenv.cc xorg.libSM xorg.libICE xorg.libXrender libselinux ];
  # Use the full Anaconda distribution, which already contains conda-build and its many dependencies,
  # so we don't have to manually deal with them.
  condaInstaller = fetchurl {
    url = "https://repo.anaconda.com/archive/Anaconda3-2019.03-Linux-x86_64.sh";
    sha256 = "0fmpdd5876ylds98mydmv5klnwlzasa461k0l1f4vhbw96vm3j25";
  };
  condaSrcChmod = runCommand "conda-src-chmod" { } "mkdir $out; cp ${condaInstaller} $out/conda-installer.sh; chmod +x $out/conda-installer.sh";
  condaInstallerEnv = buildFHSUserEnv {
    name = "conda-installer-env";
    targetPkgs = pkgs: ([ condaSrcChmod ] ++ condaDeps);
  };

  condaInstalled = runCommand "conda-installed" { }
    ''
    ${condaInstallerEnv}/bin/conda-installer-env -c "${condaSrcChmod}/conda-installer.sh -p $out -b"
    substituteInPlace $out/lib/python3.7/site-packages/conda/gateways/disk/__init__.py \
      --replace "os.chmod(path, 0o2775)" "pass"
    '';
in
  buildFHSUserEnv {
    name = "conda-builder-env";
    targetPkgs = pkgs: [ condaInstalled ] ++ condaDeps;
  }
