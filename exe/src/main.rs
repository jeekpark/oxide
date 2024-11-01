

async fn example_async() {
    println!("Hello from an async function!");
}

fn main() {
    futures::executor::block_on(example_async());
}