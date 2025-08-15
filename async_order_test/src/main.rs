mod order;
mod solution;

use solution::Solution;

#[async_std::main]
async fn main() {
    let program = Solution;
    program.solution().await;
}
