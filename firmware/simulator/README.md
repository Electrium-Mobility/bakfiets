### Setup
cd simulator
brew install sdl2
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
cargo run --example hello-world