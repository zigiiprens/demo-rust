use std::io;

fn variables() {
    println!("\nvariables");

    const HELLO: &str = "Hello, world!";
    println!("HELLO is {}", HELLO);

    let a = 3; // immutable, type specified at runtime
    let mut b: u32 = 4; // mutable, type specified at scope
    let c = -5_i128; // mutable, type specified at scope
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);

    b = 5;
    println!("b is {}", b);

    {
        b = 7;
        println!("b is {}", b);
    }

    println!("b is {}", b);

    {
        let c = 8;
        println!("c is {}", c);
    }

    println!("c is {}", c);
}

fn data_types() {
    println!("\ndata_types");

    // scalar types
    //// integers
    ////// signed
    let _: i8 = 127;
    let _: i16 = 127;
    let _: i32 = 127;   // default
    let _: i64 = 127;
    let _: i128 = 127;

    ////// unsigned
    let _: u8 = 255;
    let _: u16 = 255;
    let _: u32 = 255;
    let _: u64 = 255;
    let _: u128 = 255;

    //// floating points
    let _: f32 = 3.1;
    let _: f64 = 3.14;   // default

    //// booleans
    let _: bool = true;
    let _: bool = false;

    //// characters
    let _: char = 'a';

    // compound types
    //// tuples
    let a: (i8, i8, i8) = (1, 2, 3);
    println!("tup is {:?}", a);
    let a: (i8, bool, char) = (1, true, 'a');
    println!("tup.0 is {}", a.0);

    //// arrays
    let a: [i32; 2] = [1, 2];
    println!("array is {:?}", a);
    println!("array[0] is {}", a[0]);

}

fn read_from_console() {
    println!("\nread_from_console");

    let mut input = String::new();
    println!("Please enter String");
    io::stdin().read_line(&mut input).expect("failed");  // &mut stands to reference to input
    println!("you wrote: {}", input)
}

fn arithmetic() {
    println!("\narithmetic");

    let x = 12_000_i64;
    let y = 10 as i32;
    let z = x / (y as i64);
    println!("{}", z);
}

fn string_to_integer() {
    println!("\nstring_to_integer");

    let mut input = String::new();
    println!("Please enter Integer");
    io::stdin().read_line(&mut input).expect("failed");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input);
}

fn condition() {
    println!("\ncondition");

    let cond_fruit: &str = "bread";
    let cond = 2 >= 2;
    println!("2 >= 2 {}", cond);
    let cond = (2.0 as f32) >= 2.0;
    println!("(2.0 as f32) >= 2.0 {}", cond);
    let cond = false && cond; // and condition
    println!("false && cond {}", cond);
    let cond = true || cond; // or condition
    println!("true || cond {}", cond);
    let cond = !cond; // not condition
    println!("!cond {}", cond);

    if cond_fruit == "fruit" {
        println!("I like it");
    } else if cond_fruit == "bread" {
        println!("Kind of");
    } else {
        println!("Can not afford it");
    }
}

fn test() -> &'static str {
    return "hello"
}

fn double<const N: i32>() {
    println!("\ndouble");

    println!("doubled: {}", N * 2);
}

fn trait_implementation() {
    println!("\ntrait_implementation tutorial");

    trait Zero {
        const ZERO: Self;
        fn is_zero(&self) -> bool;
    }

    impl Zero for i32 {
        const ZERO: Self = 0;

        fn is_zero(&self) -> bool {
            *self == Self::ZERO
        }
    }

    println!("{} == 0 is {}", i32::ZERO, i32::ZERO == 0);
    println!("{}.is_zero() is {}", i32::ZERO, i32::ZERO.is_zero());
    println!("{}.is_zero() is {}", !4, !4.is_zero());
}

fn while_loop() {
    let mut i = 0;

    while i < 10 {
        println!("{} hello", i);
        i = i + 1;
    }
}

fn for_loop() {
    let mut sum = 0;
    for n in 1..11 {
        if n > 7 {
            break;
        }
        sum += n;
    }
    println!("sum is {}", sum)
}

fn ten_times<T>(function: T) where T: Fn(i32) {
    for index in 0..10 {
        function(index);
    }
}

fn main() {
    // variables();
    // data_types();
    // read_from_console();
    // arithmetic();
    // string_to_integer();
    // condition();
    // println!("test function result {}", test())
    // double::<9>()
    // trait_implementation()
    // while_loop()
    for_loop();
    ten_times(|j| println!("hello, {}", j));
    // ten_times(|j: i32| -> () {
    //     println!("hello, {}", j)
    // });
    //
    // let word = "konnichiwa".to_owned();
    // ten_times(move |j| println!("{}, {}", word, j));
}
