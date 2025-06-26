/*!
 * 快捷键绑定
 */

use serde::{Deserialize, Serialize};

/// 按键代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    // 功能键
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // 数字键
    Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0,

    // 字母键
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // 特殊键
    Space, Enter, Escape, Tab, Backspace, Delete,
    Up, Down, Left, Right,
    Home, End, PageUp, PageDown,
    Insert, PrintScreen, ScrollLock, Pause,

    // 修饰键组合
    CtrlA, CtrlB, CtrlC, CtrlD, CtrlE, CtrlF, CtrlG, CtrlH, CtrlI, CtrlJ,
    CtrlK, CtrlL, CtrlM, CtrlN, CtrlO, CtrlP, CtrlQ, CtrlR, CtrlS, CtrlT,
    CtrlU, CtrlV, CtrlW, CtrlX, CtrlY, CtrlZ,

    AltA, AltB, AltC, AltD, AltE, AltF, AltG, AltH, AltI, AltJ,
    AltK, AltL, AltM, AltN, AltO, AltP, AltQ, AltR, AltS, AltT,
    AltU, AltV, AltW, AltX, AltY, AltZ,

    ShiftF1, ShiftF2, ShiftF3, ShiftF4, ShiftF5, ShiftF6,
    ShiftF7, ShiftF8, ShiftF9, ShiftF10, ShiftF11, ShiftF12,
}

impl KeyCode {
    /// 获取按键的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",

            KeyCode::Num1 => "1",
            KeyCode::Num2 => "2",
            KeyCode::Num3 => "3",
            KeyCode::Num4 => "4",
            KeyCode::Num5 => "5",
            KeyCode::Num6 => "6",
            KeyCode::Num7 => "7",
            KeyCode::Num8 => "8",
            KeyCode::Num9 => "9",
            KeyCode::Num0 => "0",

            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",

            KeyCode::Space => "Space",
            KeyCode::Enter => "Enter",
            KeyCode::Escape => "Esc",
            KeyCode::Tab => "Tab",
            KeyCode::Backspace => "Backspace",
            KeyCode::Delete => "Delete",

            KeyCode::Up => "↑",
            KeyCode::Down => "↓",
            KeyCode::Left => "←",
            KeyCode::Right => "→",

            KeyCode::Home => "Home",
            KeyCode::End => "End",
            KeyCode::PageUp => "PgUp",
            KeyCode::PageDown => "PgDn",
            KeyCode::Insert => "Insert",
            KeyCode::PrintScreen => "PrtSc",
            KeyCode::ScrollLock => "ScrLk",
            KeyCode::Pause => "Pause",

            KeyCode::CtrlA => "Ctrl+A",
            KeyCode::CtrlB => "Ctrl+B",
            KeyCode::CtrlC => "Ctrl+C",
            KeyCode::CtrlD => "Ctrl+D",
            KeyCode::CtrlE => "Ctrl+E",
            KeyCode::CtrlF => "Ctrl+F",
            KeyCode::CtrlG => "Ctrl+G",
            KeyCode::CtrlH => "Ctrl+H",
            KeyCode::CtrlI => "Ctrl+I",
            KeyCode::CtrlJ => "Ctrl+J",
            KeyCode::CtrlK => "Ctrl+K",
            KeyCode::CtrlL => "Ctrl+L",
            KeyCode::CtrlM => "Ctrl+M",
            KeyCode::CtrlN => "Ctrl+N",
            KeyCode::CtrlO => "Ctrl+O",
            KeyCode::CtrlP => "Ctrl+P",
            KeyCode::CtrlQ => "Ctrl+Q",
            KeyCode::CtrlR => "Ctrl+R",
            KeyCode::CtrlS => "Ctrl+S",
            KeyCode::CtrlT => "Ctrl+T",
            KeyCode::CtrlU => "Ctrl+U",
            KeyCode::CtrlV => "Ctrl+V",
            KeyCode::CtrlW => "Ctrl+W",
            KeyCode::CtrlX => "Ctrl+X",
            KeyCode::CtrlY => "Ctrl+Y",
            KeyCode::CtrlZ => "Ctrl+Z",

            KeyCode::AltA => "Alt+A",
            KeyCode::AltB => "Alt+B",
            KeyCode::AltC => "Alt+C",
            KeyCode::AltD => "Alt+D",
            KeyCode::AltE => "Alt+E",
            KeyCode::AltF => "Alt+F",
            KeyCode::AltG => "Alt+G",
            KeyCode::AltH => "Alt+H",
            KeyCode::AltI => "Alt+I",
            KeyCode::AltJ => "Alt+J",
            KeyCode::AltK => "Alt+K",
            KeyCode::AltL => "Alt+L",
            KeyCode::AltM => "Alt+M",
            KeyCode::AltN => "Alt+N",
            KeyCode::AltO => "Alt+O",
            KeyCode::AltP => "Alt+P",
            KeyCode::AltQ => "Alt+Q",
            KeyCode::AltR => "Alt+R",
            KeyCode::AltS => "Alt+S",
            KeyCode::AltT => "Alt+T",
            KeyCode::AltU => "Alt+U",
            KeyCode::AltV => "Alt+V",
            KeyCode::AltW => "Alt+W",
            KeyCode::AltX => "Alt+X",
            KeyCode::AltY => "Alt+Y",
            KeyCode::AltZ => "Alt+Z",

            KeyCode::ShiftF1 => "Shift+F1",
            KeyCode::ShiftF2 => "Shift+F2",
            KeyCode::ShiftF3 => "Shift+F3",
            KeyCode::ShiftF4 => "Shift+F4",
            KeyCode::ShiftF5 => "Shift+F5",
            KeyCode::ShiftF6 => "Shift+F6",
            KeyCode::ShiftF7 => "Shift+F7",
            KeyCode::ShiftF8 => "Shift+F8",
            KeyCode::ShiftF9 => "Shift+F9",
            KeyCode::ShiftF10 => "Shift+F10",
            KeyCode::ShiftF11 => "Shift+F11",
            KeyCode::ShiftF12 => "Shift+F12",
        }
    }

    /// 获取所有可用的按键
    pub fn all_keys() -> Vec<KeyCode> {
        vec![
            // 功能键
            KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4,
            KeyCode::F5, KeyCode::F6, KeyCode::F7, KeyCode::F8,
            KeyCode::F9, KeyCode::F10, KeyCode::F11, KeyCode::F12,

            // 数字键
            KeyCode::Num1, KeyCode::Num2, KeyCode::Num3, KeyCode::Num4, KeyCode::Num5,
            KeyCode::Num6, KeyCode::Num7, KeyCode::Num8, KeyCode::Num9, KeyCode::Num0,

            // 常用组合键
            KeyCode::CtrlA, KeyCode::CtrlB, KeyCode::CtrlC, KeyCode::CtrlD, KeyCode::CtrlE,
            KeyCode::CtrlF, KeyCode::CtrlG, KeyCode::CtrlH, KeyCode::CtrlI, KeyCode::CtrlJ,

            KeyCode::AltA, KeyCode::AltB, KeyCode::AltC, KeyCode::AltD, KeyCode::AltE,
            KeyCode::AltF, KeyCode::AltG, KeyCode::AltH, KeyCode::AltI, KeyCode::AltJ,

            KeyCode::ShiftF1, KeyCode::ShiftF2, KeyCode::ShiftF3, KeyCode::ShiftF4,
            KeyCode::ShiftF5, KeyCode::ShiftF6, KeyCode::ShiftF7, KeyCode::ShiftF8,
        ]
    }

    /// 从字符串解析按键
    pub fn from_string(s: &str) -> Option<KeyCode> {
        match s.to_uppercase().as_str() {
            "F1" => Some(KeyCode::F1),
            "F2" => Some(KeyCode::F2),
            "F3" => Some(KeyCode::F3),
            "F4" => Some(KeyCode::F4),
            "F5" => Some(KeyCode::F5),
            "F6" => Some(KeyCode::F6),
            "F7" => Some(KeyCode::F7),
            "F8" => Some(KeyCode::F8),
            "F9" => Some(KeyCode::F9),
            "F10" => Some(KeyCode::F10),
            "F11" => Some(KeyCode::F11),
            "F12" => Some(KeyCode::F12),

            "1" => Some(KeyCode::Num1),
            "2" => Some(KeyCode::Num2),
            "3" => Some(KeyCode::Num3),
            "4" => Some(KeyCode::Num4),
            "5" => Some(KeyCode::Num5),
            "6" => Some(KeyCode::Num6),
            "7" => Some(KeyCode::Num7),
            "8" => Some(KeyCode::Num8),
            "9" => Some(KeyCode::Num9),
            "0" => Some(KeyCode::Num0),

            "CTRL+A" => Some(KeyCode::CtrlA),
            "CTRL+B" => Some(KeyCode::CtrlB),
            "CTRL+C" => Some(KeyCode::CtrlC),
            "CTRL+D" => Some(KeyCode::CtrlD),
            "CTRL+E" => Some(KeyCode::CtrlE),

            "ALT+A" => Some(KeyCode::AltA),
            "ALT+B" => Some(KeyCode::AltB),
            "ALT+C" => Some(KeyCode::AltC),
            "ALT+D" => Some(KeyCode::AltD),
            "ALT+E" => Some(KeyCode::AltE),

            "SPACE" => Some(KeyCode::Space),
            "ENTER" => Some(KeyCode::Enter),
            "ESC" | "ESCAPE" => Some(KeyCode::Escape),

            _ => None,
        }
    }
}

/// 快捷键绑定管理器
#[derive(Debug)]
pub struct KeyBindingManager {
    // 使用global-hotkey库实现全局快捷键
    hotkey_manager: Option<global_hotkey::GlobalHotKeyManager>,
    registered_hotkeys: std::collections::HashMap<KeyCode, global_hotkey::HotKey>,
}

impl KeyBindingManager {
    /// 创建新的快捷键管理器
    pub fn new() -> crate::error::Result<Self> {
        let hotkey_manager = global_hotkey::GlobalHotKeyManager::new()
            .map_err(|e| crate::error::AppError::soundboard(format!("无法创建全局快捷键管理器: {}", e)))?;

        Ok(Self {
            hotkey_manager: Some(hotkey_manager),
            registered_hotkeys: std::collections::HashMap::new(),
        })
    }

    /// 注册全局快捷键
    pub fn register_global_hotkey(&mut self, key: KeyCode, hotkey_id: u32) -> crate::error::Result<()> {
        use global_hotkey::{hotkey::HotKey, hotkey::Code, hotkey::Modifiers};

        if let Some(ref manager) = self.hotkey_manager {
            // 将KeyCode转换为global-hotkey的Code
            let (code, modifiers) = self.keycode_to_global_hotkey(key)?;

            let hotkey = HotKey::new(Some(modifiers), code);

            manager.register(hotkey.clone())
                .map_err(|e| crate::error::AppError::soundboard(format!("注册全局快捷键失败: {}", e)))?;

            self.registered_hotkeys.insert(key, hotkey);

            log::info!("全局快捷键注册成功: {:?}", key);
            Ok(())
        } else {
            Err(crate::error::AppError::soundboard("快捷键管理器未初始化"))
        }
    }

    /// 注销全局快捷键
    pub fn unregister_global_hotkey(&mut self, key: KeyCode) -> crate::error::Result<()> {
        if let Some(ref manager) = self.hotkey_manager {
            if let Some(hotkey) = self.registered_hotkeys.remove(&key) {
                manager.unregister(hotkey)
                    .map_err(|e| crate::error::AppError::soundboard(format!("注销全局快捷键失败: {}", e)))?;

                log::info!("全局快捷键注销成功: {:?}", key);
            }
            Ok(())
        } else {
            Err(crate::error::AppError::soundboard("快捷键管理器未初始化"))
        }
    }

    /// 注销所有快捷键
    pub fn unregister_all(&mut self) -> crate::error::Result<()> {
        if let Some(ref manager) = self.hotkey_manager {
            for (key, hotkey) in self.registered_hotkeys.drain() {
                if let Err(e) = manager.unregister(hotkey) {
                    log::warn!("注销快捷键 {:?} 失败: {}", key, e);
                }
            }
            log::info!("所有全局快捷键已注销");
            Ok(())
        } else {
            Err(crate::error::AppError::soundboard("快捷键管理器未初始化"))
        }
    }

    /// 将KeyCode转换为global-hotkey格式
    fn keycode_to_global_hotkey(&self, key: KeyCode) -> crate::error::Result<(global_hotkey::hotkey::Code, global_hotkey::hotkey::Modifiers)> {
        use global_hotkey::hotkey::{Code, Modifiers};

        let (code, modifiers) = match key {
            // 功能键
            KeyCode::F1 => (Code::F1, Modifiers::empty()),
            KeyCode::F2 => (Code::F2, Modifiers::empty()),
            KeyCode::F3 => (Code::F3, Modifiers::empty()),
            KeyCode::F4 => (Code::F4, Modifiers::empty()),
            KeyCode::F5 => (Code::F5, Modifiers::empty()),
            KeyCode::F6 => (Code::F6, Modifiers::empty()),
            KeyCode::F7 => (Code::F7, Modifiers::empty()),
            KeyCode::F8 => (Code::F8, Modifiers::empty()),
            KeyCode::F9 => (Code::F9, Modifiers::empty()),
            KeyCode::F10 => (Code::F10, Modifiers::empty()),
            KeyCode::F11 => (Code::F11, Modifiers::empty()),
            KeyCode::F12 => (Code::F12, Modifiers::empty()),

            // 数字键
            KeyCode::Num1 => (Code::Digit1, Modifiers::empty()),
            KeyCode::Num2 => (Code::Digit2, Modifiers::empty()),
            KeyCode::Num3 => (Code::Digit3, Modifiers::empty()),
            KeyCode::Num4 => (Code::Digit4, Modifiers::empty()),
            KeyCode::Num5 => (Code::Digit5, Modifiers::empty()),
            KeyCode::Num6 => (Code::Digit6, Modifiers::empty()),
            KeyCode::Num7 => (Code::Digit7, Modifiers::empty()),
            KeyCode::Num8 => (Code::Digit8, Modifiers::empty()),
            KeyCode::Num9 => (Code::Digit9, Modifiers::empty()),
            KeyCode::Num0 => (Code::Digit0, Modifiers::empty()),

            // Ctrl组合键
            KeyCode::CtrlA => (Code::KeyA, Modifiers::CONTROL),
            KeyCode::CtrlB => (Code::KeyB, Modifiers::CONTROL),
            KeyCode::CtrlC => (Code::KeyC, Modifiers::CONTROL),
            KeyCode::CtrlD => (Code::KeyD, Modifiers::CONTROL),
            KeyCode::CtrlE => (Code::KeyE, Modifiers::CONTROL),

            // Alt组合键
            KeyCode::AltA => (Code::KeyA, Modifiers::ALT),
            KeyCode::AltB => (Code::KeyB, Modifiers::ALT),
            KeyCode::AltC => (Code::KeyC, Modifiers::ALT),
            KeyCode::AltD => (Code::KeyD, Modifiers::ALT),
            KeyCode::AltE => (Code::KeyE, Modifiers::ALT),

            // 特殊键
            KeyCode::Space => (Code::Space, Modifiers::empty()),
            KeyCode::Enter => (Code::Enter, Modifiers::empty()),
            KeyCode::Escape => (Code::Escape, Modifiers::empty()),

            _ => {
                return Err(crate::error::AppError::soundboard(format!("不支持的快捷键: {:?}", key)));
            }
        };

        Ok((code, modifiers))
    }

    /// 获取已注册的快捷键列表
    pub fn get_registered_keys(&self) -> Vec<KeyCode> {
        self.registered_hotkeys.keys().cloned().collect()
    }
}
