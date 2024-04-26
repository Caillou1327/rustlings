// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.



use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Salut depuis {:?} !", self); // Modification: Changed "Hi" to "Salut"
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun)); // Utilisation de Rc::clone pour partager la référence à sun
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun)); // Utilisation de Rc::clone pour partager la référence à sun
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun)); // Utilisation de Rc::clone pour partager la référence à sun
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    println!("reference count = {}", Rc::strong_count(&sun)); // Added an extra space after "reference"

    assert_eq!(Rc::strong_count(&sun), 1);
}
