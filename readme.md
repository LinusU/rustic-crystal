# Rustic Crystal

Rustic Crystal is a project to re-impliment the classic GameBoy Color game Pokemon Crystal in Rust. The goal of this project is to create a playable version of the game that runs on modern computers, while also serving as a learning experience for those interested game development and emulation.

## Getting Started

To get started with Rustic Crystal, you will need to have Rust installed on your computer. You can download Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can clone the Rustic Crystal repository:

```sh
git clone https://github.com/LinusU/rustic-crystal.git
cd rustic-crystal
```

## ROM Files

Rustic Crystal requires a copy of the original Pokemon Crystal ROM to run. The ROM file must have the following SHA-1 hash:

- `pokecrystal11.gbc` - `f2f52230b536214ef7c9924f483392993e226cfb`

You can obtain a ROM file from various sources online, but please note that it may be illegal to download and use ROMs in some jurisdictions.

## Music

Rustic Crystal requires music in FLAC format from the "Pokémon Gold & Pokémon Silver: Super Music Collection" album. You can download the music from the following link:

https://archive.org/details/pkmn-gsc-soundtrack

Copy all of the FLAC files from the album, both from `Disc 1` and `Disc 2 (Crystal)`, into a directory named `music` in the project root.

## Running the Game

Use the following command to build and run the game:

```sh
cargo run --release
```

## Contributing

Contributions to Rustic Crystal are welcome! Feel free to open an issue if you have any questions or suggestions. If you want to contribute code, it's probably a good idea to open an issue first to discuss the change you want to make, since it's still early days for this project.

## Acknowledgements

- [Mathijs van de Nes](https://github.com/mvdnes) for their work on [mvdnes/rboy](https://github.com/mvdnes/rboy), from which this project was bootstrapped
- [The pret team](https://github.com/orgs/pret/people) for their work on [pret/pokecrystal](https://github.com/pret/pokecrystal), which was used as a reference for the disassembly of the original Pokemon Crystal ROM
- The original developers of Pokemon Crystal, for creating such a wonderful game

## License

Rustic Crystal is licensed under the MIT License. See the LICENSE file for details.
