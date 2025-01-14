use cucumber::{given, then, when, World};
use std::path::PathBuf;

use drive2git::google_drive::access::*;
use drive2git::CloudFolder;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct CloudDriveTestApp {
    given_cloud_folder_url: String,
    cloud_folder: GoogleDriveCloudFolder,
    cloud_drive: GoogleDriveAccessor,
}

impl CloudDriveTestApp {
    pub fn new() -> Self {
        Self {
            given_cloud_folder_url: String::new(),
            cloud_folder: GoogleDriveCloudFolder::new(),
            cloud_drive: GoogleDriveAccessor::new(),
        }
    }
}

#[given(regex = r"a cloud drive logged in using the credentials found in (.+),")]
fn given_logged_in_cloud_drive(
    cloud_drive_test_app: &mut CloudDriveTestApp,
    login_credentials_file: PathBuf,
) {
    cloud_drive_test_app.cloud_drive = GoogleDriveAccessor::login(login_credentials_file);
}

#[given(regex = r"a cloud drive folder found at the URL (.+),")]
fn given_cloud_folder(cloud_drive_test_app: &mut CloudDriveTestApp, cloud_folder_url: String) {
    cloud_drive_test_app.given_cloud_folder_url = cloud_folder_url;
}

#[when("the folder URL is converted into a cloud folder,")]
fn when_folder_url_converted_into_cloud_folder(cloud_drive_test_app: &mut CloudDriveTestApp) {
    cloud_drive_test_app.cloud_folder = cloud_drive_test_app
        .cloud_drive
        .get_folder_from_url(&cloud_drive_test_app.given_cloud_folder_url);
}

#[then(regex = r"the cloud folder should be called '(.+)'.")]
fn verify_cloud_folder_name(
    cloud_drive_test_app: &mut CloudDriveTestApp,
    expected_folder_name: String,
) {
    let actual_folder_name = cloud_drive_test_app.cloud_folder.get_name();
    assert_eq!(expected_folder_name, actual_folder_name);
}

fn main() {
    futures::executor::block_on(CloudDriveTestApp::run(
        "tests/feature-files/cloud_drive.feature",
    ));
}
