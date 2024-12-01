// modulus function.
pub fn modulus(a: i32, b: i32) -> i32 {
    // % is actually the remainder function, not the modulus function
    // This is the workaround way to "fix" this.
    return ((a % b) + b) % b;
}

pub fn modulusi64(a: i64, b: i64) -> i64 {
    // % is actually the remainder function, not the modulus function
    // This is the workaround way to "fix" this.
    return ((a % b) + b) % b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulus_one_neg() {
        let result = modulus(-2, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn modulus_both_neg() {
        let result = modulus(-2, -3);
        assert_eq!(result, -2);
    }

    #[test]
    fn modulus_both_pos() {
        let result = modulusi64(120i64, 3i64);
        assert_eq!(result, 0);
    }
}
