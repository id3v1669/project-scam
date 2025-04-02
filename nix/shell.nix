{pkgs}:
pkgs.mkShell {
  name = "prc devShell";
  nativeBuildInputs = with pkgs; [
    nerd-fonts.terminess-ttf
      nerd-fonts.jetbrains-mono
      noto-fonts
      noto-fonts-cjk-sans
      noto-fonts-emoji
      liberation_ttf
      fira-code
      _0xproto
      fira-code-symbols
      proggyfonts


    # Compilers
    cargo
    rustc
    scdoc

    # build Deps
    pkg-config
    pango
    glib
    gdk-pixbuf
    atkmm

    libxkbcommon

    # other
    wayland
    wayland-protocols
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    

    # Tools
    cargo-audit
    cargo-deny
    clippy
    rust-analyzer
    rustfmt
  ];

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (with pkgs; [
    vulkan-loader
    xorg.libX11
    libxkbcommon
    wayland
    wayland-protocols
  ])}";
}