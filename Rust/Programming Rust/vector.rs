fn main() {
    // using macros
    let v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    // using constructor
    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    // by iterator
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // capacity
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    // pop
    let mut v = vec!["carmen", "miranda"];
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);

    // collect args
    let ar: Vec<String> = std::env::args().skip(1).collect();
    print!("{:?}", ar);
}
