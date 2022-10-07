fn main() {
    //vectors

    let mut list = vec![1, 2, 3];
    list.push(9);
    list.remove(0);

    println!("vector is => {:?}", &list);
}
