fn ads(a: &mut Vec<i32>) {
    a.push(1);
}
fn main() {
    let mut a = vec![1,2];
    ads(&mut a);
    println!("{:?}", a);
}
