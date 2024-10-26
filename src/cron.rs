

pub fn interval_to_cron(interval: u32) -> String {
    match interval {
        1 => "1 * * * * *".to_string(),
        5 => "5 * * * * *".to_string(),
        10 => "10 * * * * *".to_string(),
        30 => "30 * * * * *".to_string(),
        60 => "* 1 * * * *".to_string(),
        _ => "1 * * * * *".to_string(),
    }
}

pub fn cron_to_interval(cron_expression: &str) -> u32 {
    match cron_expression {
        "1 * * * * *" => 1,
        "5 * * * * *" => 5,
        "10 * * * * *" => 10,
        "30 * * * * *" => 30,
        "* 1 * * * *" => 60,
        _ => 1,
    }
}
