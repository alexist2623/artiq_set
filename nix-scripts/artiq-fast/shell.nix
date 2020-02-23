{ pkgs ? import <nixpkgs> {}}:

let 
  artiqpkgs = import ./default.nix { inherit pkgs; };
in
  pkgs.mkShell {
    buildInputs = [ (pkgs.python3.withPackages(ps: [artiqpkgs.artiq])) ];
  }
