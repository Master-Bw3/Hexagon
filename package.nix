{
  lib,
  stdenv,
  buildPackages,
  rustPlatform,
  openssl,
  pkg-config,
}:

rustPlatform.buildRustPackage {
  pname = "hexagon";
  version = "1.0.0";

  src = lib.cleanSource ./.;

  cargoHash = "sha256-tnfXCIW6JNzv9Hozei3WbYkdig1zg7uUSv1n0I7vk8c=";

  nativeBuildInputs = [
    openssl.dev
    pkg-config
  ];

  buildInputs = [
  
  ];

  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig"; 

  doInstallCheck = true;
}
