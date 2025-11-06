use eframe::egui;

use crate::engine::EngineState;
use crate::sorting_algorithms::SortingAlgorithmKind;

#[derive(Clone)]
pub struct SettingsPanelState {
    pub number_of_values: usize,
    pub frames_per_second: u32,
    pub use_bubble_sort: bool,
    pub use_quick_sort: bool,
    use_insertion_sort: bool,
    use_selection_sort: bool,
    use_merge_sort: bool,
    use_heap_sort: bool,
    use_shell_sort: bool,
    use_intro_sort: bool,
    use_tim_sort: bool,
    use_cocktail_sort: bool,
    use_comb_sort: bool,
    use_gnome_sort: bool,
}

pub enum SettingsPanelAction {
    None,
    StartRequested(Vec<SortingAlgorithmKind>),
    StopRequested,
}

impl Default for SettingsPanelState {
    fn default() -> Self {
        Self {
            number_of_values: 128,
            frames_per_second: 30,
            use_bubble_sort: true,
            use_quick_sort: true,
            use_insertion_sort: false,
            use_selection_sort: false,
            use_merge_sort: false,
            use_heap_sort: false,
            use_shell_sort: false,
            use_intro_sort: true,
            use_tim_sort: true,
            use_cocktail_sort: true,
            use_comb_sort: true,
            use_gnome_sort: true,
        }
    }
}

impl SettingsPanelState {
    pub fn show(&mut self, ui: &mut egui::Ui, engine_state: &EngineState) -> SettingsPanelAction {
        ui.heading("Settings");

        ui.add(
            egui::Slider::new(&mut self.number_of_values, 32..=2500)
                .text("Values per algorithm"),
        );

        ui.add(egui::Slider::new(
            &mut self.frames_per_second, 10..=60)
                .text("Frames per second")
        );

        ui.separator();
        ui.label("Algorithms to visualize:");
            
        ui.collapsing("Basic O(n²) comparison sorts", |ui| {
            ui.label(egui::RichText::new(
                "Simple, educational sorts – good for seeing the fundamentals."
            ).small().italics());
        
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut self.use_bubble_sort, "Bubble Sort");
                ui.checkbox(&mut self.use_insertion_sort, "Insertion Sort");
                ui.checkbox(&mut self.use_selection_sort, "Selection Sort");
                ui.checkbox(&mut self.use_shell_sort, "Shell Sort");
                ui.checkbox(&mut self.use_cocktail_sort, "Cocktail Sort");
                ui.checkbox(&mut self.use_comb_sort, "Comb Sort");
                ui.checkbox(&mut self.use_gnome_sort, "Gnome Sort");
            });
        });
        
        ui.collapsing("Classic O(n log n) sorts", |ui| {
            ui.label(egui::RichText::new(
                "Divide-and-conquer algorithms used in textbooks and interviews."
            ).small().italics());
        
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut self.use_quick_sort, "Quick Sort");
                ui.checkbox(&mut self.use_merge_sort, "Merge Sort");
                ui.checkbox(&mut self.use_heap_sort, "Heap Sort");
                ui.checkbox(&mut self.use_intro_sort, "Intro Sort");
            });
        });
        
        ui.collapsing("Hybrid / practical sorts", |ui| {
            ui.label(egui::RichText::new(
                "More realistic, production-style algorithms."
            ).small().italics());
        
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut self.use_tim_sort, "Tim Sort");
            });
        });
        
        ui.separator();



        let mut action = SettingsPanelAction::None;

        match engine_state {
            EngineState::Idle => {
                if ui.button("Start").clicked() {
                    let mut selected_algorithms: Vec<SortingAlgorithmKind> = Vec::new();

                    if self.use_bubble_sort {
                        selected_algorithms.push(SortingAlgorithmKind::BubbleSort);
                    }
                    if self.use_quick_sort {
                        selected_algorithms.push(SortingAlgorithmKind::QuickSort);
                    }
                    if self.use_insertion_sort {
                        selected_algorithms.push(SortingAlgorithmKind::InsertionSort);
                    }
                    if self.use_selection_sort {
                        selected_algorithms.push(SortingAlgorithmKind::SelectionSort);
                    }
                    if self.use_merge_sort {
                        selected_algorithms.push(SortingAlgorithmKind::MergeSort);
                    }
                    if self.use_heap_sort {
                        selected_algorithms.push(SortingAlgorithmKind::HeapSort);
                    }
                    if self.use_shell_sort {
                        selected_algorithms.push(SortingAlgorithmKind::ShellSort);
                    }
                    if self.use_intro_sort {
                        selected_algorithms.push(SortingAlgorithmKind::IntroSort);
                    }
                    if self.use_comb_sort {
                        selected_algorithms.push(SortingAlgorithmKind::CombSort);
                    }
                    if self.use_tim_sort {
                        selected_algorithms.push(SortingAlgorithmKind::TimSort);
                    }
                    if self.use_cocktail_sort {
                        selected_algorithms.push(SortingAlgorithmKind::CocktailSort);
                    }
                    if self.use_gnome_sort {
                        selected_algorithms.push(SortingAlgorithmKind::GnomeSort);
                    }
                    



                    action = SettingsPanelAction::StartRequested(selected_algorithms);
                }
            }
            EngineState::Preparing { algorithms_completed, algorithms_total } => {
                ui.add_enabled(
                    false,
                    egui::Button::new(format!(
                        "Preparing... ({}/{})",
                        algorithms_completed,
                        algorithms_total
                    ))
                );
                if ui.button("Stop").clicked() {
                    action = SettingsPanelAction::StopRequested;
                }
            }
            EngineState::Running => {
                if ui.button("Stop").clicked() {
                    action = SettingsPanelAction::StopRequested;
                }
            }
        }

        action

    }
}