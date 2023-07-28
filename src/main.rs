use std::num::ParseIntError;

enum Time {
    Morning(String),
    Afternoon(String),
    Evening(String),
}

fn main() {
    foriflet();
}

fn foriflet(){
    let list = vec![1,2,3,4,5];

    for i in 0..10 {
        if let Some(el)=list.get(i){
            println!("{}",el);
        }else {
            println!("Out of bounds");
            break;
        }
    }
}

fn ifletparse(){
    let num = "7";
    let mut num_int = 0;
    let parsed = num.parse::<i32>();

    /*match parsed {
        Ok(n) => {
            num_int=n;
        }
        Err(_) => println!("Enter the number!")
    }
    */

    if let Ok(n) = parsed {
        num_int = n;
    } else { println!("Enter the number!"); }

    println!("{}", num_int);
}

fn to_compare2() {
    let num: Option<i32> = Some(7);

    match num {
        None => println!("None"),
        Some(n) => println!("{}", n)
    }
    if let Some(n) = num {
        println!("{}", n);
    } else { println!("None"); }
}

fn to_compare() {
    let t = Time::Afternoon("Hello User".to_string());

    /*match t {
        Time::Morning(m) => println!("M {}",m),
        Time::Afternoon(m) => println!("A {}",m),
        Time::Evening(m) => println!("E {}",m),
    }*/

    if let Time::Morning(m) = t {
        println!("Morning {}", m);
    } else if let Time::Evening(m) = t {
        println!("Evening {}", m);
    } else if let Time::Afternoon(m) = t {
        println!("Afternoon {}", m);
    }
}

fn ifletkortej() {
    let nums = (1, 2, 3, 4, 5);
    if let (1, a, b, c, d) = nums {
        println!("Yess :{},{},{},{}", a, b, c, d);
    } else {
        println!("Noo");
    }
}
