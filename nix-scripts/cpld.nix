{ pkgs ? import <nixpkgs> {} }:

let
  jobs = import ./cpld/default.nix { inherit pkgs; };
in
  builtins.mapAttrs (key: value: pkgs.lib.hydraJob value) jobs
