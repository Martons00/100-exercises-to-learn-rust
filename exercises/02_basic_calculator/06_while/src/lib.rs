// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    let mut i = 0;
    let mut result = 0;
    while i<=n {
        if i == 0{
            result += 1;
        }else{
            result  = result * i;
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
