use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use rand::rng;

use crate::sorting_algorithms::{SortingAlgorithmKind, SortingAlgorithmReplay};
use crate::stats::StatsSnapshot;

#[derive(Clone)]
pub struct AlgorithmStateSnapshot {
    pub algorithm_name: String,
    pub current_values: Vec<u32>,
    pub is_finished: bool,
    pub stats: StatsSnapshot,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EngineState {
    Idle,
    Preparing {
        algorithms_completed: usize,
        algorithms_total: usize,
    },
    Running,
}

impl Default for EngineState {
    fn default() -> Self {
        EngineState::Idle
    }
}

#[derive(Clone, Default)]
pub struct EngineSharedState {
    pub algorithm_states: Vec<AlgorithmStateSnapshot>,
    pub engine_state: EngineState,
}

pub struct EngineConfig {
    pub number_of_values: usize,
    pub selected_algorithms: Vec<SortingAlgorithmKind>,
    pub frames_per_second: u32,
}

pub struct EngineController {
    shared_state: Arc<Mutex<EngineSharedState>>,
    stop_flag: Arc<AtomicBool>,
    worker_handle: Option<thread::JoinHandle<()>>,
}

impl EngineController {
    pub fn new(shared_state: Arc<Mutex<EngineSharedState>>) -> Self {
        Self {
            shared_state,
            stop_flag: Arc::new(AtomicBool::new(false)),
            worker_handle: None,
        }
    }

    pub fn start_run(&mut self, config: EngineConfig) {
        // Stop any existing worker first
        self.stop();

        self.stop_flag.store(false, Ordering::SeqCst);
        let stop_flag_clone = Arc::clone(&self.stop_flag);
        let shared_state_clone = Arc::clone(&self.shared_state);

        let number_of_values = config.number_of_values;
        let selected_algorithms = config.selected_algorithms;
        let frames_per_second = config.frames_per_second.max(1);
        let target_frame_duration = Duration::from_millis((1000 / frames_per_second) as u64);

        let worker_handle = thread::spawn(move || {
            let mut base_values: Vec<u32> = (0..number_of_values as u32).collect();
            let mut random_generator = rng();
            base_values.shuffle(&mut random_generator);

            let algorithms_total = selected_algorithms.len();

            // set state to Preparing before generating frames
            {
                let mut locked_state = shared_state_clone.lock().unwrap();
                locked_state.engine_state = EngineState::Preparing {
                    algorithms_completed: 0,
                    algorithms_total,
                };
            }

            // generate frames for each algorithm, updating progress as we go
            let mut algorithm_replays: Vec<SortingAlgorithmReplay> = Vec::new();
            for (index, algorithm_kind) in selected_algorithms.into_iter().enumerate() {
                // check if stop was requested during preparation
                if stop_flag_clone.load(Ordering::SeqCst) {
                    let mut locked_state = shared_state_clone.lock().unwrap();
                    locked_state.algorithm_states.clear();
                    locked_state.engine_state = EngineState::Idle;
                    return;
                }

                // generate frames for this algorithm
                let replay = SortingAlgorithmReplay::new(algorithm_kind, &base_values);
                algorithm_replays.push(replay);

                // update progress
                {
                    let mut locked_state = shared_state_clone.lock().unwrap();
                    locked_state.engine_state = EngineState::Preparing {
                        algorithms_completed: index + 1,
                        algorithms_total,
                    };
                }
            }

            if algorithm_replays.is_empty() {
                let mut locked_state = shared_state_clone.lock().unwrap();
                locked_state.algorithm_states.clear();
                locked_state.engine_state = EngineState::Idle;
                return;
            }

            let total_algorithms = algorithm_replays.len();
            let mut current_step_index: usize = 0;
            let mut all_algorithms_finished = false;

            {
                let mut locked_state = shared_state_clone.lock().unwrap();
                locked_state.engine_state = EngineState::Running;
            }

            while !stop_flag_clone.load(Ordering::SeqCst) && !all_algorithms_finished {
                let frame_start_time = Instant::now();
                all_algorithms_finished = true;

                let mut snapshots_for_frame: Vec<AlgorithmStateSnapshot> =
                    Vec::with_capacity(total_algorithms);

                for algorithm_replay in algorithm_replays.iter() {
                    let (frame_values, is_finished_for_algorithm) =
                        algorithm_replay.frame_at(current_step_index);
                    let stats_snapshot = algorithm_replay.stats_snapshot();

                    if !is_finished_for_algorithm {
                        all_algorithms_finished = false;
                    }

                    snapshots_for_frame.push(AlgorithmStateSnapshot {
                        algorithm_name: algorithm_replay.name().to_owned(),
                        current_values: frame_values,
                        is_finished: is_finished_for_algorithm,
                        stats: stats_snapshot,
                    });
                }

                current_step_index = current_step_index.saturating_add(1);

                {
                    let mut locked_state = shared_state_clone.lock().unwrap();
                    locked_state.algorithm_states = snapshots_for_frame;
                    locked_state.engine_state = if all_algorithms_finished {
                        EngineState::Idle
                    } else {
                        EngineState::Running
                    };
                }

                let frame_elapsed_time = frame_start_time.elapsed();
                if frame_elapsed_time < target_frame_duration {
                    thread::sleep(target_frame_duration - frame_elapsed_time);
                }
            }

            let mut locked_state = shared_state_clone.lock().unwrap();
            locked_state.engine_state = EngineState::Idle;
        });

        self.worker_handle = Some(worker_handle);
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);

        if let Some(join_handle) = self.worker_handle.take() {
            let _ = join_handle.join();
        }
    }
}

impl Drop for EngineController {
    fn drop(&mut self) {
        self.stop();
    }
}
