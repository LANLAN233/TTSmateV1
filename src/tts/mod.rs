/*!
 * TTS客户端模块
 */

pub mod client;
pub mod cache;
pub mod error;

pub use client::TTSClient;
pub use error::TTSError;
