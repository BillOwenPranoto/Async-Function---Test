use crate::order::Order;
use rand::Rng;
use std::time::Duration;
use async_std::task::sleep;

pub struct Solution;

impl Solution {
    pub async fn generate_orders() -> Vec<Order> {
        let mut orders = Vec::new();

        for i in 1..=4 {
            let delay = rand::thread_rng().gen_range(500..=2000);
            sleep(Duration::from_millis(delay)).await;

            let order = Order {
                id: i,
                price: rand::thread_rng().gen_range(80..=120),
                is_buy: rand::thread_rng().gen_bool(0.5),
            };

            println!(
                "Received: {} Order {{ id: {}, price: {} }}",
                if order.is_buy { "Buy" } else { "Sell" },
                order.id,
                order.price
            );

            orders.push(order);
        }

        orders
    }

    pub async fn solution(&self) {
        let _orders = Solution::generate_orders().await;
    }
}
