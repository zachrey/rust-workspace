use std::{time::Duration};
use tokio::time::sleep;

pub async fn interval_print(count: u8) {
    let mut count = count;
    loop {
        if count <= 0 {
            break;
        }

        println!("sleep ... {}", count);
        sleep(Duration::from_secs_f32(1.0)).await;

        count = count - 1;
    }
}

