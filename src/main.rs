use rand::prelude::*;
use std::process::exit;
use colored::*;
use serde_derive::{Deserialize, Serialize};

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
    print!("{}","\n\nMonty Hall\n".cyan());
    print!("{}","press q at any time to quit\n".yellow());

    let mut score = Score {correct: 0., incorrect: 0., correct_stay: 0., incorrect_stay: 0., correct_switch: 0., incorrect_switch: 0., correct_percent: 0., correct_switch_percent: 0., correct_stay_percent: 0. }; 


    loop {
        let mut doors:[bool; 3] = [false, false, false];

        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..2);

        let mut switched:bool = false;

        doors[x] = true;

        print!("{}","------------------------------------------------------------------\n".cyan());
        print!("{}","there are 3 doors please choose a door you think is the winner\n\n".yellow());
        print!("{}","Door 1, Door 2, Door 3\n".cyan());

        let mut line: String = String::new();
        let mut _input = std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().eq("q") {
            save_and_exit(&mut score);
        }

        //use match to continue to next loop if input not a number
        let mut choice: i32 = match line.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };


        //if the choice is not one of the 3 doors continue
        if 0 >= choice || choice >3 {
            print!("{}", "invalid door\n".red());
            continue;
        }

        choice = choice - 1;

        print!("\n{} {} \n\n", "you picked door ".blue(), (choice+1).to_string().blue());

        let mut open: i32 = 0;
        let mut other: i32 = 1;

        //depending on chosen door open the corisponding losing door
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

        print!("{} {} {}","door".cyan(), (open+1).to_string().cyan(), " has been opened and was not the winner\n\n".cyan());

        print!("{} {} {} {}\n\n", "would you like to switch your guess from door ".yellow(), (choice + 1).to_string().yellow(), " to door ".yellow(), (other+1).to_string().yellow() );
    
        line = String::new();
        _input = std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().eq("y") {
            switched = true;
            let inter: i32 = other;
            choice = other;
            other = inter;
        }
        else if line.trim().eq("q") {
            save_and_exit(&mut score);
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

            print!("**********************************************************\n");
            print!("{} {} {}","you chose door".green(), (choice+1).to_string().green(), "which was the winning door\n".green());
            print!("**********************************************************\n\n\n");
        }
        else {
            print!("\n{} {} {} {} {}","you chose door".red(), (choice+1).to_string().red(), "which was not the winning door. door".red(),(other+1).to_string().red(), "was the winning door\n\n\n".red());
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

        score.correct_percent = (score.correct / (score.incorrect+score.correct)).into();

        println!("Correct: {}, Incorrect {}, total Correct: {}%, Correct_stay: {}, Incorrect_stay: {}, Stay percent: {}%, Correct_switch: {}, Incorrect_switch: {}, Correct_switch_percent: {}%\n", score.correct, score.incorrect, score.correct_percent*100., score.correct_stay, score.incorrect_stay, score.correct_stay_percent*100., score.correct_switch, score.incorrect_switch, score.correct_switch_percent*100.);
    } 
}

#[warn(unused_must_use)]
fn save_and_exit(score:&mut Score) {
    std::fs::write(
    "saves/save.json",
    serde_json::to_string_pretty(&score).unwrap(),
    );

    exit(0);
}
