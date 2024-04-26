// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // Modification 1: Utilisation de `if let Some(_) = my_option` pour vérifier si `my_option` contient une valeur.
    if my_option.is_some() {
        panic!("This should never happen!");
    }

    let my_arr = &[
        -1, -2, -3,  // Modification 2: Correction de la virgule manquante dans la déclaration de tableau.
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Modification 3: Utilisation de `Vec::new()` pour créer un vecteur vide.
    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // Modification 4: Utilisation de `std::mem::swap()` pour échanger les valeurs `value_a` et `value_b`.
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
