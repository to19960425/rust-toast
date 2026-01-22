//! # rust-toast
//!
//! クロスプラットフォーム対応のトースト通知ライブラリ
//!
//! ## 対応プラットフォーム
//!
//! | OS | バックエンド | 説明 |
//! |-----|-------------|------|
//! | Linux | D-Bus (notify-rust) | デスクトップ通知 |
//! | WSL | PowerShell | Windows 通知を送信 |
//! | macOS | osascript | 通知センターに送信 |
//! | Windows | PowerShell | バルーン通知 |
//!
//! ## 使用例
//!
//! ### ライブラリとして使用
//!
//! ```ignore
//! use rust_toast::notifier::NotificationBuilder;
//!
//! // Builder パターンで通知を構築して送信
//! NotificationBuilder::new()
//!     .title("Hello")
//!     .message("World!")
//!     .timeout(5000)
//!     .send()?;
//! ```
//!
//! ### プラットフォームを指定して送信
//!
//! ```ignore
//! use rust_toast::{notifier::NotificationBuilder, platform::Platform};
//!
//! NotificationBuilder::new()
//!     .title("Hello")
//!     .message("From macOS!")
//!     .backend(Platform::MacOs)
//!     .send()?;
//! ```
//!
//! ## モジュール構成
//!
//! ```text
//! rust_toast
//! ├── cli        # CLI 引数定義（clap）
//! ├── error      # エラー型定義
//! ├── notifier   # 通知システムのコア
//! │   ├── mod    # トレイト定義、Builder、ディスパッチ
//! │   ├── linux  # Linux バックエンド
//! │   ├── macos  # macOS バックエンド
//! │   └── windows# Windows バックエンド
//! └── platform   # プラットフォーム検出
//! ```
//!
//! ## 学習できる Rust の概念
//!
//! - **モジュールシステム**: `mod`, `pub`, `use` によるコード整理
//! - **トレイト**: `Notifier` トレイトによる抽象化
//! - **Builder パターン**: `NotificationBuilder` による構築
//! - **エラーハンドリング**: カスタムエラー型と `Result`
//! - **条件付きコンパイル**: `#[cfg]` によるプラットフォーム分岐
//! - **外部コマンド実行**: `std::process::Command`

// ============================================================
// モジュール宣言
// ============================================================

/// CLI 引数定義モジュール
pub mod cli;

/// エラー型定義モジュール
pub mod error;

/// 通知システムのコアモジュール
pub mod notifier;

/// プラットフォーム検出モジュール
pub mod platform;

// ============================================================
// 便利な再エクスポート
// ============================================================
//
// ユーザーが `use rust_toast::NotificationBuilder;` のように
// 短く書けるようにするため、よく使う型を再エクスポートします。

/// エラー型の再エクスポート
pub use error::{NotificationError, Result};

/// 通知関連の型の再エクスポート
pub use notifier::{Notification, NotificationBuilder, Notifier, UrgencyLevel};

/// プラットフォーム関連の再エクスポート
pub use platform::{detect_platform, Platform};
