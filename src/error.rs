//! エラー型定義モジュール
//!
//! アプリケーション全体で使用するエラー型を定義します。
//!
//! # 学習ポイント
//! - カスタムエラー型の定義方法
//! - `std::error::Error` トレイトの実装
//! - `From` トレイトによるエラー変換
//! - `Result` 型エイリアスの活用

use std::fmt;

/// 通知処理のエラー型
///
/// 各バリアントは異なるエラーケースを表します。
/// `#[derive(Debug)]` でデバッグ出力を自動実装。
#[derive(Debug)]
pub enum NotificationError {
    /// 通知送信失敗
    /// - `backend`: 使用したバックエンド名（Linux, Windows, macOS）
    /// - `reason`: 失敗の理由
    SendFailed { backend: String, reason: String },

    /// サポートされていないプラットフォーム
    UnsupportedPlatform(String),

    /// 外部コマンド実行エラー（PowerShell, osascript等）
    CommandExecution(std::io::Error),

    /// その他のエラー
    Other(String),
}

/// 結果型のエイリアス
///
/// `Result<T, NotificationError>` を短く `Result<T>` と書けるようにします。
/// これにより、関数のシグネチャが簡潔になります。
///
/// # 例
/// ```ignore
/// fn send_notification() -> Result<()> { ... }
/// // これは以下と同じ:
/// // fn send_notification() -> Result<(), NotificationError> { ... }
/// ```
pub type Result<T> = std::result::Result<T, NotificationError>;

/// `Display` トレイトの実装
///
/// エラーメッセージを人間が読みやすい形式で表示します。
/// `println!("{}", error)` や `error.to_string()` で使用されます。
impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendFailed { backend, reason } => {
                write!(f, "{} notification failed: {}", backend, reason)
            }
            Self::UnsupportedPlatform(platform) => {
                write!(f, "Unsupported platform: {}", platform)
            }
            Self::CommandExecution(err) => {
                write!(f, "Command execution error: {}", err)
            }
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

/// `std::error::Error` トレイトの実装
///
/// これを実装することで、`Box<dyn Error>` として扱えるようになり、
/// 他のエラー型と統一的に扱えます。
impl std::error::Error for NotificationError {
    /// エラーの原因（ソース）を返す
    ///
    /// `CommandExecution` バリアントの場合、内部の `io::Error` を返します。
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CommandExecution(err) => Some(err),
            _ => None,
        }
    }
}

/// `std::io::Error` からの変換
///
/// `From` トレイトを実装することで、`?` 演算子での自動変換が可能になります。
///
/// # 例
/// ```ignore
/// let output = Command::new("cmd").output()?; // io::Error が自動変換される
/// ```
impl From<std::io::Error> for NotificationError {
    fn from(err: std::io::Error) -> Self {
        Self::CommandExecution(err)
    }
}

/// 文字列からの変換（便利メソッド）
impl From<String> for NotificationError {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

/// `&str` からの変換
impl From<&str> for NotificationError {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
}

/// `Box<dyn Error>` からの変換
///
/// 汎用エラー型からの変換を可能にします。
impl From<Box<dyn std::error::Error>> for NotificationError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::Other(err.to_string())
    }
}

/// notify-rust のエラーからの変換（Linux のみ）
#[cfg(target_os = "linux")]
impl From<notify_rust::error::Error> for NotificationError {
    fn from(err: notify_rust::error::Error) -> Self {
        Self::SendFailed {
            backend: "Linux".to_string(),
            reason: err.to_string(),
        }
    }
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_send_failed() {
        let err = NotificationError::SendFailed {
            backend: "Windows".to_string(),
            reason: "PowerShell not found".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Windows notification failed: PowerShell not found"
        );
    }

    #[test]
    fn test_display_unsupported_platform() {
        let err = NotificationError::UnsupportedPlatform("FreeBSD".to_string());
        assert_eq!(err.to_string(), "Unsupported platform: FreeBSD");
    }

    #[test]
    fn test_from_string() {
        let err: NotificationError = "Something went wrong".into();
        assert!(matches!(err, NotificationError::Other(_)));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: NotificationError = io_err.into();
        assert!(matches!(err, NotificationError::CommandExecution(_)));
    }
}
