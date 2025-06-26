/*!
 * TTSmate构建脚本
 */

#[cfg(windows)]
extern crate winres;

fn main() {
    // Windows平台特定的构建配置
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        
        // 设置应用程序图标
        if std::path::Path::new("assets/icon.ico").exists() {
            res.set_icon("assets/icon.ico");
        }
        
        // 设置版本信息
        res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 1, 0, 0, 0);
        res.set_version_info(winres::VersionInfo::FILEVERSION, 1, 0, 0, 0);
        
        // 设置产品信息
        res.set("ProductName", "TTSmate V1");
        res.set("FileDescription", "智能语音合成客户端");
        res.set("CompanyName", "TTSmate Team");
        res.set("LegalCopyright", "Copyright © 2025 TTSmate Team");
        res.set("OriginalFilename", "ttsmate.exe");
        res.set("InternalName", "ttsmate");
        
        // 设置语言为中文简体
        res.set_language(0x0804);
        
        // 编译资源
        if let Err(e) = res.compile() {
            eprintln!("警告: Windows资源编译失败: {}", e);
        }
    }
    
    // 输出构建信息
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/icon.ico");
    
    // 设置构建时间
    println!("cargo:rustc-env=BUILD_TIME={}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
    
    // 设置Git提交哈希（如果可用）
    if let Ok(output) = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIT_HASH={}", git_hash);
        }
    }
}
