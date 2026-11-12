{
  description = "PhEcT - a tiny minimalist widget showing live hc terminals";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;

      nativeLibs = pkgs: with pkgs; [
        fontconfig
        freetype
        libGL
        libxkbcommon
        wayland
        libxcb
        libx11
        libxcursor
        libxi
        libxrandr
      ];
    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          desktopItem = pkgs.makeDesktopItem {
            name = "phect";
            desktopName = "PhEcT";
            comment = "Live hc terminals at a glance";
            exec = "phect";
            icon = "utilities-system-monitor";
            categories = [ "Utility" "Monitor" ];
            terminal = false;
          };
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "phect";
            version = "0.1.0";

            # Keep the flake source limited to what the build needs.
            src = pkgs.lib.sourceByRegex ./. [
              "^src(/.*)?$"
              "^ui(/.*)?$"
              "^Cargo\\.toml$"
              "^Cargo\\.lock$"
              "^build\\.rs$"
            ];

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [ pkg-config makeWrapper ];
            buildInputs = nativeLibs pkgs;

            postInstall = ''
              install -Dm644 ${desktopItem}/share/applications/phect.desktop \
                $out/share/applications/phect.desktop
            '';

            # Winit dlopens wayland/xkbcommon at runtime; point the wrapped binary at them.
            postFixup = ''
              wrapProgram $out/bin/phect \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath (nativeLibs pkgs)}
            '';

            meta = with pkgs.lib; {
              description = "PhEcT - tiny minimalist always-on-top widget showing live hc terminals";
              mainProgram = "phect";
              platforms = platforms.linux;
              desktopFile = "${desktopItem}/share/applications/phect.desktop";
            };
          };
        });

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustc
              cargo
              rustfmt
              clippy
              rust-analyzer
              pkg-config
            ];

            buildInputs = nativeLibs pkgs;

            # Winit dlopens wayland/xkbcommon at runtime; Nix doesn't put them on the search path.
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (nativeLibs pkgs);

            shellHook = ''
              echo "PhEcT dev shell — cargo build / cargo run"
            '';
          };
        });
    };
}
