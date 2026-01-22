//! macOS 通知バックエンド
//!
//! `osascript` コマンドを使用して AppleScript 経由で通知センターに通知を送信します。
//!
//! # 学習ポイント
//! - `std::process::Command` による外部コマンド実行
//! - AppleScript の構文
//! - 文字列のエスケープ処理

use crate::error::{NotificationError, Result};
use crate::notifier::{Notification, Notifier};
use std::process::Command;

/// macOS 通知バックエンド
pub struct MacOsNotifier;

impl Notifier for MacOsNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        // AppleScript 用にエスケープ
        let title = escape_applescript(&notification.title);
        let message = escape_applescript(&notification.message);
        let subtitle = escape_applescript(&notification.subtitle);

        // AppleScript を構築
        // 構文: display notification "メッセージ" with title "タイトル" subtitle "サブ" sound name "音"
        let mut script = format!(
            r#"display notification "{}" with title "{}""#,
            message, title
        );

        // サブタイトルがあれば追加
        if !subtitle.is_empty() {
            script.push_str(&format!(r#" subtitle "{}""#, subtitle));
        }

        // 通知音を追加
        script.push_str(&format!(r#" sound name "{}""#, notification.sound));

        // osascript を実行
        // osascript は macOS の AppleScript インタープリタ
        let output = Command::new("osascript")
            .arg("-e") // -e: スクリプトを引数として実行
            .arg(&script)
            .output()?; // io::Error は NotificationError に自動変換

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(NotificationError::SendFailed {
                backend: "macOS".to_string(),
                reason: stderr.to_string(),
            })
        }
    }

    fn is_available(&self) -> bool {
        // macOS でのみ利用可能
        // cfg! マクロは bool を返す（#[cfg] とは異なる）
        cfg!(target_os = "macos")
    }

    fn backend_name(&self) -> &'static str {
        "macOS (osascript)"
    }
}

/// AppleScript 用の文字列エスケープ
///
/// AppleScript では以下の文字をエスケープする必要があります:
/// - `\` → `\\`（バックスラッシュ）
/// - `"` → `\"`（ダブルクォート）
///
/// # 例
/// ```ignore
/// escape_applescript(r#"Hello "World""#) // => r#"Hello \"World\""#
/// ```
fn escape_applescript(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_applescript_basic() {
        assert_eq!(escape_applescript("Hello"), "Hello");
    }

    #[test]
    fn test_escape_applescript_quotes() {
        assert_eq!(escape_applescript(r#"Hello "World""#), r#"Hello \"World\""#);
    }

    #[test]
    fn test_escape_applescript_backslash() {
        assert_eq!(escape_applescript(r"path\to\file"), r"path\\to\\file");
    }

    #[test]
    fn test_escape_applescript_combined() {
        assert_eq!(
            escape_applescript(r#"Say "Hi\" there"#),
            r#"Say \"Hi\\\" there"#
        );
    }

    #[test]
    fn test_backend_name() {
        let notifier = MacOsNotifier;
        assert_eq!(notifier.backend_name(), "macOS (osascript)");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_macos_notifier_available() {
        let notifier = MacOsNotifier;
        assert!(notifier.is_available());
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_macos_notifier_unavailable() {
        let notifier = MacOsNotifier;
        assert!(!notifier.is_available());
    }
}
