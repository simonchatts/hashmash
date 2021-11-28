![CI checks](https://github.com/simonchatts/hashmash/workflows/CI%20checks/badge.svg)
![Cachix binaries](https://github.com/simonchatts/hashmash/workflows/Cachix%20binaries/badge.svg)
![Release-candidate binaries](https://github.com/simonchatts/hashmash/workflows/Release-candidate%20binaries/badge.svg)

# `hashmash`: randomize cryptographic hashes and GUIDs

A small tool to process text files, and pick out strings that look like
cryptographic hashes, GUIDs etc:

![A screenshot of a sample with highlighted hashes](https://github.com/simonchatts/hashmash/blob/main/example.png?raw=true)

This isn't achieved just through fixed regular expressions for standard hash
types.  Rather, it uses [trigrams](https://en.wikipedia.org/wiki/Trigram) to
know what English text sort of looks like, and picks out substrings that
deviate wildly from that. (As a result, it might have more false positives with
different languages.)

By default the tool just highlights these, but if you pass in the `--replace`
argument, it randomizes all the hashes it finds - this can be handy for things
like constructing documentation without leaking secrets. By default this just
writes the new file to stdout, but `--in-place` (which implies `--replace`)
does a destructive in-place edit of all the provided files.

The randomization reserves character classes, so for example:

    W6B43240-ad76s==62231DH00

might get randomized to

    Q1L83073-mn13q==03510AP62

## Install

Debian packages, and standalone binaries for common platforms, are available on
the [release page](https://github.com/simonchatts/hashmash/releases).

If you use [Nix](https://nixos.org) then just use this repo's `flake.nix` or
import its `default.nix`; in addition, if you have [Cachix](https://cachix.org)
then `cachix use simonchatts` gives you access to the binaries pre-built by
GitHub (x86_64 linux/macOS).

Otherwise, `cargo build --release`.

## Highlighting

Colour highlighting is done if (and only if) stdout is a terminal.

The `--debug` flag might be handy if looking at the implementation - this
highlights in green those substrings that pass the basic pre-filter, but that
aren't categorized as hashes by the actual trigram algorithm.
