{ pkgs ? import <nixpkgs> {}}:

let
  artiqpkgs = import ./default.nix { inherit pkgs; };
  vivado = import ./vivado.nix { inherit pkgs; };
in
  pkgs.mkShell {
    buildInputs = [
      vivado
      pkgs.gnumake
      (pkgs.python3.withPackages(ps: (with ps; [ jinja2 numpy paramiko ]) ++ (with artiqpkgs; [ migen microscope misoc jesd204b migen-axi artiq ])))
      pkgs.cargo
      artiqpkgs.rustc
      artiqpkgs.binutils-or1k
      artiqpkgs.binutils-arm
      artiqpkgs.llvm-or1k
      artiqpkgs.openocd
    ];
    TARGET_AR="or1k-linux-ar";
  }
