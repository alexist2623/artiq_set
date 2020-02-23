{ host }:

{ config, pkgs, ... }:

{
  deployment.targetHost = host;

  imports =
    [
      (./. + "/${host}-hardware-configuration.nix")
    ];

  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  networking.hostName = host;

  time.timeZone = "Asia/Hong_Kong";

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  nixpkgs.config.allowUnfree = true;
  environment.systemPackages = with pkgs; [
    wget vim git firefox thunderbird hexchat usbutils pciutils mplayer vlc youtube-dl file lm_sensors cryptsetup audacious acpi
    gwenview okular gimp imagemagick
    (python3.withPackages(ps: with ps; [ numpy scipy matplotlib qtconsole pypdf2 reportlab pygments regex ]))
    texlive.combined.scheme-full mosh psmisc libreoffice-fresh
    xc3sprog gtkwave xournal xsane telnet whois transmission-gtk unzip zip inkscape tigervnc gnupg
    wireshark qrencode yosys symbiyosys yices z3 boolector cvc4 pavucontrol keepassx poppler_utils
    jq ark sublime3 rink qemu_kvm konsole
  ];
  programs.wireshark.enable = true;

  services.openssh.enable = true;
  services.openssh.forwardX11 = true;

  # Enable CUPS to print documents.
  services.printing = {
      enable = true;
  };
  services.avahi = {
      enable = true;
      nssmdns = true;
  };

  # Enable sound.
  sound.enable = true;
  hardware.pulseaudio = {
    enable = true;
    extraModules = [ pkgs.pulseaudio-modules-bt ];
    package = pkgs.pulseaudioFull;
  };

  i18n.inputMethod = {
    enabled = "fcitx";
    fcitx.engines = with pkgs.fcitx-engines; [ table-extra m17n ];
  };
  fonts.fonts = [ pkgs.noto-fonts pkgs.noto-fonts-cjk pkgs.noto-fonts-emoji pkgs.noto-fonts-extra ];

  # Enable the X11 windowing system.
  services.xserver.enable = true;
  services.xserver.layout = "us";
  services.xserver.xkbOptions = "eurosign:e";

  # Enable touchpad support.
  services.xserver.libinput.enable = true;

  # Enable the KDE Desktop Environment.
  services.xserver.displayManager.sddm.enable = true;
  services.xserver.displayManager.sddm.autoLogin.enable = true;
  services.xserver.displayManager.sddm.autoLogin.user = "harry";
  services.xserver.desktopManager.plasma5.enable = true;

  hardware.bluetooth.enable = true;

  programs.fish.enable = true;
  users.defaultUserShell = pkgs.fish;
  users.extraGroups.plugdev = { };
  users.extraUsers.sb = {
    isNormalUser = true;
    extraGroups = ["wheel" "plugdev" "dialout"];
  };
  users.extraUsers.harry = {
    isNormalUser = true;
    extraGroups = ["plugdev" "dialout" "wireshark"];
  };
  security.sudo.wheelNeedsPassword = false;
  services.udev.packages = [ pkgs.openocd pkgs.hackrf ];
  services.udev.extraRules = ''
ATTRS{idProduct}=="0003", ATTRS{idVendor}=="1eaf", MODE="664", GROUP="plugdev" SYMLINK+="maple"
ATTRS{idProduct}=="0004", ATTRS{idVendor}=="1eaf", MODE="664", GROUP="plugdev" SYMLINK+="maple"
  '';

  nix.binaryCachePublicKeys = ["nixbld.m-labs.hk-1:5aSRVA5b320xbNvu30tqxVPXpld73bhtOeH6uAjRyHc="];
  nix.binaryCaches = ["https://nixbld.m-labs.hk" "https://cache.nixos.org"];
  nix.sandboxPaths = ["/opt"];

  # This value determines the NixOS release with which your system is to be
  # compatible, in order to avoid breaking some software such as database
  # servers. You should change this only after NixOS release notes say you
  # should.
  system.stateVersion = "19.03"; # Did you read the comment?
}
