{ pkgs ? import <nixpkgs> {},
  diskImageSize ? "22G",
  qemuMem ? "4G",
}:

with pkgs;

let
  windowsIso = fetchurl {
    url = "https://software-download.microsoft.com/download/sg/17763.107.101029-1455.rs5_release_svc_refresh_CLIENT_LTSC_EVAL_x64FRE_en-us.iso";
    sha256 = "668fe1af70c2f7416328aee3a0bb066b12dc6bbd2576f40f812b95741e18bc3a";
  };
  anaconda = fetchurl {
    url = "https://repo.anaconda.com/archive/Anaconda3-2019.03-Windows-x86_64.exe";
    sha256 = "1f9icm5rwab6l1f23a70dw0qixzrl62wbglimip82h4zhxlh3jfj";
  };

  escape = builtins.replaceStrings [ "\\" ] [ "\\\\" ];
  qemu = import ./qemu.nix {
    inherit pkgs qemuMem;
    diskImage = "c.img";
  };
  # Double-escape because we produce a script from a shell heredoc
  ssh = cmd: qemu.ssh (escape cmd);
  scp = qemu.scp;

  sshCondaEnv = cmd: ssh "anaconda\\scripts\\activate && ${cmd}";
  condaEnv = "artiq-env";
  condaDepSpecs =
    builtins.concatStringsSep " "
    (map (s: "\"${s}\"")
     (import ../conda-artiq-deps.nix));

  instructions =
    builtins.toFile "install.txt"
    (builtins.readFile ./install.txt);
in
stdenv.mkDerivation {
  name = "windows-installer";
  src = windowsIso;
  setSourceRoot = "sourceRoot=`pwd`";
  unpackCmd = ''
    ln -s $curSrc windows.iso
  '';
  propagatedBuildInputs = qemu.inputs;
  dontBuild = true;
  installPhase = ''
    mkdir -p $out/bin $out/data
    ln -s $(readlink windows.iso) $out/data/windows.iso
    cat > $out/bin/windows-installer.sh << EOF
    #!/usr/bin/env bash
    set -e -m

    ${qemu.qemu-img} create -f qcow2 c.img ${diskImageSize}
    ${qemu.runQemu false [] [
      "-boot" "order=d"
      "-drive" "file=c.img,index=0,media=disk,cache=unsafe"
      "-drive" "file=$out/data/windows.iso,index=1,media=cdrom,cache=unsafe"
    ]} &
    cat ${instructions}
    wait
    EOF

    cat > $out/bin/anaconda-installer.sh << EOF
    #!/usr/bin/env bash
    set -e -m

    ${qemu.runQemu false [] [
      "-boot" "order=c"
      "-drive" "file=c.img,index=0,media=disk"
    ]} &
    sleep 10
    ${ssh "ver"}

    ${scp anaconda "Anaconda.exe"}
    ${ssh "start /wait \"\" Anaconda.exe /S /D=%cd%\\anaconda"}

    ${sshCondaEnv "conda config --add channels conda-forge"}
    ${sshCondaEnv "conda config --add channels m-labs"}
    ( ${sshCondaEnv "conda update -y conda"} ) || true
    ${sshCondaEnv "conda update -y --all"}
    ${sshCondaEnv "conda create -y -n ${condaEnv}"}
    ${sshCondaEnv "conda install -y -n ${condaEnv} ${condaDepSpecs}"}
    ${ssh "shutdown /p /f"}

    echo "Waiting for qemu exit"
    wait
    EOF
    chmod a+x $out/bin/*.sh
  '';
}
