pub mod bubble_sort;
pub mod quick_sort;
pub mod insertion_sort;
pub mod selection_sort;
pub mod merge_sort;
pub mod heap_sort;
pub mod shell_sort;
pub mod cocktail_sort;
pub mod comb_sort;
pub mod gnome_sort;
pub mod intro_sort;
pub mod tim_sort;

use std::time::Instant;

use crate::sorting_algorithms::bubble_sort::bubble_sort_with_recording;
use crate::sorting_algorithms::quick_sort::quick_sort_with_recording;
use crate::sorting_algorithms::insertion_sort::insertion_sort_with_recording;
use crate::sorting_algorithms::merge_sort::merge_sort_with_recording;
use crate::sorting_algorithms::heap_sort::heap_sort_with_recording;
use crate::sorting_algorithms::selection_sort::selection_sort_with_recording;
use crate::sorting_algorithms::shell_sort::shell_sort_with_recording;
use crate::sorting_algorithms::cocktail_sort::cocktail_sort_with_recording;
use crate::sorting_algorithms::comb_sort::comb_sort_with_recording;
use crate::sorting_algorithms::gnome_sort::gnome_sort_with_recording;
use crate::sorting_algorithms::intro_sort::intro_sort_with_recording;
use crate::sorting_algorithms::tim_sort::tim_sort_with_recording;

use crate::stats::{ SortStats, StatsSnapshot };

#[derive(Clone, Copy, Debug)]
pub enum SortingAlgorithmKind {
    BubbleSort,
    QuickSort,
    InsertionSort,
    SelectionSort,
    MergeSort,
    HeapSort,
    ShellSort,
    IntroSort,
    TimSort,
    CocktailSort,
    CombSort,
    GnomeSort,
}

impl SortingAlgorithmKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            SortingAlgorithmKind::BubbleSort => "Bubble Sort",
            SortingAlgorithmKind::QuickSort => "Quick Sort",
            SortingAlgorithmKind::InsertionSort => "Insertion Sort",
            SortingAlgorithmKind::SelectionSort => "Selection Sort",
            SortingAlgorithmKind::MergeSort => "Merge Sort",
            SortingAlgorithmKind::HeapSort => "Heap Sort",
            SortingAlgorithmKind::ShellSort => "Shell Sort",
            SortingAlgorithmKind::IntroSort => "Intro Sort",
            SortingAlgorithmKind::TimSort => "Tim Sort",
            SortingAlgorithmKind::CocktailSort => "Cocktail Shaker Sort",
            SortingAlgorithmKind::CombSort => "Comb Sort",
            SortingAlgorithmKind::GnomeSort => "Gnome Sort",
        }
    }
}

pub struct SortingAlgorithmReplay {
    algorithm_name: String,
    frames: Vec<Vec<u32>>,
    stats: SortStats,
}

impl SortingAlgorithmReplay {
    pub fn new(algorithm_kind: SortingAlgorithmKind, base_values: &[u32]) -> Self {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        let start_time = Instant::now();

        match algorithm_kind {
            SortingAlgorithmKind::BubbleSort => {
                bubble_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::QuickSort => {
                quick_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::InsertionSort => {
                insertion_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::SelectionSort => {
                selection_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::MergeSort => {
                merge_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::HeapSort => {
                heap_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::ShellSort => {
                shell_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::IntroSort => {
                intro_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::TimSort => {
                tim_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::CocktailSort => {
                cocktail_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::CombSort => {
                comb_sort_with_recording(base_values, &mut frames);
            }
            SortingAlgorithmKind::GnomeSort => {
                gnome_sort_with_recording(base_values, &mut frames);
            }
        }

        let end_time = Instant::now();

        if frames.is_empty() {
            frames.push(base_values.to_vec());
        }

        let total_steps = frames.len() as u64;
        let duration = end_time.duration_since(start_time);
        let stats = SortStats::from_measurements(total_steps, duration);

        SortingAlgorithmReplay {
            algorithm_name: algorithm_kind.display_name().to_owned(),
            frames,
            stats,
        }
    }

    pub fn name(&self) -> &str {
        &self.algorithm_name
    }

    pub fn frame_at(&self, frame_index: usize) -> (Vec<u32>, bool) {
        let last_index = self.frames.len().saturating_sub(1);

        if self.frames.is_empty() {
            return (Vec::new(), true);
        }

        if frame_index >= self.frames.len() {
            (self.frames[last_index].clone(), true)
        } else {
            let is_finished = frame_index >= last_index;
            (self.frames[frame_index].clone(), is_finished)
        }
    }

    pub fn stats_snapshot(&self) -> StatsSnapshot {
        self.stats.to_snapshot()
    }
}
