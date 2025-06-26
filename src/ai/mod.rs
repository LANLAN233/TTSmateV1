/*!
 * AI文案生成模块
 */

pub mod generator;
pub mod template;
pub mod content;

pub use generator::AIContentGenerator;
pub use content::{ContentType, GeneratedContent};
