// When matching expressions we have to deal with two different types:

// Refutable: Patterns that have to match a specific value 
// Irrefutable: Patterns that match any value passed

// x can match any value so it is irrefutable
let x = 4;

// ERROR: value could be none (refutable)
let Some(x) = some_option_value; 


// We can fix line 9 like this 
// If the pattern doesn't match, it will skip the block
if let Some(x) = some_option_value {
    println!("{}", x);
}






