# cdo: Change directory and do

A dead simple command that lets you run other commands in a different directory.
This saves you the trouble of having to `cd` all over the place.

## Usage

Here's a short example:

```sh
cdo path/to/some/dir my-command arg0 arg1 --flag --other-flag
```

This will run `my-command arg0 arg1 --flag --other-flag` in the
`path/to/some/dir` directory.

## Install

### From source

For this, you'll need rust installed. See <https://rustup.rs/>.

1. Clone this repository

    ```sh
    git clone https://github.com/dotboris/cdo
    cd cdo
    ```

1. Install the binary onto your system

    ```sh
    cargo install --path .
    ```

1. If it's not already done, add the cargo bin directory (`$HOME/.cargo/bin`) to
   your `PATH` environment variable.

    This will vary depending on your operating system shell and system
    configuration.

1. Verify that `cdo` is installed correctly

    ```sh
    cdo --help
    ```

### Nix flake

This uses the [Nix Flakes](https://nixos.wiki/wiki/Flakes) system. You'll need
to have that enabled.

1. Install `cdo`

    ```sh
    # Install in profile
    nix profile install github:dotboris/cdo
    # Run in an ephemeral shell
    nix shell github:dotboris/cdo
    ```

    There are many options for installing and using package in nix, this is a
    sampling of common methods.

1. Verify that `cdo` is installed correctly

    ```sh
    cdo --help
    ```
