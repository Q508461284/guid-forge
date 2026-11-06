fn main() {
    // 仅在目标平台为 Windows 时执行
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico"); // 设置图标路径
        res.set_language(0x0804); // 设置资源语言为简体中文 (0x0804)

        res.compile().unwrap();
    }
}
