
struct Door {
    id: i32,
    winning: bool
}

fn main() {
    let door1: Door = Door {id: 1, winning: false};
    let door2: Door = Door {id: 2, winning: false};
    let door3: Door = Door {id: 3, winning: true};

    let doors:[Door; 3] = [door1, door2, door3];

    print!("\t\tMonty Hall\n");
    print!("there are 3 doors please choose a door you think is the winner\n");
    let mut line: String = String::new();
    let input = std::io::stdin().read_line(&mut line).unwrap();

    print!("{}\n", input);
    let choice: i32 = (input as i32)-1;

    
    
}
