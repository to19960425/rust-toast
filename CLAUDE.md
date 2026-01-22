# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

rust-toast は Rust 学習用のクロスプラットフォーム対応トースト通知ツールです。CLI とライブラリの両方として使用できます。

## ビルド・テストコマンド

```bash
# ビルド
cargo build

# リリースビルド
cargo build --release

# テスト実行
cargo test

# 単一テストの実行
cargo test test_urgency_conversion

# 特定モジュールのテスト
cargo test cli::tests

# 実行（開発時）
cargo run -- -m "メッセージ"

# CLI オプション例
cargo run -- -t "タイトル" -m "メッセージ" -T 3000 --urgency critical
cargo run -- -m "Test" --backend windows  # バックエンド強制指定
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
         ↑                 ↑                 ↑
         └─────────────────┴─────────────────┘
                  Notifier トレイト
```

## 主要パターン

### Notifier トレイト（`src/notifier/mod.rs`）
全バックエンドが実装する共通インターフェース。`send()`, `is_available()`, `backend_name()` を提供。

### 条件付きコンパイル
- Linux バックエンド（`src/notifier/linux.rs`）: `#[cfg(target_os = "linux")]` で実装を分岐
- WSL 検出（`src/platform.rs`）: `/proc/version` を読んで Microsoft/WSL を判定

### プラットフォーム対応
| Platform | バックエンド | 実装 |
|----------|-------------|------|
| Linux | notify-rust (D-Bus) | linux.rs |
| WSL | PowerShell | windows.rs |
| Windows | PowerShell | windows.rs |
| macOS | osascript | macos.rs |

## 依存クレート

- `clap`: CLI 引数パース（derive API 使用）
- `notify-rust`: Linux D-Bus 通知（Linux ターゲットのみ）
