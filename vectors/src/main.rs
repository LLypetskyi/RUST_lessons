fn main() {
    //vectors

    let mut list = vec![1, 2, 3, 78, 98, 9];
    list.push(9);
    list.remove(0);

    println!("vector is => {:?}", &list);

    //get => option: Some & None

    match list.get(2) {
        Some(el) => {
            println!("Element with find index is {}", el);
        }
        None => {
            println!("Element doesn't exist!");
        }
    }

    //loops
    println!("\n even elements - ");
    for el in &list {
        if el % 2 == 0 {
            println!("{}", el);
        }
    }

    println!("\n elements that are completely divisible by 3");
    for el in list.iter() {
        if el % 3 == 0 {
            println!("{}", el);
        }
    }

    let mut new_list = vec![10, 20, 30, 45, 80, 90];
    println!("\n arithmetic mean of numbers => {}", find_avg(&new_list));

    fn find_avg(l: &Vec<i32>) -> f64 {
        let mut sum = 0;
        for el in l {
            sum += el;
        }
        let length = (l.len()) as i32;
        (sum / length) as f64
    }
}
