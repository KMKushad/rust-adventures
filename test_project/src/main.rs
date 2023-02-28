#[derive(Debug)]
struct Cube {
    width: u32,
    height: u32,
    depth: u32,
}

impl Cube {
    fn double(&mut self) -> &Cube {
        self.height = self.height * 2;
        self
    }
}

fn main() {
    let mut cube1 = Cube {
        width: 30,
        height: 50,
        depth: 30
    };

    println!("rect1 is {:?}", cube1);

    cube1.double();

    println!("rect1 after being doubled is {:?}", cube1);
}

fn area(cube: &Cube) -> u32 {
    cube.width * cube.height * cube.depth
}