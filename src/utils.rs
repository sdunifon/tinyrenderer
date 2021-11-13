use super::*;
use rand::random;

#[macro_export]
macro_rules! swap_vars {
    ($e:expr, $a:ident, $b:ident) => {
        let ($a, $b) = if $e { ($b, $a) } else { ($a, $b) };
    };
}

pub fn random_color() -> Px {
    Px {
        r: random(),
        g: random(),
        b: random(),
    }
}

pub fn gray() -> Px {
    let gray_value = random();

    Px {
        r: gray_value,
        g: gray_value,
        b: gray_value,
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn swap_vars() {
        let a = 1;
        let b = 2;

        swap_vars!(1 > 2, a, b);
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        let c = 1;
        let d = 2;

        swap_vars!(3 > 2, c, d);
        assert_eq!(c, 2);
        assert_eq!(d, 1);
    }
}
