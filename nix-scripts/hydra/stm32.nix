{ pkgs ? import <nixpkgs> {}}:
{
  jobsets = pkgs.runCommand "spec.json" {}
    ''
    cat > $out << EOF
    {
        "stm32": {
            "enabled": 1,
            "hidden": false,
            "description": "STM32 firmware",
            "nixexprinput": "nixScripts",
            "nixexprpath": "stm32.nix",
            "checkinterval": 300,
            "schedulingshares": 10,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "mozillaOverlay": { "type": "git", "value": "git://github.com/mozilla/nixpkgs-mozilla.git", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "stabilizerSrc": { "type": "git", "value": "git://github.com/quartiq/stabilizer.git", "emailresponsible": false },
                "thermostatSrc": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/thermostat.git", "emailresponsible": false }
            }
        }
    }
    EOF
    '';
}
