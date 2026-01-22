# rust-toast

クロスプラットフォーム対応のトースト通知ツール / ライブラリ

Rust の学習プロジェクトとして、トレイト、Builder パターン、エラーハンドリング、条件付きコンパイルなどの概念を実践的に学べます。

## 特徴

- **クロスプラットフォーム対応**: Linux, WSL, Windows, macOS で動作
- **CLI & ライブラリ**: コマンドラインツールとしても、Rust ライブラリとしても使用可能
- **Builder パターン**: 流暢な API で通知を構築
- **学習教材**: 豊富な日本語コメントで Rust の概念を解説

## 対応プラットフォーム

| OS | バックエンド | 説明 |
|----|-------------|------|
| Linux | D-Bus (notify-rust) | デスクトップ通知 |
| WSL | PowerShell | Windows 側に通知を送信 |
| Windows | PowerShell | バルーン通知 |
| macOS | osascript | 通知センターに送信 |

## インストール

### ソースからビルド

```bash
git clone https://github.com/your-username/rust-toast.git
cd rust-toast
cargo build --release
```

ビルドされたバイナリは `target/release/rust-toast` に生成されます。

### Cargo でインストール（ローカル）

```bash
cargo install --path .
```

## 使用方法

### CLI として使用

```bash
# 基本的な使い方
rust-toast -m "Hello, World!"

# タイトル付き
rust-toast -t "通知タイトル" -m "通知メッセージ"

# タイムアウト指定（ミリ秒）
rust-toast -t "タイトル" -m "メッセージ" -T 3000

# 緊急度を指定
rust-toast -m "重要なお知らせ" --urgency critical

# バックエンドを強制指定
rust-toast -m "Windows通知" --backend windows
```

### CLI オプション一覧

| オプション | 短縮 | デフォルト | 説明 |
|-----------|------|-----------|------|
| `--title` | `-t` | "Notification" | 通知のタイトル |
| `--message` | `-m` | (必須) | 通知のメッセージ |
| `--timeout` | `-T` | 5000 | 表示時間（ミリ秒、0=無制限） |
| `--icon` | `-i` | "dialog-information" | アイコン名/パス（Linux） |
| `--urgency` | `-u` | normal | 緊急度（low/normal/critical） |
| `--subtitle` | `-s` | "" | サブタイトル（macOS） |
| `--sound` | | "default" | 通知音（macOS） |
| `--backend` | | (自動検出) | 強制バックエンド（linux/windows/macos） |

### ライブラリとして使用

`Cargo.toml` に追加:

```toml
[dependencies]
rust-toast = { path = "path/to/rust-toast" }
```

コード例:

```rust
use rust_toast::NotificationBuilder;

fn main() -> rust_toast::Result<()> {
    // Builder パターンで通知を構築して送信
    NotificationBuilder::new()
        .title("Hello")
        .message("World!")
        .timeout(5000)
        .send()?;

    Ok(())
}
```

プラットフォームを指定する場合:

```rust
use rust_toast::{NotificationBuilder, Platform};

fn main() -> rust_toast::Result<()> {
    NotificationBuilder::new()
        .title("macOS通知")
        .message("osascript経由で送信")
        .backend(Platform::MacOs)
        .send()?;

    Ok(())
}
```

## アーキテクチャ

```
                     ┌──────────┐
                     │  main.rs │  薄いエントリーポイント
                     └────┬─────┘
                          │ Args::parse() → into_builder() → send()
                          ▼
┌──────────┐      ┌──────────────────┐
│  cli.rs  │──────│ NotificationBuilder │  Builder パターン
└──────────┘      └────────┬─────────┘
                           │ send()
                           ▼
                  ┌─────────────────┐
                  │ select_notifier │  プラットフォーム選択
                  └────────┬────────┘
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
   ┌──────────┐     ┌───────────┐     ┌──────────┐
   │LinuxNotifier│  │WindowsNotifier│  │MacOsNotifier│
   │ (D-Bus)     │  │ (PowerShell) │  │ (osascript) │
   └──────────┘     └───────────┘     └──────────┘
```

## 学習できる Rust の概念

このプロジェクトでは以下の Rust の概念を実践的に学べます:

- **モジュールシステム**: `mod`, `pub`, `use` によるコード整理
- **トレイト**: `Notifier` トレイトによる抽象化と動的ディスパッチ
- **Builder パターン**: `NotificationBuilder` によるオブジェクト構築
- **カスタムエラー型**: `NotificationError` と `Result` 型エイリアス
- **From トレイト**: エラー変換と `?` 演算子との統合
- **条件付きコンパイル**: `#[cfg]` によるプラットフォーム分岐
- **外部コマンド実行**: `std::process::Command` の使用
- **clap の derive API**: 宣言的な CLI 引数定義

## 開発

### コマンド

```bash
# ビルド
cargo build

# テスト実行
cargo test

# Lint チェック
cargo clippy

# フォーマット
cargo fmt

# ドキュメント生成
cargo doc --open
```

### ディレクトリ構成

```
rust-toast/
├── Cargo.toml           # パッケージ定義
├── README.md            # このファイル
└── src/
    ├── main.rs          # CLI エントリーポイント
    ├── lib.rs           # ライブラリルート
    ├── cli.rs           # CLI 引数定義
    ├── error.rs         # エラー型定義
    ├── platform.rs      # プラットフォーム検出
    └── notifier/
        ├── mod.rs       # Notifier トレイト・Builder
        ├── linux.rs     # Linux バックエンド
        ├── windows.rs   # Windows/WSL バックエンド
        └── macos.rs     # macOS バックエンド
```

## 依存クレート

- [clap](https://crates.io/crates/clap) 4.5 - CLI 引数パース（derive API）
- [notify-rust](https://crates.io/crates/notify-rust) 4.11 - Linux D-Bus 通知

## ライセンス

MIT License
