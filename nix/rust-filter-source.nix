# Filter out the subset of the repo required to build a rust executable

let

  # Just include: Cargo.toml, Cargo.lock, *.1, src/**
  regex = ".*/Cargo\.(lock|toml)|.*\.1|.*/src($|/.*)";

in

 builtins.filterSource (path: _: builtins.match regex path != null)
