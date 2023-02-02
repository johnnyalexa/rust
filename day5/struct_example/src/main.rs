#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };


    println!("{:#?}", rect1);
    dbg!(&rect1);
    println!(
        "the aread of the rectabgle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 15,
    };

    let rect3 = Rectangle {
        width: 70,
        height: 25,
    };

    println!("Can rect1 hold rect2> {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3> {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(5);
    dbg!(&rect4);
}

impl Rectangle {
    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}

