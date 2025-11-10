const MINIMUM_RUN_LENGTH: usize = 32;

pub fn tim_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();
    if length <= 1 {
        if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
            frames.push(values);
        }
        return;
    }

    let run_length = if length < MINIMUM_RUN_LENGTH {
        length
    } else {
        MINIMUM_RUN_LENGTH
    };

    // sort small runs with insertion sort
    let mut start_index: usize = 0;
    while start_index < length {
        let end_index = (start_index + run_length).min(length);
        insertion_sort_range(&mut values, start_index, end_index, frames);
        start_index = end_index;
    }

    // merge runs bottom-up
    let mut current_run_size = run_length;
    while current_run_size < length {
        let mut merge_start_index: usize = 0;

        while merge_start_index < length {
            let middle_index = (merge_start_index + current_run_size).min(length);
            if middle_index >= length {
                break;
            }

            let merge_end_index =
                (merge_start_index + 2 * current_run_size).min(length);

            merge_ranges(
                &mut values,
                merge_start_index,
                middle_index,
                merge_end_index,
                frames,
            );

            merge_start_index += 2 * current_run_size;
        }

        current_run_size *= 2;
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}

fn insertion_sort_range(
    values: &mut Vec<u32>,
    start_index: usize,
    end_index: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    if end_index <= start_index + 1 {
        return;
    }

    for unsorted_index in (start_index + 1)..end_index {
        let current_value = values[unsorted_index];
        let mut insert_index = unsorted_index;

        while insert_index > start_index && values[insert_index - 1] > current_value {
            values[insert_index] = values[insert_index - 1];
            insert_index -= 1;
            frames.push(values.clone());
        }

        values[insert_index] = current_value;
        frames.push(values.clone());
    }
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
