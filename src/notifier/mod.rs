//! 通知システムのコアモジュール
//!
//! トレイトベースの抽象化により、各プラットフォームの実装を分離しています。
//!
//! # 学習ポイント
//! - トレイト（`Notifier`）による抽象化
//! - Builder パターンによる構造体の構築
//! - `Box<dyn Trait>` による動的ディスパッチ
//! - サブモジュールの公開と再エクスポート
//!
//! # アーキテクチャ
//! ```text
//! ┌─────────────────────┐
//! │  NotificationBuilder │ ← ユーザーが使う構築API
//! └──────────┬──────────┘
//!            │ .send()
//!            ▼
//! ┌─────────────────────┐
//! │   select_notifier   │ ← プラットフォームに応じて選択
//! └──────────┬──────────┘
//!            │
//!   ┌────────┼────────┐
//!   ▼        ▼        ▼
//! Linux   Windows   macOS   ← 各バックエンドの実装
//! ```

// サブモジュールの宣言
mod linux;
mod macos;
mod windows;

// 各バックエンドの Notifier 実装を公開
pub use linux::LinuxNotifier;
pub use macos::MacOsNotifier;
pub use windows::WindowsNotifier;

use crate::error::{NotificationError, Result};
use crate::platform::{detect_platform, Platform};

// ============================================================
// Notifier トレイト
// ============================================================

/// 通知バックエンドの共通インターフェース
///
/// 各プラットフォーム（Linux, Windows, macOS）はこのトレイトを実装します。
/// これにより、呼び出し側はプラットフォームの違いを意識せずに通知を送信できます。
///
/// # 学習ポイント
/// - トレイトは Rust のインターフェース/抽象クラスに相当
/// - `&self` で自身への参照を受け取る
/// - `Result<()>` でエラーハンドリングを統一
pub trait Notifier {
    /// 通知を送信する
    ///
    /// # 引数
    /// - `notification`: 送信する通知の内容
    ///
    /// # 戻り値
    /// - `Ok(())`: 送信成功
    /// - `Err(NotificationError)`: 送信失敗
    fn send(&self, notification: &Notification) -> Result<()>;

    /// この Notifier が現在のプラットフォームで利用可能か
    ///
    /// 例: `LinuxNotifier` は Linux でのみ利用可能
    fn is_available(&self) -> bool;

    /// バックエンド名を返す（ログ/デバッグ用）
    fn backend_name(&self) -> &'static str;
}

// ============================================================
// 緊急度レベル
// ============================================================

/// 通知の緊急度レベル
///
/// CLI引数としても使用するため、`clap::ValueEnum` を derive しています。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UrgencyLevel {
    /// 低: 緊急性の低い通知（バックグラウンド処理完了など）
    Low,
    /// 通常: デフォルトの緊急度
    #[default]
    Normal,
    /// 重要: ユーザーの注意を引く通知（エラー、警告など）
    Critical,
}

// ============================================================
// Notification 構造体
// ============================================================

/// 通知の内容を表す構造体
///
/// Builder パターンで構築され、各 `Notifier` 実装に渡されます。
#[derive(Debug, Clone)]
pub struct Notification {
    /// 通知のタイトル
    pub title: String,
    /// 通知のメッセージ本文
    pub message: String,
    /// 表示時間（ミリ秒）、0 = 無制限
    pub timeout: u32,
    /// アイコン名またはパス（Linux のみ）
    pub icon: String,
    /// 緊急度レベル
    pub urgency: UrgencyLevel,
    /// サブタイトル（macOS のみ）
    pub subtitle: String,
    /// 通知音（macOS のみ）
    pub sound: String,
    /// 強制的に使用するバックエンド（None = 自動検出）
    pub backend_override: Option<Platform>,
}

// ============================================================
// NotificationBuilder（Builder パターン）
// ============================================================

/// 通知を構築するための Builder
///
/// # Builder パターンとは
/// 複雑なオブジェクトを段階的に構築するデザインパターンです。
/// 各メソッドが `self` を返すことで、メソッドチェーンが可能になります。
///
/// # 使用例
/// ```ignore
/// NotificationBuilder::new()
///     .title("Hello".to_string())
///     .message("World".to_string())
///     .timeout(5000)
///     .send()?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct NotificationBuilder {
    title: Option<String>,
    message: Option<String>,
    timeout: Option<u32>,
    icon: Option<String>,
    urgency: Option<UrgencyLevel>,
    subtitle: Option<String>,
    sound: Option<String>,
    backend: Option<Platform>,
}

impl NotificationBuilder {
    /// 新しい Builder を作成
    pub fn new() -> Self {
        Self::default()
    }

    /// タイトルを設定
    ///
    /// # 学習ポイント
    /// `mut self` を受け取り `Self` を返すことで、メソッドチェーンを実現
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// メッセージを設定
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// タイムアウトを設定（ミリ秒）
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// アイコンを設定（Linux のみ）
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// 緊急度を設定
    pub fn urgency(mut self, urgency: UrgencyLevel) -> Self {
        self.urgency = Some(urgency);
        self
    }

    /// サブタイトルを設定（macOS のみ）
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// 通知音を設定（macOS のみ）
    pub fn sound(mut self, sound: impl Into<String>) -> Self {
        self.sound = Some(sound.into());
        self
    }

    /// 使用するバックエンドを強制指定
    pub fn backend(mut self, backend: Platform) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Notification を構築（送信はしない）
    pub fn build(self) -> Notification {
        Notification {
            title: self.title.unwrap_or_else(|| "Notification".to_string()),
            message: self.message.unwrap_or_default(),
            timeout: self.timeout.unwrap_or(5000),
            icon: self.icon.unwrap_or_else(|| "dialog-information".to_string()),
            urgency: self.urgency.unwrap_or_default(),
            subtitle: self.subtitle.unwrap_or_default(),
            sound: self.sound.unwrap_or_else(|| "default".to_string()),
            backend_override: self.backend,
        }
    }

    /// Notification を構築して送信
    ///
    /// # 処理の流れ
    /// 1. `build()` で `Notification` を構築
    /// 2. `select_notifier()` で適切なバックエンドを選択
    /// 3. `notifier.send()` で送信
    pub fn send(self) -> Result<()> {
        let notification = self.build();
        let notifier = select_notifier(&notification)?;

        // デバッグ情報を出力
        eprintln!(
            "(Platform: {}, using {} backend)",
            notification
                .backend_override
                .unwrap_or_else(detect_platform),
            notifier.backend_name()
        );

        notifier.send(&notification)
    }
}

// ============================================================
// バックエンド選択ロジック
// ============================================================

/// 適切な Notifier を選択
///
/// # 学習ポイント
/// - `Box<dyn Notifier>`: トレイトオブジェクト（動的ディスパッチ）
/// - 実行時に具体的な型が決まる場合に使用
/// - `dyn` は "dynamic" の略
fn select_notifier(notification: &Notification) -> Result<Box<dyn Notifier>> {
    // バックエンドの強制指定があればそれを使用、なければ自動検出
    let platform = notification.backend_override.unwrap_or_else(detect_platform);

    // プラットフォームに応じた Notifier を作成
    // Box::new() でヒープに配置し、Box<dyn Notifier> として返す
    let notifier: Box<dyn Notifier> = match platform {
        Platform::Linux => Box::new(LinuxNotifier),
        Platform::Wsl | Platform::Windows => Box::new(WindowsNotifier),
        Platform::MacOs => Box::new(MacOsNotifier),
        Platform::Unknown => {
            return Err(NotificationError::UnsupportedPlatform(
                "Unknown platform. Use --backend to specify manually.".to_string(),
            ));
        }
    };

    // 選択された Notifier が利用可能かチェック
    if !notifier.is_available() {
        return Err(NotificationError::UnsupportedPlatform(format!(
            "{} notifier is not available on this platform",
            notifier.backend_name()
        )));
    }

    Ok(notifier)
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_builder_defaults() {
        let notification = NotificationBuilder::new().build();

        assert_eq!(notification.title, "Notification");
        assert_eq!(notification.message, "");
        assert_eq!(notification.timeout, 5000);
        assert_eq!(notification.urgency, UrgencyLevel::Normal);
    }

    #[test]
    fn test_notification_builder_with_values() {
        let notification = NotificationBuilder::new()
            .title("Test Title")
            .message("Test Message")
            .timeout(1000)
            .urgency(UrgencyLevel::Critical)
            .build();

        assert_eq!(notification.title, "Test Title");
        assert_eq!(notification.message, "Test Message");
        assert_eq!(notification.timeout, 1000);
        assert_eq!(notification.urgency, UrgencyLevel::Critical);
    }

    #[test]
    fn test_urgency_level_default() {
        let urgency = UrgencyLevel::default();
        assert_eq!(urgency, UrgencyLevel::Normal);
    }
}
