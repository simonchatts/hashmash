# `hashmash`: randomize cryptographic hashes and GUIDs

A small tool to process text files, and pick out strings that look like
cryptographic hashes, GUIDs etc:

![A screenshot of a sample with highlighted hashes](https://github.com/simonchatts/hashmash/blob/main/example.png?raw=true)

This isn't based on regular expressions or other fixed notions of what a hash
is, in terms of length or character set. Rather, it uses
[trigrams](https://en.wikipedia.org/wiki/Trigram) to know what English
language sort of looks like, and picks out substrings that definitely aren't
that. (As a result, it might not work so well with very different languages.)

By default the tool just highlights these, but if you pass in the `-r` argument,
it randomizes all the hashes it finds. It preserves character classes, so eg
`123-abc-DEF` might get transformed to `943-qpz-ANQ`.

## Install

If you use [nix](https://nixos.org) then just `import "${src/default.nix} {}`
where `src` uses `pkgs.fetchFromGitHub` in the usual way. (You can also
`cachix use simonchatts` to avoid building.)

Otherwise, `cargo build --release`.

## Use

By default, whether or not hashes are randomized with `-r`, it highlights
identified hashes in red if stdout is a terminal (and not otherwise).

## Implementation

The `-d` flag might be handy if looking at the implementation - this
highlights in green those substrings that pass the basic pre-filter, but that
aren't categorized as hashes by the actual trigram algorithm.
