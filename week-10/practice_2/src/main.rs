fn main() {
    let v = vec![10, 20, 30];
    let _v2 = v;

    display(_v2.clone());
    println!("In main {:?}", _v2);
}

fn display(v: Vec<i32>) {
    println!("Inside display {:?}", v);
}