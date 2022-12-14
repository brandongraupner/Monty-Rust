use rand::prelude::*;

fn main() {
    let mut doors:[bool; 3] = [false, false, false];

    let mut rng = rand::thread_rng();
    let x: usize = rng.gen_range(0..2);

    doors[x] = true;

    print!("\t\tMonty Hall\n");
    print!("there are 3 doors please choose a door you think is the winner\n");
    let mut line: String = String::new();
    let mut _input = std::io::stdin().read_line(&mut line).unwrap();

    let mut choice: i32 = (line.trim().parse::<i32>().unwrap())-1;

    let mut open: i32 = 0;
    let mut other: i32 = 1;

    if choice == 0 {
        if doors[1] == false {
            open = 1; 
            other = 2;
        }
        else {
            open = 2;
            other = 1;
        }
    }
    else if choice == 1 {
        if doors[0] == false {
            open = 0;
            other = 2;
        }
        else {
            open = 2;
            other = 0;
        }

    }
    else if choice == 2 {
        if doors[1] == false {
            open = 1;
            other = 0;
        }
        else {
            open = 0;
            other = 1;
        }

    }

    print!("door {} has been opened and was not the winner\n", open+1);

    print!("would you like to switch your guess to door {}?\n", other+1);
    
    line = String::new();
    _input = std::io::stdin().read_line(&mut line).unwrap();

    if line.trim() == "y" {
        let inter: i32 = other;
        choice = other;
        other = inter;
    }

    if doors[choice as usize] == true {
        print!("you chose door {} which was the winning door\n", choice+1);
    }
    else {
        print!("you chose door {} which was not the winning door. door {} was the winning door\n", choice+1, other+1);
    }
    
}
