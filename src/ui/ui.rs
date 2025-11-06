use std::sync::{Arc, Mutex};

use eframe::egui::{self, Color32};
use eframe::egui::epaint::Hsva;

use crate::engine::{AlgorithmStateSnapshot, EngineConfig, EngineController, EngineSharedState, EngineState};
use crate::ui::settings_panel::{SettingsPanelAction, SettingsPanelState};

// Maximum number of columns in the algorithm grid
const MAX_GRID_COLUMNS: usize = 4;

pub struct SortVisApp {
    shared_state: Arc<Mutex<EngineSharedState>>,
    engine_controller: EngineController,
    settings_state: SettingsPanelState,
}

impl SortVisApp {
    pub fn new(
        _creation_context: &eframe::CreationContext<'_>,
        shared_state: Arc<Mutex<EngineSharedState>>,
    ) -> Self {
        let engine_controller = EngineController::new(Arc::clone(&shared_state));

        Self {
            shared_state,
            engine_controller,
            settings_state: SettingsPanelState::default(),
        }
    }

    fn handle_settings_action(&mut self, action: SettingsPanelAction) {
        match action {
            SettingsPanelAction::None => {}
            SettingsPanelAction::StopRequested => {
                self.engine_controller.stop();
            }
            SettingsPanelAction::StartRequested(selected_algorithms) => {
                if selected_algorithms.is_empty() {
                    return;
                }

                let engine_config = EngineConfig {
                    number_of_values: self.settings_state.number_of_values,
                    selected_algorithms,
                    frames_per_second: self.settings_state.frames_per_second,
                };

                self.engine_controller.start_run(engine_config);
            }
        }
    }

    fn draw_algorithm_grid(&self, ui: &mut egui::Ui, engine_state_snapshot: &EngineSharedState) {
        // Check if we're preparing algorithms
        if let EngineState::Preparing { algorithms_completed, algorithms_total } = engine_state_snapshot.engine_state {
            ui.centered_and_justified(|center_ui| {
                center_ui.vertical_centered(|vertical_ui| {
                    vertical_ui.add_space(20.0);

                    vertical_ui.heading("Generating sorting frames...");
                    vertical_ui.add_space(10.0);

                    vertical_ui.label(format!(
                        "Algorithm {} of {} complete",
                        algorithms_completed,
                        algorithms_total
                    ));

                    vertical_ui.add_space(10.0);

                    let progress = algorithms_completed as f32 / algorithms_total as f32;
                    let progress_bar = egui::ProgressBar::new(progress)
                        .show_percentage()
                        .desired_width(300.0);
                    vertical_ui.add(progress_bar);
                });
            });
            return;
        }

        let algorithm_count = engine_state_snapshot.algorithm_states.len();
        if algorithm_count == 0 {
            ui.centered_and_justified(|center_ui| {
                center_ui.label("No algorithms running. Configure settings and press Start.");
            });
            return;
        }

        let available_size = ui.available_size();

        // Define minimum cell dimensions
        let minimum_cell_width = 220.0;
        let minimum_cell_height = 160.0;

        // Calculate how many columns fit horizontally
        let columns_from_width = (available_size.x / minimum_cell_width)
            .floor()
            .max(1.0) as usize;

        // Calculate how many rows fit vertically
        let rows_from_height = (available_size.y / minimum_cell_height)
            .floor()
            .max(1.0) as usize;

        // Apply maximum column cap
        let columns_capped = columns_from_width.min(MAX_GRID_COLUMNS);

        // Determine column count considering both dimensions and row preference
        let column_count = if rows_from_height >= 3 && algorithm_count >= 3 {
            // Vertical space allows 3+ rows, enforce minimum 3 rows
            // max_columns = ceil(algorithm_count / 3)
            let max_columns_for_three_rows = (algorithm_count + 2) / 3;
            columns_capped
                .min(max_columns_for_three_rows)
                .min(algorithm_count)
        } else if rows_from_height >= 2 && algorithm_count > 1 {
            // Vertical space allows 2+ rows, enforce minimum 2 rows
            let max_columns_for_two_rows = (algorithm_count + 1) / 2;
            columns_capped
                .min(max_columns_for_two_rows)
                .min(algorithm_count)
        } else {
            // Not enough vertical space, or only 1 algorithm
            columns_capped.min(algorithm_count)
        };

        let row_count = ((algorithm_count + column_count - 1) / column_count).max(1);

        // Compute cell dimensions that grow/shrink with window
        let cell_width = (available_size.x / column_count as f32).max(minimum_cell_width);
        let cell_height = (available_size.y / row_count as f32).max(minimum_cell_height);
        let cell_size = egui::vec2(cell_width, cell_height);

        // Layout cells using nested loops
        let mut algorithm_index = 0;
        for _row_index in 0..row_count {
            ui.horizontal(|row_ui| {
                for _col_index in 0..column_count {
                    if algorithm_index < algorithm_count {
                        let algorithm_state = &engine_state_snapshot.algorithm_states[algorithm_index];
                        row_ui.allocate_ui(cell_size, |cell_ui| {
                            cell_ui.set_min_size(cell_size);
                            self.draw_algorithm_panel(cell_ui, algorithm_state);
                        });
                        algorithm_index += 1;
                    } else {
                        // Fill empty cells to maintain grid alignment
                        row_ui.add_space(cell_size.x);
                    }
                }
            });

            // Add spacing between rows
            ui.add_space(8.0);
        }
    }

    fn draw_algorithm_panel(
        &self,
        ui: &mut egui::Ui,
        algorithm_state: &AlgorithmStateSnapshot,
    ) {
        ui.vertical(|panel_ui| {
            panel_ui.group(|group_ui| {
                // Enhanced title formatting
                if algorithm_state.is_finished {
                    group_ui.label(
                        egui::RichText::new(format!("{} (finished)", algorithm_state.algorithm_name))
                            .strong()
                            .italics()
                    );
                } else {
                    group_ui.label(
                        egui::RichText::new(&algorithm_state.algorithm_name)
                            .strong()
                    );
                }

                let stats_text = format!(
                    "Steps: {} | Time: {:.2} ms ({:.4} s)",
                    algorithm_state.stats.total_steps,
                    algorithm_state.stats.duration_milliseconds,
                    algorithm_state.stats.duration_seconds,
                );
                group_ui.label(stats_text);

                // Use dynamic sizing based on available space
                let available_size = group_ui.available_size();
                let reserved_height_for_labels = 40.0;
                let chart_height = (available_size.y - reserved_height_for_labels).max(40.0);
                let chart_size = egui::vec2(available_size.x, chart_height);

                let (response, painter) =
                    group_ui.allocate_painter(chart_size, egui::Sense::hover());

                let bounding_rect = response.rect;

                let chart_margin_top = 6.0;
                let chart_rect = egui::Rect::from_min_max(
                    egui::pos2(bounding_rect.left(), bounding_rect.top() + chart_margin_top),
                    egui::pos2(bounding_rect.right(), bounding_rect.bottom()),
                );

                self.draw_bar_chart(
                    &painter,
                    chart_rect,
                    &algorithm_state.current_values,
                    algorithm_state.is_finished,
                );
            });
        });
    }

    fn bar_fill_color(
        &self,
        visuals: &egui::Visuals,
        normalized_value: f32,
        is_finished: bool,
    ) -> Color32 {
        let clamped_value = normalized_value.clamp(0.0, 1.0);

        // Snapshot palette settings (short borrows, no borrow conflicts)
        let use_custom_palette = self.settings_state.use_custom_palette;
        let palette_base_hue_degrees = self.settings_state.palette_base_hue_degrees;
        let palette_saturation = self.settings_state.palette_saturation;
        let palette_brightness = self.settings_state.palette_brightness;
        let palette_gradient_strength = self.settings_state.palette_gradient_strength;

        // Base hue: either theme-based, or user-defined
        let base_hue = if use_custom_palette {
            (palette_base_hue_degrees / 360.0).rem_euclid(1.0)
        } else if visuals.dark_mode {
            0.58 // teal-ish
        } else {
            0.13 // orange-ish
        };

        // Gradient strength: how much hue varies with value
        let gradient_strength = if use_custom_palette {
            palette_gradient_strength
        } else {
            0.18
        };

        let hue_variation = gradient_strength * (clamped_value - 0.5);
        let hue = (base_hue + hue_variation).rem_euclid(1.0);

        // Saturation: either fixed theme-based, or user slider with a slight
        // desaturation for finished algorithms
        let saturation = if use_custom_palette {
            let base = palette_saturation.clamp(0.0, 1.0);
            if is_finished {
                base * 0.7
            } else {
                base
            }
        } else if is_finished {
            0.5
        } else {
            0.85
        };

        // Brightness: either theme-based, or user slider
        let value_brightness = if use_custom_palette {
            palette_brightness.clamp(0.0, 1.0)
        } else if visuals.dark_mode {
            0.9
        } else {
            0.7
        };

        Hsva::new(hue, saturation, value_brightness, 1.0).into()
    }

    fn chart_background_color(
        &self,
        visuals: &egui::Visuals,
        is_finished: bool,
    ) -> Color32 {
        let mut background = visuals.extreme_bg_color;

        // Brighten for finished algorithms, darken for active ones
        if is_finished {
            background = background.linear_multiply(1.05);
        } else {
            background = background.linear_multiply(0.95);
        }

        background
    }

    fn draw_bar_chart(
        &self,
        painter: &egui::Painter,
        chart_rect: egui::Rect,
        values: &[u32],
        is_finished: bool,
    ) {
        if values.is_empty() {
            return;
        }

        let maximum_value = values.iter().copied().max().unwrap_or(1) as f32;
        if maximum_value <= 0.0 {
            return;
        }

        let visuals = &painter.ctx().style().visuals;

        // Draw background with subtle color
        let chart_background_color = self.chart_background_color(visuals, is_finished);
        painter.rect_filled(chart_rect, 4.0, chart_background_color);

        // Compute bar layout
        let bar_count = values.len();
        let bar_width = chart_rect.width() / bar_count as f32;
        let bar_spacing_factor = 0.9;

        for (value_index, value) in values.iter().enumerate() {
            let normalized_height = (*value as f32) / maximum_value;
            let bar_height = chart_rect.height() * normalized_height.max(0.0);

            let left_position = chart_rect.left() + bar_width * value_index as f32;
            let right_position = left_position + bar_width * bar_spacing_factor;

            let bottom_position = chart_rect.bottom();
            let top_position = bottom_position - bar_height;

            let bar_rect = egui::Rect::from_min_max(
                egui::pos2(left_position, top_position),
                egui::pos2(right_position, bottom_position),
            );

            // Get color based on normalized height and finished state
            let bar_color = self.bar_fill_color(visuals, normalized_height, is_finished);

            // Draw bar with rounded corners
            painter.rect_filled(bar_rect, 2.0, bar_color);
        }
    }
}

impl eframe::App for SortVisApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let engine_state_snapshot = {
            let locked_state = self.shared_state.lock().unwrap();
            locked_state.clone()
        };

        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            ui.horizontal(|horizontal_ui| {
                horizontal_ui.heading("Sorting Visualizer");
                match &engine_state_snapshot.engine_state {
                    EngineState::Idle => {
                        horizontal_ui.label("Status: Idle");
                    }
                    EngineState::Preparing { algorithms_completed, algorithms_total } => {
                        horizontal_ui.label(format!(
                            "Status: Preparing... ({}/{})",
                            algorithms_completed,
                            algorithms_total
                        ));
                    }
                    EngineState::Running => {
                        horizontal_ui.label("Status: Running");
                    }
                }
            });
        });

        egui::SidePanel::left("settings_panel")
            .resizable(true)
            .default_width(220.0)
            .show(context, |ui| {
                let action =
                    self.settings_state
                        .show(ui, &engine_state_snapshot.engine_state);
                self.handle_settings_action(action);
            });

        egui::CentralPanel::default().show(context, |ui| {
            self.draw_algorithm_grid(ui, &engine_state_snapshot);
        });

        // Continuous animation
        context.request_repaint();
    }
}
