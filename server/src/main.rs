use crate::ActionType::{Add, Divide, Multiply, Subtract};

enum ActionType {
    Add,
    Subtract,
    Multiply,
    Divide
}
fn calculator(mut num: i32, modifier:i32, action: ActionType) -> i32{
    match action{
        Add => {num+=modifier;}
        Subtract => {num-=modifier;}
        Multiply => {num*=modifier;}
        Divide => {num/=modifier}
    }
    num
}

// main
fn main() {
    let mut ans=0;
    ans=calculator(6,3,Add);
    println!("Divide:{}",ans);

    ans=calculator(6,3,Subtract);
    println!("Divide:{}",ans);

    ans=calculator(6,3,Multiply);
    println!("Divide:{}",ans);

    ans =calculator(6,3,Divide);
    println!("Divide:{}",ans)
}
