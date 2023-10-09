use std::fs::OpenOptions;
use std::io::{Read, Write};

/// Reads data from the specified interface and returns it as a String.
pub fn read(interface: &String) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .create(false)
        .open(&interface)
        .expect(format!(
            "failed to open kernel interface ({}) for reading",
            &interface
        ).as_str());

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect(format!(
            "failed to read data from kernel interface ({})",
            &interface
        ).as_str());
    
    println!(
        "Read data ({:?}) from kernel interface ({})",
        &data,
        &interface
    );

    data
}

/// Writes data in the form of a String to the specified interface.
pub fn write(interface: &String, data: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(false)
        .open(&interface)
        .expect(format!(
            "failed to open kernel interface ({}) for writing",
            &interface
        ).as_str());

    file.write_all(data.trim().as_bytes())
        .expect(format!(
            "failed to write data ({:?}) to kernel interface ({})",
            &data,
            &interface
        ).as_str());

    println!(
        "Wrote data ({:?}) to kernel interface ({})",
        &data,
        &interface
    );
}
