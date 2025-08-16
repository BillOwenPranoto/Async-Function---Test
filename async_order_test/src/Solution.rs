use crate::order::Order;
use rand::Rng;
use std::time::Duration;
use async_std::task::sleep;

pub struct Solution;

impl Solution {
    pub async fn generate_orders() -> Vec<Order> {

        //Variables declared
        let orders = Vec::new();
        let mut buy_orders: Vec<Order> = Vec::new();
        let mut sell_orders: Vec<Order> = Vec::new();

        // Will print the order 4 times for now, for the sake of simplicity.
        // With the delay timer between 0.5 to 2 seconds (500 ms to 2000 ms)
        for i in 1..=4 {
            let delay = rand::thread_rng().gen_range(500..=2000);
            sleep(Duration::from_millis(delay)).await;

            // Create a new order when the timer is up
            let order = Order {
                id: i,
                price: rand::thread_rng().gen_range(80..=120),
                is_buy: rand::thread_rng().gen_bool(0.5),
            };

            // Print the order created.
            println!(
                "Received: {} Order {{ id: {}, price: {} }}",
                if order.is_buy { "Buy" } else { "Sell" },
                order.id,
                order.price
            );

             // Try to match the new order
            if order.is_buy {
                // This is a buy order, look for matching sell orders
                if let Some(sell_index) = Self::find_matching_sell(&sell_orders, &order) {
                    let sell_order = sell_orders.remove(sell_index);
                    println!(
                        "Matched: Buy Order {} and Sell Order {} at price {}",
                        order.id, sell_order.id, order.price
                    );
                } else {
                    // No match found, add to buy orders queue
                    buy_orders.push(order.clone());
                }
            } else {
                // This is a sell order, look for matching buy orders
                if let Some(buy_index) = Self::find_matching_buy(&buy_orders, &order) {
                    let buy_order = buy_orders.remove(buy_index);
                    println!(
                        "Matched: Buy Order {} and Sell Order {} at price {}",
                        buy_order.id, order.id, buy_order.price
                    );
                } else {
                    // No match found, add to sell orders queue
                    sell_orders.push(order.clone());
                }
            }
        }

        orders
    }

    // Find a sell order that can match with the given buy order
    // Buy matches sell if buy_price >= sell_price
    fn find_matching_sell(sell_orders: &[Order], buy_order: &Order) -> Option<usize> {
        sell_orders
            .iter()
            .position(|sell| buy_order.price >= sell.price)
    }

    // Find a buy order that can match with the given sell order
    // Buy matches sell if buy_price >= sell_price
    fn find_matching_buy(buy_orders: &[Order], sell_order: &Order) -> Option<usize> {
        buy_orders
            .iter()
            .position(|buy| buy.price >= sell_order.price)
    }

    // Function call
    pub async fn solution(&self) {
        let _orders = Solution::generate_orders().await;
    }
}