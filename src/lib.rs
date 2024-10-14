use esp_idf_svc::{
    sntp::{EspSntp, SyncStatus},
    sys::EspError,
};
use log::debug;
use std::{thread, time::Duration};

const RETRY: Duration = Duration::from_secs(1);

/// SNTP initialize
pub fn initialize<'a>() -> Result<EspSntp<'a>, EspError> {
    let sntp = EspSntp::new_default()?;
    while SyncStatus::Completed != sntp.get_sync_status() {
        debug!("SNTP is not completed, wait {}", RETRY.as_secs());
        thread::sleep(RETRY);
    }
    Ok(sntp)
}
