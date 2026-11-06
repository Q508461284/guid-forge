use gpui_kit::component::{
    Sizable as _,
    button::{Button, ButtonVariants},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex,
};
use gpui_kit::gpui::prelude::*;
use gpui_kit::gpui::*;
use std::time::Duration;
use uuid::Uuid;

// ── 格式枚举 ──────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuidFormat {
    Hyphenated,
    Simple,
    Braced,
    Urn,
    WindowsClsid,
    Parenthesized,
    StructInit,
}

impl GuidFormat {
    const ALL: [GuidFormat; 7] = [
        Self::Hyphenated,
        Self::Simple,
        Self::Braced,
        Self::Urn,
        Self::WindowsClsid,
        Self::Parenthesized,
        Self::StructInit,
    ];

    fn as_str(self) -> &'static str {
        match self {
            Self::Hyphenated => "连字符格式",
            Self::Simple => "简单格式",
            Self::Braced => "花括号格式",
            Self::Urn => "URN 格式",
            Self::WindowsClsid => "Windows CLSID",
            Self::Parenthesized => "圆括号格式",
            Self::StructInit => "C 结构体格式",
        }
    }

    fn format(self, uuid: &Uuid) -> String {
        match self {
            Self::Hyphenated => uuid.hyphenated().to_string(),
            Self::Simple => uuid.simple().to_string(),
            Self::Braced => uuid.braced().to_string(),
            Self::Urn => uuid.urn().to_string(),
            Self::WindowsClsid => uuid.braced().to_string(),
            Self::Parenthesized => format!("({})", uuid.hyphenated()),
            Self::StructInit => {
                let bytes = uuid.as_bytes();
                format!(
                    "{{0x{:08X},0x{:04X},0x{:04X},{{0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X}}}}}",
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    u16::from_be_bytes([bytes[4], bytes[5]]),
                    u16::from_be_bytes([bytes[6], bytes[7]]),
                    bytes[8],
                    bytes[9],
                    bytes[10],
                    bytes[11],
                    bytes[12],
                    bytes[13],
                    bytes[14],
                    bytes[15],
                )
            }
        }
    }
}

// ── 每行条目 ─────────────────────────────────────────────
struct GuidEntry {
    format: GuidFormat,
    uuid: Uuid,
    input: Entity<InputState>,
}

struct GuidGenerator {
    entries: Vec<GuidEntry>,
    copied: Vec<bool>,
}

impl GuidGenerator {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let entries = GuidFormat::ALL
            .iter()
            .map(|&format| {
                let uuid = Uuid::new_v4();
                let text = format.format(&uuid);
                let input = cx.new(|cx| InputState::new(window, cx).default_value(text));
                GuidEntry {
                    format,
                    uuid,
                    input,
                }
            })
            .collect::<Vec<_>>();
        let copied = vec![false; entries.len()];
        Self { entries, copied }
    }

    fn regenerate_entry(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let entry = &mut self.entries[index];
        entry.uuid = Uuid::new_v4();
        let text = entry.format.format(&entry.uuid);
        let input = entry.input.clone();
        input.update(cx, |state, cx| {
            state.set_value(text, window, cx);
        });
        cx.notify();
    }

    fn copy_entry(&mut self, index: usize, cx: &mut Context<Self>) {
        let text = {
            let entry = &self.entries[index];
            entry.format.format(&entry.uuid)
        };
        cx.write_to_clipboard(ClipboardItem::new_string(text));

        self.copied[index] = true;
        cx.notify();

        let task = cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(1500))
                .await;
            this.update(cx, |this, cx| {
                this.copied[index] = false;
                cx.notify();
            })
            .ok();
        });
        task.detach();
    }
}

// ── 渲染 ─────────────────────────────────────────────────
impl Render for GuidGenerator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // macOS 色板
        let window_bg = rgb(0xF2F2F7);
        let card_bg = rgb(0xFFFFFF);
        let card_border = rgb(0xE5E5EA);

        let label_color = rgb(0x86868B); // 左侧标签：浅灰
        let text_color = rgb(0x1D1D1F); // 标题和主文本：深黑
        let code_color = rgb(0x0055CC); // GUID 文本：深蓝
        let accent_blue = rgb(0x007AFF); // 复制按钮：系统蓝
        let btn_gray = rgb(0x333333); // 生成按钮：深灰（明确可点击）

        let mut rows = Vec::with_capacity(self.entries.len());

        for i in 0..self.entries.len() {
            let format = self.entries[i].format;
            let input = self.entries[i].input.clone();
            let copied = self.copied[i];

            let row = h_flex()
                .gap_3()
                .items_center()
                .py_1()
                // 左侧标签
                .child(
                    div().w(px(110.0)).child(
                        Label::new(format.as_str())
                            .text_sm()
                            .text_color(label_color),
                    ),
                )
                // 中间输入框
                .child(
                    div().flex_1().child(
                        Input::new(&input)
                            .font_family("monospace")
                            .text_sm()
                            .readonly(true)
                            .text_color(code_color), // 尝试设置输入框文字颜色
                    ),
                )
                // “生成”按钮
                .child(
                    Button::new(format!("regen-{}", i))
                        .small()
                        .ghost()
                        // 换成深灰色，明确表示可点击。若编译报错请注释掉此行
                        .text_color(btn_gray)
                        .label("生成")
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.regenerate_entry(i, window, cx);
                        })),
                    /*
                    // 备选方案：如果希望它看起来更像一个实体按钮，可以去掉 .ghost() 并加上底色
                    // Button::new(format!("regen-{}", i))
                    //     .small()
                    //     .bg(rgb(0xE5E5EA))
                    //     .text_color(rgb(0x1D1D1F))
                    //     .label("生成")
                     */
                )
                // “复制”按钮
                .child(
                    Button::new(format!("copy-{}", i))
                        .small()
                        .ghost()
                        .w(px(64.0))
                        .text_color(accent_blue) // 亮蓝色，表示主要操作
                        .label(if copied { "已复制" } else { "复制" })
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.copy_entry(i, cx);
                        })),
                );

            rows.push(row);
        }

        v_flex()
            .size_full()
            .pt_6()
            .px_8()
            .pb_6()
            .gap_6()
            .bg(window_bg)
            // 标题区
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Label::new("GUID 生成器")
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(text_color),
                    )
                    .child(
                        Label::new("七种格式 · 每行独立生成与复制")
                            .text_sm()
                            .text_color(label_color),
                    ),
            )
            // 卡片容器
            .child(
                v_flex()
                    .bg(card_bg)
                    .rounded_2xl()
                    .border_1()
                    .border_color(card_border)
                    .shadow_sm()
                    .p_4()
                    .gap_1()
                    .children(rows),
            )
    }
}

// ── 入口 ─────────────────────────────────────────────────
fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx| {
            gpui_kit::init(cx);

            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(200.0), px(150.0)),
                    size: size(px(960.0), px(540.0)),
                })),
                is_resizable: false,
                ..Default::default()
            };

            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| GuidGenerator::new(window, cx))
            })
            .expect("Failed to open window");
        });
}
