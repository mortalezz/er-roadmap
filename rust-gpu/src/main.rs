use std::sync::Arc;
#[path = "../../rust-port/src/compiler.rs"]
mod compiler;
#[path = "../../rust-port/src/generated_catalog.rs"]
mod generated_catalog;
#[path = "../../rust-port/src/models.rs"]
mod models;
#[path = "../../rust-port/src/session.rs"]
mod session;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;

use eframe::egui::{
    self, Color32, FontData, FontDefinitions, FontFamily, FontId, Margin, Pos2, Rect, RichText,
    Rounding, Sense, Stroke, TextureHandle, TextureOptions, Vec2,
};

use compiler::{
    compile_run_plan, search_catalog, toggle_order_item, toggle_step_key,
};
use models::{OrderEntry, RunPlan};
use session::{decode_session_cookies, encode_session_cookies, export_json, parse_imported_json};

// Exact Tailwind Zinc + Amber + Emerald + Red palette
const ZINC_950: Color32 = Color32::from_rgb(9, 9, 11);
const ZINC_900: Color32 = Color32::from_rgb(24, 24, 27);
const ZINC_900_SOFT: Color32 = Color32::from_rgb(19, 19, 22);
const ZINC_800: Color32 = Color32::from_rgb(39, 39, 42);
const ZINC_700: Color32 = Color32::from_rgb(63, 63, 70);
const ZINC_400: Color32 = Color32::from_rgb(161, 161, 170);
const ZINC_500: Color32 = Color32::from_rgb(113, 113, 122);
const ZINC_100: Color32 = Color32::from_rgb(244, 244, 245);

const AMBER_400: Color32 = Color32::from_rgb(251, 191, 36);
const AMBER_300: Color32 = Color32::from_rgb(252, 211, 77);
const AMBER_950: Color32 = Color32::from_rgb(56, 22, 4);
const AMBER_BORDER: Color32 = Color32::from_rgb(180, 83, 9);

const EMERALD_400: Color32 = Color32::from_rgb(52, 211, 153);
const EMERALD_300: Color32 = Color32::from_rgb(110, 231, 183);
const EMERALD_950: Color32 = Color32::from_rgb(2, 44, 34);
const EMERALD_BORDER: Color32 = Color32::from_rgb(6, 95, 70);

const RED_300: Color32 = Color32::from_rgb(252, 165, 165);
const RED_200: Color32 = Color32::from_rgb(254, 202, 202);
const RED_950: Color32 = Color32::from_rgb(58, 10, 10);
const RED_BORDER: Color32 = Color32::from_rgb(185, 28, 28);

fn state_file_path() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let dir = PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("ER-Roadmap");
        let _ = fs::create_dir_all(&dir);
        dir.join("state.v2")
    } else {
        PathBuf::from("er-roadmap-state.v2")
    }
}

/// Loads and decodes high-res PNG emblem artwork into an egui GPU texture
fn build_emblem_texture(ctx: &egui::Context) -> TextureHandle {
    let png_bytes = include_bytes!("../../static/icon-192.png");
    let img = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png)
        .expect("Failed to decode static/icon-192.png");
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        rgba.as_raw(),
    );
    return ctx.load_texture("er_emblem", color_image, TextureOptions::LINEAR);
}

fn configure_apple_fonts_and_theme(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Load native macOS SF Pro font so typography matches Tailwind/macOS natively
    let sf_candidates = [
        "/System/Library/Fonts/SFNS.ttf",
        "/System/Library/Fonts/SFNSDisplay.ttf",
        "/System/Library/Fonts/HelveticaNeue.ttc",
    ];
    for path in &sf_candidates {
        if let Ok(bytes) = fs::read(path) {
            fonts
                .font_data
                .insert("sf_pro".to_owned(), FontData::from_owned(bytes));
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "sf_pro".to_owned());
            break;
        }
    }

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(10.0, 6.0);
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style.spacing.scroll.bar_width = 0.0; // Clean scrollbar-free look matching mobile/web

    let mut vis = egui::Visuals::dark();
    vis.panel_fill = ZINC_950;
    vis.window_fill = ZINC_900;
    vis.faint_bg_color = ZINC_900_SOFT;
    // Fix the blinding white TextEdit input background!
    vis.extreme_bg_color = ZINC_950;
    vis.code_bg_color = ZINC_950;
    vis.override_text_color = Some(ZINC_100);

    vis.selection.bg_fill = Color32::from_rgba_premultiplied(245, 158, 11, 55);
    vis.selection.stroke = Stroke::new(1.5_f32, AMBER_400);

    vis.widgets.noninteractive.bg_fill = ZINC_900;
    vis.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, ZINC_800);
    vis.widgets.noninteractive.rounding = Rounding::same(8.0);
    vis.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, ZINC_100);

    vis.widgets.inactive.bg_fill = ZINC_900;
    vis.widgets.inactive.weak_bg_fill = ZINC_900;
    vis.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, ZINC_700);
    vis.widgets.inactive.rounding = Rounding::same(6.0);
    vis.widgets.inactive.fg_stroke = Stroke::new(1.2_f32, ZINC_100);

    vis.widgets.hovered.bg_fill = ZINC_800;
    vis.widgets.hovered.weak_bg_fill = ZINC_800;
    vis.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, AMBER_400);
    vis.widgets.hovered.rounding = Rounding::same(6.0);

    vis.widgets.active.bg_fill = AMBER_950;
    vis.widgets.active.bg_stroke = Stroke::new(1.5_f32, AMBER_400);
    vis.widgets.active.rounding = Rounding::same(6.0);

    style.visuals = vis;
    ctx.set_style(style);
}

fn draw_pill(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32, border: Color32) {
    egui::Frame::none()
        .fill(bg)
        .stroke(Stroke::new(1.0_f32, border))
        .rounding(Rounding::same(5.0))
        .inner_margin(Margin::symmetric(7.0, 2.5))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(11.0).strong().color(fg));
        });
}

fn draw_gradient_progress_bar(ui: &mut egui::Ui, fraction: f32) {
    let desired_size = Vec2::new(ui.available_width(), 5.0);
    let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, Rounding::same(2.5), ZINC_800);
        let fill_w = (rect.width() * fraction.clamp(0.0, 1.0)).max(0.0);
        if fill_w > 1.0 {
            let steps = 24usize;
            let step_w = fill_w / (steps as f32);
            for i in 0..steps {
                let t = i as f32 / (steps as f32);
                // Smooth Amber (#f59e0b) -> Emerald (#34d399) gradient
                let r = (245.0 * (1.0 - t) + 52.0 * t) as u8;
                let g = (158.0 * (1.0 - t) + 211.0 * t) as u8;
                let b = (11.0 * (1.0 - t) + 153.0 * t) as u8;
                let seg = Rect::from_min_max(
                    Pos2::new(rect.min.x + (i as f32) * step_w, rect.min.y),
                    Pos2::new(rect.min.x + ((i + 1) as f32) * step_w + 0.5, rect.max.y),
                );
                painter.rect_filled(seg, Rounding::same(2.0), Color32::from_rgb(r, g, b));
            }
        }
    }
}

struct ErRoadmapGpuApp {
    order: Vec<OrderEntry>,
    steps: HashMap<String, bool>,
    search_query: String,
    plan: RunPlan,
    status_toast: Option<String>,
    emblem_tex: TextureHandle,
}

impl ErRoadmapGpuApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_apple_fonts_and_theme(&cc.egui_ctx);
        let emblem_tex = build_emblem_texture(&cc.egui_ctx);

        let mut order = Vec::new();
        let mut steps = HashMap::new();
        if let Ok(content) = fs::read_to_string(state_file_path()) {
            let mut lines = content.lines();
            let raw_o = lines.next().unwrap_or("");
            let raw_s = lines.next().unwrap_or("");
            let (o, s) = decode_session_cookies(raw_o, raw_s);
            order = o;
            steps = s;
        }
        let plan = compile_run_plan(&order, &steps);

        Self {
            order,
            steps,
            search_query: String::new(),
            plan,
            status_toast: None,
            emblem_tex,
        }
    }

    fn sync_and_recompile(&mut self) {
        let (c_o, c_s) = encode_session_cookies(&self.order, &self.steps);
        let _ = fs::write(state_file_path(), format!("{}\n{}\n", c_o, c_s));
        self.plan = compile_run_plan(&self.order, &self.steps);
    }
}

impl eframe::App for ErRoadmapGpuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Top Header Bar
        egui::TopBottomPanel::top("header_panel")
            .frame(
                egui::Frame::none()
                    .fill(ZINC_950)
                    .inner_margin(Margin::symmetric(24.0, 16.0))
                    .stroke(Stroke::new(1.0_f32, ZINC_800)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    egui::Frame::none()
                        .rounding(Rounding::same(11.0))
                        .stroke(Stroke::new(1.0_f32, AMBER_BORDER))
                        .fill(ZINC_950)
                        .inner_margin(Margin::same(0.0))
                        .show(ui, |ui| {
                            ui.image((self.emblem_tex.id(), Vec2::new(44.0, 44.0)));
                        });
                    ui.add_space(6.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Elden Ring Current Run Roadmap")
                                    .font(FontId::proportional(21.0))
                                    .strong()
                                    .color(AMBER_400),
                            );
                            draw_pill(
                                ui,
                                "PURE RUST GPU • SF PRO",
                                AMBER_300,
                                AMBER_950,
                                AMBER_BORDER,
                            );
                        });
                        ui.label(
                            RichText::new(
                                "Demand-driven geographical route & Point-of-No-Return solver (Base Game + Shadow of the Erdtree)",
                            )
                            .size(12.5)
                            .color(ZINC_400),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Reset Slate").size(12.0).strong().color(RED_300),
                                )
                                .fill(RED_950)
                                .stroke(Stroke::new(1.0_f32, RED_BORDER))
                                .rounding(Rounding::same(7.0)),
                            )
                            .clicked()
                        {
                            self.order.clear();
                            self.steps.clear();
                            self.sync_and_recompile();
                            self.status_toast = Some("Slate cleared".to_string());
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("📂 Import JSON")
                                        .size(12.0)
                                        .strong()
                                        .color(EMERALD_300),
                                )
                                .fill(ZINC_900)
                                .stroke(Stroke::new(1.0_f32, ZINC_700))
                                .rounding(Rounding::same(7.0)),
                            )
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JSON Roadmap", &["json"])
                                .pick_file()
                            {
                                match fs::read_to_string(&path) {
                                    Ok(raw) => {
                                        let (new_order, new_steps) = parse_imported_json(&raw);
                                        if !new_order.is_empty() || raw.contains("\"current_run_order\"") {
                                            self.order = new_order;
                                            self.steps = new_steps;
                                            self.sync_and_recompile();
                                            let name = path
                                                .file_name()
                                                .map(|n| n.to_string_lossy().to_string())
                                                .unwrap_or_else(|| "roadmap.json".to_string());
                                            self.status_toast = Some(format!("Imported {}", name));
                                        } else {
                                            self.status_toast = Some("Invalid or empty roadmap JSON".to_string());
                                        }
                                    }
                                    Err(e) => {
                                        self.status_toast = Some(format!("Failed to read file: {}", e));
                                    }
                                }
                            }
                        }

                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("💾 Export JSON")
                                        .size(12.0)
                                        .strong()
                                        .color(AMBER_300),
                                )
                                .fill(ZINC_900)
                                .stroke(Stroke::new(1.0_f32, ZINC_700))
                                .rounding(Rounding::same(7.0)),
                            )
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name("elden-ring-roadmap.json")
                                .add_filter("JSON Roadmap", &["json"])
                                .save_file()
                            {
                                let json = export_json(&self.order, &self.steps);
                                if let Err(e) = fs::write(&path, json) {
                                    self.status_toast = Some(format!("Export failed: {}", e));
                                } else {
                                    let name = path
                                        .file_name()
                                        .map(|n| n.to_string_lossy().to_string())
                                        .unwrap_or_else(|| "roadmap.json".to_string());
                                    self.status_toast = Some(format!("Exported to {}", name));
                                }
                            }
                        }
                    });
                });
            });

        // 2. Main Unified Scrollable Workspace (Search + Progress + 2-Column Grid, zero splitters!)
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(ZINC_950))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("unified_workspace_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .inner_margin(Margin::symmetric(24.0, 16.0))
                            .show(ui, |ui| {
                                let full_w = ui.available_width();

                                // Search Card
                                egui::Frame::none()
                                    .fill(ZINC_900)
                                    .stroke(Stroke::new(1.0_f32, ZINC_800))
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::same(14.0))
                                    .show(ui, |ui| {
                                        ui.set_min_width(full_w - 28.0);
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(
                                                    "ADD DESIRABLES TO CURRENT RUN ROADMAP (COMPILES GEOGRAPHICAL ROUTE & PONR GATES)",
                                                )
                                                .size(11.0)
                                                .strong()
                                                .color(AMBER_400),
                                            );
                                            if let Some(msg) = &self.status_toast {
                                                ui.with_layout(
                                                    egui::Layout::right_to_left(egui::Align::Center),
                                                    |ui| {
                                                        ui.label(
                                                            RichText::new(msg)
                                                                .size(11.5)
                                                                .color(EMERALD_400),
                                                        );
                                                    },
                                                );
                                            }
                                        });
                                        ui.add_space(6.0);

                                        // Dark Zinc Search Input Box
                                        egui::Frame::none()
                                            .fill(ZINC_950)
                                            .stroke(Stroke::new(1.0_f32, ZINC_700))
                                            .rounding(Rounding::same(7.0))
                                            .inner_margin(Margin::symmetric(10.0, 7.0))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    let w = (ui.available_width()
                                                        - if self.search_query.is_empty() {
                                                            8.0
                                                        } else {
                                                            65.0
                                                        })
                                                    .max(120.0);
                                                    let resp = ui.add_sized(
                                                        [w, 20.0],
                                                        egui::TextEdit::singleline(
                                                            &mut self.search_query,
                                                        )
                                                        .frame(false)
                                                        .text_color(ZINC_100)
                                                        .hint_text(
                                                            RichText::new("Type any Base Game or DLC item (e.g. Magic Scorpion, Bolt of Gransax, Florissax, Euporia)...")
                                                                .color(ZINC_500),
                                                        ),
                                                    );
                                                    if !self.search_query.is_empty() {
                                                        if ui
                                                            .add(
                                                                egui::Button::new(
                                                                    RichText::new("Clear")
                                                                        .size(11.0)
                                                                        .color(ZINC_400),
                                                                )
                                                                .fill(ZINC_800)
                                                                .rounding(Rounding::same(4.0)),
                                                            )
                                                            .clicked()
                                                        {
                                                            self.search_query.clear();
                                                            resp.request_focus();
                                                        }
                                                    }
                                                });
                                            });

                                        let q_trimmed = self.search_query.trim().to_string();
                                        if !q_trimmed.is_empty() {
                                            ui.add_space(8.0);
                                            let matches = search_catalog(&q_trimmed, 8);
                                            let ordered_ids: HashSet<String> = self
                                                .order
                                                .iter()
                                                .map(|i| i.item_id.clone())
                                                .collect();

                                            let mut add_id: Option<String> = None;
                                            let mut rem_id: Option<String> = None;

                                            if matches.is_empty() {
                                                ui.label(
                                                    RichText::new("No matching item in catalog.")
                                                        .size(12.5)
                                                        .color(ZINC_400),
                                                );
                                            } else {
                                                for r in matches {
                                                    let in_order = ordered_ids.contains(r.item_id);
                                                    egui::Frame::none()
                                                        .fill(ZINC_950)
                                                        .stroke(Stroke::new(1.0_f32, ZINC_800))
                                                        .rounding(Rounding::same(6.0))
                                                        .inner_margin(Margin::symmetric(10.0, 6.0))
                                                        .show(ui, |ui| {
                                                            ui.horizontal(|ui| {
                                                                ui.vertical(|ui| {
                                                                    ui.horizontal(|ui| {
                                                                        ui.label(
                                                                            RichText::new(r.name)
                                                                                .size(13.0)
                                                                                .strong()
                                                                                .color(ZINC_100),
                                                                        );
                                                                        if r.is_missable {
                                                                            draw_pill(
                                                                                ui,
                                                                                "Missable",
                                                                                AMBER_300,
                                                                                AMBER_950,
                                                                                AMBER_BORDER,
                                                                            );
                                                                        }
                                                                        if in_order {
                                                                            draw_pill(
                                                                                ui,
                                                                                "In Roadmap",
                                                                                EMERALD_300,
                                                                                EMERALD_950,
                                                                                EMERALD_BORDER,
                                                                            );
                                                                        }
                                                                    });
                                                                    ui.label(
                                                                        RichText::new(format!(
                                                                            "{} • {} • {}",
                                                                            r.campaign,
                                                                            r.category,
                                                                            r.region
                                                                        ))
                                                                        .size(11.5)
                                                                        .color(ZINC_400),
                                                                    );
                                                                });

                                                                ui.with_layout(
                                                                    egui::Layout::right_to_left(
                                                                        egui::Align::Center,
                                                                    ),
                                                                    |ui| {
                                                                        if in_order {
                                                                            if ui
                                                                                .add(
                                                                                    egui::Button::new(
                                                                                        RichText::new("Remove")
                                                                                            .size(11.5)
                                                                                            .color(RED_200),
                                                                                    )
                                                                                    .fill(RED_950)
                                                                                    .stroke(Stroke::new(
                                                                                        1.0_f32,
                                                                                        RED_BORDER,
                                                                                    )),
                                                                                )
                                                                                .clicked()
                                                                            {
                                                                                rem_id = Some(
                                                                                    r.item_id
                                                                                        .to_string(),
                                                                                );
                                                                            }
                                                                        } else if ui
                                                                            .add(
                                                                                egui::Button::new(
                                                                                    RichText::new(
                                                                                        "+ Add to Run",
                                                                                    )
                                                                                    .size(11.5)
                                                                                    .strong()
                                                                                    .color(ZINC_950),
                                                                                )
                                                                                .fill(AMBER_400)
                                                                                .stroke(Stroke::NONE),
                                                                            )
                                                                            .clicked()
                                                                        {
                                                                            add_id = Some(
                                                                                r.item_id.to_string(),
                                                                            );
                                                                        }
                                                                    },
                                                                );
                                                            });
                                                        });
                                                    ui.add_space(3.0);
                                                }
                                            }

                                            if let Some(id) = add_id {
                                                if !self.order.iter().any(|i| i.item_id == id) {
                                                    self.order.push(OrderEntry {
                                                        item_id: id,
                                                        is_collected: false,
                                                    });
                                                    self.search_query.clear();
                                                    self.sync_and_recompile();
                                                }
                                            }
                                            if let Some(id) = rem_id {
                                                self.order.retain(|i| i.item_id != id);
                                                self.sync_and_recompile();
                                            }
                                        }
                                    });

                                ui.add_space(12.0);

                                // Progress Counter Card with Golden Grace -> Emerald Gradient Bar
                                egui::Frame::none()
                                    .fill(ZINC_900)
                                    .stroke(Stroke::new(1.0_f32, ZINC_800))
                                    .rounding(Rounding::same(10.0))
                                    .inner_margin(Margin::symmetric(18.0, 12.0))
                                    .show(ui, |ui| {
                                        ui.set_min_width(full_w - 36.0);
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    RichText::new("TARGETS QUEUED")
                                                        .size(10.5)
                                                        .color(ZINC_400),
                                                );
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} / {} Collected",
                                                        self.plan.collected_count,
                                                        self.plan.order_items.len()
                                                    ))
                                                    .size(16.0)
                                                    .strong()
                                                    .color(AMBER_400),
                                                );
                                            });
                                            ui.add_space(16.0);
                                            ui.separator();
                                            ui.add_space(16.0);
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    RichText::new("ROUTE CHECKPOINTS")
                                                        .size(10.5)
                                                        .color(ZINC_400),
                                                );
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{} / {} Steps Done ({}%)",
                                                        self.plan.done_steps,
                                                        self.plan.total_steps,
                                                        self.plan.pct()
                                                    ))
                                                    .size(16.0)
                                                    .strong()
                                                    .color(EMERALD_400),
                                                );
                                            });
                                            ui.add_space(16.0);
                                            ui.separator();
                                            ui.add_space(16.0);
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    RichText::new("ROUTE LEGS")
                                                        .size(10.5)
                                                        .color(ZINC_400),
                                                );
                                                ui.label(
                                                    RichText::new(format!(
                                                        "{}",
                                                        self.plan.epochs.len()
                                                    ))
                                                    .size(16.0)
                                                    .strong()
                                                    .color(ZINC_100),
                                                );
                                            });
                                        });

                                        if self.plan.total_steps > 0 {
                                            ui.add_space(8.0);
                                            let frac = self.plan.done_steps as f32
                                                / self.plan.total_steps as f32;
                                            draw_gradient_progress_bar(ui, frac);
                                        }
                                    });

                                // Conflict Alert Banner (if any)
                                if !self.plan.conflicts.is_empty() {
                                    ui.add_space(12.0);
                                    let mut drop_id: Option<String> = None;
                                    egui::Frame::none()
                                        .fill(RED_950)
                                        .stroke(Stroke::new(1.5_f32, RED_BORDER))
                                        .rounding(Rounding::same(10.0))
                                        .inner_margin(Margin::same(14.0))
                                        .show(ui, |ui| {
                                            ui.set_min_width(full_w - 28.0);
                                            ui.label(
                                                RichText::new(format!(
                                                    "[!] SAME-RUN MUTUAL EXCLUSIVITY COLLISION DETECTED ({})",
                                                    self.plan.conflicts.len()
                                                ))
                                                .size(12.5)
                                                .strong()
                                                .color(RED_300),
                                            );
                                            for c in &self.plan.conflicts {
                                                ui.add_space(4.0);
                                                ui.label(
                                                    RichText::new(c.reason)
                                                        .size(12.5)
                                                        .color(ZINC_100),
                                                );
                                                ui.horizontal_wrapped(|ui| {
                                                    for item in &c.items {
                                                        if ui
                                                            .add(
                                                                egui::Button::new(
                                                                    RichText::new(format!(
                                                                        "Drop \"{}\" from Current Run",
                                                                        item.name
                                                                    ))
                                                                    .size(11.5)
                                                                    .color(RED_200),
                                                                )
                                                                .fill(ZINC_950)
                                                                .stroke(Stroke::new(
                                                                    1.0_f32,
                                                                    RED_BORDER,
                                                                )),
                                                            )
                                                            .clicked()
                                                        {
                                                            drop_id =
                                                                Some(item.item_id.to_string());
                                                        }
                                                    }
                                                });
                                            }
                                        });
                                    if let Some(id) = drop_id {
                                        self.order.retain(|i| i.item_id != id);
                                        self.sync_and_recompile();
                                    }
                                }

                                ui.add_space(14.0);

                                // Proportional 2-Column Layout (360px Left Targets + 16px Gap + Remaining Right Roadmap)
                                let col_gap = 16.0;
                                let left_w = 360.0_f32.min((full_w * 0.35).max(290.0));
                                let right_w = (full_w - left_w - col_gap).max(320.0);

                                ui.horizontal_top(|ui| {
                                    ui.spacing_mut().item_spacing.x = col_gap;

                                    // Left Column: Current Run Targets
                                    ui.allocate_ui_with_layout(
                                        Vec2::new(left_w, 0.0),
                                        egui::Layout::top_down(egui::Align::Min),
                                        |ui| {
                                            ui.set_width(left_w);
                                            egui::Frame::none()
                                                .fill(ZINC_900)
                                                .stroke(Stroke::new(1.0_f32, ZINC_800))
                                                .rounding(Rounding::same(10.0))
                                                .inner_margin(Margin::same(14.0))
                                                .show(ui, |ui| {
                                                    let inner_left_w = left_w - 28.0;
                                                    ui.set_min_width(inner_left_w);
                                                    ui.label(
                                                        RichText::new(format!(
                                                            "CURRENT RUN TARGETS ({})",
                                                            self.plan.order_items.len()
                                                        ))
                                                        .size(12.5)
                                                        .strong()
                                                        .color(AMBER_400),
                                                    );
                                                    ui.add_space(10.0);

                                                    if self.plan.order_items.is_empty() {
                                                        ui.label(
                                                            RichText::new(
                                                                "Clean slate — no items queued yet. Search above to queue targets.",
                                                            )
                                                            .size(12.5)
                                                            .color(ZINC_500),
                                                        );
                                                    }

                                                    let mut toggle_id: Option<String> = None;
                                                    let mut remove_id: Option<String> = None;

                                                    for item in &self.plan.order_items {
                                                        ui.push_id(format!("target_row_{}", item.item_id), |ui| {
                                                        egui::Frame::none()
                                                            .fill(ZINC_950)
                                                            .stroke(Stroke::new(1.0_f32, ZINC_800))
                                                            .rounding(Rounding::same(8.0))
                                                            .inner_margin(Margin::symmetric(
                                                                10.0, 8.0,
                                                            ))
                                                            .show(ui, |ui| {
                                                                let row_w = inner_left_w - 20.0;
                                                                ui.set_min_width(row_w);
                                                                ui.horizontal(|ui| {
                                                                    let mut checked =
                                                                        item.is_collected;
                                                                    if ui
                                                                        .checkbox(&mut checked, "")
                                                                        .changed()
                                                                    {
                                                                        toggle_id = Some(
                                                                            item.item_id
                                                                                .to_string(),
                                                                        );
                                                                    }

                                                                    // Constrain text width so long names wrap cleanly and never overlap [Remove]
                                                                    let text_col_w =
                                                                        (row_w - 94.0).max(100.0);
                                                                    ui.allocate_ui_with_layout(
                                                                        Vec2::new(text_col_w, 0.0),
                                                                        egui::Layout::top_down(
                                                                            egui::Align::Min,
                                                                        ),
                                                                        |ui| {
                                                                            ui.set_max_width(
                                                                                text_col_w,
                                                                            );
                                                                            let mut title =
                                                                                RichText::new(
                                                                                    item.name,
                                                                                )
                                                                                .size(13.0);
                                                                            if item.is_collected {
                                                                                title = title
                                                                                    .strikethrough()
                                                                                    .color(
                                                                                        ZINC_500,
                                                                                    );
                                                                            } else {
                                                                                title = title
                                                                                    .strong()
                                                                                    .color(
                                                                                        ZINC_100,
                                                                                    );
                                                                            }
                                                                            ui.add(
                                                                                egui::Label::new(
                                                                                    title,
                                                                                )
                                                                                .wrap(),
                                                                            );
                                                                            ui.label(
                                                                                RichText::new(
                                                                                    format!(
                                                                                        "{} • {}",
                                                                                        item.campaign,
                                                                                        item.category
                                                                                    ),
                                                                                )
                                                                                .size(11.0)
                                                                                .color(ZINC_400),
                                                                            );
                                                                        },
                                                                    );

                                                                    ui.with_layout(
                                                                        egui::Layout::right_to_left(
                                                                            egui::Align::Center,
                                                                        ),
                                                                        |ui| {
                                                                            if ui
                                                                                .add(
                                                                                    egui::Button::new(
                                                                                        RichText::new("Remove")
                                                                                            .size(11.0)
                                                                                            .color(RED_300),
                                                                                    )
                                                                                    .fill(RED_950)
                                                                                    .stroke(Stroke::new(
                                                                                        1.0_f32,
                                                                                        RED_BORDER,
                                                                                    ))
                                                                                    .rounding(
                                                                                        Rounding::same(5.0),
                                                                                    ),
                                                                                )
                                                                                .clicked()
                                                                            {
                                                                                remove_id = Some(
                                                                                    item.item_id
                                                                                        .to_string(),
                                                                                );
                                                                            }
                                                                        },
                                                                    );
                                                                });
                                                            });
                                                        ui.add_space(6.0);
                                                        });
                                                    }

                                                    if let Some(id) = toggle_id {
                                                        toggle_order_item(
                                                            &mut self.order,
                                                            &mut self.steps,
                                                            &id,
                                                        );
                                                        self.sync_and_recompile();
                                                    }
                                                    if let Some(id) = remove_id {
                                                        self.order.retain(|i| i.item_id != id);
                                                        self.sync_and_recompile();
                                                    }
                                                });
                                        },
                                    );

                                    // Right Column: Compiled Geographical Roadmap & PONR Work Order
                                    ui.allocate_ui_with_layout(
                                        Vec2::new(right_w, 0.0),
                                        egui::Layout::top_down(egui::Align::Min),
                                        |ui| {
                                            ui.set_width(right_w);
                                            egui::Frame::none()
                                                .fill(ZINC_900)
                                                .stroke(Stroke::new(1.0_f32, ZINC_800))
                                                .rounding(Rounding::same(10.0))
                                                .inner_margin(Margin::same(16.0))
                                                .show(ui, |ui| {
                                                    let inner_right_w = right_w - 32.0;
                                                    ui.set_min_width(inner_right_w);
                                                    ui.label(
                                                        RichText::new(
                                                            "COMPILED GEOGRAPHICAL ROADMAP & PONR WORK ORDER",
                                                        )
                                                        .size(13.0)
                                                        .strong()
                                                        .color(AMBER_400),
                                                    );
                                                    ui.add_space(10.0);

                                                    if self.plan.epochs.is_empty() {
                                                        egui::Frame::none()
                                                            .fill(ZINC_950)
                                                            .stroke(Stroke::new(1.0_f32, ZINC_800))
                                                            .rounding(Rounding::same(8.0))
                                                            .inner_margin(Margin::same(24.0))
                                                            .show(ui, |ui| {
                                                                ui.set_min_width(
                                                                    inner_right_w - 48.0,
                                                                );
                                                                ui.label(
                                                                    RichText::new(
                                                                        "Add items to your Current Run Roadmap above. The solver unrolls multi-region quest chains, sorts checkpoints by geographical progression weight along natural game flow, detects same-run mutual exclusivity collisions, and places Point-of-No-Return stop barriers inline.",
                                                                    )
                                                                    .size(12.5)
                                                                    .color(ZINC_400),
                                                                );
                                                            });
                                                    }

                                                    let mut toggled_step: Option<String> = None;

                                                    for (idx, ep) in
                                                        self.plan.epochs.iter().enumerate()
                                                    {
                                                        let leg_border = if ep.all_clear {
                                                            EMERALD_BORDER
                                                        } else {
                                                            ZINC_700
                                                        };
                                                        egui::Frame::none()
                                                            .fill(ZINC_950)
                                                            .stroke(Stroke::new(1.0_f32, leg_border))
                                                            .rounding(Rounding::same(10.0))
                                                            .inner_margin(Margin::same(12.0))
                                                            .show(ui, |ui| {
                                                                let leg_inner_w =
                                                                    inner_right_w - 24.0;
                                                                ui.set_min_width(leg_inner_w);
                                                                let header_text = format!(
                                                                    "ROUTE LEG {} • {}",
                                                                    idx + 1,
                                                                    ep.gate.campaign.to_uppercase()
                                                                );

                                                                egui::CollapsingHeader::new(
                                                                    RichText::new(header_text)
                                                                        .size(12.5)
                                                                        .strong()
                                                                        .color(if ep.all_clear {
                                                                            EMERALD_400
                                                                        } else {
                                                                            ZINC_100
                                                                        }),
                                                                )
                                                                .id_salt(format!("epoch_leg_{}_{}", idx, ep.gate.gate_id))
                                                                .default_open(!ep.all_clear)
                                                                .show(ui, |ui| {
                                                                    ui.add_space(6.0);
                                                                    for step in &ep.steps {
                                                                        ui.push_id(format!("step_row_{}_{}", idx, step.step_key), |ui| {
                                                                        egui::Frame::none()
                                                                            .fill(ZINC_900_SOFT)
                                                                            .stroke(Stroke::new(
                                                                                1.0_f32, ZINC_800,
                                                                            ))
                                                                            .rounding(
                                                                                Rounding::same(8.0),
                                                                            )
                                                                            .inner_margin(
                                                                                Margin::same(12.0),
                                                                            )
                                                                            .show(ui, |ui| {
                                                                                ui.set_min_width(
                                                                                    leg_inner_w
                                                                                        - 32.0,
                                                                                );
                                                                                ui.horizontal_top(
                                                                                    |ui| {
                                                                                        let mut done =
                                                                                            step.is_done;
                                                                                        if ui
                                                                                            .checkbox(
                                                                                                &mut done,
                                                                                                "",
                                                                                            )
                                                                                            .changed()
                                                                                        {
                                                                                            toggled_step = Some(
                                                                                                step.step_key
                                                                                                    .clone(),
                                                                                            );
                                                                                        }
                                                                                        ui.vertical(
                                                                                            |ui| {
                                                                                                ui.horizontal_wrapped(|ui| {
                                                                                                    draw_pill(
                                                                                                        ui,
                                                                                                        step.region_label,
                                                                                                        EMERALD_300,
                                                                                                        ZINC_800,
                                                                                                        ZINC_700,
                                                                                                    );
                                                                                                    if step.is_gate_critical {
                                                                                                        draw_pill(
                                                                                                            ui,
                                                                                                            "⚡ PONR-Sensitive Prerequisite",
                                                                                                            AMBER_300,
                                                                                                            AMBER_950,
                                                                                                            AMBER_BORDER,
                                                                                                        );
                                                                                                    } else {
                                                                                                        draw_pill(
                                                                                                            ui,
                                                                                                            "On-The-Way Regional Sweep",
                                                                                                            ZINC_400,
                                                                                                            ZINC_950,
                                                                                                            ZINC_800,
                                                                                                        );
                                                                                                    }
                                                                                                });
                                                                                                ui.add_space(4.0);
                                                                                                let mut step_txt = RichText::new(&step.text).size(13.5);
                                                                                                if step.is_done {
                                                                                                    step_txt = step_txt.strikethrough().color(ZINC_500);
                                                                                                } else {
                                                                                                    step_txt = step_txt.color(ZINC_100);
                                                                                                }
                                                                                                if ui.add(egui::Label::new(step_txt).sense(Sense::click())).clicked() {
                                                                                                    toggled_step = Some(step.step_key.clone());
                                                                                                }
                                                                                                ui.add_space(4.0);
                                                                                                ui.horizontal_wrapped(|ui| {
                                                                                                    for t in &step.targets {
                                                                                                        draw_pill(
                                                                                                            ui,
                                                                                                            &format!("Target: {}", t.name),
                                                                                                            AMBER_300,
                                                                                                            ZINC_950,
                                                                                                            ZINC_800,
                                                                                                        );
                                                                                                    }
                                                                                                });
                                                                                            },
                                                                                        );
                                                                                    },
                                                                                );
                                                                            });
                                                                        ui.add_space(6.0);
                                                                        });
                                                                    }

                                                                    if ep.gate.gate_id != "GATE_NONE"
                                                                    {
                                                                        let (
                                                                            banner_bg,
                                                                            banner_border,
                                                                            banner_fg,
                                                                            prefix,
                                                                        ) = if ep.all_clear {
                                                                            (
                                                                                EMERALD_950,
                                                                                EMERALD_BORDER,
                                                                                EMERALD_300,
                                                                                "✓ SAFE TO CROSS POINT OF NO RETURN:",
                                                                            )
                                                                        } else {
                                                                            (
                                                                                RED_950,
                                                                                RED_BORDER,
                                                                                RED_200,
                                                                                "[!] POINT OF NO RETURN — DO NOT TRIGGER YET:",
                                                                            )
                                                                        };
                                                                        egui::Frame::none()
                                                                            .fill(banner_bg)
                                                                            .stroke(Stroke::new(
                                                                                1.0_f32,
                                                                                banner_border,
                                                                            ))
                                                                            .rounding(
                                                                                Rounding::same(8.0),
                                                                            )
                                                                            .inner_margin(
                                                                                Margin::symmetric(
                                                                                    12.0, 10.0,
                                                                                ),
                                                                            )
                                                                            .show(ui, |ui| {
                                                                                ui.set_min_width(
                                                                                    leg_inner_w
                                                                                        - 32.0,
                                                                                );
                                                                                ui.label(
                                                                                    RichText::new(
                                                                                        format!(
                                                                                            "{} {}",
                                                                                            prefix,
                                                                                            ep.gate
                                                                                                .title
                                                                                        ),
                                                                                    )
                                                                                    .size(12.0)
                                                                                    .strong()
                                                                                    .color(
                                                                                        banner_fg,
                                                                                    ),
                                                                                );
                                                                                ui.add_space(2.0);
                                                                                ui.label(
                                                                                    RichText::new(
                                                                                        format!(
                                                                                            "Trigger: {} — {}",
                                                                                            ep.gate
                                                                                                .trigger_desc,
                                                                                            ep.gate
                                                                                                .warning_desc
                                                                                        ),
                                                                                    )
                                                                                    .size(12.0)
                                                                                    .color(
                                                                                        ZINC_100,
                                                                                    ),
                                                                                );
                                                                            });
                                                                    }
                                                                });
                                                            });
                                                        ui.add_space(10.0);
                                                    }

                                                    if let Some(sk) = toggled_step {
                                                        toggle_step_key(
                                                            &mut self.order,
                                                            &mut self.steps,
                                                            &sk,
                                                        );
                                                        self.sync_and_recompile();
                                                    }
                                                });
                                        },
                                    );
                                });
                            });
                    });
            });
    }
}

fn load_app_icon() -> Arc<egui::IconData> {
    let png_bytes = include_bytes!("../../static/icon-512.png");
    let img = image::load_from_memory_with_format(png_bytes, image::ImageFormat::Png)
        .expect("Failed to decode static/icon-512.png");
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    Arc::new(egui::IconData {
        rgba: rgba.into_raw(),
        width,
        height,
    })
}

fn main() -> eframe::Result<()> {
    let icon = load_app_icon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 860.0])
            .with_min_inner_size([720.0, 520.0])
            .with_title("Elden Ring Current Run Roadmap — Pure Rust GPU (egui)")
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "Elden Ring Roadmap (Pure Rust GPU)",
        options,
        Box::new(|cc| Ok(Box::new(ErRoadmapGpuApp::new(cc)))),
    )
}
