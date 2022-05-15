use std::collections::HashMap;

fn main() {
    // Declare map
    let mut teams = HashMap::new();
    teams.insert(String::from("Barcelona"), teams.len() + 1);
    teams.insert(String::from("Real Madrid"), teams.len() + 1);

    // Barcelona : 1
    // Real Madrid : 2
    for (key, value) in &teams {
        println!("{} : {}\n", key, value);
    }

    // Access specific value 
    // Value is returned as an Option, which needs to be unwraped
    println!("{:?}", teams.get(&String::from("Barcelona")).unwrap()); // 1
    // or
    let barc = String::from("Barcelona");
    println!("{}", teams.get(&barc).unwrap());

    // UPDATE - entry()
    // We can overwrite or insert a new value if key has no value
    // Overwrite
    teams.entry(String::from("Barcelona")).or_insert(5); // Barcelona - some(1) 
    // AM does not exist so it insert new entry
    teams.entry(String::from("Ac Milan")).or_insert(4); // insert AM - some(4)
    println!("\n{:?} - {:?}\n",
            teams.get(&String::from("Barcelona")),
            teams.get(&String::from("Ac Milan"))
    ); 


    // Word counter
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Split text into sub-strings + remove white space
    // 41-42: return val then increment count
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"wonderful": 1, "hello": 1, "world": 2}
}