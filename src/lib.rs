pub mod google_drive;

pub trait CloudFolder {
    fn get_name(&self) -> String;
}

pub trait CloudDrive {}
