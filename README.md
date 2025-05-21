# project-scam

# Build instructions:
1) install nix package manager
2) clone project
3) enter command `direnv allow`, to enable automatic load of dev shell in directory
3.1) to manualy enter dev she use command `nix develop`
4) use `cargo build --release` for linux and `cargo build --release --target x86_64-pc-windows-gnu` for windows

# Adding quiz questions:
1) Go to `src/objects/game_data.rs`
2) edit value with comment `//quiz ammount` to add or reduce ammount of quizes
3) add or edit values in variable `DYNAMIC_OBJECTS_QUIZ` under one of `//quizX` comments
4) follow structure:

|-|-|
|---|---|
|button option 1| |
|button option 2| |
|button option 3| |
|button option 4| |
|correct option numbers| |
|Leave empty string, tech value| |
|Question| |
|Answer| |