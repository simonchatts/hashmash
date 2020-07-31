# `hashmash`: randomize cryptographic hashes and GUIDs

A small tool to process text files, and pick out strings that look like
cryptographic hashes, GUIDs etc:

![A screenshot of a sample with highlighted hashes](https://github.com/simonchatts/hashmash/blob/main/example.png?raw=true)

This isn't achieved just through fixed regular expressions for standard hash
types.  Rather, it uses [trigrams](https://en.wikipedia.org/wiki/Trigram) to
know what English text sort of looks like, and picks out substrings that
wildly deviate from that. (As a result, it might not work so well with very
different languages.)

By default the tool just highlights these, but if you pass in the `-r` argument,
it randomizes all the hashes it finds. It preserves character classes, so for
example:

    W6B43240-ad76s==62231DH00

might get randomized to

    Q1L83073-mn13q==03510AP62

This can be handy for things like constructing documentation without leaking
searchable secrets.

## Install

If you use [nix](https://nixos.org) then just import this repo's
`default.nix`.  (If you use [cachix](https://cachix.org) then `cachix use
simonchatts` might save you building.)

Otherwise, `cargo build --release`.

## Use

By default, whether or not hashes are randomized with `-r`, it highlights
identified hashes in red if stdout is a terminal (and not otherwise).

## Implementation

The `-d` flag might be handy if looking at the implementation - this
highlights in green those substrings that pass the basic pre-filter, but that
aren't categorized as hashes by the actual trigram algorithm.
