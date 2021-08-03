
fn main() {

}

#[test]
fn test_main() {

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);

}

#[test]
fn test_ref_var() {
    let x = 10;
    let y = 20;
    let mut r = &x;

    if true { r = &y; }
    assert!(*r == 10 || *r == 20)
}

#[test]
fn test_ref_ref() {
    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    assert_eq!(rrr.y, 729);
}

#[test]
fn test_ref_comp() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
}