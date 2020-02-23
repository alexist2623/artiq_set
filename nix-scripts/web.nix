let
   pkgs = import <nixpkgs> {};
   src = <webSrc>;
in
  {
    web = pkgs.runCommand "web" {} "cd ${src}; ${pkgs.zola}/bin/zola build -o $out";
  }
