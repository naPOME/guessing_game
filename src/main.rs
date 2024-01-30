extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
    
    println!("Please enter the number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line");

    println!("your guess is :{}",guess);   
    //generating random number using crate 
   
      
    //Shadowing the guess var to change  its datatype
    let guess:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue
    };
        
    match guess.cmp(&secret_number){
        Ordering::Less =>println!("too small"),
        Ordering::Greater =>println!("too big"),
        Ordering::Equal =>{
            println!("you win");
            break;}
    }

    }
}
