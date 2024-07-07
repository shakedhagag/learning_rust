fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4, 5].iter().map(|x| x + 1).collect();
    println!("{:?}", a);
}
