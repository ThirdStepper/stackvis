pub fn heap_sort_with_recording(initial_values: &[u32], frames: &mut Vec<Vec<u32>>) {
    let mut values = initial_values.to_vec();
    if values.is_empty() {
        return;
    }

    frames.push(values.clone());

    let length = values.len();

    // build max heap
    if length > 1 {
        let mut heap_index = length / 2;
        while heap_index > 0 {
            heap_index -= 1;
            sift_down(&mut values, heap_index, length, frames);
        }
    }

    // extract max repeatedly
    let mut unsorted_size = length;
    while unsorted_size > 1 {
        unsorted_size -= 1;

        values.swap(0, unsorted_size);
        frames.push(values.clone());

        sift_down(&mut values, 0, unsorted_size, frames);
    }

    if frames.last().map(|last_frame| last_frame.as_slice()) != Some(values.as_slice()) {
        frames.push(values);
    }
}

fn sift_down(
    values: &mut Vec<u32>,
    start_index: usize,
    heap_size: usize,
    frames: &mut Vec<Vec<u32>>,
) {
    let mut root_index = start_index;

    loop {
        let left_child_index = 2 * root_index + 1;
        let right_child_index = left_child_index + 1;

        if left_child_index >= heap_size {
            break;
        }

        let mut index_of_largest = root_index;

        if values[left_child_index] > values[index_of_largest] {
            index_of_largest = left_child_index;
        }

        if right_child_index < heap_size && values[right_child_index] > values[index_of_largest] {
            index_of_largest = right_child_index;
        }

        if index_of_largest == root_index {
            break;
        }

        values.swap(root_index, index_of_largest);
        frames.push(values.clone());

        root_index = index_of_largest;
    }
}
