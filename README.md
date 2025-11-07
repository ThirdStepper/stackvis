# StackVis: Sorting Algorithm Visualizer

## Overview

StackVis is an educational tool for visualizing and comparing sorting algorithms in real-time. Implemented in Rust, it provides concurrent visual representations of multiple algorithms operating on identical datasets, enabling direct comparative analysis of algorithmic behavior and performance.

## Architecture

- **Engine** ([engine.rs](src/engine.rs)): Orchestrates algorithm execution and maintains synchronized state snapshots
- **Algorithms** ([sorting_algorithms/](src/sorting_algorithms/)): Implements twelve sorting algorithms with frame-by-frame recording
- **Interface** ([ui/](src/ui/)): Built on egui/eframe for responsive visualization
- **Statistics** ([stats.rs](src/stats.rs)): Tracks performance metrics and operation counts
- **Audio**: Provides auditory feedback with stereo panning based on element position

## Implemented Algorithms

- **Comparison-based**: Bubble, Cocktail Shaker, Insertion, Selection, Gnome
- **Divide-and-conquer**: Quick, Merge, Heap
- **Gap-based**: Shell, Comb
- **Hybrid**: Intro Sort, Tim Sort

## Features

- Parallel visualization of multiple algorithms
- Configurable array size and frame rate
- Real-time statistical analysis
- Audio synthesis with positional stereo imaging
- Custom color palette support

## Usage

```bash
cargo run --release
```

Select algorithms from the control panel, configure parameters, and initiate visualization to observe comparative behavior.

## References

- Knuth, D. E. (1998). *The Art of Computer Programming, Volume 3: Sorting and Searching*. Addison-Wesley.
- Cormen, T. H., et al. (2009). *Introduction to Algorithms* (3rd ed.). MIT Press.
