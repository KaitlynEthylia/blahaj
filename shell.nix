{
  diesel-cli,
  clippy,
  rustfmt,
  cargo-shear,
  callPackage,
  rust-analyzer,
}:
let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
    # Additional rust tooling
    diesel-cli
    clippy
    rustfmt
    rust-analyzer
    cargo-shear
  ] ++ (oa.nativeBuildInputs or [ ]);
})
