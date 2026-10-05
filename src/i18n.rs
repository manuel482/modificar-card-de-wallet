use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    SimplifiedChinese,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl Language {
    pub fn load() -> Self {
        let path = settings_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&contents) {
                return settings.language;
            }
        }

        let system_locale = std::env::var("LANG")
            .or_else(|_| std::env::var("LANGUAGE"))
            .or_else(|_| std::env::var("LC_ALL"))
            .unwrap_or_default()
            .to_ascii_lowercase();

        if system_locale.starts_with("zh") {
            Self::SimplifiedChinese
        } else if system_locale.starts_with("es") {
            Self::Spanish
        } else {
            Self::English
        }
    }

    pub fn save(self) {
        let path = settings_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let settings = Settings { language: self };
        if let Ok(contents) = serde_json::to_string_pretty(&settings) {
            let _ = fs::write(path, contents);
        }
    }

    pub fn text<'a>(self, source: &'a str) -> &'a str {
        if self == Self::English {
            return source;
        }


        if self == Self::Spanish {
            return match source {
                "Wallet" => "Wallet",
                "Passcode" => "Código",
                "Help" => "Ayuda",
                "Refresh" => "Actualizar",
                "Auto (USB preferred)" => "Automático (USB preferido)",
                "USB only" => "Solo USB",
                "WiFi only" => "Solo WiFi",
                "No device" => "Sin dispositivo",
                "Ready" => "Listo",
                "Unavailable" => "No disponible",
                "Logs" => "Registros",
                "Logs [x]" => "Registros [x]",
                "Copy Logs" => "Copiar registros",
                "Save to File..." => "Guardar en archivo...",
                "Clear" => "Limpiar",
                "No events logged yet." => "Aún no hay eventos registrados.",
                "Language" => "Idioma",
                "English" => "Inglés",
                "Spanish" => "Español",
                "Simplified Chinese" => "Chino simplificado",
                "Language changed." => "Idioma cambiado.",
                "Transport mode" => "Modo de conexión",
                "entries" => "entradas",
                "Ready. Connect iPhone via USB or paired WiFi and unlock it." => "Listo. Conecta el iPhone por USB o WiFi emparejado y desbloquéalo.",
                "No iPhone connected via USB or paired WiFi." => "No hay ningún iPhone conectado por USB o WiFi emparejado.",
                "Please select a connected iPhone." => "Selecciona un iPhone conectado.",
                "Disconnect the USB cable and refresh to guarantee the full AirTraffic path uses WiFi." => "Desconecta el cable USB y actualiza para garantizar que toda la conexión AirTraffic use WiFi.",
                "Please enter or scan a target card hash." => "Introduce o escanea el hash de la tarjeta.",
                "Please choose a card skin image first." => "Primero selecciona una imagen para la tarjeta.",
                "Crop position updated." => "Posición de recorte actualizada.",
                "Scanning syslog... Open Wallet or tap your card on iPhone." => "Escaneando syslog... Abre Wallet o toca tu tarjeta en el iPhone.",
                "Syslog scanning stopped." => "Escaneo de syslog detenido.",
                "Writing card skin to iPhone..." => "Aplicando diseño de tarjeta al iPhone...",
                "Please select a .passthm theme file first." => "Primero selecciona un archivo de tema .passthm.",
                "Writing passcode theme buttons..." => "Aplicando botones del tema de código...",
                "Syslog scan finished" => "Escaneo de syslog finalizado",
                "Card skin successfully flashed! Force quit Wallet on iPhone and reopen it." => "¡Diseño de tarjeta aplicado correctamente! Fuerza el cierre de Wallet en el iPhone y vuelve a abrirlo.",
                "Passcode theme applied! Lock your iPhone to view the new keypad." => "¡Tema de código aplicado! Bloquea el iPhone para ver el nuevo teclado.",
                "Card skin updated successfully!" => "¡Diseño de tarjeta actualizado correctamente!",
                "Restoring original card face..." => "Restaurando diseño original de la tarjeta...",
                "Restoring original card artwork..." => "Restaurando imagen original de la tarjeta...",
                "Clearing .cache cache..." => "Limpiando caché .cache...",
                "Clearing .pkcache cache..." => "Limpiando caché .pkcache...",
                "Original card face restored successfully!" => "¡Diseño original de la tarjeta restaurado correctamente!",
                "Original card face restored. Force close Wallet and reopen it." => "Diseño original restaurado. Fuerza el cierre de Wallet y vuelve a abrirlo.",
                "Original card backup not found." => "No se encontró la copia de seguridad de la tarjeta original.",
                "Apply a card skin once to create an original backup." => "Aplica un diseño una vez para crear una copia del original.",
                "Passcode theme applied successfully!" => "¡Tema de código aplicado correctamente!",
                "Failed to parse theme:" => "No se pudo procesar el tema:",
                "Card captured" => "Tarjeta capturada",
                "Loaded" => "Cargado",
                "target:" => "objetivo:",
                "lang:" => "idioma:",
                "bold:" => "negrita:",
                "ON" => "ACTIVADO",
                "OFF" => "DESACTIVADO",
                "Error: " => "Error: ",
                "Found" => "Encontrados",
                "connected device(s); transport mode:" => "dispositivo(s) conectado(s); modo de conexión:",
                "Could not enumerate devices:" => "No se pudieron enumerar los dispositivos:",
                "Selected iPhone has no" => "El iPhone seleccionado no tiene conexión",
                "connection. Refresh devices or change transport mode." => "disponible. Actualiza los dispositivos o cambia el modo de conexión.",
                "Scanning syslog..." => "Escaneando syslog...",
                "Open Wallet on iPhone and tap your card" => "Abre Wallet en el iPhone y toca tu tarjeta",
                "Stop" => "Detener",
                "Scan" => "Escanear",
                "Card Configuration" => "Configuración de tarjeta",
                "Target your card and choose replacement artwork" => "Selecciona tu tarjeta y elige la imagen de reemplazo",
                "Target Card Hash" => "Hash de la tarjeta",
                "Base64 pass hash..." => "Hash de tarjeta en Base64...",
                "Saved cards" => "Tarjetas guardadas",
                "Select..." => "Seleccionar...",
                "Card Skin Artwork" => "Diseño de la tarjeta",
                "PNG, JPG, WebP - auto-scaled to 1536x969" => "PNG, JPG, WebP - escalado automáticamente a 1536x969",
                "Drag inside the preview to reposition the crop." => "Arrastra dentro de la vista previa para ajustar el recorte.",
                "Choose Image..." => "Elegir imagen...",
                "Export PNG" => "Exportar PNG",
                "Write to iPhone" => "Escribir en iPhone",
                "Apply Card Skin" => "Aplicar diseño",
                "Restore Original" => "Restaurar original",
                "connect iPhone" => "conectar iPhone",
                "choose available transport" => "elegir conexión disponible",
                "enter card hash" => "introducir hash de tarjeta",
                "choose image" => "elegir imagen",
                "select theme" => "seleccionar tema",
                "Need: " => "Falta: ",
                "Wallet Preview" => "Vista previa de Wallet",
                "1536 x 969 px pass canvas" => "Lienzo de tarjeta de 1536 x 969 px",
                "No artwork loaded" => "No hay imagen cargada",
                "No image" => "Sin imagen",
                "After applying, force close Apple Wallet and reopen it." => "Después de aplicar, fuerza el cierre de Apple Wallet y vuelve a abrirlo.",
                "Passcode Theme" => "Tema del código de bloqueo",
                "Custom lockscreen keypad from Cowabunga or Nugget" => "Teclado de bloqueo personalizado de Cowabunga o Nugget",
                "Theme Package" => "Paquete de tema",
                "Choose a .passthm archive containing dialer artwork" => "Selecciona un archivo .passthm que contenga las imágenes del teclado",
                "Choose .passthm..." => "Elegir .passthm...",
                "assets" => "recursos",
                "Target iOS Cache" => "Caché de iOS objetivo",
                "Select cache format based on connected iOS version" => "Selecciona el formato de caché según la versión de iOS conectada",
                "Auto (TelephonyUI-10)" => "Automático (TelephonyUI-10)",
                "TelephonyUI-10 (iOS 18+)" => "TelephonyUI-10 (iOS 18+)",
                "TelephonyUI-9 (iOS 16-17)" => "TelephonyUI-9 (iOS 16-17)",
                "TelephonyUI-8 (Legacy)" => "TelephonyUI-8 (Antiguo)",
                "Keypad Language" => "Idioma del teclado",
                "Subtext alphabet layout (English, Russian, Ukrainian, Japanese, or Universal)" => "Distribución de letras secundarias (inglés, ruso, ucraniano, japonés o universal)",
                "Russian" => "Ruso",
                "Ukrainian" => "Ucraniano",
                "Japanese" => "Japonés",
                "All Languages (Universal)" => "Todos los idiomas (Universal)",
                "Bold Text (iOS Accessibility)" => "Texto en negrita (Accesibilidad de iOS)",
                "Generates *-bold.png for devices with Bold Text turned ON in iPhone Settings -> Display" => "Genera *-bold.png para dispositivos con Texto en negrita activado en Ajustes del iPhone -> Pantalla",
                "Apply Passcode Theme" => "Aplicar tema de código",
                "Keypad Preview" => "Vista previa del teclado",
                "Dialer button artwork" => "Diseño de botones del teclado",
                "No theme loaded" => "No hay tema cargado",
                "3x4 Keypad" => "Teclado 3x4",
                "No theme" => "Sin tema",
                "After applying, lock your iPhone to see the new keypad." => "Después de aplicar, bloquea el iPhone para ver el nuevo teclado.",
                "Setup & Card Hash Guide" => "Guía de configuración y hash de tarjeta",
                "Everything you need to connect and capture your card" => "Todo lo necesario para conectar y capturar tu tarjeta",
                "Prerequisites" => "Requisitos previos",
                "- 64-bit iTunes or Apple Mobile Device Support installed" => "- iTunes de 64 bits o Apple Mobile Device Support instalado",
                "- First-time setup: connect by USB and tap \"Trust this Computer\"" => "- Primera configuración: conecta por USB y pulsa \"Confiar en este ordenador\"",
                "- WiFi: enable WiFi sync, then use the same local network" => "- WiFi: activa la sincronización WiFi y usa la misma red local",
                "- Select Auto, USB only, or WiFi only in the top bar" => "- Selecciona Automático, Solo USB o Solo WiFi en la barra superior",
                "Finding Your Card Hash" => "Cómo encontrar el hash de tu tarjeta",
                "1. Click \"Scan\" in the Wallet tab" => "1. Pulsa \"Escanear\" en la pestaña Wallet",
                "2. Open Apple Wallet on your iPhone" => "2. Abre Apple Wallet en tu iPhone",
                "3. Tap the card you want to customize" => "3. Toca la tarjeta que deseas personalizar",
                "4. AirCard captures the pass hash automatically" => "4. La aplicación captura automáticamente el hash de la tarjeta",
                "5. Click \"Stop\" once detected" => "5. Pulsa \"Detener\" cuando sea detectada",
                "Activation & Theme Guide" => "Guía de aplicación y temas",
                "Applying skins and dialer keypad packages" => "Aplicación de diseños y paquetes de teclado",
                "Activating Apple Wallet Skin" => "Aplicar diseño de Apple Wallet",
                "1. Click \"Apply Card Skin\" and wait for completion" => "1. Pulsa \"Aplicar diseño\" y espera a que termine",
                "2. Open App Switcher on iPhone (swipe up from bottom)" => "2. Abre el selector de apps del iPhone (desliza desde abajo)",
                "3. Force close Apple Wallet by swiping up on it" => "3. Fuerza el cierre de Apple Wallet deslizándolo hacia arriba",
                "4. Reopen Wallet - your new skin appears!" => "4. Vuelve a abrir Wallet: ¡aparecerá tu nuevo diseño!",
                "Passcode Themes (.passthm)" => "Temas de código (.passthm)",
                "- Compatible with Cowabunga & Nugget theme packages" => "- Compatible con paquetes de temas Cowabunga y Nugget",
                "- iOS 18+: Select \"Auto (TelephonyUI-10)\"" => "- iOS 18+: selecciona \"Automático (TelephonyUI-10)\"",
                "- iOS 16-17: Select \"TelephonyUI-9\"" => "- iOS 16-17: selecciona \"TelephonyUI-9\"",
                "- Lock screen to verify your updated keypad artwork" => "- Bloquea la pantalla para comprobar el nuevo diseño del teclado",
                _ => source,
            };
        }

        match source {
            "Wallet" => "钱包",
            "Passcode" => "锁屏密码",
            "Help" => "帮助",
            "Refresh" => "刷新",
            "Auto (USB preferred)" => "自动（优先 USB）",
            "USB only" => "仅 USB",
            "WiFi only" => "仅 WiFi",
            "No device" => "没有设备",
            "Ready" => "就绪",
            "Unavailable" => "不可用",
            "Logs" => "日志",
            "Logs [x]" => "日志 [x]",
            "Copy Logs" => "复制日志",
            "Save to File..." => "保存到文件...",
            "Clear" => "清空",
            "No events logged yet." => "暂无日志记录。",
            "Language" => "语言",
            "English" => "英语",
            "Simplified Chinese" => "简体中文",
            "Language changed." => "语言已切换。",
            "Transport mode" => "连接方式",
            "entries" => "条记录",
            "Ready. Connect iPhone via USB or paired WiFi and unlock it." => "就绪。请通过 USB 或已配对的 WiFi 连接并解锁 iPhone。",
            "No iPhone connected via USB or paired WiFi." => "未检测到通过 USB 或已配对 WiFi 连接的 iPhone。",
            "Please select a connected iPhone." => "请选择已连接的 iPhone。",
            "Disconnect the USB cable and refresh to guarantee the full AirTraffic path uses WiFi." => "请断开 USB 线并刷新，以确保完整的 AirTraffic 路径使用 WiFi。",
            "Please enter or scan a target card hash." => "请输入或扫描目标卡片 Hash。",
            "Please choose a card skin image first." => "请先选择卡片皮肤图片。",
            "Crop position updated." => "裁切位置已更新。",
            "Scanning syslog... Open Wallet or tap your card on iPhone." => "正在扫描 syslog... 请在 iPhone 上打开钱包并点击卡片。",
            "Syslog scanning stopped." => "syslog 扫描已停止。",
            "Writing card skin to iPhone..." => "正在将卡片皮肤写入 iPhone...",
            "Please select a .passthm theme file first." => "请先选择一个 .passthm 主题文件。",
            "Writing passcode theme buttons..." => "正在将锁屏密码主题按钮写入 iPhone...",
            "Syslog scan finished" => "syslog 扫描完成",
            "Card skin successfully flashed! Force quit Wallet on iPhone and reopen it." => "卡片皮肤应用成功！请在 iPhone 上强制关闭并重新打开 Wallet。",
            "Passcode theme applied! Lock your iPhone to view the new keypad." => "锁屏密码主题应用成功！锁定 iPhone 查看新的键盘样式。",
            "Card skin updated successfully!" => "卡片皮肤更新成功！",
            "Restoring original card face..." => "正在恢复原卡面...",
            "Restoring original card artwork..." => "正在恢复原卡面图片...",
            "Clearing .cache cache..." => "正在清理 .cache 缓存...",
            "Clearing .pkcache cache..." => "正在清理 .pkcache 缓存...",
            "Original card face restored successfully!" => "原卡面恢复成功！",
            "Original card face restored. Force close Wallet and reopen it." => "原卡面已恢复。请强制关闭并重新打开 Wallet。",
            "Original card backup not found." => "未找到原卡面备份。",
            "Apply a card skin once to create an original backup." => "先应用一次卡片皮肤以创建原卡面备份。",
            "Passcode theme applied successfully!" => "锁屏密码主题应用成功！",
            "Failed to parse theme:" => "解析主题失败：",
            "Card captured" => "已捕获卡片",
            "Loaded" => "已加载",
            "target:" => "目标：",
            "lang:" => "语言：",
            "bold:" => "粗体：",
            "ON" => "开",
            "OFF" => "关",
            "Error: " => "错误：",
            "Found" => "已找到",
            "connected device(s); transport mode:" => "台已连接设备；连接方式：",
            "Could not enumerate devices:" => "无法枚举设备：",
            "Selected iPhone has no" => "选中的 iPhone 没有",
            "connection. Refresh devices or change transport mode." => "连接。请刷新设备或更改连接方式。",
            "Scanning syslog..." => "正在扫描 syslog...",
            "Open Wallet on iPhone and tap your card" => "请在 iPhone 上打开钱包并点击目标卡片",
            "Stop" => "停止",
            "Scan" => "扫描",
            "Card Configuration" => "卡片配置",
            "Target your card and choose replacement artwork" => "选择目标卡片和替换图片",
            "Target Card Hash" => "目标卡片 Hash",
            "Base64 pass hash..." => "Base64 卡片 Hash...",
            "Saved cards" => "已保存的卡片",
            "Select..." => "请选择...",
            "Card Skin Artwork" => "卡片皮肤图片",
            "PNG, JPG, WebP - auto-scaled to 1536x969" => "PNG、JPG、WebP，将自动缩放到 1536x969",
            "Drag inside the preview to reposition the crop." => "在预览区域内拖动以调整裁切位置。",
            "Choose Image..." => "选择图片...",
            "Export PNG" => "导出 PNG",
            "Write to iPhone" => "写入 iPhone",
            "Apply Card Skin" => "应用卡片皮肤",
            "Restore Original" => "恢复原卡面",
            "connect iPhone" => "连接 iPhone",
            "choose available transport" => "选择可用的连接方式",
            "enter card hash" => "输入卡片 Hash",
            "choose image" => "选择图片",
            "select theme" => "选择主题",
            "Need: " => "需要：",
            "Wallet Preview" => "钱包预览",
            "1536 x 969 px pass canvas" => "1536 x 969 像素卡片画布",
            "No artwork loaded" => "尚未加载图片",
            "No image" => "没有图片",
            "After applying, force close Apple Wallet and reopen it." => "应用后，请强制关闭 Apple Wallet 并重新打开。",
            "Passcode Theme" => "锁屏密码主题",
            "Custom lockscreen keypad from Cowabunga or Nugget" => "来自 Cowabunga 或 Nugget 的自定义锁屏键盘",
            "Theme Package" => "主题包",
            "Choose a .passthm archive containing dialer artwork" => "选择包含拨号键盘图片的 .passthm 压缩包",
            "Choose .passthm..." => "选择 .passthm...",
            "assets" => "个资源",
            "Target iOS Cache" => "目标 iOS 缓存",
            "Select cache format based on connected iOS version" => "根据连接设备的 iOS 版本选择缓存格式",
            "Auto (TelephonyUI-10)" => "自动（TelephonyUI-10）",
            "TelephonyUI-10 (iOS 18+)" => "TelephonyUI-10（iOS 18+）",
            "TelephonyUI-9 (iOS 16-17)" => "TelephonyUI-9（iOS 16-17）",
            "TelephonyUI-8 (Legacy)" => "TelephonyUI-8（旧版本）",
            "Keypad Language" => "键盘语言",
            "Subtext alphabet layout (English, Russian, Ukrainian, Japanese, or Universal)" => "按键副文字母布局（英语、俄语、乌克兰语、日语或通用）",
            "Russian" => "俄语",
            "Ukrainian" => "乌克兰语",
            "Japanese" => "日语",
            "All Languages (Universal)" => "全部语言（通用）",
            "Bold Text (iOS Accessibility)" => "粗体文字（iOS 辅助功能）",
            "Generates *-bold.png for devices with Bold Text turned ON in iPhone Settings -> Display" => "为 iPhone 设置 -> 显示与亮度中启用粗体文字的设备生成 *-bold.png",
            "Apply Passcode Theme" => "应用锁屏密码主题",
            "Keypad Preview" => "键盘预览",
            "Dialer button artwork" => "拨号按钮图片",
            "No theme loaded" => "尚未加载主题",
            "3x4 Keypad" => "3x4 键盘",
            "No theme" => "没有主题",
            "After applying, lock your iPhone to see the new keypad." => "应用后，请锁定 iPhone 查看新的键盘样式。",
            "Setup & Card Hash Guide" => "设置与卡片 Hash 指南",
            "Everything you need to connect and capture your card" => "连接设备并捕获卡片所需的全部信息",
            "Prerequisites" => "使用前准备",
            "- 64-bit iTunes or Apple Mobile Device Support installed" => "- 已安装 64 位 iTunes 或 Apple Mobile Device Support",
            "- First-time setup: connect by USB and tap \"Trust this Computer\"" => "- 首次使用：通过 USB 连接并点击“信任此电脑”",
            "- WiFi: enable WiFi sync, then use the same local network" => "- WiFi：启用 WiFi 同步，并确保双方处于同一局域网",
            "- Select Auto, USB only, or WiFi only in the top bar" => "- 在顶部栏选择自动、仅 USB 或仅 WiFi",
            "Finding Your Card Hash" => "查找卡片 Hash",
            "1. Click \"Scan\" in the Wallet tab" => "1. 在钱包页点击“扫描”",
            "2. Open Apple Wallet on your iPhone" => "2. 在 iPhone 上打开 Apple Wallet",
            "3. Tap the card you want to customize" => "3. 点击要自定义的卡片",
            "4. AirCard captures the pass hash automatically" => "4. AirCard 会自动捕获卡片 Hash",
            "5. Click \"Stop\" once detected" => "5. 检测到后点击“停止”",
            "Activation & Theme Guide" => "应用与主题指南",
            "Applying skins and dialer keypad packages" => "应用卡片皮肤和拨号键盘主题",
            "Activating Apple Wallet Skin" => "应用 Apple Wallet 皮肤",
            "1. Click \"Apply Card Skin\" and wait for completion" => "1. 点击“应用卡片皮肤”并等待完成",
            "2. Open App Switcher on iPhone (swipe up from bottom)" => "2. 在 iPhone 上打开 App 切换器（从底部向上滑动）",
            "3. Force close Apple Wallet by swiping up on it" => "3. 向上滑动并强制关闭 Apple Wallet",
            "4. Reopen Wallet - your new skin appears!" => "4. 重新打开钱包，即可看到新的皮肤！",
            "Passcode Themes (.passthm)" => "锁屏密码主题（.passthm）",
            "- Compatible with Cowabunga & Nugget theme packages" => "- 兼容 Cowabunga 和 Nugget 主题包",
            "- iOS 18+: Select \"Auto (TelephonyUI-10)\"" => "- iOS 18+：选择“自动（TelephonyUI-10）”",
            "- iOS 16-17: Select \"TelephonyUI-9\"" => "- iOS 16-17：选择“TelephonyUI-9”",
            "- Lock screen to verify your updated keypad artwork" => "- 锁定屏幕以查看更新后的键盘图片",
            _ => source,
        }
    }

    pub fn option_label(self, option: Self) -> &'static str {
        match (self, option) {
            (Self::English, Self::English) => "English",
            (Self::English, Self::Spanish) => "Spanish",
            (Self::English, Self::SimplifiedChinese) => "Simplified Chinese",
            (Self::Spanish, Self::English) => "Inglés",
            (Self::Spanish, Self::Spanish) => "Español",
            (Self::Spanish, Self::SimplifiedChinese) => "Chino simplificado",
            (Self::SimplifiedChinese, Self::English) => "英语",
            (Self::SimplifiedChinese, Self::Spanish) => "西班牙语",
            (Self::SimplifiedChinese, Self::SimplifiedChinese) => "简体中文",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    language: Language,
}

fn settings_path() -> PathBuf {
    let local_app_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| r"C:\Users\Default\AppData\Local".to_string());
    PathBuf::from(local_app_data)
        .join("AirCard")
        .join("settings.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chinese_translation_has_english_fallback() {
        assert_eq!(Language::SimplifiedChinese.text("Wallet"), "钱包");
        assert_eq!(
            Language::SimplifiedChinese.text("unknown string"),
            "unknown string"
        );
    }

    #[test]
    fn spanish_translation_has_english_fallback() {
        assert_eq!(Language::Spanish.text("Help"), "Ayuda");
        assert_eq!(
            Language::Spanish.text("unknown string"),
            "unknown string"
        );
    }

    #[test]
    fn language_option_labels_follow_current_language() {
        assert_eq!(
            Language::English.option_label(Language::SimplifiedChinese),
            "Simplified Chinese"
        );
        assert_eq!(
            Language::Spanish.option_label(Language::Spanish),
            "Español"
        );
        assert_eq!(
            Language::SimplifiedChinese.option_label(Language::SimplifiedChinese),
            "简体中文"
        );
    }
}
