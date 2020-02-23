{ pkgs ? import <nixpkgs> {}}:
{
  jobsets = pkgs.runCommand "spec.json" {}
    ''
    cat > $out << EOF
    {
        "web": {
            "enabled": 1,
            "hidden": false,
            "description": "M-Labs website",
            "nixexprinput": "nixScripts",
            "nixexprpath": "web.nix",
            "checkinterval": 300,
            "schedulingshares": 10,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "webSrc": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/web2019.git", "emailresponsible": false }
            }
        }
    }
    EOF
    '';
}
