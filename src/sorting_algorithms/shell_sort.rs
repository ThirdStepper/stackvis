pub fn shell_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();
    let mut gap_size = length / 2;

    while gap_size > 0 {
        let mut current_index = gap_size;

        while current_index < length {
            let mut insert_index = current_index;

            while insert_index >= gap_size
                && values[insert_index - gap_size] > values[insert_index]
            {
                values.swap(insert_index, insert_index - gap_size);
                frames.push(values.clone());
                insert_index -= gap_size;
            }

            current_index += 1;
        }

        gap_size /= 2;
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}
