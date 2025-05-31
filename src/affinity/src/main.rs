fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let thread_handles = core_ids.into_iter().map(|id| {
        std::thread::spawn(move || {
            let success = core_affinity::set_for_current(id);

            if success {
                println!("Hello from a thread on core {id:?}");
            } else {
                println!("Unable to set affinity to core {id:?}");
            }
        })
    }).collect::<Vec<_>>();

    thread_handles.into_iter().for_each(|handle| handle.join().unwrap());
}
