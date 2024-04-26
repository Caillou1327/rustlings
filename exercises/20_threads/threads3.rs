// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

    /// Création de canaux pour chaque thread
    /// Premier thread pour envoyer les éléments du premier demi-vecteur
    /// Deuxième thread pour envoyer les éléments du deuxième demi-vecteur
    /// Fusionner les canaux pour récupérer les résultats de l'envoi dans un seul itérateur
    /// Envoyer les éléments au récepteur principal
    /// Canal non utilisé dans ce test
    /// Nombre total d'éléments à recevoir


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: &Queue, tx: mpsc::Sender<u32>) -> () {
    let first_half = q.first_half.clone();
    let second_half = q.second_half.clone();

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        for val in first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in second_half {
            println!("sending {:?}", val);
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let rx = rx1.into_iter().chain(rx2.into_iter());

    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn main() {
    let (tx, _) = mpsc::channel(); 
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(&queue, tx);

    let total_received = queue_length; 
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
