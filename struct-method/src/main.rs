struct Rectangle {
    width: i64,
    height: i64,
}

impl Rectangle {
    fn square(size: i64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> i64 {
        self.width * self.height
    }

    fn reset(&mut self) {
        self.width = 0;
        self.height = 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut my_rect: Rectangle = Rectangle {
        width: 50,
        height: 20
    };

    let my_rect2: Rectangle = Rectangle {
        width: 40,
        height: 10
    };

    let my_rect3: Rectangle = Rectangle {
        width: 60,
        height: 20
    };

    let my_square: Rectangle = Rectangle::square(8);

    println!("Can my_rect hold my_rect2? {}", my_rect.can_hold(&my_rect2));
    println!("Can my_rect hold my_rect3? {}", my_rect.can_hold(&my_rect3));

    println!("Area my_rect: {}", my_rect.area());

    my_rect.reset();

    println!("Area my_rect after reset: {}", my_rect.area());
    println!("Area square: {}", my_square.area());

}
