pub fn selection_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    // Record initial unsorted state
    frames.push(values.clone());

    let length = values.len();

    for sorted_boundary_index in 0..length {
        let mut index_of_minimum = sorted_boundary_index;

        for candidate_index in (sorted_boundary_index + 1)..length {
            if values[candidate_index] < values[index_of_minimum] {
                index_of_minimum = candidate_index;
            }
        }

        if index_of_minimum != sorted_boundary_index {
            values.swap(sorted_boundary_index, index_of_minimum);
            frames.push(values.clone());
        }
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}
