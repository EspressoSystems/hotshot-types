{
  description = "Shared types for HotShot consensus library";

  nixConfig = {
    extra-substituters = [ "https://espresso-systems-private.cachix.org" ];
    extra-trusted-public-keys = [
      "espresso-systems-private.cachix.org-1:LHYk03zKQCeZ4dvg3NctyCq88e44oBZVug5LpYKjPRI="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    crate2nix = {
      url = "github:balsoft/crate2nix/balsoft/fix-broken-ifd";
      flake = false;
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cargo-careful = {
      url = "github:RalfJung/cargo-careful";
      flake = false;
    };
  };

  outputs =
    { self, nixpkgs, flake-compat, utils, crate2nix, fenix, cargo-careful }:
    utils.lib.eachDefaultSystem (system:
    let
      fenixNightly = fenix.packages.${system}.latest.withComponents [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
        "llvm-tools-preview"
      ];
      fenixStable = fenix.packages.${system}.stable.withComponents [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
        "llvm-tools-preview"
      ];

      CARGO_TARGET_DIR = "target_dirs/nix_rustc";

      pkgs = import nixpkgs {
        inherit system;
      };

      heapstack_pkgs = import nixpkgs { inherit system; };

      careful = pkgs.rustPlatform.buildRustPackage {
        pname = "cargo-careful";
        version = "master";

        src = cargo-careful;

        cargoSha256 = "sha256-5H6Dp3YANVGYxvphTdnd92+0h0ddFX7u5SWT24YxzV4=";

        meta = {
          description = "A cargo undefined behaviour checker";
          homepage = "https://github.com/RalfJung/cargo-careful";
        };
      };

      cargo-llvm-cov = pkgs.rustPlatform.buildRustPackage rec {
        pname = "cargo-llvm-cov";
        version = "0.3.0";

        doCheck = false;

        buildInputs = [ pkgs.libllvm ];

        src = builtins.fetchTarball {
          url =
            "https://crates.io/api/v1/crates/${pname}/${version}/download";
          sha256 =
            "sha256:0iswa2cdaf2123vfc42yj9l8jx53k5jm2y51d4xqc1672hi4620l";
        };

        cargoSha256 = "sha256-RzIkW/eytU8ZdZ18x0sGriJ2xvjVW+8hB85In12dXMg=";
        meta = {
          description = "Cargo llvm cov generates code coverage via llvm.";
          homepage = "https://github.com/taiki-e/cargo-llvm-cov";
        };
      };

      # DON'T FORGET TO PUT YOUR PACKAGE NAME HERE, REMOVING `throw`
      crateName = "hotshot-types";

      inherit (import "${crate2nix}/tools.nix" { inherit pkgs; })
        generatedCargoNix;

      project = import
        (generatedCargoNix {
          name = crateName;
          src = ./.;
        })
        {
          inherit pkgs;
          rootFeatures = [ "docs" "blake3" ];
          defaultCrateOverrides = pkgs.defaultCrateOverrides // {
            # Crate dependency overrides go here
            # pass in protobuf
            prost-build = attrs: {
              buildInputs = [ pkgs.protobuf ];
              PROTOC = "${pkgs.protobuf}/bin/protoc";
              PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            };
            libp2p-core = attrs: {
              buildInputs = [ pkgs.protobuf ];
              PROTOC = "${pkgs.protobuf}/bin/protoc";
              PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            };
          };
        };

      buildDeps = with pkgs;
        [
          cargo-vet
          curl.out
          cargo-expand
          cargo-workspaces
          cargo-audit
          nixpkgs-fmt
          git-chglog
          protobuf
          python3
          zlib.dev
          zlib.out
          fenix.packages.${system}.rust-analyzer
          just
          pkg-config
          openssl.dev
          openssl.out
        ] ++ lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.Security
          darwin.apple_sdk.frameworks.CoreServices
          pkgs.libiconv
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];
    in
    {
      devShell = pkgs.mkShell {
        inherit CARGO_TARGET_DIR;
        buildInputs = [ fenixStable ] ++ buildDeps;
      };

      devShells = {
        # usage: check correctness
        correctnessShell = pkgs.mkShell {
          inherit CARGO_TARGET_DIR;
          shellHook = ''
            ulimit -n 1024
          '';
          RUST_SRC_PATH = "${fenixNightly}/lib/rustlib/src/rust/library";
          RUST_LIB_SRC = "${fenixNightly}/lib/rustlib/src/rust/library";
          buildInputs = [ careful pkgs.git fenixNightly pkgs.cargo-udeps ] ++ buildDeps;

        };

        semverShell = pkgs.mkShell {
          inherit CARGO_TARGET_DIR;
          buildInputs = [
            (pkgs.cargo-semver-checks.overrideAttrs (final: prev: { doCheck = false; }))
            fenixStable
          ] ++ buildDeps;
        };

        # usage: evaluate performance (llvm-cov + flamegraph)
        perfShell = pkgs.mkShell {
          inherit CARGO_TARGET_DIR;
          buildInputs = with pkgs;
            [ cargo-flamegraph fd cargo-llvm-cov fenixStable ripgrep ]
            ++ buildDeps ++ lib.optionals stdenv.isLinux [
              heapstack_pkgs.heaptrack
              pkgs.valgrind
            ];
        };

        # usage: brings in debugging tools including:
        # - lldb: a debugger to be used with vscode
        debugShell = pkgs.mkShell {
          inherit CARGO_TARGET_DIR;
          buildInputs = with pkgs; [ fenixStable lldb ] ++ buildDeps;
        };
      };
    });
}
