{pkgs, fenix}:
pkgs.mkShell {
  name = "prc devShell";
  buildInputs = with pkgs; let 
  mixTargets = [
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
    "x86_64-pc-windows-msvc"
    "x86_64-pc-windows-gnu"
    "aarch64-pc-windows-msvc"
    "aarch64-pc-windows-gnu"
    "x86_64-pc-windows-gnu"
  ];
  fenixPkgs = fenix.packages.${pkgs.system};
  rust-toolchain = with fenixPkgs.stable; fenixPkgs.combine [
    cargo
    rustc
    scdoc
    rust-analyzer
    rustfmt
    clippy
    cargo-audit
    cargo-deny
    clippy
    rust-analyzer
    rustfmt
    (pkgs.lib.forEach mixTargets (target: fenixPkgs.targets."${target}".stable.rust-std))
  ];
  in [
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
    rust-toolchain
    # cargo
    # rustc
    # scdoc

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
    
  ];

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (with pkgs; [
    vulkan-loader
    xorg.libX11
    libxkbcommon
    wayland
    wayland-protocols
  ])}";
}