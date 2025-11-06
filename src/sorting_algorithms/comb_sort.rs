pub fn comb_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
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

    let mut gap_size: usize = length;
    let shrink_factor: f32 = 1.3;
    let mut swap_performed_in_pass = true;

    while gap_size > 1 || swap_performed_in_pass {
        gap_size = ((gap_size as f32) / shrink_factor).floor() as usize;
        if gap_size < 1 {
            gap_size = 1;
        }

        swap_performed_in_pass = false;

        let mut left_index: usize = 0;
        while left_index + gap_size < length {
            let right_index = left_index + gap_size;
            if values[left_index] > values[right_index] {
                values.swap(left_index, right_index);
                frames.push(values.clone());
                swap_performed_in_pass = true;
            }
            left_index += 1;
        }
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}
