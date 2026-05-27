with import <nixpkgs> { };
let
  micropython-src = fetchgit {
    url = "https://github.com/micropython/micropython.git";
    rev = "v1.28.0"; # e0e9fbb17ed6fd06bb76e266ae554784c9c80804
    hash = "sha256-mox9EbEX7MbNqg0eB1Wg/6bD39yMNts13GPR1BCzOOs=";
    fetchSubmodules = false;
  };
in
mkShell {
  packages = [
    micropython-src
    gcc-arm-embedded
    mpremote
    (python3.withPackages (ps: [
      ps.ar
      ps.pyelftools
    ]))
  ];

  shellHook = ''
    export MPY_DIR=${micropython-src}
    arm-none-eabi-gcc --version
    echo "build with: nix-shell --run make"
  '';
}
