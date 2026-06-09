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

  cargoHash = "sha256-ROY2KvBXUOUqI3EhKH4l9hX2gBOk471SAn4c/b10ABo=";

  nativeBuildInputs = [
    openssl.dev
    pkg-config
  ];

  buildInputs = [
  
  ];

  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig"; 

  doInstallCheck = true;
}
