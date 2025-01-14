use std::path::PathBuf;

use crate::CloudFolder;

#[derive(Debug)]
pub struct GoogleDriveCloudFolder {}

impl GoogleDriveCloudFolder {
    pub fn new() -> Self {
        Self {}
    }
}

impl CloudFolder for GoogleDriveCloudFolder {
    fn get_name(&self) -> String {
        String::new()
    }
}

#[derive(Debug)]
pub struct GoogleDriveAccessor {}

impl GoogleDriveAccessor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn login(login_credentials_filepath: PathBuf) -> Self {
        Self {}
    }

    pub fn get_folder_from_url(&mut self, url: &String) -> GoogleDriveCloudFolder {
        GoogleDriveCloudFolder::new()
    }
}
