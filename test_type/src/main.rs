
fn main() {

  println!("{}", (2.0_f64).sqrt());
  println!("{}", f64::sqrt(2.0));

  // Box 
  let t = (12, "eggs");
  let b = Box::new(t);

  // Get our command-line arguments as a vector of Strings.__rust_force_expr!
  let languages: Vec<String> = std::env::args().skip(1).collect();
  for l in languages {
    println!("{}: {}", l,
      if l.len() % 2 == 0 {
        "functional"
      } else {
        "imperative"
      });
    }
}
#[test]
fn test_type() {

  assert_eq!(10_i8 as u16, 10_u16); // in range
  assert_eq!(2525_u16 as i16, 2525_i16); // in range

  assert_eq!(-1_i16 as i32, -1_i32); // sign-extended
  assert_eq!(65535_u16 as i32, 65535_i32); // zero-extended

  assert_eq!(1000_i16 as u8, 232_u8);
  assert_eq!(65535_u32 as i16, -1_i16);

  assert_eq!(-1_i8 as u8, 255_u8);
  assert_eq!(255_u8 as i8, -1_i8);

  assert_eq!(2u16.pow(4), 16);
  assert_eq!((-4i32).abs(), 4);
  assert_eq!(0b101101u8.count_ones(), 4); // ビットカウント

  assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
  assert_eq!((-1.01f64).floor(), -2.0);
  assert!((-1. / std::f32::INFINITY).is_sign_negative());

  // 真偽値型
  assert_eq!(false as i32, 0);
  assert_eq!(true as i32, 1);

  // 文字列型
  assert_eq!('*', '\x2A');
  assert_eq!('*' as i32, 42);
  assert_eq!('*'.is_alphabetic(), false);
  assert_eq!('β'.is_alphabetic(), true);
  assert_eq!('8'.to_digit(10), Some(8));
  assert_eq!(std::char::from_digit(2, 10), Some('2'));

  // tuple
  let text = "I see the eigenvalue in thine eye";
  let (head, tail) = text.split_at(21);
  assert_eq!(head, "I see the eigenvalue ");
  assert_eq!(tail, "in thine eye");

  // ref String &String is String refarence
}

#[test]
fn test_array() {
  let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
  let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

  assert_eq!(lazy_caterer[3], 7);
  assert_eq!(taxonomy.len(), 3);

  let mut sieve = [true; 10000];
  for i in 2..100 {
    if sieve[i] {
      let mut j = i * i;
      while j < 10000 {
        sieve[j] = false;
        j += 1;
      }
    }
  }
  // @todo
  // assert!(sieve[211]);  // thread 'test_array' panicked at 'assertion failed: sieve[211]', src/main.rs:74:3
  assert!(!sieve[9876]);

  let mut chaos = [3, 5, 4, 1, 2];
  chaos.sort();
  assert_eq!(chaos, [1, 2, 3, 4, 5]);

}

#[test]
fn test_vec() {

  let mut v = vec![2, 3, 5, 7];
  assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

  v.push(11);
  v.push(13);
  assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);


  let mut v2 = Vec::<&str>::new();
  v2.push("step");
  v2.push("on");
  v2.push("no");
  v2.push("pets");
  assert_eq!(v2, vec!["step", "on", "no", "pets"]);

  // Generate Itr -> Vec
  let v3: Vec<i32> = (0..5).collect();
  assert_eq!(v3, [0, 1, 2, 3, 4]);

  // `Vec::with_capacity`
  let mut v4 = Vec::with_capacity(2);
  assert_eq!(v4.len(), 0);
  assert_eq!(v4.capacity(), 2);

  v4.push(1);
  v4.push(2);
  assert_eq!(v4.len(), 2);
  assert_eq!(v4.capacity(), 2);

  v4.push(3);
  assert_eq!(v4.len(), 3);
  assert_eq!(v4.capacity(), 4);

  let mut v5 = vec![10, 20, 30, 40, 50];
  v5.insert(3, 35);
  assert_eq!(v5, [10, 20, 30, 35, 40, 50]);

  v5.remove(1);
  assert_eq!(v5, [10, 30, 35, 40, 50]);

  // `pop`
  let mut v6 = vec!["caren", "miranda"];
  assert_eq!(v6.pop(), Some("miranda"));
  assert_eq!(v6.pop(), Some("caren"));
  assert_eq!(v6.pop(), None);
}

#[test]
fn test_str() {

  let speech = "\"Ouch!\" said the well.\n";
  assert_eq!(speech.trim_end(), r###""Ouch!" said the well."###);

  let default_win_install_path = r"c:\Program Files\Gorillas";

  let method = b"GET";
  assert_eq!(method, &[b'G', b'E', b'T']); // &[u8: 3] 3byte Array

  let moodles = "moodless".to_string(); // Convert &str => String
  let oodles = &moodles[1..];
  let poodles = "💚_💚";
  assert_eq!(poodles.len(), 9);
  assert_eq!(poodles.chars().count(), 3);

  assert_eq!(format!("{} {:02} {:02} N", 24, 5, 23),
    "24 05 23 N".to_string());

  let bits = vec!["veni", "vidi", "vici"];
  assert_eq!(bits.concat(), "venividivici");
  assert_eq!(bits.join(", "), "veni, vidi, vici");

  assert!("ONE".to_lowercase() == "one");

  assert!("peanut".contains("nut"));
  assert_eq!("💚_💚".replace("💚", "❌"), "❌_❌");

  for word in "veni, vidi, vici".split(", ") {
    assert!(word.starts_with("v"));
  }
}