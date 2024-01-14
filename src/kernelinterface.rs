use std::fs::OpenOptions;
use std::io::{Read, Write};

/// Reads data from the specified interface and returns it as a String.
pub fn read(interface: &String) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .create(false)
        .open(interface)
        .unwrap_or_else(|_| {
            panic!(
                "failed to open kernel interface ({}) for reading",
                &interface
            )
        });

    let mut data = String::new();
    file.read_to_string(&mut data)
        .unwrap_or_else(|_| panic!("failed to read data from kernel interface ({})", &interface));

    println!(
        "Read data ({:?}) from kernel interface ({})",
        &data, &interface
    );

    data
}

/// Writes data in the form of a String to the specified interface.
pub fn write(interface: &String, data: &String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .open(interface)
        .unwrap_or_else(|_| {
            panic!(
                "failed to open kernel interface ({}) for writing",
                &interface
            )
        });

    file.write_all(data.trim().as_bytes()).unwrap_or_else(|_| {
        panic!(
            "failed to write data ({:?}) to kernel interface ({})",
            &data, &interface
        )
    });

    println!(
        "Wrote data ({:?}) to kernel interface ({})",
        &data, &interface
    );
}
