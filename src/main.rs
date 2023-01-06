fn main() {
    if_statement();
    match_statement();
    pattern_matching();
    for_loop();
    while_loop();
}

fn if_statement() {
    //IF False Demo make this 3
    let demo = 1;
    if demo == 1 {                      //This is How IF is written
        println!("The value is 1");
    } else if demo == 2 {               //This is How ELSE IF is written
        println!("The value is 2");
    } else {                            //This is How ELSE is written   
        println!("This is the ELSE statement");
    }
}
fn match_statement() {
    let demo = 4;
    
    match demo {
        1 => println!("The value is 1"),
        2 => println!("The value is 2"),
        3 => println!("The value is 3"),
        4 => println!("The value is 4"),
        5 => println!("The value is 5"),
        _ => println!("This is the ELSE statement"),
    }

}

fn pattern_matching() {
    let demo :i32 = 0;
    for demo in 0..15 {
        println!("The value is: {}", demo);
        match demo {
            0 => println!("Normal Matching"),
            1..=6 => println!("Range Matching"),
            7|8 => println!("This or That Matching"),
            _ if (demo % 2 == 0) => println!("If Divisible by 2 Pattern Matching using _ if"),
            _ => println!("Default Matching"),
        }
    }

}

fn for_loop() {
    let mut count = 0;
    for count in 0..10 {
        println!("Counting up {}", count);
    }

    let pets = ["Dog", "Cat", "Bird", "Fish", "Snake", "Deer", "Rabbit", "Horse", "Cow", "Pig", "Death", "Duck"];
    for pet in pets.iter() {
        if pet == &"Deer" {
            continue;
        }
        else if pet == &"Death" {
            println!("This is not a Animal and broke the loop, way to go bud... {}", pet);
            break;
        }
        println!("The pet is: {}", pet);
    }
}

fn while_loop() {

    //Continue and Break work here too not just for loops
    let mut count = 0;
    while count < 10 {
        println!("Counting up {}", count);
        count += 1;
    }

    //While(true) and Loop are the same thing
    let mut count2 = 0;
    loop {
        println!("Counting up {}", count);
        count += 1;
        if count == 10 {
            break; //Need this to break out of the loop
        }
    }
}

