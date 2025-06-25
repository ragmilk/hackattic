use {
    itertools::Itertools,
    std::fs::File,
    std::io::{BufWriter, Write},
    std::sync::{Arc, Mutex},
    std::time::Instant,
};

pub fn password_generator() -> std::io::Result<()> {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let chars_arc = Arc::new(chars);
    let file = File::create("passwords.txt")?;
    let writer = Arc::new(Mutex::new(BufWriter::new(file)));
    let capacity = 1024 * 1024 * 100;

    let start_time = Instant::now();

    println!("Generating passwords of length 4 and 5");
    let mut buffer = String::with_capacity(capacity);
    for len in 4..=5 {
        println!("  Generating length: {}...", len);
        let product = (0..len).map(|_| chars_arc.iter()).multi_cartesian_product();

        for pass_chars in product {
            let password: String = pass_chars.into_iter().collect();
            buffer.push_str(&password);
            buffer.push('\n');
            if buffer.len() >= capacity {
                let mut writer_lock = writer.lock().unwrap();
                writer_lock.write_all(buffer.as_bytes())?;
                buffer.clear();
            }
        }
    }

    if !buffer.is_empty() {
        let mut writer_lock = writer.lock().unwrap();
        writer_lock.write_all(buffer.as_bytes())?;
        buffer.clear();
    }
    println!("Generating passwords of length: 6...");
    let len = 6;
    crossbeam::scope(|s| {
        for &start_char in chars_arc.iter() {
            let writer_clone = Arc::clone(&writer);
            let chars_clone = Arc::clone(&chars_arc);

            s.spawn(move |_| {
                let mut local_buffer = String::with_capacity(capacity);
                let suffix_len = len - 1;
                let product = (0..suffix_len)
                    .map(|_| chars_clone.iter())
                    .multi_cartesian_product();

                for pass_suffix_chars in product {
                    let mut password = String::with_capacity(len + 1);
                    password.push(start_char);
                    password.extend(pass_suffix_chars);
                    password.push('\n');

                    local_buffer.push_str(&password);

                    if local_buffer.len() > capacity {
                        writer_clone
                            .lock()
                            .unwrap()
                            .write_all(local_buffer.as_bytes())
                            .unwrap();
                        local_buffer.clear();
                    }
                }

                if !local_buffer.is_empty() {
                    writer_clone
                        .lock()
                        .unwrap()
                        .write_all(local_buffer.as_bytes())
                        .unwrap();
                }
            });
        }
    })
    .unwrap();

    writer.lock().unwrap().flush()?;

    println!("Total time: {:?}", start_time.elapsed());
    println!("Done. Passwords saved to 'passwords.txt'.");
    Ok(())
}
