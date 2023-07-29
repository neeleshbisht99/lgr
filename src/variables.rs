fn main()
{
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 1000000;
    let tup = ("asd", "123");
    let (name, count) = tup;

    let count = tup.1;
    my_fn();
}

fn my_fn(){
    println!("Another fuction!");
}