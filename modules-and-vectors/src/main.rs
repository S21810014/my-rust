mod timothy;
use timothy::external::vector;

fn main() {
    let mut vec3: Vec<i32> = vector::create_vec3((1, 2, 3));

    vector::add_scalar_to_vec3(&mut vec3, 5);
    
    for (index, elem) in vec3.iter().enumerate() {
        println!("{} {}", index, elem);
    }
}
