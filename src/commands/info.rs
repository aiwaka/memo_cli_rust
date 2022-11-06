use crate::APP_CONFIG;

pub(super) fn info_command(version: &bool, storage: &bool, port: &bool) {
    let mut version = *version;
    let mut storage = *storage;
    let mut port = *port;
    let mut flags = [&mut version, &mut storage, &mut port];
    if !flags.iter().any(|flag| **flag) {
        for flag in flags.iter_mut() {
            **flag = true;
        }
    }
    if version {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        println!("{} {}", name, version);
    }
    if storage {
        let storage_dir = APP_CONFIG.get().unwrap().storage_dir.clone();
        println!("storage directory: {}", storage_dir);
    }
    if port {
        let port = APP_CONFIG.get().unwrap().server_port;
        println!("server port: {}", port);
    }
}
