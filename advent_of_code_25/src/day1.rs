use std::fs;

pub fn day1_part1() {
    let contents = fs::read_to_string("data/day1.txt").expect("Could not read data");
    let mut value = 50;
    let mut zeros = 0;
    for line in contents.lines(){
        let line = line.trim();
        let direction = line.chars().next().unwrap();
        let amount = line[1..].parse::<i32>().ok().unwrap();
        
        if direction == 'L'{
            value -= amount;
        } else if direction =='R'{
            value += amount;
        }
        value = value.rem_euclid(100);
        
        if value == 0{
                zeros += 1;
        }
    }
    println!("The password is: {zeros}")
}

pub fn day1_part2() {
    let contents = fs::read_to_string("data/day1.txt").expect("Could not read data");
    let mut value = 50;
    let mut zeros = 0;
    let mut step:i32;
    for line in contents.lines(){
        let line = line.trim();
        let direction = line.chars().next().unwrap();
        let amount = line[1..].parse::<i32>().ok().unwrap();

        if direction == 'L'{
            step = -1;
        } else {
            step = 1;
        }

        for _i in 0..amount {
            value += step;
            if value == -1 {
                    value = 99
            }
            if value == 100 {
                    value = 0
                }
            if value == 0 {
                zeros+=1;
            }

        }

    }
    println!("The password is: {zeros}");
}