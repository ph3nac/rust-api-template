# Rust APIテンプレート - 開発環境構築計画

## 📋 プロジェクト概要

このプロジェクトは以下の技術スタックで構成されています：

- **フレームワーク**: Axum (非同期Rust Webフレームワーク)
- **データベース**: PostgreSQL
- **ORM**: SQLx
- **コンテナ化**: Docker + Docker Compose
- **その他**: 依存性注入パターン、単体テスト、モッキング

## 🏗️ 開発環境構築手順

### 1. 前提条件

- **Rustツールチェーン**
  - rustup: Rustのバージョン管理ツール
  - cargo: Rustのパッケージマネージャー
- **Docker + Docker Compose**: データベース環境のコンテナ化
- **Git**: バージョン管理
- **SQLxツール**: マイグレーション管理

### 2. 詳細なセットアップ手順

#### 2.1 Rustツールチェーンのインストール

```bash
# Rustupのインストール (未インストールの場合)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Rust最新版に更新
rustup update

# SQLxのCLIツールをインストール
cargo install sqlx-cli
```

#### 2.2 環境変数の設定

既存の`.env`ファイルを以下のように更新します：

```
DATABASE_URL=postgres://postgres:password@localhost:5432/rust_api_template
DATABASE_USER=postgres
DATABASE_PASSWORD=password
DATABASE_NAME=rust_api_template
DATABASE_PORT=5432
SERVER_ADDRESS=127.0.0.1:3000
```

#### 2.3 データベースの起動

Docker Composeを使用してPostgreSQLコンテナを起動します：

```bash
# Dockerコンテナを起動
docker-compose up -d
```

#### 2.4 データベースマイグレーションの作成と実行

SQLxのマイグレーション機能を使ってデータベースのスキーマを作成します：

https://dottrail.codemountains.org/rust-sqlx-cli-migration/

#### 2.5 アプリケーションのビルドと実行

```bash
# アプリケーションのビルド
cargo build

# アプリケーションの実行
cargo run

# または開発モードでホットリロード機能を使用する場合
# cargo-watchをインストール
cargo install cargo-watch

# ホットリロードでアプリケーションを実行
cargo watch -x run
```

## 🔍 開発環境の検証方法

環境構築が正常に完了したことを確認するために、以下の検証を行います：

1. **データベース接続の確認**

```bash
# PostgreSQLコンテナへの接続確認
docker exec -it db psql -U postgres -d rust_api_template -c "SELECT 'Connection successful';"
```

2. **APIエンドポイントの確認**

アプリケーション起動後、以下のcurlコマンドでAPIの動作を確認します：

```bash
# ユーザー登録
curl -X POST http://127.0.0.1:3000/users -H "Content-Type: application/json" -d '{"name":"テストユーザー"}'

# 登録したユーザーのIDを使用して、ユーザー情報を取得
curl -X GET http://127.0.0.1:3000/users/{上記で返されたユーザーID}
```

## 📝 補足事項

1. **テストの実行方法**

```bash
# すべてのテストを実行
cargo test

# 特定のテストを実行
cargo test -- --nocapture test_name
```

2. **トラブルシューティング**

- データベース接続エラーが発生した場合：
  - `.env`ファイルの設定値を確認
  - PostgreSQLコンテナが実行中かを確認 (`docker ps`)
  - ネットワークポートが競合していないか確認

- コンパイルエラーが発生した場合：
  - `cargo check`コマンドでエラーの詳細を確認

3. **APIのテスト**
# ユーザー登録
```sh
curl -X POST http://127.0.0.1:3000/users -H "Content-Type: application/json" -d '{"name":"テストユーザー"}'
```

# 登録したユーザーのIDを使用して、ユーザー情報を取得
```sh
curl -X GET http://127.0.0.1:3000/users/{上記で返されたユーザーID}
```
