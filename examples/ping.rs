use std::time::Duration;

use fastping_rs::ping_v4;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let localhost = "127.0.0.1".parse().unwrap();
    let one_one_one_one = "1.1.1.1".parse().unwrap();

    let addrs = [localhost, one_one_one_one].into_iter();
    let rtt = Duration::from_secs(5);
    let size = 64;
    let pings = ping_v4(addrs, rtt, size).unwrap();

    println!("{:?}", pings)
}