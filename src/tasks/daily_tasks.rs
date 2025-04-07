use chrono::{Duration, Local, Timelike};
use tokio::time::sleep;

#[allow(dead_code)]
pub async fn run_daily_task(hour: u32, minute: u32) {
    loop {
        let now = Local::now();
        let next_run = now.with_hour(hour).unwrap().with_minute(minute).unwrap();
        println!("next run: {:?}", next_run);

        let duration_until_next_run = if next_run > now {
            (next_run - now).to_std().unwrap()
        } else {
            (next_run + Duration::days(1) - now).to_std().unwrap()
        };

        println!("Next daily task scheduled at: {}", next_run);
        sleep(duration_until_next_run).await;

        println!("Executing the daily task at {}", Local::now());
        // Add your task logic here
    }
}
