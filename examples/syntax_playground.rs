use tinyrenderer::utils::swap_vars;
use tinyrenderer::*;

fn main() {
    println!("Hello playground world",);
    let a = "a";
    let b = "b";

    if true {
        println!("we are in swap",);
        let (a, b) = (b, a);
        println!("in swap a{:?}, b{:?}", a, b);
    }

    println!(" out of swap a{:?}, b{:?}", a, b);

    let (a, b) = if false { (b, a) } else { (a, b) };

    println!(" alt swap a{:?}, b{:?}", a, b);
    let a = "a";
    let b = "b";
    swap_vars!(4 > 6, a, b);
    println!(" alt swap a{:?}, b{:?}", a, b);
}
