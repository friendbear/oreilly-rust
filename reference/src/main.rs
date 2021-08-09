use ::std::collections::HashMap;

// Not Copy type
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    // <== Move table
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

struct File {
    descriptor: i32
}
fn new_file(d: i32) -> File {
    File { descriptor: d }
}
fn clone_from(this: &mut File, rhs: &File) {
    //close(this.descriptor);
    //this.descriptor = nix::unistd::dup(rhs.descriptor);
}
fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    // show(table);

    //❌ assert_eq!(table["Gesuald"][0], "many madrigals");
    //     error[E0382]: borrow of moved value: `table`
    //   --> src/main.rs:29:16
    //    |
    // 16 |     let mut table = Table::new();
    //    |         --------- move occurs because `table` has type `HashMap<String, Vec<String>>`, which does not implement the `Copy` trait
    // ...
    // 27 |     show(table);
    //    |          ----- value moved here
    // 28 |
    // 29 |     assert_eq!(table["Cesualdo"][0], "many madrigals");
    //    |                ^^^^^ value borrowed here after move

    // change pass by reference
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}

#[test]
fn test_ref_share() {
    let mut wave: Vec<f32> = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = vec![0.0, -1.0];

    wave.extend(&head);
    wave.extend(&tail);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
}