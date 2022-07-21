use messenger::chat_utils::{ funs, model} ;
use model::Person;

#[test]
fn test_chat_input() {
    println!("Enter some input");

    let person = Person::new("Laurent");
    let content = String::from("Look at him go!");

    assert_eq!(
        funs::chat_input(content, &person), 
        format!("{}: '{}'", 
            &person.name, 
            String::from("Look at him go!"))
    );
}


