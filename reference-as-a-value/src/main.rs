
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
fn test_ref_cmp() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rx, ry));
}

#[test]
fn test_ref_fn() {
    fn factrial(n: usize) -> usize {
        (1..n +1).fold(1, |a, b| a * b)
    }

    let r = &factrial(6);

    assert_eq!(r + &1009, 1729);
}

#[test]
fn test_ref_rtn() {
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s { s = r}
        }
        s
    }
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0);
}