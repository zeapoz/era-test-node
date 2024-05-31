{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }: let
    supportedSystems = ["x86_64-linux"];
  in
    flake-utils.lib.eachSystem supportedSystems (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      rust-toolchain =
        pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in {
      devShell = with pkgs;
        mkShell {
          buildInputs = [
            rust-toolchain
            rust-analyzer
            cargo-sort
            openssl
            pkg-config
            clang
            protobuf
          ];

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        };
    });
}
