use crate::cli::args::SleepArgs;
use std::time::Duration;
use tokio::time::sleep;

/// print log
async fn print_log(count: u32, unit: u32, interval_report: u32) {
    let mut count = count * unit;
    loop {
        if count <= 0 {
            println!("sleep ... Done");
            break;
        }

        println!("sleep ... {} S", count);

        sleep(Duration::from_secs_f32(interval_report as f32)).await;

        count = count - interval_report;
    }
}

/// print log v2
pub async fn interval_print_v2(sleep_args: &SleepArgs) {
    let SleepArgs { time, minute, hour } = sleep_args;
    let mut unit = 1;
    let mut interval_report = 1;

    match time {
        Some(time) => {
            if *minute {
                interval_report = 15;
                unit = 60;
            } else if *hour {
                interval_report = 60;
                unit = 60 * 60;
                print_log(*time as u32, unit, interval_report).await;
            }
            print_log(*time as u32, unit, interval_report).await;
        }
        None => {}
    }
}
