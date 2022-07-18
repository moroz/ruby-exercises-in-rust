// 给定一个数组包含 Hash，请过滤和排序

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    let arr = vec![
        Person {
            name: "Peter",
            age: 30,
        },
        Person {
            name: "John",
            age: 15,
        },
        Person {
            name: "David",
            age: 45,
        },
        Person {
            name: "Steven",
            age: 22,
        },
        Person {
            name: "Vincent",
            age: 6,
        },
    ];

    let mut adults: Vec<_> = arr.iter().filter(|person| person.age > 18).collect();
    adults.sort_by_key(|person| person.age);
    println!("所有成年人，并由小到大: {:#?}", adults);
    // 答案应该是
    //[
    //  { "name" => "Steven", "age" => 22 },
    //  { "name" => "Peter", "age" => 30 },
    //  { "name" => "David", "age" => 45 }
    //]
}
