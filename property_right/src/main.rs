
use::std::rc::Rc;
fn main() {

    println!("main is empty. Please run 'cargo test'");
}

#[test]
fn test_move() {

    // Build a vector of the strings "101", "102", .. "105"
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector
    let fifth = v.pop().unwrap();

    // 2. Move a value out of the middle of the vector
    let second = v.swap_remove(1);

    // 3. Swap in another value for the one we're taking out
    let third = std::mem::replace(&mut v[2], "substitute".to_string());

    assert_eq!(fifth, "105");
    assert_eq!(second, "102");
    assert_eq!(third, "103");
    // v
    assert_eq!(v, vec!["101", "104", "substitute"]);

    struct Person { name: Option<String>, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person {name: Some("Bob".to_string()), birth: 1525 });
    composers.push(Person {name: Some("Arice".to_string()), birth: 1200 });

    //let first_name = composers[0].name; // ❌ 

    let name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(name, Some("Bob".to_string()));
    assert_eq!(composers[0].name, None);

    assert_eq!(composers[1].name.take(), Some("Arice".to_string()));
}


// コピー型: 移動の例外
// 機械語レベルの整数や浮動小数点、charとbool型
//  Copy型のタプルや固定帳の配列もCopy型になる
#[test]
fn test_copy_type() {

    // ユーザ定義型がCopy型にならないのは単なるデフォルトの動作
    // 構造体の全てのフィールドがCopy型なのであれば、構造型もCopy型にすることができる。
    #[derive(Copy, Clone)]
    struct Label { number: u32 }

    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };
    print(l);
    println!("My label number is: {}", l.number);
}

// Rc とArc(atomic reference count)
#[test]
fn test_rc() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shirataki"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}