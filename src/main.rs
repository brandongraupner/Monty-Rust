use rand::prelude::*;
use std::process::exit;
use colored::*;
use std::io::*;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use serde_json::{Value};

#[derive(Deserialize, Serialize, Debug)]
struct Score {
    correct: f32,
    incorrect: f32,
    correct_stay: f32,
    incorrect_stay: f32,
    correct_switch: f32,
    incorrect_switch: f32,
    correct_percent: f32,
    correct_switch_percent: f32,
    correct_stay_percent: f32,
}

fn main() {
    loop {
        let mut score = Score {correct: 0., incorrect: 0., correct_stay: 0., incorrect_stay: 0., correct_switch: 0., incorrect_switch: 0., correct_percent: 0., correct_switch_percent: 0., correct_stay_percent: 0. }; 

        println!("Welcome to Monty Rust\n The Monty Hall problem game made in rust\n");
        println!("Instructions:\n Play: 'p'\n Load game: 'l'\n Quit: \n");

        std::io::stdout().flush().unwrap();
        let mut line: String = String::new();
        let mut _input = std::io::stdin().read_line(&mut line).unwrap();
    
        //depending on user input execute action
        if line.trim().eq("q") {
            exit(0);
        }
        else if line.trim().eq("l") {
            score = load_save();
        }
        else if line.trim().eq("p") {
            game_loop(score);
        }
    }
}

//handles main game logic
fn game_loop(mut score: Score) {
    loop {
        let mut doors:[bool; 3] = [false, false, false];

        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..2);

        let mut switched:bool = false;

        doors[x] = true;
        
        //TODO make main menu with load play and exit options 

        println!("{}","------------------------------------------------------------------\n".cyan());
        println!("{}","there are 3 doors please choose a door you think is the winner\n\n".yellow());
        println!("{}","Door 1, Door 2, Door 3\n".cyan());
        
        std::io::stdout().flush().unwrap();
        let mut line: String = String::new();
        let mut _input = std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().eq("q") || line.trim().eq("s") {
            save_and_exit(&mut score);
        }
        else if line.trim().eq("l") {
            score = load_save();
        }

        //use match to continue to next loop if input not a number
        let mut choice: i32 = match line.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };


        //if the choice is not one of the 3 doors continue
        if 0 >= choice || choice >3 {
            println!("{}", "invalid door\n".red());
            continue;
        }

        choice = choice - 1;

        println!("\n{} {} \n\n", "you picked door ".blue(), (choice+1).to_string().blue());

        

        let open_other = chose_door(choice, doors);
        let open:i32 = open_other.0;
        let mut other:i32 = open_other.1;

        println!("{} {} {}","door".cyan(), (open+1).to_string().cyan(), " has been opened and was not the winner\n\n".cyan());

        println!("{} {} {} {}\n\n", "would you like to switch your guess from door ".yellow(), (choice + 1).to_string().yellow(), " to door ".yellow(), (other+1).to_string().yellow() );
    
        std::io::stdout().flush().unwrap();
        line = String::new();
        _input = std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().eq("y") {
            switched = true;
            let inter: i32 = other;
            choice = other;
            other = inter;
        }
        else if line.trim().eq("q") || line.trim().eq("s") {
            save_and_exit(&mut score);
        }
        else if line.trim().eq("l") {
            score = load_save();
        }

        if doors[choice as usize] == true {
            score.correct +=1.;

            //if the player has switched add a correct switch and calculate correct_switch_percent
            if switched == true {
                score.correct_switch += 1.;
                score.correct_switch_percent = (score.correct_switch / (score.incorrect_switch+score.correct_switch)).into();
            }
            //if player hasn't switched add a correct_stay and calculate correct_stay_percent
            else {
                score.correct_stay +=1.;
                score.correct_stay_percent = (score.correct_stay / (score.incorrect_stay+score.correct_stay)).into();
            }

            println!("**********************************************************\n");
            println!("{} {} {}","you chose door".green(), (choice+1).to_string().green(), "which was the winning door\n".green());
            println!("**********************************************************\n\n\n");
        }
        else {
            println!("\n{} {} {} {} {}","you chose door".red(), (choice+1).to_string().red(), "which was not the winning door. door".red(),(other+1).to_string().red(), "was the winning door\n\n\n".red());
            score.incorrect +=1.;

            //if the player has switched add and incorrect switch and calculate
            //correct_switch_percent
            if switched == true {
                score.incorrect_switch +=1.;
                score.correct_switch_percent = (score.correct_switch / (score.incorrect_switch+score.correct_switch)).into();
            }
            //if player hasn't switched add an incorrect stay and calculate correct_stay_percent
            else {
                score.incorrect_stay +=1.;
                score.correct_stay_percent = (score.correct_stay / score.incorrect_stay).into();
            }
        }

        //calculate percentage of correct answers
        score.correct_percent = (score.correct / (score.incorrect+score.correct)).into();

        print_score(&score);
    } 
}

//choses which door to open based on which door the user picks
fn chose_door(choice: i32, doors: [bool; 3]) -> (i32, i32){

    let mut open_other: (i32, i32) = (0,0);

    //depending on chosen door open the corisponding losing door
    if choice == 0 {
        if doors[1] == false {
            open_other = (1,2);
        }
        else {
            open_other = (2,1);
        }
    }
    else if choice == 1 {
        if doors[0] == false {
            open_other = (0,2);
        }
        else {
            open_other = (2,0);
        }
    }
    else if choice == 2 {
        if doors[1] == false {
            open_other = (1,0);
        }
        else {
            open_other = (0,1);
        }
    }

    return open_other;
}

//either saves the score data to a file or exits
#[warn(unused_must_use)]
fn save_and_exit(score:&mut Score) {
    println!("enter the name for your save or enter nothing to not save\n");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    //get user input
    let _input = std::io::stdin().read_line(&mut line).unwrap();

    //if player has entered a value save the data
    if !line.trim().eq("") {
        let mut save_file = "saves/".to_owned();
        save_file.push_str(line.trim());
        save_file.push_str(".json");
        std::fs::write(
        save_file,
        serde_json::to_string_pretty(&score).unwrap(),
        );
    }
    exit(0);
}

fn load_save() -> Score{
    println!("enter name of save you want to load(to not include file type)");

    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    let mut _input = std::io::stdin().read_line(&mut line).unwrap();

    let file_name = line.trim();

    let mut name: String = "saves/".to_owned();
    //adds save path to file_name
    name.push_str(file_name);
    name.push_str(".json");
    let mut file = File::open(name).unwrap();
    let mut data = String::new();

    //reads from file to data
    file.read_to_string(&mut data).unwrap();

    //convers the str to json
    let json: Value = serde_json::from_str(&data).unwrap();

    //creates a Score struct with the values read from the save file
    let score = Score {correct: json["correct"].to_string().parse::<f32>().unwrap(), incorrect: json["incorrect"].to_string().parse::<f32>().unwrap(), correct_stay: json["correct_stay"].to_string().parse::<f32>().unwrap(), incorrect_stay: json["incorrect_stay"].to_string().parse::<f32>().unwrap(), correct_switch: json["correct_switch"].to_string().parse::<f32>().unwrap(), incorrect_switch: json["incorrect_switch"].to_string().parse::<f32>().unwrap(), correct_percent: json["correct_percent"].to_string().parse::<f32>().unwrap(), correct_switch_percent: json["correct_switch_percent"].to_string().parse::<f32>().unwrap(), correct_stay_percent: json["correct_stay_percent"].to_string().parse::<f32>().unwrap()};
    
    print_score(&score);
    return score;
}

//prints all values from the score
fn print_score(score: &Score) {
        println!("Correct: {}, Incorrect {}, total Correct: {}%, Correct_stay: {}, Incorrect_stay: {}, Stay percent: {}%, Correct_switch: {}, Incorrect_switch: {}, Correct_switch_percent: {}%\n", score.correct, score.incorrect, score.correct_percent*100., score.correct_stay, score.incorrect_stay, score.correct_stay_percent*100., score.correct_switch, score.incorrect_switch, score.correct_switch_percent*100.);
}
