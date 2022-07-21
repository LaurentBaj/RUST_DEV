pub mod chat_utils {
    pub mod funs { 
        use super::model::{Message, Person};
        use std::thread;
        use std::sync::{mpsc};
        use std::time::Duration;

        pub fn chat_input(input: String, p: &Person) -> String {
            let (tx, rx) = mpsc::channel();
            let author = p.name.clone();
            let msg = Message::new(input.as_str(), &p);

            tx.send(msg.content).unwrap();
            thread::sleep(Duration::from_secs(2));
            let res = rx.recv().unwrap();

            format!("{}: '{}'", author, res)
        }
    }

    pub mod model {
        #[derive(Debug, PartialEq)]
        pub struct Person {
            pub name: String
        }

        #[derive(Debug, PartialEq)]
        pub struct Message<'a> {
            pub content: String,
            pub author: &'a Person
        }

        impl Message<'_> {
            pub fn new<'a>(c: &str, a: &'a Person) -> Message<'a> {
                Message {
                    author: a,
                    content: String::from(c)
                }
            }
        }

        impl Person {
            pub fn new(name: &str) -> Person {
                Person { name: String::from(name) }
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::chat_utils::{ model };
    use model::{ Message, Person };

    #[test]
    fn test_person_creation(){
        let p = Person::new("Ben");

        assert_eq!(p, Person { name: String::from("Ben") });
    }

    #[test]
    fn test_message_creation(){
        let p: Person = Person::new("Ben");
        let m = Message::new("Hello world", &p);

        assert_eq!(
            m,
            Message {
                content: String::from("Hello world"),
                author: &p
            }  
        );
    }
}