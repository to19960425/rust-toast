//! プラットフォーム検出モジュール
//!
//! 実行環境（Linux, WSL, macOS, Windows）を検出する機能を提供します。
//!
//! # 学習ポイント
//! - `std::env::consts::OS` によるコンパイル時のターゲットOS取得
//! - ファイルシステムを使った実行時の環境検出
//! - `Display` トレイトによるカスタム表示
//! - `Copy`, `Clone`, `PartialEq` などの derive マクロ

use std::fmt;
use std::fs;

/// 実行環境を表す列挙型
///
/// # derive の説明
/// - `Debug`: `{:?}` でデバッグ出力可能に
/// - `Clone`: `.clone()` で複製可能に
/// - `Copy`: 暗黙的にコピー可能に（小さな値なので）
/// - `PartialEq`, `Eq`: `==` で比較可能に
/// - `Hash`: HashMap のキーとして使用可能に
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Platform {
    /// ネイティブ Linux 環境
    Linux,
    /// Windows Subsystem for Linux
    Wsl,
    /// macOS / Darwin
    MacOs,
    /// ネイティブ Windows
    Windows,
    /// 不明なプラットフォーム
    Unknown,
}

/// `Display` トレイトの実装
///
/// `println!("{}", platform)` で人間が読みやすい形式で出力します。
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linux => write!(f, "Linux"),
            Self::Wsl => write!(f, "WSL (Windows Subsystem for Linux)"),
            Self::MacOs => write!(f, "macOS"),
            Self::Windows => write!(f, "Windows"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Platform {
    /// このプラットフォームが Windows 系かどうかを判定
    ///
    /// WSL も Windows 通知を使用するため、true を返します。
    pub fn is_windows_like(&self) -> bool {
        matches!(self, Self::Wsl | Self::Windows)
    }

    /// このプラットフォームが Unix 系かどうかを判定
    pub fn is_unix_like(&self) -> bool {
        matches!(self, Self::Linux | Self::MacOs)
    }
}

/// 現在の実行環境を検出
///
/// # 検出ロジック
/// 1. `std::env::consts::OS` でコンパイル時のターゲットOSを取得
/// 2. Linux の場合、追加で WSL かどうかをチェック
///
/// # 例
/// ```
/// use rust_toast::platform::{Platform, detect_platform};
///
/// let platform = detect_platform();
/// println!("Running on: {}", platform);
/// ```
pub fn detect_platform() -> Platform {
    // std::env::consts::OS はコンパイル時に決定される定数
    // 可能な値: "linux", "macos", "windows", "freebsd", etc.
    match std::env::consts::OS {
        "macos" => Platform::MacOs,
        "windows" => Platform::Windows,
        "linux" => {
            // Linux の場合、WSL 環境かどうかを追加でチェック
            if is_wsl() {
                Platform::Wsl
            } else {
                Platform::Linux
            }
        }
        _ => Platform::Unknown,
    }
}

/// WSL（Windows Subsystem for Linux）環境かどうかを判定
///
/// # 検出方法
/// `/proc/version` ファイルの内容に "microsoft" または "wsl" が
/// 含まれているかどうかで判定します。
///
/// # 注意
/// この関数は Linux 環境でのみ意味を持ちます。
/// macOS/Windows では常に false を返します。
fn is_wsl() -> bool {
    // /proc/version はLinuxカーネルのバージョン情報を含むファイル
    // WSL の場合、"Microsoft" や "WSL" という文字列が含まれる
    //
    // 例: "Linux version 5.15.167.4-microsoft-standard-WSL2 ..."
    fs::read_to_string("/proc/version")
        .map(|content| {
            let lower = content.to_lowercase();
            lower.contains("microsoft") || lower.contains("wsl")
        })
        .unwrap_or(false) // ファイルが読めない場合は false
}

// ============================================================
// テスト
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_platform_returns_valid() {
        let platform = detect_platform();
        // 何らかの有効なプラットフォームが返ることを確認
        assert!(matches!(
            platform,
            Platform::Linux
                | Platform::Wsl
                | Platform::MacOs
                | Platform::Windows
                | Platform::Unknown
        ));
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(format!("{}", Platform::Linux), "Linux");
        assert_eq!(format!("{}", Platform::MacOs), "macOS");
        assert_eq!(format!("{}", Platform::Windows), "Windows");
        assert_eq!(
            format!("{}", Platform::Wsl),
            "WSL (Windows Subsystem for Linux)"
        );
        assert_eq!(format!("{}", Platform::Unknown), "Unknown");
    }

    #[test]
    fn test_is_windows_like() {
        assert!(Platform::Windows.is_windows_like());
        assert!(Platform::Wsl.is_windows_like());
        assert!(!Platform::Linux.is_windows_like());
        assert!(!Platform::MacOs.is_windows_like());
    }

    #[test]
    fn test_is_unix_like() {
        assert!(Platform::Linux.is_unix_like());
        assert!(Platform::MacOs.is_unix_like());
        assert!(!Platform::Windows.is_unix_like());
        assert!(!Platform::Wsl.is_unix_like());
    }

    #[test]
    fn test_platform_equality() {
        assert_eq!(Platform::Linux, Platform::Linux);
        assert_ne!(Platform::Linux, Platform::Windows);
    }

    #[test]
    fn test_platform_clone_and_copy() {
        let p1 = Platform::MacOs;
        let p2 = p1; // Copy により暗黙コピー
        let p3 = p1.clone(); // Clone による明示コピー
        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
    }
}
