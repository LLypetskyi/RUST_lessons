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

    //Доступ до значень через лінку на ключ
    println!("{}", map[&n2]);

    //Перевірка чи існує запис через лінку на ключ
    match map.get(&n1) {
        Some(mark) => {
            println!("Mark is => {}", mark)
        }
        None => {
            println!("Element doesent exist!")
        }
    }

    //Перебір колекції циклом

    for (name, mark) in &map {
        println!("The student {} has mark - {}", name, mark);
    }

    // Зміна HashMap.
    //ПЕревіряємо чи є запис "Влад", якщо є - нам поверне цей запис, якщо немає - ми створимо запис по ключу "Влад" із значенням 9
    let n4 = "Vlad".to_string();
    map.entry(&n4).or_insert(9);
    println!("{:?}", map);
}
