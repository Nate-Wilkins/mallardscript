function build() {
  cargo build --all-targets --release

  # Generate icon.
  # TODO: Create an icon.
  # /usr/bin/convert -background none -density 1200 -resize 128x128 ./icon.svg ./target/release/icon_128.png

  # Generate completions.
  # Clean.
  rm -rf './target/release/completions/'
  mkdir -p './target/release/completions/'
  # Generate zsh.
  './target/release/mallardscript' completions --type zsh > './target/release/completions/_bookit.zsh'
}

function start() {
  cargo run
}

function test() {
  cargo test
}

function publish() {
  cargo login
  cargo publish
}
