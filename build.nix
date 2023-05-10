{ pkgs, rustPlatform, makeRustPlatform, nightly-rust }: let
    platform = makeRustPlatform {
        cargo = nightly-rust;
        rustc = nightly-rust;
    };
in platform.buildRustPackage rec {
    name = "imgur_link_generator";
    cargoLock.lockFile = ./Cargo.lock;

    src = ./.;

    nativeBuildInputs = with pkgs; [
        pkg-config
    ];


    buildInputs = with pkgs; [
        openssl
    ];
}
