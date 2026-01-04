{
  description = "Lazarus - Lightweight PKM for everyone";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    
    flake-utils.url = "github:numtide/flake-utils";
    
    # Rust 빌드를 위한 naersk (crane보다 단순)
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # 크로스 컴파일 타겟 정의
        crossTargets = {
          "aarch64-linux" = {
            rustTarget = "aarch64-unknown-linux-gnu";
            pkgsCross = pkgs.pkgsCross.aarch64-multiplatform;
          };
          "armv7l-linux" = {
            rustTarget = "armv7-unknown-linux-gnueabihf";
            pkgsCross = pkgs.pkgsCross.armv7l-hf-multiplatform;
          };
        };

        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust 툴체인 설정
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ 
            "aarch64-unknown-linux-gnu"
            "armv7-unknown-linux-gnueabihf"
          ];
        };

        # naersk 빌드러 설정
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        # 공통 빌드 입력
        commonBuildInputs = with pkgs; [
          zstd
          openssl
        ];

        commonNativeBuildInputs = with pkgs; [
          pkg-config
        ];

        # 메인 패키지 빌드
        lazarus = naersk-lib.buildPackage {
          pname = "lazarus";
          version = "0.1.0";
          src = ./.;
          
          buildInputs = commonBuildInputs;
          nativeBuildInputs = commonNativeBuildInputs;

          # 릴리스 최적화
          release = true;
          
          # 환경 변수
          ZSTD_SYS_USE_PKG_CONFIG = "1";
        };

      in {
        # 기본 패키지
        packages = {
          default = lazarus;
          lazarus = lazarus;
        };

        # 개발 환경
        devShells.default = pkgs.mkShell {
          buildInputs = commonBuildInputs ++ (with pkgs; [
            rustToolchain
            
            # 개발 도구
            cargo-watch      # 파일 변경 시 자동 재빌드
            cargo-edit       # cargo add/rm
            cargo-outdated   # 의존성 업데이트 확인
            cargo-audit      # 보안 취약점 검사
            
            # 디버깅
            lldb
            
            # 크로스 컴파일 도구
            qemu              # ARM 에뮬레이션 테스트용
          ]);

          nativeBuildInputs = commonNativeBuildInputs;

          # 환경 변수
          RUST_BACKTRACE = "1";
          RUST_LOG = "debug";
          ZSTD_SYS_USE_PKG_CONFIG = "1";

          shellHook = ''
            echo "🦀 Lazarus 개발 환경"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "Rust: $(rustc --version)"
            echo ""
            echo "명령어:"
            echo "  cargo build          - 빌드"
            echo "  cargo watch -x run   - 핫 리로드"
            echo "  cargo test           - 테스트"
            echo "  nix build            - 릴리스 빌드"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          '';
        };

        # 크로스 컴파일용 개발 쉘 (Pi용)
        devShells.cross-aarch64 = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkgsCross.aarch64-multiplatform.stdenv.cc
          ];

          nativeBuildInputs = commonNativeBuildInputs;

          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = 
            "${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc";
          
          shellHook = ''
            echo "🎯 크로스 컴파일 환경 (aarch64-linux)"
            echo "빌드: cargo build --release --target aarch64-unknown-linux-gnu"
          '';
        };

        # Pi Zero 2W용
        devShells.cross-armv7 = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkgsCross.armv7l-hf-multiplatform.stdenv.cc
          ];

          nativeBuildInputs = commonNativeBuildInputs;

          CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER = 
            "${pkgs.pkgsCross.armv7l-hf-multiplatform.stdenv.cc}/bin/armv7l-unknown-linux-gnueabihf-gcc";
          
          shellHook = ''
            echo "🎯 크로스 컴파일 환경 (armv7l-linux / Pi Zero 2W)"
            echo "빌드: cargo build --release --target armv7-unknown-linux-gnueabihf"
          '';
        };

        # NixOS 모듈 (시스템 서비스로 설치할 때)
        nixosModules.default = { config, lib, pkgs, ... }: {
          options.services.lazarus = {
            enable = lib.mkEnableOption "Lazarus PKM service";
            
            port = lib.mkOption {
              type = lib.types.port;
              default = 8080;
              description = "HTTP 서버 포트";
            };

            dataDir = lib.mkOption {
              type = lib.types.path;
              default = "/var/lib/lazarus";
              description = "데이터 저장 경로";
            };

            user = lib.mkOption {
              type = lib.types.str;
              default = "lazarus";
              description = "서비스 실행 사용자";
            };
          };

          config = lib.mkIf config.services.lazarus.enable {
            systemd.services.lazarus = {
              description = "Lazarus PKM Server";
              wantedBy = [ "multi-user.target" ];
              after = [ "network.target" ];

              serviceConfig = {
                Type = "simple";
                User = config.services.lazarus.user;
                ExecStart = "${self.packages.${system}.default}/bin/lazarus --port ${toString config.services.lazarus.port} --data ${config.services.lazarus.dataDir}";
                Restart = "on-failure";
                RestartSec = 5;
                
                # 보안 강화
                NoNewPrivileges = true;
                ProtectSystem = "strict";
                ProtectHome = true;
                ReadWritePaths = [ config.services.lazarus.dataDir ];
              };
            };

            users.users.${config.services.lazarus.user} = {
              isSystemUser = true;
              group = config.services.lazarus.user;
              home = config.services.lazarus.dataDir;
              createHome = true;
            };

            users.groups.${config.services.lazarus.user} = {};
          };
        };
      }
    );
}
