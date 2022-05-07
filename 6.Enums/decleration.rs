#[derive(Debug)]
enum IpAddressKind {
    // V4, Can store any time of data
    V4(String), // Stores string
    V6(u8, u8, u8, u8) // Stores a touple of four u8
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String
}

fn main() {
    let ip1 = IpAddressKind::V4; // Declare enum type

    // V6(127.0.0.1)
    let ip2 = IpAddressKind::V6(127, 0, 0, 1);
    println!("{:?}", ip2);

    // Odd example
    let ip3 = IpAddress {
        kind: IpAddressKind::V4(String::from("V4")),
        address: String::from("127.0.0.1")
    };
    println!("{:?}", ip3);
}




/* Enums can also have functions
    imlp IpAddressKind {
        fn call(&self) {
            // some method body 
        }
    }   
*/
