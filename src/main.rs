fn main() {
    let v1 = vec![7, 1, 5, 6, 3, 4];
    let v2 = max_profit(v1);
    println!("{}", v2);
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut minprice = i32::MAX;
    let mut maxprofit: i32 = 0;
    for i in 0..prices.len() {
         if prices[i] < minprice {
             minprice = prices[i];
         }else if (prices[i] - minprice) > maxprofit {
              maxprofit = prices[i] - minprice;
         }
    }
    return maxprofit;
}
