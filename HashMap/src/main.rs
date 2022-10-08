use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    let n1 = "Denis".to_string();
    let n2 = "Kate".to_string();
    let n3 = "John".to_string();

    // Ключ - значення
    // Імена = ключі, оцінки = значення
    // Передаєм завжди лінку. Правило володіння

    map.insert(&n1, 10);
    map.insert(&n2, 11);
    map.insert(&n3, 9);

    println!("{:?}", map);
    println!("{}", n1);
    println!("{}", n2);
    println!("{}", n3);

    //Доступ до значень через ключ
    println!("{}", map[&n2]);

    match  {
         => 
        _ => 
    }
}
