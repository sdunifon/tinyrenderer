// var_swap

//     let (a, b) = if false { (b, a) } else { (a, b) };

#[macro_export]
macro_rules! swap_vars {
    ($e:expr, $a:ident, $b:ident) => {
        let ($a, $b) = if $e { ($b, $a) } else { ($a, $b) };
    };
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(c, 1);
        assert_eq!(d, 2);
    }
}
