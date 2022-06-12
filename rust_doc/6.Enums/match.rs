enum Vare {
    brus,
    snus,
    gitar(Gitar)
}

#[derive(Debug)]
enum Gitar {
    Fender,
    Gibson, 
    Strandberg
}

fn main() {
    // Du kjøpte en Fender-gitar!
    verdi(Vare::gitar(Gitar::Fender));
}

fn verdi(vare: Vare) -> u32 {
    match vare {
        Vare::brus => 26,
        Vare::snus => 75,
        Vare::gitar(gr) => {
            println!("Du kjøpte en {:?}-gitar!", gr);
            20000
        }
    }
}