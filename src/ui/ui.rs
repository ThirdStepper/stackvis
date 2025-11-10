use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::time::Duration;
use rand::random;

use eframe::egui::{ self, Color32 };
use eframe::egui::epaint::Hsva;
use rodio::{ OutputStream, OutputStreamBuilder, Sink, Source, source::SineWave, source::Spatial };

use crate::engine::{
    AlgorithmStateSnapshot,
    EngineConfig,
    EngineController,
    EngineSharedState,
    EngineState,
};
use crate::ui::settings_panel::{ SettingsPanelAction, SettingsPanelState };

// max grid columns
const MAX_GRID_COLUMNS: usize = 4;

// c major scale
const C_MAJOR_DEGREES: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];

// spatial audio constants
const LEFT_EAR_POS: [f32; 3] = [-1.0, 0.0, 0.0];
const RIGHT_EAR_POS: [f32; 3] = [1.0, 0.0, 0.0];
// pushes the sound emitter to the front of the listener
const EMITTER_Z: f32 = 1.0;

pub struct SortVisApp {
    shared_state: Arc<Mutex<EngineSharedState>>,
    engine_controller: EngineController,
    settings_state: SettingsPanelState,

    // must keep _audio_stream alive for audio to work
    _audio_stream: Option<OutputStream>,
    audio_sink: Option<Sink>,
    previous_values_for_audio: HashMap<String, Vec<u32>>,
}

impl SortVisApp {
    pub fn new(
        _creation_context: &eframe::CreationContext<'_>,
        shared_state: Arc<Mutex<EngineSharedState>>
    ) -> Self {
        let engine_controller = EngineController::new(Arc::clone(&shared_state));

        // initialize audio, handle failure gracefully
        let (_audio_stream, audio_sink) = match OutputStreamBuilder::open_default_stream() {
            Ok(stream) => {
                let sink = Sink::connect_new(&stream.mixer());
                sink.set_volume(0.2);
                (Some(stream), Some(sink))
            }
            Err(_) => (None, None),
        };

        Self {
            shared_state,
            engine_controller,
            settings_state: SettingsPanelState::default(),
            _audio_stream,
            audio_sink,
            previous_values_for_audio: HashMap::new(),
        }
    }

    fn handle_settings_action(&mut self, action: SettingsPanelAction) {
        match action {
            SettingsPanelAction::None => {}
            SettingsPanelAction::StopRequested => {
                self.engine_controller.stop();
                // stop queued sounds and reset audio tracking
                self.clear_audio_state();
            }
            SettingsPanelAction::StartRequested(selected_algorithms) => {
                if selected_algorithms.is_empty() {
                    return;
                }

                // randomize base hue (0..360)
                self.settings_state.palette_base_hue_degrees = random::<f32>() * 360.0;

                // reset audio and create fresh sink
                self.clear_audio_state();
                self.ensure_audio_sink();

                let engine_config = EngineConfig {
                    number_of_values: self.settings_state.number_of_values,
                    selected_algorithms,
                    frames_per_second: self.settings_state.frames_per_second,
                };

                self.engine_controller.start_run(engine_config);
            }
        }
    }

    // audio state management
    fn clear_audio_state(&mut self) {
        // dropping sink stops queued sounds immediately
        if let Some(sink) = self.audio_sink.take() {
            sink.stop();
        }

        // clear to avoid detecting bogus changes
        self.previous_values_for_audio.clear();
    }

    fn ensure_audio_sink(&mut self) {
        // sink exists, nothing to do
        if self.audio_sink.is_some() {
            return;
        }

        // lazily recreate sink if stream exists
        if let Some(ref stream) = self._audio_stream {
            let sink = Sink::connect_new(&stream.mixer());
            sink.set_volume(0.2);
            self.audio_sink = Some(sink);
        }
    }

    // audio detection helpers
    fn detect_first_changed_index(&self, previous: &[u32], current: &[u32]) -> Option<usize> {
        let min_len = previous.len().min(current.len());

        // check common range for changes
        for i in 0..min_len {
            if previous[i] != current[i] {
                return Some(i);
            }
        }

        // length difference is a change at min_len
        if previous.len() != current.len() {
            return Some(min_len);
        }

        None
    }

    fn play_audio_for_change(&self, current_values: &[u32], changed_index: usize) {
        let Some(sink) = &self.audio_sink else {
            return;
        };

        if !sink.empty() {
            return;
        }

        let len = current_values.len();
        if len == 0 || changed_index >= len {
            return;
        }

        let value = current_values[changed_index];
        let maximum_value = current_values.iter().copied().max().unwrap_or(1) as f32;
        if maximum_value <= 0.0 {
            return;
        }

        // horizontal: 0.0 left, 1.0 right
        let normalized_index = if len > 1 {
            (changed_index as f32) / ((len - 1) as f32)
        } else {
            0.5
        };

        // bar height for loudness
        let normalized_value = (value as f32) / maximum_value;

        // c major scale mapping
        let frequency = c_major_scale_frequency(normalized_index);

        // map bar position to 3d emitter for panning
        let emitter_pos = emitter_position_from_normalized_index(normalized_index);

        // envelope loudness
        let amplitude = 0.05 + 0.15 * normalized_value; // 0.05–0.20
        let duration = Duration::from_millis(40);
        let attack = Duration::from_millis(5);
        let release = Duration::from_millis(40);

        let base_source = SineWave::new(frequency)
            .take_duration(duration)
            .fade_in(attack)
            .fade_out(release)
            .amplify(amplitude);

        let spatial_source = Spatial::new(base_source, emitter_pos, LEFT_EAR_POS, RIGHT_EAR_POS);

        sink.append(spatial_source);
    }

    fn handle_audio_for_frame(&mut self, engine_state_snapshot: &EngineSharedState) {
        // if audio disabled, clear and return
        if !self.settings_state.enable_audio {
            self.clear_audio_state();
            return;
        }

        // check if sink exists
        self.ensure_audio_sink();
        if self.audio_sink.is_none() {
            // audio backend init failed
            return;
        }

        let mut tone_played_this_frame = false;

        for algorithm_state in &engine_state_snapshot.algorithm_states {
            let algorithm_name = &algorithm_state.algorithm_name;
            let current_values = &algorithm_state.current_values;

            if let Some(previous_values) = self.previous_values_for_audio.get(algorithm_name) {
                if !tone_played_this_frame {
                    if
                        let Some(changed_index) = self.detect_first_changed_index(
                            previous_values,
                            current_values
                        )
                    {
                        self.play_audio_for_change(current_values, changed_index);
                        tone_played_this_frame = true;
                    }
                }
            }

            self.previous_values_for_audio.insert(algorithm_name.clone(), current_values.clone());
        }
    }

    fn draw_algorithm_grid(&self, ui: &mut egui::Ui, engine_state_snapshot: &EngineSharedState) {
        // check if preparing
        if
            let EngineState::Preparing { algorithms_completed, algorithms_total } =
                engine_state_snapshot.engine_state
        {
            ui.centered_and_justified(|center_ui| {
                center_ui.vertical_centered(|vertical_ui| {
                    vertical_ui.add_space(20.0);

                    vertical_ui.heading("Generating sorting frames...");
                    vertical_ui.add_space(10.0);

                    vertical_ui.label(
                        format!(
                            "Algorithm {} of {} complete",
                            algorithms_completed,
                            algorithms_total
                        )
                    );

                    vertical_ui.add_space(10.0);

                    let progress = (algorithms_completed as f32) / (algorithms_total as f32);
                    let progress_bar = egui::ProgressBar
                        ::new(progress)
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

        let minimum_cell_width = 220.0;
        let minimum_cell_height = 160.0;

        let columns_from_width = (available_size.x / minimum_cell_width).floor().max(1.0) as usize;

        let rows_from_height = (available_size.y / minimum_cell_height).floor().max(1.0) as usize;

        let columns_capped = columns_from_width.min(MAX_GRID_COLUMNS);

        let column_count = if rows_from_height >= 3 && algorithm_count >= 3 {
            // enforce min 3 rows if space allows
            let max_columns_for_three_rows = (algorithm_count + 2) / 3;
            columns_capped.min(max_columns_for_three_rows).min(algorithm_count)
        } else if rows_from_height >= 2 && algorithm_count > 1 {
            // enforce min 2 rows if space allows
            let max_columns_for_two_rows = (algorithm_count + 1) / 2;
            columns_capped.min(max_columns_for_two_rows).min(algorithm_count)
        } else {
            // insufficient vertical space or single algo
            columns_capped.min(algorithm_count)
        };

        let row_count = ((algorithm_count + column_count - 1) / column_count).max(1);

        let cell_width = (available_size.x / (column_count as f32)).max(minimum_cell_width);
        let cell_height = (available_size.y / (row_count as f32)).max(minimum_cell_height);
        let cell_size = egui::vec2(cell_width, cell_height);

        let mut algorithm_index = 0;
        for _row_index in 0..row_count {
            ui.horizontal(|row_ui| {
                for _col_index in 0..column_count {
                    if algorithm_index < algorithm_count {
                        let algorithm_state =
                            &engine_state_snapshot.algorithm_states[algorithm_index];
                        row_ui.allocate_ui(cell_size, |cell_ui| {
                            cell_ui.set_min_size(cell_size);
                            self.draw_algorithm_panel(cell_ui, algorithm_state);
                        });
                        algorithm_index += 1;
                    } else {
                        row_ui.add_space(cell_size.x);
                    }
                }
            });

            ui.add_space(8.0);
        }
    }

    fn draw_algorithm_panel(&self, ui: &mut egui::Ui, algorithm_state: &AlgorithmStateSnapshot) {
        ui.vertical(|panel_ui| {
            panel_ui.group(|group_ui| {
                if algorithm_state.is_finished {
                    group_ui.label(
                        egui::RichText
                            ::new(format!("{} (finished)", algorithm_state.algorithm_name))
                            .strong()
                            .italics()
                    );
                } else {
                    group_ui.label(egui::RichText::new(&algorithm_state.algorithm_name).strong());
                }

                let stats_text = format!(
                    "Steps: {} | Time: {:.2} ms ({:.4} s)",
                    algorithm_state.stats.total_steps,
                    algorithm_state.stats.duration_milliseconds,
                    algorithm_state.stats.duration_seconds
                );
                group_ui.label(stats_text);

                let available_size = group_ui.available_size();
                let reserved_height_for_labels = 40.0;
                let chart_height = (available_size.y - reserved_height_for_labels).max(40.0);
                let chart_size = egui::vec2(available_size.x, chart_height);

                let (response, painter) = group_ui.allocate_painter(
                    chart_size,
                    egui::Sense::hover()
                );

                let bounding_rect = response.rect;

                let chart_margin_top = 6.0;
                let chart_rect = egui::Rect::from_min_max(
                    egui::pos2(bounding_rect.left(), bounding_rect.top() + chart_margin_top),
                    egui::pos2(bounding_rect.right(), bounding_rect.bottom())
                );

                self.draw_bar_chart(
                    &painter,
                    chart_rect,
                    &algorithm_state.current_values,
                    algorithm_state.is_finished
                );
            });
        });
    }

    fn bar_fill_color(
        &self,
        visuals: &egui::Visuals,
        normalized_value: f32,
        is_finished: bool
    ) -> Color32 {
        let clamped_value = normalized_value.clamp(0.0, 1.0);

        // snapshot palette settings to avoid borrow conflicts
        let use_custom_palette = self.settings_state.use_custom_palette;
        let palette_base_hue_degrees = self.settings_state.palette_base_hue_degrees;
        let palette_saturation = self.settings_state.palette_saturation;
        let palette_brightness = self.settings_state.palette_brightness;
        let palette_gradient_strength = self.settings_state.palette_gradient_strength;

        // base hue from theme or user setting
        let base_hue = if use_custom_palette {
            (palette_base_hue_degrees / 360.0).rem_euclid(1.0)
        } else if visuals.dark_mode {
            0.58 // teal-ish
        } else {
            0.13 // orange-ish
        };

        // gradient strength controls hue variation
        let gradient_strength = if use_custom_palette { palette_gradient_strength } else { 0.18 };

        let hue_variation = gradient_strength * (clamped_value - 0.5);
        let hue = (base_hue + hue_variation).rem_euclid(1.0);

        // saturation from theme or user, desaturated if finished
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

        // brightness from theme or user setting
        let value_brightness = if use_custom_palette {
            palette_brightness.clamp(0.0, 1.0)
        } else if visuals.dark_mode {
            0.9
        } else {
            0.7
        };

        Hsva::new(hue, saturation, value_brightness, 1.0).into()
    }

    fn chart_background_color(&self, visuals: &egui::Visuals, is_finished: bool) -> Color32 {
        let mut background = visuals.extreme_bg_color;

        // brighten if finished, darken if active
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
        is_finished: bool
    ) {
        if values.is_empty() {
            return;
        }

        let maximum_value = values.iter().copied().max().unwrap_or(1) as f32;
        if maximum_value <= 0.0 {
            return;
        }

        let visuals = &painter.ctx().style().visuals;

        let chart_background_color = self.chart_background_color(visuals, is_finished);
        painter.rect_filled(chart_rect, 4.0, chart_background_color);

        let bar_count = values.len();
        let bar_width = chart_rect.width() / (bar_count as f32);
        let bar_spacing_factor = 0.9;

        for (value_index, value) in values.iter().enumerate() {
            let normalized_height = (*value as f32) / maximum_value;
            let bar_height = chart_rect.height() * normalized_height.max(0.0);

            let left_position = chart_rect.left() + bar_width * (value_index as f32);
            let right_position = left_position + bar_width * bar_spacing_factor;

            let bottom_position = chart_rect.bottom();
            let top_position = bottom_position - bar_height;

            let bar_rect = egui::Rect::from_min_max(
                egui::pos2(left_position, top_position),
                egui::pos2(right_position, bottom_position)
            );

            let bar_color = self.bar_fill_color(visuals, normalized_height, is_finished);

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
                        horizontal_ui.label(
                            format!(
                                "Status: Preparing... ({}/{})",
                                algorithms_completed,
                                algorithms_total
                            )
                        );
                    }
                    EngineState::Running => {
                        horizontal_ui.label("Status: Running");
                    }
                }
            });
        });

        egui::SidePanel
            ::left("settings_panel")
            .resizable(true)
            .default_width(220.0)
            .show(context, |ui| {
                let action = self.settings_state.show(ui, &engine_state_snapshot.engine_state);
                self.handle_settings_action(action);
            });

        egui::CentralPanel::default().show(context, |ui| {
            self.draw_algorithm_grid(ui, &engine_state_snapshot);
        });

        if let Some(sink) = &self.audio_sink {
            sink.set_volume(self.settings_state.audio_volume);
        }
        self.handle_audio_for_frame(&engine_state_snapshot);

        context.request_repaint();
    }
}

fn emitter_position_from_normalized_index(normalized_index: f32) -> [f32; 3] {
    // 0..1 -> -1..1
    let x = normalized_index.clamp(0.0, 1.0) * 2.0 - 1.0;
    [x, 0.0, EMITTER_Z]
}

fn freq_from_semitones(base_freq: f32, semitone_offset: i32) -> f32 {
    base_freq * (2.0f32).powf((semitone_offset as f32) / 12.0)
}

fn c_major_scale_frequency(normalized_index: f32) -> f32 {
    let x = normalized_index.clamp(0.0, 1.0);

    // 3 octaves of c major = 21 steps
    let total_steps = 21;

    // map to 0..20
    let step_index = (x * ((total_steps - 1) as f32)).round() as i32;

    // split into octave and degree
    let degrees_per_octave = C_MAJOR_DEGREES.len() as i32;
    let octave = step_index / degrees_per_octave;
    let degree_index = (step_index % degrees_per_octave).max(0);

    // lookup semitone offset
    let degree_semitones = C_MAJOR_DEGREES[degree_index as usize];

    // total = octave * 12 + degree
    let total_semitones = octave * 12 + degree_semitones;

    // base: a4 = 440 hz
    let base_freq = 440.0;

    freq_from_semitones(base_freq, total_semitones)
}
