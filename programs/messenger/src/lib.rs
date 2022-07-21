use crate::chat_ops;


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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_creation(){
        let p: Person = Person::new("Ben");

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