{ lib
, rustPlatform
, makeWrapper
, pkg-config
, pkgs
}:
rustPlatform.buildRustPackage rec {

  pname = "project-scam";
  version = "0.0.1";

  src = lib.cleanSource ../.;

  cargoLock.lockFile = "${src}/Cargo.lock";

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    pango
    glib
    gdk-pixbuf
    atkmm
    nerd-fonts.terminess-ttf

    fontconfig
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  postFixup = ''
    patchelf $out/bin/rs-nc \
      --add-rpath ${lib.makeLibraryPath (with pkgs; [ vulkan-loader xorg.libX11 libxkbcommon wayland ])}
  '';
}