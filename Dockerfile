# Rustの公式イメージを使用
FROM rust:latest as builder

# ワークディレクトリを設定
WORKDIR /usr/src/app

# Cargo.tomlをコピー 
COPY backend/Cargo.toml ./

# Cargo.lockが存在する場合のみコピー
COPY backend/Cargo.lock ./ 

# ソースコードをコピー
COPY backend/src ./src

# 依存関係をビルド
RUN cargo install cargo-watch
RUN cargo build --release

# リリースビルドを作成
RUN cargo build --release

# 実行用のイメージを作成 (軽量化のため)
# FROM debian:buster-slim # コメントアウトまたは削除

# 実行ファイルをコピー
# COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend # コメントアウトまたは削除

# 実行
CMD ["cargo", "watch", "-x", "run"]