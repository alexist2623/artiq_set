{ pkgs ? import <nixpkgs> {}}:
{
  jobsets = pkgs.runCommand "spec.json" {}
    ''
    cat > $out << EOF
    {
        "heavyx": {
            "enabled": 1,
            "hidden": false,
            "description": "HeavyX SoC toolkit experiment",
            "nixexprinput": "heavyx",
            "nixexprpath": "release.nix",
            "checkinterval": 300,
            "schedulingshares": 10,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.03", "emailresponsible": false },
                "heavyx": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/HeavyX.git", "emailresponsible": false }
            }
        }
    }
    EOF
    '';
}
