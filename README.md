# Goblin Sheet
Simple web-based character sheet for the fifth edition of the 
world's most prolific fantasy roleplaying game.

Built in Rust with the [Leptos](https://github.com/leptos-rs/leptos)
framework. 5e data is pulled from the [Open5e](https://open5e.com/)
Api, and character data is stored on the client's browser via
Local Storage.

# Building
- Install the [Trunk](https://trunkrs.dev/) build tool, as well
as the [Rust](https://www.rust-lang.org/) compiler itself if not
already installed. 
- Run `trunk serve` to run a dev instance locally (add the `--open`)
option to automatically launch in your browser.