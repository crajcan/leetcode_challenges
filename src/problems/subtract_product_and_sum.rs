fn digits(n: i32) -> Vec<i32> {
    match n / 10 {
        0 => vec![n],
        _ => [digits(n / 10), vec![n % 10]].concat(),
    }
}

pub fn subtract_product_and_sum(n: i32) -> i32 {
    let product: i32 = digits(n).iter().product();
    let sum: i32 = digits(n).iter().sum();

    product - sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sub_tract_product_and_sum() {
        assert_eq!(subtract_product_and_sum(234), 15);
    }
}
