use std::collections::HashMap;

pub fn calculate_mean(target_vector: &Vec<i32>) -> f32 {
    return {let mut sum = 0; for elem in target_vector {sum += elem;} sum} as f32 / target_vector.len() as f32;
}

pub fn calculate_median(target_vector: &Vec<i32>) -> i32 {
    let mut reference_vector: Vec<i32> = target_vector.clone();
    reference_vector.sort();

    reference_vector[reference_vector.len() / 2]
}

pub fn calculate_mode(target_vector: &Vec<i32>) -> (i32, i32) {
    let mut elem_map: HashMap<i32, i32> = HashMap::new();

    for elem in target_vector {
        let elem_count: &mut i32 = elem_map.entry(elem.clone()).or_insert(0);
        *elem_count += 1;
    }

    let mut elem_map: Vec<(i32, i32)> = elem_map.into_iter().collect();

    elem_map.sort_by(|b, a| a.1.cmp(&b.1));
    
    return elem_map[0];
}