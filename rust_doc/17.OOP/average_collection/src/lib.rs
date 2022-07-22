pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let tos = self.list.pop(); // top of stack
        match tos {
            Some(n) => {
                self.update_average();
                Some(n)
            }
            None => None
        }
        
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let avg: i32 = self.list.iter().sum();
        self.average = avg as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut collection = AveragedCollection { 
            list: Vec::new(),
            average: 0.0
        };

        collection.add(5);
    }

    #[test]
    fn test_remove() {
        let mut collection = AveragedCollection { 
            list: Vec::new(),
            average: 0.0
        };

        collection.add(7);
        collection.add(8);
        collection.add(9);

        collection.remove();

        assert_eq!(collection.average(), 7.5);
    }

    #[test]
    fn test_average() {
        let mut collection = AveragedCollection { 
            list: Vec::new(),
            average: 0.0
        };

        collection.add(7);
        collection.add(8);
        collection.add(9);

        assert_eq!(collection.average(), 8.0);
    }
}