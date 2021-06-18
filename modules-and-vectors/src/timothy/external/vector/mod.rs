pub fn create_vec3(num: (i32, i32, i32)) -> Vec<i32>{
    let mut v: Vec<i32> = Vec::new();

    v.push(num.0);
    v.push(num.1);
    v.push(num.2);

    v
}

pub fn add_scalar_to_vec3(target_vec:&mut Vec<i32>, scalar: i32) {
    for elem in target_vec {
        *elem += scalar;    
    }
}