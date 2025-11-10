pub fn insertion_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();

    for unsorted_index in 1..length {
        let current_value = values[unsorted_index];
        let mut insert_index = unsorted_index;

        while insert_index > 0 && values[insert_index - 1] > current_value {
            values[insert_index] = values[insert_index - 1];
            insert_index -= 1;
            frames.push(values.clone());
        }

        values[insert_index] = current_value;
        frames.push(values.clone());
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}
