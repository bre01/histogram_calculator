use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_histogram(points: Vec<f64>) -> Vec<u32> {
    let interval_count = 40;
    let interval_width = 0.025;
    let mut histogram = vec![0; interval_count];

    for &point in &points {
        let interval_index = (point / interval_width) as usize;
        if interval_index < interval_count {
            histogram[interval_index] += 1;
        }
    }

    histogram
}
#[wasm_bindgen]
pub fn modify_by(input_array: Vec<f64>, threshold: f64) -> Vec<u32> {
    let mut modified_array: Vec<u32> = Vec::new();

    for i in (0..input_array.len()).step_by(4) {
        let a_value = input_array[i];
        //let b_value = input_array[i + 3];

        if a_value > threshold {
            modified_array.push(255);
            modified_array.push(255);
            modified_array.push(255);
            modified_array.push(255);
        } else {
            modified_array.push(0);
            modified_array.push(0);
            modified_array.push(0);
            modified_array.push(255);
        }
    }

    modified_array
}





