{ pkgs,
  diskImage,
  qemuMem,
  sshUser ? "user",
  sshPassword ? "user",
}:

with pkgs;

let
  qemu-img = "${qemu_kvm}/bin/qemu-img";
  runQemu = isolateNetwork: forwardedPorts: extraArgs:
    let
      restrict =
        if isolateNetwork
        then "on"
        else "off";
      # use socat instead of `tcp:…` to allow multiple connections
      guestfwds =
        builtins.concatStringsSep ""
        (map ({ listenAddr, targetAddr, port }:
          ",guestfwd=tcp:${listenAddr}:${toString port}-cmd:${socat}/bin/socat\\ -\\ tcp:${targetAddr}:${toString port}"
        ) forwardedPorts);
      args = [
        "-enable-kvm"
        "-m" qemuMem
        "-bios" "${OVMF.fd}/FV/OVMF.fd"
        "-netdev" "user,id=n1,net=192.168.1.0/24,restrict=${restrict},hostfwd=tcp::2022-:22${guestfwds}"
        "-device" "e1000,netdev=n1"
      ];
      argStr = builtins.concatStringsSep " " (args ++ extraArgs);
    in "${qemu_kvm}/bin/qemu-system-x86_64 ${argStr}";

  # Pass empty config file to prevent ssh from failing to create ~/.ssh
  sshOpts = "-F /dev/null -o StrictHostKeyChecking=accept-new -o UserKnownHostsFile=\$TMP/known_hosts";
  sshWithQuotes = quotes: cmd: ''
    echo ssh windows ${quotes}${cmd}${quotes}
    ${sshpass}/bin/sshpass -p${sshPassword} -- \
      ${openssh}/bin/ssh  -np 2022 ${sshOpts} \
      ${sshUser}@localhost \
      ${quotes}${cmd}${quotes}
  '';
  ssh = sshWithQuotes "'";
  scp = src: target: ''
    echo "Copy ${src} to ${target}"
    ${sshpass}/bin/sshpass -p${sshPassword} -- \
      ${openssh}/bin/scp -P 2022 ${sshOpts} \
      "${src}" "${sshUser}@localhost:${target}"
  '';
  
in
{
  inherit qemu-img runQemu ssh sshWithQuotes scp;
  inputs = [ qemu_kvm openssh sshpass ];
}
