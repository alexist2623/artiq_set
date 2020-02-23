{
  rpi-1 = import ./rpi.nix { host = "rpi-1"; rpi4 = false; };
  rpi-2 = import ./rpi.nix { host = "rpi-2"; rpi4 = false; };
  rpi-3 = {
    deployment.nix_path.nixpkgs = "https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz";
    imports = [(import ./rpi.nix { host = "rpi-3"; rpi4 = true; })];
  };
  rpi-4 = {
    deployment.nix_path.nixpkgs = "https://nixos.org/channels/nixos-unstable/nixexprs.tar.xz";
    imports = [(import ./rpi.nix { host = "rpi-4"; rpi4 = true; })];
  };
  juno = import ./desktop.nix { host = "juno"; };
}
