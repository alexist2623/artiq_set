{ pkgs ? import <nixpkgs> {}}:
{
  jobsets = pkgs.runCommand "spec.json" {}
    ''
    cat > $out << EOF
    {
        "fast-beta": {
            "enabled": 1,
            "hidden": false,
            "description": "Core ARTIQ packages to build fast for CI purposes (beta version)",
            "nixexprinput": "nixScripts",
            "nixexprpath": "artiq-fast.nix",
            "checkinterval": 300,
            "schedulingshares": 10,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "artiqSrc": { "type": "git", "value": "git://github.com/m-labs/artiq.git master 1", "emailresponsible": false }
            }
        },
        "full-beta": {
            "enabled": 1,
            "hidden": false,
            "description": "Full set of ARTIQ packages (beta version)",
            "nixexprinput": "nixScripts",
            "nixexprpath": "artiq-full.nix",
            "checkinterval": 86400,
            "schedulingshares": 1,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "sinaraSystemsSrc": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/sinara-systems.git master 1", "emailresponsible": false },
                "artiq-fast": { "type": "sysbuild", "value": "artiq:fast-beta:generated-nix", "emailresponsible": false }
            }
        },
        "fast": {
            "enabled": 1,
            "hidden": false,
            "description": "Core ARTIQ packages to build fast for CI purposes",
            "nixexprinput": "nixScripts",
            "nixexprpath": "artiq-fast.nix",
            "checkinterval": 300,
            "schedulingshares": 10,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "artiqSrc": { "type": "git", "value": "git://github.com/m-labs/artiq.git release-5 1", "emailresponsible": false }
            }
        },
        "full": {
            "enabled": 1,
            "hidden": false,
            "description": "Full set of ARTIQ packages",
            "nixexprinput": "nixScripts",
            "nixexprpath": "artiq-full.nix",
            "checkinterval": 86400,
            "schedulingshares": 1,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "sinaraSystemsSrc": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/sinara-systems.git master 1", "emailresponsible": false },
                "artiq-fast": { "type": "sysbuild", "value": "artiq:fast:generated-nix", "emailresponsible": false }
            }
        },
        "cpld": {
            "enabled": 1,
            "hidden": false,
            "description": "CPLD gateware",
            "nixexprinput": "nixScripts",
            "nixexprpath": "cpld.nix",
            "checkinterval": 172800,
            "schedulingshares": 1,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "urukulSrc": { "type": "git", "value": "git://github.com/quartiq/urukul", "emailresponsible": false },
                "mirnySrc": { "type": "git", "value": "git://github.com/quartiq/mirny", "emailresponsible": false }
            }
        },
        "zc706": {
            "enabled": 1,
            "hidden": false,
            "description": "Rust on ZYNQ",
            "nixexprinput": "nixScripts",
            "nixexprpath": "zc706.nix",
            "checkinterval": 300,
            "schedulingshares": 1,
            "enableemail": false,
            "emailoverride": "",
            "keepnr": 10,
            "inputs": {
                "nixpkgs": { "type": "git", "value": "git://github.com/NixOS/nixpkgs-channels nixos-19.09", "emailresponsible": false },
                "nixScripts": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/nix-scripts.git", "emailresponsible": false },
                "zc706": { "type": "git", "value": "https://git.m-labs.hk/M-Labs/zc706.git", "emailresponsible": false },
                "mozillaOverlay": { "type": "git", "value": "git://github.com/mozilla/nixpkgs-mozilla", "emailresponsible": false }
            }
        }
    }
    EOF
    '';
}
