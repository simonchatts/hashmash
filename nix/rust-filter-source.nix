# Filter out the subset of the repo required to build a rust executable

let 

  # Just include: Cargo.toml, Cargo.lock, src/**
  regex = ".*/Cargo\.(lock|toml)|.*/src($|/.*)";

in

 builtins.filterSource (path: _: builtins.match regex path != null) 
