use rand::Rng;
use crate::Suit::{Club, Diamond, Heart, Spade};

fn main() {
    let mut rnd = rand::thread_rng();
    let num = rnd.gen_range(0,11);
    /*if num >= 5 {
        println!("Number {} is greater than or equal to 5 ", num);
    } else {
        println!("Number {} is smaller than 5", num);
    }*/
    if num > 5 {
        println!("{} > 5",num);
    } else if num == 5 {
        println!("{} = 5", num);

    }else {
        println!("{} < 5", num);
    }

    let res = if num >=5 {true} else {false};
    println!("{}", res);
    print_choice(Heart);
    print_choice(Spade);
    print_choice(Club);
    print_choice(Diamond);

    country(44);
    country(34);
    country(125);
    country(-15);

    for i in 0..35{
        println!("{}. I have {} oranges", i, get_oranges(i));
    }
    let point = (0, 0);
    let point = (56, 0);
    let point = (0, 39);
    let point = (56, 39);

    match point {
        (0, 0) => println!("origin"),
        (x,0) => println!("x axis ({}, 0",x),
        (0,y) => println!("y axis (0, {}",y),
        (x,y) => println!("({},{})",x ,y)
    }
    for i in 1..11{
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "chihuahua", "hamster", "bear"];
    for pet in pets.iter() {
        if pet == &"chihuahua"{
            println!("{} barks too much", pet);
            continue
        }
        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break
        }
        println!("I love my {}", pet);
    }
    for (pos,i) in (1..11).enumerate() {
        let square = i*i;
        let nb = pos + 1;
        println!("{0} * {0} = {1}", nb, square);
    }
    get_squares(3478);
    get_cubes(4938);


}
fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid"
    };
    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}
fn print_choice(choice: Suit){
    match choice {
        Heart => {println!("\u{2665}")}
        Spade=> {println!("\u{2660}")}
        Club => {println!("\u{2663}")}
        Diamond=> {println!("\u{2666}")}
    }
}
fn get_oranges(amount:i32)->&'static str{
    return match amount{
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of"
    }
}
fn get_squares(limit: i32) {
    let mut x = 1;
    while x * x < limit {
        println!("{0} * {0} = {1}", x,x*x);
        x+=1;
    }

}
fn get_cubes(limit: i32){
    let mut x = 1;
    loop{
        println!("{0} * {0} * {0} = {1}",x ,x*x*x);
        x+=1;
        if x*x*x > limit {
            break
        }
    }
}
