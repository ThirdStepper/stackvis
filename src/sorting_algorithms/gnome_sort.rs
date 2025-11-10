pub fn gnome_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
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

    let mut current_index: usize = 1;
    let mut next_index: usize = 2;

    while current_index < length {
        if values[current_index - 1] <= values[current_index] {
            current_index = next_index;
            next_index += 1;
        } else {
            values.swap(current_index - 1, current_index);
            frames.push(values.clone());

            if current_index > 1 {
                current_index -= 1;
            } else {
                current_index = next_index;
                next_index += 1;
            }
        }
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}
