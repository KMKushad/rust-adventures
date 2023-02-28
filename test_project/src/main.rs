#[derive(Debug)]
struct Cube {
    width: u32,
    height: u32,
    depth: u32,
}
#[derive(Debug)]
struct tablething {
    legs: u32,
    height: u32,
    area: u32,
    name: String,
    items: Vec<String>,
}

impl Cube {
    fn double(&mut self) -> &Cube {
        self.height = self.height * 2;
        self
    }
}

impl tablething {
    fn put(&mut self, item: String) -> &tablething {
        self.items.push(item);
        self
    }
}

fn main() {
    let mut cube1 = Cube {
        width: 30,
        height: 50,
        depth: 30,
    };

    println!("rect1 is {:?}", cube1);

    cube1.double();

    println!("rect1 after being doubled is {:?}", cube1);

    let mut table1: tablething = tablething {
        legs: 4,
        height: 10,
        area: 50,
        name: String::from("My table"),
        items: vec![],
    };

    println!("table1 is {:?}", table1);

    table1.put(String::from("lamp"));
    table1.put(String::from("laptop"));
    table1.put(String::from("book"));
    table1.put(String::from("pencil"));
    table1.put(String::from("coffee"));

    println!("table1 is {:?}", table1);

    for element in table1.items {
        println!("{}", element);
    }
}

fn area(cube: &Cube) -> u32 {
    cube.width * cube.height * cube.depth
}
