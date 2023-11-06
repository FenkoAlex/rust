#[derive(Debug)]
struct Point(f32, f32);
#[derive(Debug)]
struct Circle {
    point: Point,
    r: f32,
}

impl Circle {
    fn area(&self) -> f32 {
        core::f32::consts::PI * self.r.powf(2.0)
    }
    fn can_hold(&self, other: &Circle) -> bool {
        self.r > other.r
    }
}

fn main() {
    let circle1 = Circle {
        r: 5_f32,
        point: Point(2_f32, 2_f32),
    };
    let circle2 = Circle {
        r: 2_f32,
        point: Point(3.0, 3.0),
    };
    let circle3 = Circle {
        r: 10_f32,
        point: Point(4.0, 4.0),
    };

    println!("circle is {:#?}", circle1);

    println!(
        "The area of the circle is {} square pixels.",
        area(circle1.r)
    );

    println!("Can circle1 hold circle2? {}", circle1.can_hold(&circle2));
    println!("Can circle1 hold circle3? {}", circle1.can_hold(&circle3));
}

fn area(r: f32) -> f32 {
    core::f32::consts::PI * r.powf(2.0)
}
