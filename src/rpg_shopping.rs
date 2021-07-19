fn solve(prices: Vec<u32>, amount: u32, buy: Vec<(u32, u32)>) -> u32 {
    let mut amount = amount;
    for b in buy {
        let index = (b.0 - 1) as usize;
        let count = b.1;
        let price = prices[index] * count;
        if amount > price {
            amount -= price;
        }
    }
    amount
}

#[cfg(test)]
mod tests {
    #[test]
    fn test0() {
        let prices = vec![10, 100, 50];
        let amount = 300;
        let buy = vec![(1, 5), (2, 3), (3, 1), (2, 1), (1, 3)];
        let want = 70;
        let got = super::solve(prices, amount, buy);
    }
}
