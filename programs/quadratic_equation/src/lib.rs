// Attempt at creating a quadratic equation for polynomial functions
// Program does not take complex numbers into consideration
// BUG: Declaring 'c' as something different than zero results in NaN for x1 and x2


#[derive(Debug, PartialEq)]
struct Res {
    x1: f32,
    x2: f32
}

impl Res {
    fn quadratic_pol(a: i32, b: i32, c: i32) -> Res {
        // only f32/f64 has sqrt()
        let a = a as f32;
        let b = b as f32;
        let c = c as f32;
    
        // Splitting parts of top and bottom fn of equation
        let top = b*b - (4 as f32 * a * c);
        let bottom = 2 as f32 * a;
    
        let mut x1 = -b + top.sqrt();
        let mut x2 = -b - top.sqrt();
        x1 = x1 / bottom;
        x2 = x2 / bottom;
        
        Res { x1, x2 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affirm_flaky_quadratic_function() {
        let res1: Res = Res::quadratic_pol(4, 2, 0);
        let res2: Res = Res::quadratic_pol(7, 21, 3);
        
        assert_eq!(Res{x1: 0.0, x2: -0.5}, res1);
        assert_ne!(Res{x1:-0.15039688373634, x2: 2.8496031162637 }, res2);
    }
}