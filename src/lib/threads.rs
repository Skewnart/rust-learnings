extern crate rayon;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, channel};
use rayon::prelude::*; 

pub fn using_threads() {
    // classic_spawn();
    // rayon_method();
    // using_par_iter();
    using_channel();
}

fn classic_spawn() {
    let thread_2 = thread::spawn(move || {
        for i in 1..=20 {
            println!("Thread 2 : {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=10 {
        println!("Thread 1 : {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    thread_2.join().unwrap();
}

fn rayon_method() {
    let (variable_1, variable_2) = rayon::join(rayon_fn_1, rayon_fn_2);
}

fn rayon_fn_1() {
    for i in 1..=20 {
        println!("Thread 1 : {}", i);
        thread::sleep(Duration::from_millis(50));
    }
}

fn rayon_fn_2() {
    for i in 1..=10 {
        println!("Thread 2 : {}", i);
        thread::sleep(Duration::from_millis(50));
    }
}

fn using_par_iter() {
    let range = (1..=100);
    range.into_par_iter().for_each(|x| println!("{}", x));
}

fn using_channel() {
    let (tx, rx) = channel();
    
    //Clone du transmetteur. On peuet très bien avoir plusieurs threads qui envoie et un seul qui reçoit, ou pas.
    let tx_clone = mpsc::Sender::clone(&tx);

    // Thread qui envoie. 
    thread::spawn(move|| { 
        
        let id = thread::current().id(); 
        println!("A - Je suis le thread qui envoie : {:?}.", id); 
        
        let valeur = String::from("Coucou");  
        
        tx_clone.send(valeur).unwrap(); 
    });
        
    // Thread qui reçoit, par ailleurs thread principal. 
    let id = thread::current().id(); 
    println!("B - Je suis le thread qui reçoit : {:?}.", id);
    
    let message = rx.recv().unwrap(); 
    println!("Message reçu par B envoyé par A : {}", message);
}
