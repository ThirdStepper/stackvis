use crate::sorting_algorithms::heap_sort::heap_sort_with_recording;

pub fn intro_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();
    if length > 1 {
        let depth_limit = (2.0 * (length as f64).log2().floor()) as usize;
        intro_sort_recursive(&mut values, 0, length, depth_limit, frames);
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}

fn intro_sort_recursive(
    values: &mut Vec<u32>,
    start_index: usize,
    end_index: usize,
    depth_limit: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    let range_length = end_index.saturating_sub(start_index);
    if range_length <= 1 {
        return;
    }

    if depth_limit == 0 {
        heap_sort_range(values, start_index, end_index, frames);
        return;
    }

    let pivot_final_index = partition_range(values, start_index, end_index, frames);

    if pivot_final_index > start_index {
        intro_sort_recursive(
            values,
            start_index,
            pivot_final_index,
            depth_limit - 1,
            frames,
        );
    }

    if pivot_final_index + 1 < end_index {
        intro_sort_recursive(
            values,
            pivot_final_index + 1,
            end_index,
            depth_limit - 1,
            frames,
        );
    }
}

fn partition_range(
    values: &mut Vec<u32>,
    start_index: usize,
    end_index: usize,
    frames: &mut Vec<Vec<u32>>,
) -> usize {
    let pivot_index = end_index - 1;
    let pivot_value = values[pivot_index];

    let mut store_index = start_index;

    for scan_index in start_index..pivot_index {
        if values[scan_index] < pivot_value {
            if scan_index != store_index {
                values.swap(scan_index, store_index);
                frames.push(values.clone());
            }
            store_index += 1;
        }
    }

    if store_index != pivot_index {
        values.swap(store_index, pivot_index);
        frames.push(values.clone());
    }

    store_index
}

fn heap_sort_range(
    values: &mut Vec<u32>,
    start_index: usize,
    end_index: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    if end_index <= start_index {
        return;
    }

    let segment: Vec<u32> = values[start_index..end_index].to_vec();
    let mut local_frames: Vec<Vec<u32>> = Vec::new();

    heap_sort_with_recording(&segment, &mut local_frames);

    // map segment frames to full array
    for (frame_index, segment_state) in local_frames.into_iter().enumerate() {
        // skip initial frame to avoid duplication
        if frame_index == 0 {
            continue;
        }

        if segment_state.len() != end_index - start_index {
            continue;
        }

        values[start_index..end_index].clone_from_slice(&segment_state);
        frames.push(values.clone());
    }
}
