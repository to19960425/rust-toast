//! rust-toast CLI エントリーポイント
//!
//! このファイルは CLI アプリケーションのエントリーポイントです。
//! ロジックは `lib.rs` と各モジュールに委譲し、ここでは
//! 引数のパースと結果の表示のみを行います。
//!
//! # 学習ポイント
//! - `main.rs` と `lib.rs` の分離
//! - 薄いエントリーポイントの設計
//! - エラーハンドリングの統合

use clap::Parser;

// ライブラリからインポート
// クレート名は Cargo.toml の [package] name から決まる
// ハイフンはアンダースコアに変換される（rust-toast → rust_toast）
use rust_toast::cli::Args;
use rust_toast::Result;

/// メイン関数
///
/// # 処理の流れ
/// 1. CLI 引数をパース（`Args::parse()`）
/// 2. 引数から `NotificationBuilder` を構築
/// 3. 通知を送信
/// 4. 結果を表示
///
/// # 戻り値
/// - `Ok(())`: 正常終了
/// - `Err(NotificationError)`: エラー終了（Rust が自動的にメッセージを表示）
fn main() -> Result<()> {
    // Step 1: CLI 引数をパース
    // clap が自動的に --help, --version を処理し、
    // 必須引数が不足している場合はエラーメッセージを表示して終了
    let args = Args::parse();

    // Step 2-3: NotificationBuilder を構築して送信
    // into_builder() で Args → NotificationBuilder に変換
    // send() で通知を送信
    args.into_builder().send()?;

    // Step 4: 成功メッセージを表示
    println!("✓ Toast notification sent successfully");

    Ok(())
}
