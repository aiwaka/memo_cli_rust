use crate::APP_CONFIG;

pub(super) fn info_command(version: &bool, storage: &bool, all: &bool) {
    if *version {
        println!("memos_cli 0.1.0");
    }
    if *storage {
        let storage_dir = APP_CONFIG.get().unwrap().storage_dir.clone();
        println!("storage directory: {}", storage_dir);
    }
}
