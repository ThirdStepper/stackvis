pub fn bubble_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();

    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();
    let mut is_swapped = true;

    while is_swapped {
        is_swapped = false;

        for left_index in 0..length.saturating_sub(1) {
            let right_index = left_index + 1;

            if values[left_index] > values[right_index] {
                values.swap(left_index, right_index);
                frames.push(values.clone());
                is_swapped = true;
            }
        }
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }

}