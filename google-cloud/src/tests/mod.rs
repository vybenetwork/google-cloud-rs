#[cfg(feature = "datastore")]
mod datastore;
#[cfg(feature = "pubsub")]
mod pubsub;
#[cfg(feature = "storage")]
mod storage;
#[cfg(feature = "vision")]
mod vision;

use crate::authorize::ApplicationCredentials;
use std::{fs::File, io::Read};
fn load_creds() -> ApplicationCredentials {
    // let creds = std::env::var("GCP_TEST_CREDENTIALS").expect("env GCP_TEST_CREDENTIALS not set");
    // json::from_str::<ApplicationCredentials>(&creds)
    //     .expect("incorrect application credentials format")

    let gcp_creds_path =
        std::env::var("GOOGLE_APPLICATION_CREDENTIALS").expect("env GCP_TEST_CREDENTIALS not set");
    let file = File::open(gcp_creds_path.as_str());
    match file {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents).unwrap();
            json::from_str::<ApplicationCredentials>(&contents)
                .expect("incorrect application credentials format")
        }
        Err(e) => {
            panic!(
                "Failed to open the file at {} for GOOGLE_APPLICATION_CREDENTIALS. Error: {}",
                gcp_creds_path, e
            );
        }
    }
}
