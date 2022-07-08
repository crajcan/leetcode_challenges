pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
     
    if prices.len() < 2 {
        return profit;
    } 
    
    let mut min = prices[0];
    for end_day in 1..prices.len() {
        if prices[end_day] < min {
            min = prices[end_day];
        }
        
        let potential_profit = prices[end_day] - min;
        
        if potential_profit > profit {
            profit = potential_profit;
        }
    }
    
    profit 
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}