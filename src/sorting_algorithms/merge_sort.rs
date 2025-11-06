pub fn merge_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    // Record initial unsorted state
    frames.push(values.clone());

    let length = values.len();
    if length > 1 {
        merge_sort_recursive(&mut values, 0, length, frames);
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}

fn merge_sort_recursive(
    values: &mut Vec<u32>,
    start_index: usize,
    end_index: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    if end_index.saturating_sub(start_index) <= 1 {
        return;
    }

    let middle_index = start_index + (end_index - start_index) / 2;

    merge_sort_recursive(values, start_index, middle_index, frames);
    merge_sort_recursive(values, middle_index, end_index, frames);

    merge_ranges(values, start_index, middle_index, end_index, frames);
}

fn merge_ranges(
    values: &mut Vec<u32>,
    start_index: usize,
    middle_index: usize,
    end_index: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    let mut left_index = start_index;
    let mut right_index = middle_index;

    let mut temporary_values: Vec<u32> = Vec::with_capacity(end_index - start_index);

    while left_index < middle_index && right_index < end_index {
        if values[left_index] <= values[right_index] {
            temporary_values.push(values[left_index]);
            left_index += 1;
        } else {
            temporary_values.push(values[right_index]);
            right_index += 1;
        }
    }

    while left_index < middle_index {
        temporary_values.push(values[left_index]);
        left_index += 1;
    }

    while right_index < end_index {
        temporary_values.push(values[right_index]);
        right_index += 1;
    }

    for (offset, temporary_value) in temporary_values.into_iter().enumerate() {
        values[start_index + offset] = temporary_value;
        frames.push(values.clone());
    }
}
