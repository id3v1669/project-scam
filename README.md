# project-scam

# Build instructions:
1) install nix package manager
2) clone project
3) enter command `direnv allow`, to enable automatic load of dev shell in directory
3.1) to manualy enter dev she use command `nix develop`
4) use `cargo build --release` for linux and `cargo build --release --target x86_64-pc-windows-gnu` for windows