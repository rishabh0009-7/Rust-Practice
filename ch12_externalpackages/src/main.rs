// using external package Chrono lets you do data and time in rust 

use chrono::{Local, Utc};


fn main (){
    let local_time = Local::now ();
    let utc_time = Utc::now();
    println!("the local time is {}" , local_time);
    println!("the native  time is {}" , Utc_time);

}