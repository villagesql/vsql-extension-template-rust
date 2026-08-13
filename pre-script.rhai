// This repo carries its own .github/ (the CLA workflow, which applies to
// contributions to the template itself and must not travel into a generated
// project). cargo-generate copies it into the working tree before hooks run,
// so `rename` would fail with "Directory not empty" — delete it first, then
// promote the scaffold's _github/ into its place.
file::delete(".github");
file::rename("_github", ".github");
