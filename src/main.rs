struct Door {
    id: i32,
    winning: bool
}

fn main() {
    let door1: Door = Door {id: 0, winning: false};
    let door2: Door = Door {id: 1, winning: false};
    let door3: Door = Door {id: 2, winning: true};

    let doors:[Door; 3] = [door1, door2, door3];

    print!("\t\tMonty Hall\n");
    print!("there are 3 doors please choose a door you think is the winner\n");
    let mut line: String = String::new();
    let mut input = std::io::stdin().read_line(&mut line).unwrap();

    let mut choice: i32 = (line.trim().parse::<i32>().unwrap())-1;

    let mut open: i32 = 0;
    let mut other: i32 = 1;

    if choice == 0 {
        if doors[1].winning == false {
            open = 1; 
            other = 2;
        }
        else {
            open = 2;
            other = 1;
        }
    }
    else if choice == 1 {
        if doors[0].winning == false {
            open = 0;
            other = 2;
        }
        else {
            open = 2;
            other = 0;
        }

    }
    else if choice == 2 {
        if doors[1].winning == false {
            open = 1;
            other = 0;
        }
        else {
            open = 0;
            other = 1;
        }

    }

    print!("door {} has been opened and was not the winner\n", other);

    print!("would you like to switch your guess to door {}?\n", other+1);
    
    line = String::new();
    input = std::io::stdin().read_line(&mut line).unwrap();

    if line.trim() == "y" {
        let inter: i32 = other;
        choice = other;
        other = inter;
    }

    if doors[choice as usize].winning == true {
        print!("you chose door {} which was the winning door\n", choice+1);
    }
    else {
        print!("you chose door {} which was not the winning door. door {} was the winning door\n", choice+1, other+1);
    }
    
}
