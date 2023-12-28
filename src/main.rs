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

