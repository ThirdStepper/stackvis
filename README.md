# StackVis: A Comparative Sorting Algorithm Visualizer

## Abstract

StackVis is an interactive educational tool for visualizing and comparing sorting algorithm performance in real-time. Implemented in Rust, the application provides concurrent visual representations of multiple sorting algorithms operating on identical datasets, enabling direct comparative analysis of algorithmic behavior and computational efficiency.

## Overview

This application addresses the pedagogical challenge of understanding abstract algorithmic concepts by providing concrete visual feedback. By rendering multiple sorting algorithms simultaneously, users can observe the distinct operational characteristics of each approach, including step count, execution patterns, and relative performance metrics.

## Technical Architecture

The system is architected with the following components:

- **Engine Module** ([engine.rs](src/engine.rs)): Manages the core visualization state machine, orchestrating algorithm execution across worker threads and maintaining synchronized state snapshots for concurrent algorithm visualization.

- **Sorting Algorithms** ([sorting_algorithms/](src/sorting_algorithms/)): Implements twelve canonical sorting algorithms with frame-by-frame state recording capabilities for replay-based visualization.

- **User Interface** ([ui/](src/ui/)): Built on the egui immediate-mode GUI framework via eframe, providing responsive controls and real-time rendering of algorithm states.

- **Statistics Module** ([stats.rs](src/stats.rs)): Collects and aggregates performance metrics including operation counts and temporal measurements.

- **Audio Engine**: Integrates the rodio audio library to provide auditory feedback correlated with algorithmic operations, featuring stereo panning based on element position.

## Implemented Algorithms

The application currently supports the following sorting algorithms:

- **Comparison-based sorts**: Bubble Sort, Cocktail Shaker Sort, Insertion Sort, Selection Sort, Gnome Sort
- **Divide-and-conquer sorts**: Quick Sort, Merge Sort, Heap Sort
- **Gap-based sorts**: Shell Sort, Comb Sort
- **Hybrid sorts**: Intro Sort (introspective sort), Tim Sort

Each algorithm is instrumented to record intermediate states, enabling frame-accurate replay and synchronization across concurrent visualizations.

## Features

- **Parallel Visualization**: Simultaneous execution and rendering of multiple sorting algorithms
- **Configurable Parameters**: Adjustable array size and visualization frame rate
- **Statistical Analysis**: Real-time display of step counts and execution duration
- **Audio Synthesis**: Sonification of sorting operations with positional stereo imaging
- **Color Customization**: Support for custom color palettes to enhance visual distinction

## System Requirements

- Rust toolchain (edition 2021 or later)
- Compatible operating system: Windows, macOS, or Linux

## Dependencies

The project leverages the following external crates:

```toml
eframe = "0.33.0"  # GUI framework
rand = "0.9.2"     # Pseudorandom number generation
rodio = "0.21.1"   # Audio playback
```

## Building and Execution

To compile and run the application:

```bash
cargo run --release
```

The `--release` flag is recommended for optimal performance, particularly when visualizing large datasets or multiple algorithms concurrently.

## Usage

1. Launch the application to access the control panel
2. Select desired sorting algorithms from the available options
3. Configure visualization parameters (array size, frame rate)
4. Initiate visualization to observe comparative algorithm behavior
5. Monitor statistical outputs for quantitative performance analysis

## Educational Applications

This tool is designed for:

- **Algorithm Education**: Providing intuitive understanding of sorting algorithm mechanics
- **Comparative Analysis**: Demonstrating performance characteristics across different algorithmic paradigms
- **Computational Complexity Visualization**: Illustrating the practical implications of theoretical complexity bounds

## Implementation Notes

The visualization engine employs a replay-based approach where algorithms are executed to completion during a preparation phase, with all intermediate states captured as frames. This design decision ensures:

- Deterministic playback across multiple synchronized visualizations
- Consistent frame-accurate timing independent of algorithmic complexity
- Elimination of race conditions in concurrent visualization contexts

## License

This project is provided as-is for educational and research purposes.

## References

For theoretical background on the implemented algorithms, refer to:

- Knuth, D. E. (1998). *The Art of Computer Programming, Volume 3: Sorting and Searching*. Addison-Wesley.
- Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2009). *Introduction to Algorithms* (3rd ed.). MIT Press.
