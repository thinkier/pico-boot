pub fn send_to_bootloader(port: &str) {
    let open = tokio_serial::new(port, 1200).open();

    if let Err(e) = open {
        if e.description != "A device which does not exist was specified." {
            eprintln!("{}: {}", port, e.description);
        } else {
            println!("{}: OK", port);
        }
    } else {
        println!("{}: OK", port);
    }
}
