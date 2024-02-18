use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Seek, SeekFrom};
use std::path::PathBuf;
use serde::Serialize;
use serde::Deserialize;
use chrono::{serde::ts_seconds, DateTime, Local, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Alert {
    pub time: u8,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub enabled: bool
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<30} [{}]", self.time, created_at)
    }
}

impl Alert {
    pub fn new(time: u8) -> Alert {
        let created_at: DateTime<Utc> = Utc::now();
        let enabled: bool = true;
        Alert { time, created_at , enabled}
    }
}
fn get_alerts(mut file: &File) -> std::io::Result<Vec<Alert>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let alert = match serde_json::from_reader(file) {
        Ok(alert) => alert,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(alert)
}

pub fn add_alert(config_file: PathBuf, alert: Alert) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_file)?;
    let mut alerts = get_alerts(&file)?;
    alerts.push(alert);
    serde_json::to_writer(file, &alerts)?;
    Ok(())
}

pub fn list_alerts(config_file: PathBuf) -> std::io::Result<()> {
    // Open the file.
    let file = OpenOptions::new().read(true).open(config_file)?;
    // Parse the file and collect the tasks.
    let alerts = get_alerts(&file)?;

    // Enumerate and display tasks, if any.
    if alerts.is_empty() {
        println!("Alerts config file is empty!");
    }
    else {
        println!("Alert Idx. | Alert interval |  Status  | Created at");
        println!("=================================================================");
        for (idx, alert) in alerts.iter().enumerate() {
            let time: u8 = alert.time;
            let created_at: DateTime<Utc> = alert.created_at;
            let status: &str = if alert.enabled {"enabled"} else {"disabled"};
            println!("{:^10} | {:^14} | {:^8} | {:^} |", idx, time, status, created_at)
        }
    }
    Ok(())
}

pub fn disable_alert(config_file: PathBuf, alert_id: usize) -> std::io::Result<()> {
    let mut file: File = OpenOptions::new().write(true).read(true).open(config_file)?;
    let mut alerts: Vec<Alert> = get_alerts(&file)?;

    if alerts.is_empty() {
        println!("Alerts config file is empty!");
    }
    else {
        if alert_id > alerts.len() {
            return Err(
                Error::new(
                    ErrorKind::InvalidInput, format!(
                        "Invalid alert index {} - max alerts index is {}", alert_id, alerts.len()
                    ))
            );
        }
        for (idx, alert) in alerts.iter_mut().enumerate() {
            if idx == alert_id && alert.enabled == false {
                return Err(
                    Error::new(
                        ErrorKind::AlreadyExists, format!(
                            "Alert with index {} already disabled!", alert_id
                        )
                    )
                );
            }
            else if idx == alert_id && alert.enabled == true {
                alert.enabled = false;
                println!("Alert {} disabled!", idx)
            }
        }
        file.seek(SeekFrom::Start(0))?;
        file.set_len(0)?;
        serde_json::to_writer(file, &alerts)?;
    }
    Ok(())
}