use std::collections::VecDeque;

use shared_data::CollectorCommandV1;

mod data_collector;
mod sender;
mod errors;

fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");

    if path.exists() {
        let content = std::fs::read_to_string(path).unwrap();
        content.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
    }
}

fn main() {
    let uuid = get_uuid();
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(uuid, tx);
    });


    let mut send_queue = VecDeque::with_capacity(120);

    // Listen for commands to send
    while let Ok(command) = rx.recv() {
        let encoded_command = shared_data::encode_v1(&command);
        send_queue.push_back(encoded_command);
        let result = sender::send_queue(&mut send_queue);

        if result.is_err() {
            println!("{result:?}");
        }
    }
}
