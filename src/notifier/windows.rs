//! Windows 通知バックエンド
//!
//! PowerShell を使用して Windows のバルーン通知を送信します。
//! WSL（Windows Subsystem for Linux）からも使用可能です。
//!
//! # 学習ポイント
//! - PowerShell スクリプトの生成
//! - `std::process::Command` による外部プロセス実行
//! - raw 文字列リテラル `r#"..."#`

use crate::error::{NotificationError, Result};
use crate::notifier::{Notification, Notifier};
use std::process::Command;

/// Windows 通知バックエンド
pub struct WindowsNotifier;

impl Notifier for WindowsNotifier {
    fn send(&self, notification: &Notification) -> Result<()> {
        // PowerShell 用にエスケープ
        let title = escape_powershell(&notification.title);
        let message = escape_powershell(&notification.message);

        // PowerShell スクリプトを構築
        // System.Windows.Forms.NotifyIcon を使用してバルーン通知を表示
        //
        // # 学習ポイント: raw 文字列リテラル
        // r#"..."# を使うと、エスケープなしで文字列を書けます。
        // 特に PowerShell のような特殊文字が多いスクリプトで便利です。
        let ps_script = format!(
            r#"
            Add-Type -AssemblyName System.Windows.Forms
            $balloon = New-Object System.Windows.Forms.NotifyIcon
            $balloon.Icon = [System.Drawing.SystemIcons]::Information
            $balloon.BalloonTipTitle = '{}'
            $balloon.BalloonTipText = '{}'
            $balloon.Visible = $true
            $balloon.ShowBalloonTip({})
            Start-Sleep -Milliseconds 100
            $balloon.Dispose()
            "#,
            title, message, notification.timeout
        );

        // PowerShell を実行
        // WSL からは powershell.exe として呼び出せる（Windows 側のパスが自動解決）
        let output = Command::new("powershell.exe")
            .arg("-NoProfile") // プロファイルを読み込まない（高速化）
            .arg("-NonInteractive") // 対話モードを無効化
            .arg("-Command") // 後続の引数をコマンドとして実行
            .arg(&ps_script)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(NotificationError::SendFailed {
                backend: "Windows".to_string(),
                reason: stderr.to_string(),
            })
        }
    }

    fn is_available(&self) -> bool {
        // Windows または WSL（Linux からも Windows 通知を送信可能）
        cfg!(target_os = "windows") || cfg!(target_os = "linux")
    }

    fn backend_name(&self) -> &'static str {
        "Windows (PowerShell)"
    }
}

/// PowerShell 用の文字列エスケープ
///
/// PowerShell のシングルクォート文字列では、
/// シングルクォート自体を `''`（二重）でエスケープします。
///
/// # 例
/// ```ignore
/// escape_powershell("It's working") // => "It''s working"
/// ```
fn escape_powershell(s: &str) -> String {
    s.replace('\'', "''")
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_powershell_basic() {
        assert_eq!(escape_powershell("Hello"), "Hello");
    }

    #[test]
    fn test_escape_powershell_single_quote() {
        assert_eq!(escape_powershell("It's working"), "It''s working");
    }

    #[test]
    fn test_escape_powershell_multiple_quotes() {
        assert_eq!(escape_powershell("'Hello' 'World'"), "''Hello'' ''World''");
    }

    #[test]
    fn test_backend_name() {
        let notifier = WindowsNotifier;
        assert_eq!(notifier.backend_name(), "Windows (PowerShell)");
    }

    #[test]
    fn test_windows_notifier_available_on_linux_or_windows() {
        let notifier = WindowsNotifier;
        // Linux（WSL）または Windows では利用可能
        if cfg!(target_os = "linux") || cfg!(target_os = "windows") {
            assert!(notifier.is_available());
        }
    }
}
