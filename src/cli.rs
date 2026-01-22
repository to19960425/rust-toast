//! CLI 引数定義モジュール
//!
//! `clap` クレートを使用してコマンドライン引数を定義します。
//!
//! # 学習ポイント
//! - `clap` の derive API（`#[derive(Parser)]`）
//! - `ValueEnum` による型安全な選択肢
//! - CLI 引数からライブラリ型への変換

use clap::{Parser, ValueEnum};

use crate::notifier::{NotificationBuilder, UrgencyLevel};
use crate::platform::Platform;

// ============================================================
// CLI 引数の定義
// ============================================================

/// コマンドライン引数の構造体
///
/// `#[derive(Parser)]` により、`clap` が自動的に
/// 引数のパース処理を生成します。
///
/// # 学習ポイント
/// - `///` コメントは `--help` で表示される
/// - `#[arg(...)]` で引数の詳細を設定
/// - `short` と `long` でショートオプションとロングオプションを指定
#[derive(Parser, Debug)]
#[command(name = "rust-toast")]
#[command(author, version, about = "Cross-platform toast notification tool")]
pub struct Args {
    /// Notification title (通知のタイトル)
    #[arg(short, long, default_value = "Notification")]
    pub title: String,

    /// Notification message (通知のメッセージ) - Required
    #[arg(short, long)]
    pub message: String,

    /// Timeout in milliseconds, 0 = no timeout (タイムアウト時間)
    #[arg(short = 'T', long, default_value = "5000")]
    pub timeout: u32,

    /// Icon name or path (アイコン名またはパス、Linux only)
    #[arg(short, long, default_value = "dialog-information")]
    pub icon: String,

    /// Urgency level (緊急度レベル)
    #[arg(short, long, default_value = "normal", value_enum)]
    pub urgency: CliUrgencyLevel,

    /// Subtitle (サブタイトル、macOS only)
    #[arg(short, long, default_value = "")]
    pub subtitle: String,

    /// Sound name (通知音、macOS only)
    ///
    /// Available sounds: default, Basso, Blow, Bottle, Frog, Funk,
    /// Glass, Hero, Morse, Ping, Pop, Purr, Sosumi, Submarine, Tink
    #[arg(long, default_value = "default")]
    pub sound: String,

    /// Force specific backend (強制的に特定のバックエンドを使用)
    #[arg(long, value_enum)]
    pub backend: Option<CliBackend>,
}

// ============================================================
// CLI 用の列挙型
// ============================================================

/// CLI 用の緊急度レベル
///
/// `clap::ValueEnum` を derive することで、
/// CLI 引数として直接パースできるようになります。
///
/// # 学習ポイント
/// `#[value(name = "...")]` でCLIでの表示名を指定できます。
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliUrgencyLevel {
    Low,
    Normal,
    Critical,
}

/// CLI 用のバックエンド選択
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliBackend {
    /// Linux D-Bus notification
    Linux,
    /// Windows notification via PowerShell
    Windows,
    /// macOS notification via osascript
    Macos,
}

// ============================================================
// 型変換の実装
// ============================================================

/// CliUrgencyLevel から UrgencyLevel への変換
///
/// `From` トレイトを実装することで、`.into()` で変換できます。
impl From<CliUrgencyLevel> for UrgencyLevel {
    fn from(level: CliUrgencyLevel) -> Self {
        match level {
            CliUrgencyLevel::Low => UrgencyLevel::Low,
            CliUrgencyLevel::Normal => UrgencyLevel::Normal,
            CliUrgencyLevel::Critical => UrgencyLevel::Critical,
        }
    }
}

/// CliBackend から Platform への変換
impl From<CliBackend> for Platform {
    fn from(backend: CliBackend) -> Self {
        match backend {
            CliBackend::Linux => Platform::Linux,
            CliBackend::Windows => Platform::Windows,
            CliBackend::Macos => Platform::MacOs,
        }
    }
}

// ============================================================
// Args のメソッド
// ============================================================

impl Args {
    /// CLI 引数から NotificationBuilder を構築
    ///
    /// CLI の責務（引数パース）と通知の責務（送信）を分離するため、
    /// Args は直接通知を送信せず、Builder を返します。
    ///
    /// # 使用例
    /// ```ignore
    /// let args = Args::parse();
    /// args.into_builder().send()?;
    /// ```
    pub fn into_builder(self) -> NotificationBuilder {
        let mut builder = NotificationBuilder::new()
            .title(self.title)
            .message(self.message)
            .timeout(self.timeout)
            .icon(self.icon)
            .urgency(self.urgency.into())
            .subtitle(self.subtitle)
            .sound(self.sound);

        // バックエンドの強制指定があれば設定
        if let Some(backend) = self.backend {
            builder = builder.backend(backend.into());
        }

        builder
    }
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgency_conversion() {
        assert_eq!(
            UrgencyLevel::from(CliUrgencyLevel::Low),
            UrgencyLevel::Low
        );
        assert_eq!(
            UrgencyLevel::from(CliUrgencyLevel::Normal),
            UrgencyLevel::Normal
        );
        assert_eq!(
            UrgencyLevel::from(CliUrgencyLevel::Critical),
            UrgencyLevel::Critical
        );
    }

    #[test]
    fn test_backend_conversion() {
        assert_eq!(Platform::from(CliBackend::Linux), Platform::Linux);
        assert_eq!(Platform::from(CliBackend::Windows), Platform::Windows);
        assert_eq!(Platform::from(CliBackend::Macos), Platform::MacOs);
    }

    #[test]
    fn test_args_into_builder() {
        // Args を手動で構築（通常は clap::Parser::parse() で取得）
        let args = Args {
            title: "Test".to_string(),
            message: "Hello".to_string(),
            timeout: 1000,
            icon: "icon.png".to_string(),
            urgency: CliUrgencyLevel::Critical,
            subtitle: "Sub".to_string(),
            sound: "Ping".to_string(),
            backend: Some(CliBackend::Macos),
        };

        let notification = args.into_builder().build();

        assert_eq!(notification.title, "Test");
        assert_eq!(notification.message, "Hello");
        assert_eq!(notification.timeout, 1000);
        assert_eq!(notification.urgency, UrgencyLevel::Critical);
        assert_eq!(notification.backend_override, Some(Platform::MacOs));
    }
}
