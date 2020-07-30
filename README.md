# `hashmash`: randomize cryptographic hashes and GUIDs

A small tool to process text files, and pick out strings that look like
cryptographic hashes, eg the highlighted bits below:

```
    "nixpkgs": {
        "branch": "nixpkgs-unstable",
        "description": "Snapshotted on 18 June 2020",
        "homepage": "https://github.com/NixOS/nixpkgs",
        "rev": "9d0c3ffe6783d59b427d018e8341e0084737fde9",
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        "sha256": "1wlkw8jw63vg1xa5hx63kshag71kl81ncdzfaxi3g1mq376m4bb0",
                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        "url_template": "https://github.com/<owner>/<repo>/archive/<rev>.tar.gz"
    }
```

By default the tool just highlights these, but if you pass in the `-r` argument,
it randomizes all the hashes it finds. It preserves character classes, so eg
`123-abc-DEF` might get transformed to `943-qpz-ANQ`.

## Install

If you use [nix](https://nixos.org) then just use the `default.nix`.

Otherwise, `cargo build --release`.

## Use

By default, whether or not hashes are randomized with `-r`, it highlights
identified hashes in red if stdout is a terminal (and not otherwise).

## Implementation

The algorithm is a simple use of English letter
[trigrams](https://en.wikipedia.org/wiki/Trigram), so may not work so well for
other languages. The `-d` flag displays a basic pre-classification that may be
handy if you're looking further at this.
