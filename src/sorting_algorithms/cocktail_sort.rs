pub fn cocktail_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    // Record initial unsorted state
    frames.push(values.clone());

    let length = values.len();
    if length <= 1 {
        if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
            frames.push(values);
        }
        return;
    }

    let mut start_index: usize = 0;
    let mut end_index: usize = length - 1;
    let mut has_swapped_in_pass = true;

    while has_swapped_in_pass {
        has_swapped_in_pass = false;

        // Forward pass: left -> right
        for left_index in start_index..end_index {
            let right_index = left_index + 1;
            if values[left_index] > values[right_index] {
                values.swap(left_index, right_index);
                frames.push(values.clone());
                has_swapped_in_pass = true;
            }
        }

        if !has_swapped_in_pass {
            break;
        }

        has_swapped_in_pass = false;

        if end_index == 0 {
            break;
        }
        end_index -= 1;

        // Backward pass: right -> left
        let mut right_index = end_index;
        while right_index > start_index {
            let left_index = right_index - 1;
            if values[left_index] > values[right_index] {
                values.swap(left_index, right_index);
                frames.push(values.clone());
                has_swapped_in_pass = true;
            }

            if right_index == 0 {
                break;
            }
            right_index -= 1;
        }

        start_index += 1;
        if start_index >= end_index {
            break;
        }
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}
