#![allow(unused)]

// use core::panicking::panic;
use std::collections::btree_map::Keys;

use std::{char, error, io};
// use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
mod restaurant;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
// use crate::restaurant::order_food;


fn say_hello() {
    println!("hello");

}
fn get_sum(x:i32,y:i32) {
    println!("{}+{}={}",x,y,x+y);

    
}
fn get_sum2(x:i32,y:i32)->i32{
    // x+y
    return x+y;

    
}
fn get2(x:i32)->(i32,i32){
    // x+y
    return (x+1,x+2);


    
}
fn sum_list(list:&[i32])->i32 {
    
    let mut sum=0;
    for &val in list.iter()  {
        sum+=val;

        
    }
    sum


    
}
fn get_sum_gen<T:Add<Output = T>>(x:T,y:T)-> T{
    return  x+y;


}
fn print_str(x:String){
    println!("{}",x);

}
fn print_return_str(x:String)->String {
    println!("{}",x);
    x
}
fn change_str(name:&mut String) {
    name.push_str("is happy");
    println!("{}",name);

}
pub struct Bank {
    balance: f32
}
fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt:f32){
    let mut bank_ref = the_bank.lock().unwrap();

    if bank_ref.balance < 5.00{
        println!("Current Balance : {} Withdrawal a smaller amount",
        bank_ref.balance);
    } else {
        bank_ref.balance -= amt;
        println!("Customer withdrew {} Current Balance {}",
        amt, bank_ref.balance);
    }
}
fn customer(the_bank: Arc<Mutex<Bank>>) {
    withdraw(&the_bank, 5.00);
}

fn main() {
    let bank: Arc<Mutex<Bank>> =
      Arc::new(Mutex::new(Bank { balance: 30.00 }));
      let handles = (0..10).map(|_| {

        
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

  println!("Total: {}", bank.lock().unwrap().balance);



    let thread1=thread::spawn(||{
        for i in 1..25{
            println!("Spawned thread : {}",i);
            thread::sleep(Duration::from_millis(1));

        }
    });
    for i in 1..20 {
        println!("main thread {}",i);
        thread::sleep(Duration::from_millis(1));
         

        
    }

    thread1.join().unwrap();




    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1)
    .left(TreeNode::new(2))
    .right(TreeNode::new(3));


    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote : {}", can_vote(8));
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));


    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter() {
        println!("{}", val);
    }

    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());


    let path="line.txt";
    let output=File::create(path);
    let mut output=match output {
        Ok(file)=>file,
        Err(error)=>{
        panic!("Problem creating file {:?}",error);
        }

    };
    write!(output,"Just some\nRandom words").expect("Failed to write to file");
    let input=File::open(path).unwrap();
    let buffered=BufReader::new(input);
    for line in buffered.lines(){
        println!("{}",line.unwrap());

    } 
    let output2=File::create("rand.txt");
    let output2=match output2 {
        Ok(file)=>file,
        Err(error)=>match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
            
        }
        
    };

    // order_food();

    struct Rectangle<T,U>{
        length:T,
        height:U,
    }
    let rec=Rectangle{length:4,height:10.5};
    trait Shape {
        fn new(length:f32,width:f32)->Self;
        fn area(&self)->f32;

    }
    struct Rectangle2{length:f32,width:f32};

    impl Shape for Rectangle2 {
        fn new(length:f32,width:f32)->Rectangle2 {
            return Rectangle2{length,width};
        }
        fn area(&self)->f32 {
            return self.length*self.width;
        }
    }

    let rec:Rectangle2=Shape::new(10.0, 10.0);
    println!("Area of rectangle {}",rec.area());


    struct Customer{
        name:String,
        address:String,
        balance:f32,
    }
    let mut shivam=Customer{
        name:String::from("Shivam Datta"),
        address:String::from("kolkata"),
        balance:234.50
    };
    shivam.address=String::from("new york");


    let mut heroes=HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Spiderman", "Peter Parker");

    for (k,v) in heroes.iter() {
        println!("{} = {}",k,v);

    }
    if heroes.contains_key(&"Spiderman") {
        let the_spiderman=heroes.get(&"Spiderman");
        match the_spiderman {
            Some(x)=>println!("Spiderman is hero"),
            None=>println!("Spiderman is not a hero"),

            
        }
        
    }
    let str1=String::from("World");
    // let str2=str1;
    let str2=str1.clone();
    println!("{}",str1);


    

    println!("5+4= {}",get_sum_gen(5, 4));
    println!("5+4= {}",get_sum_gen(5.2, 4.6));
    

    let num_list=vec![1,2,3,4];
    println!("{}",sum_list(&num_list));


    let (val1,val2)=get2(5);

    println!("{} {}",val1,val2);

    say_hello();
    get_sum(5, 4);
    println!("{}",get_sum2(5, 4));


    println!("what is your name?");
    let mut name=String::new();
    let greeting="good morning";
    io::stdin().read_line(&mut name).expect("Didnot receive input");

    println!("Hello, {} {}",name.trim_end(),greeting);
    const ONE_HUNDRED:u32=100;
    const PI:f32=3.14;
    let age="47";
    let mut age:u32=age.trim().parse().expect("age was not assigned a number");
    age=age+1;
    println!("I am {} and i want ${}",age,ONE_HUNDRED);
    println!("Max u32 {}",u32::MAX);
    let is_true=true;
    let my_grade='A';
    let num_1:f32=1.111111;
    println!("f32:{}",num_1+0.11111);
    let mut num_3:u32=5;
    let num_2:u32=4;
    println!("5+4={}",num_2+num_3);
    num_3+=1;
    let age=8;
    if age>=1 &&age<=18 {
        println!("imp");
    }
    else if age==21||age==50 {
        println!("very imp")
    }
    else if age>=65 {
        println!("vvip");
        
    }
    else {
        println!("not imp");

    }
    let mut my_age=47;
    let can_vote=if my_age>=18{
        true
    }
    else{
        false
    };
    println!("Can vote: {}",can_vote);
    let age2=8;
    match age2{
        1..=18=>println!("Important Birthday"),
        21|50=>println!("Important Birthday"),
        65..=i32::MAX=>println!("Important birthday"),
        _=>println!("not imp"),
    };
    let my_age=18;
    let voting_age=18;
    match my_age.cmp(&voting_age){
        Ordering::Less=>println!("cant vote"),
        Ordering::Greater=>println!("can vote"),
        Ordering::Equal=>println!("you now reached voting age"),

    };
    let arr_2=[1,2,3,4,5,6,7,8,9];
    println!("1st:{}",arr_2[0]);
    println!("Length{}",arr_2.len());
    let mut i=0;
    loop {
        if arr_2[i]%2==0{
            i+=1;
            continue;

        }
        if arr_2[i]==9{
            break;;
        }
        println!("Val {}",arr_2[i]);
        i+=1;

    }
    let mut j=0;
    while j<arr_2.len() {
        println!("{}",arr_2[j]);
        j+=1;

        
    }
    let mut k=0;
    for k in arr_2.iter()  {
        println!("Val {}",k);

        
    }
    let my_tuple:(u8,String)=(25,"Shivam".to_string());
    println!("Name:{}",my_tuple.1);
    let (v1,v2)=my_tuple;
    println!("Age {}",v1);
    

    let mut  st1=String::new();
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace() {
        println!("{}",word);
        
    }
    let st2=st1.replace("A", "Another");
    println!("{}",st2);

    let st3=String::from("s h i v a m");
    let mut v1:Vec<char>=st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1{
        println!("{}",char);

    }
    let st4="Random String";
    let mut st5=st4.to_string();
    println!("{}",st5);
    let st6=&st5[0..6];
    println!("{}",st6.len());
    st5.clear();
    let st6=String::from("just some");
    let st7=String::from("words");
    let st8=st6+ &st7;
    for char in st8.bytes(){
        println!("{}",char);
    }
    let int_u8=5;
    let int2_u8=4;
    let int3_u32=(int_u8 as u32) + (int2_u8 as u32);
    
    
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    
    let today:Day = Day::Monday;

    
    match today {
        Day::Monday => println!("Monday"),
        Day::Tuesday => println!("Tuesday"),
        Day::Wednesday => println!("Wednesday"),
        Day::Thursday => println!("Thursday"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend!!!"),
        Day::Sunday => println!("Weekend!!!"),
    }

    println!("Is today the weekend {}", today.is_weekend());
    let vec1:Vec<i32>=Vec::new();
    let mut vec2=vec![1,2,3,4];
    vec2.push(5);
    println!("1st {}",vec2[0]);
    let second=&vec2[1];
    match vec2.get(1) {
        Some(second)=>println!("2nd :{}",second),
        None=>println!("No 2nd value"),

    }
    for i in &mut vec2{
        *i*=2;

    }
    for i in &mut vec2{
        println!("{}",i);

        
    }
    println!("{}",vec2.len());
    println!("Pop :{:?}",vec2.pop());

}
