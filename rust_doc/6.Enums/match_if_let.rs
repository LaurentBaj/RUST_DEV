fn main() {
    // If none of the conditions are met,
    // Trigger the _=> arm
    let some_value = Some(3);
    match some_value {
        Some(some_value) => { // This will trigger in this case
            println!("They matched!");
        }
        _=> {
            println!("Did not match!");
        }
    }

    // if-let: can sometimes shorten code for matching
    if let Some(3) = some_value {
        println!("They matched!");
    } else {
        println!("Did not match!");
    }
}