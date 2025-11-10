pub fn quick_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    
    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();

    if length > 1 {
        quick_sort_recursive(&mut values, 0, length - 1, frames);
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}

fn quick_sort_recursive(
    values: &mut [u32],
    low_index: usize,
    high_index: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    if low_index >= high_index {
        return;
    }

    let partition_index = partition(values, low_index, high_index, frames);

    if partition_index > 0 {
        quick_sort_recursive(values, low_index, partition_index - 1, frames);
    }

    if partition_index < high_index {
        quick_sort_recursive(values, partition_index + 1, high_index, frames);
    }
}

fn partition(
    values: &mut [u32],
    low_index: usize,
    high_index: usize,
    frames: &mut Vec<Vec<u32>>,
) -> usize {
    let pivot_value = values[high_index];
    let mut store_index = low_index;

    for scan_index in low_index..high_index {
        if values[scan_index] < pivot_value {
            if scan_index != store_index {
                values.swap(store_index, scan_index);
                frames.push(values.to_vec());
            }
            store_index += 1;
        }
    }

    if store_index != high_index {
        values.swap(store_index, high_index);
        frames.push(values.to_vec());
    }

    store_index
}