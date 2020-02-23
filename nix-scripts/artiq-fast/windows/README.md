# Preparation steps

## Install a Windows image

```shell
nix-build install.nix -I artiqSrc=…/artiq
result/bin/windows-installer.sh
```

Follow the instructions.

## Install Anaconda to the image

```shell
result/bin/anaconda-installer.sh
```

Move the image `c.img` to one of Nix' `extra-sandbox-paths` (`nix.sandboxPaths` on NixOS).


# Running the tests manually

```shell
nix-build --pure --arg diskImage "\"…/c.img\"" -I artiqSrc=…/artiq manual-test-run.nix
```
