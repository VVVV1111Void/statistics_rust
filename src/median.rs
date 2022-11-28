pub fn median(array_sorted: &[i32]) -> f32 {
    let mid = array_sorted.len() / 2;
    if array_sorted.len() % 2 == 0 { // even situation
        (array_sorted[mid - 1] + array_sorted[mid]) as f32 / 2.0 // calculate mean whilst converting types
    }
    else {
        array_sorted[mid] as f32
    }
}