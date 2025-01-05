# SteamGridDB DL

## Using this tool for dummies

_To be written. Will link to its own document._

## Installation

If you have Cargo installed, all you need to do is run
```bash
cargo install steamgriddb-dl
```
After which the tool can be run using the following command in your shell:

```
steamgriddb-dl <id> <options>
```

Alternatively, the binary can be downloaded from the [releases](https://github.com/TheRealGLH/SteamGridDB-DL/releases/latest) page, after which you can easily run it from the downloaded directory, or after adding it to a folder in your $PATH.

## Usage

```bash
steamgriddb-dl <id> <options>
```

Where ``<id>`` is the id-number for a collection on SteamGridDB (ie. ``https://www.steamgriddb.com/collection/[number]``). This does not parse the full URL, so only the number will work.

| Option| Purpose      |
| ------------- | ------------- |
| ``-n`` | Dry run. Runs the tool as normal— printing all of the files that are to be saved and their directories, but without actually saving them. |
| ``--directory=<dir>`` | In some cases the tool cannot find your Steam installation's personal configuration directory, or you may want to save the images in a collection elsewhere. This option lets the user set an override directory.  |
|``-h`` | Prints help information.|

## Build

At the moment, no special actions have to be taken to build the binaries. If you have Cargo installed, you can simply use
```bash
cargo build
```
or for release binaries
```bash
cargo build -r
```
...to build the executable yourself.
See also the [Rust documentation](https://doc.rust-lang.org/cargo/commands/build-commands.html).
