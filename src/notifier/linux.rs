//! Linux 通知バックエンド
//!
//! `notify-rust` クレートを使用して D-Bus 経由で通知を送信します。
//!
//! # 学習ポイント
//! - 条件付きコンパイル `#[cfg(target_os = "linux")]`
//! - 外部クレートのラッピング
//! - 同一関数の異なるプラットフォーム向け実装

use crate::error::Result;
#[cfg(not(target_os = "linux"))]
use crate::error::NotificationError;
use crate::notifier::{Notification, Notifier, UrgencyLevel};

// notify-rust は Linux でのみ使用
#[cfg(target_os = "linux")]
use notify_rust::{Notification as RustNotification, Timeout, Urgency};

/// Linux 通知バックエンド
///
/// ユニット構造体（フィールドを持たない構造体）として定義。
/// 状態を持たないため、シンプルに実装できます。
pub struct LinuxNotifier;

// ============================================================
// Linux 向け実装
// ============================================================

#[cfg(target_os = "linux")]
impl Notifier for LinuxNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        // タイムアウトの変換
        let timeout = if notification.timeout == 0 {
            Timeout::Never
        } else {
            Timeout::Milliseconds(notification.timeout)
        };

        // 緊急度の変換
        let urgency = match notification.urgency {
            UrgencyLevel::Low => Urgency::Low,
            UrgencyLevel::Normal => Urgency::Normal,
            UrgencyLevel::Critical => Urgency::Critical,
        };

        // notify-rust の API を使用して通知を送信
        RustNotification::new()
            .summary(&notification.title)
            .body(&notification.message)
            .icon(&notification.icon)
            .timeout(timeout)
            .urgency(urgency)
            .show()?; // エラーは From トレイトで自動変換

        Ok(())
    }

    fn is_available(&self) -> bool {
        // Linux 向けにコンパイルされていれば利用可能
        true
    }

    fn backend_name(&self) -> &'static str {
        "Linux (D-Bus)"
    }
}

// ============================================================
// Linux 以外のプラットフォーム向けスタブ実装
// ============================================================

/// Linux 以外では、エラーを返すスタブ実装を提供
///
/// # 学習ポイント
/// `#[cfg(not(...))]` で「〜以外」を指定できます。
/// これにより、Linux でコンパイルされた場合とそれ以外で
/// 異なる実装を提供できます。
#[cfg(not(target_os = "linux"))]
impl Notifier for LinuxNotifier {
    fn send(&self, _notification: &Notification) -> Result<()> {
        Err(NotificationError::UnsupportedPlatform(
            "Linux notification requires a binary compiled for Linux".to_string(),
        ))
    }

    fn is_available(&self) -> bool {
        false
    }

    fn backend_name(&self) -> &'static str {
        "Linux (unavailable)"
    }
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_name() {
        let notifier = LinuxNotifier;
        // プラットフォームに関係なく、backend_name は呼び出せる
        let _ = notifier.backend_name();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_linux_notifier_available() {
        let notifier = LinuxNotifier;
        assert!(notifier.is_available());
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn test_linux_notifier_unavailable() {
        let notifier = LinuxNotifier;
        assert!(!notifier.is_available());
    }
}
