use crossbeam_channel::{Receiver, Sender};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub fn start_password_reader(
    file_path: PathBuf,
    send_password: Sender<String>,
    secret: Arc<Mutex<String>>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name("password-reader".to_string())
        .spawn(move || {
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if !secret.lock().unwrap().is_empty() {
                    break;
                }

                match send_password.send(line.unwrap()) {
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        })
        .unwrap()
}

pub fn password_checker(
    index: usize,
    file_path: PathBuf,
    receive_password: Receiver<String>,
    secret: Arc<Mutex<String>>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name(format!("worker-{}", index))
        .spawn(move || {
            let file = std::fs::File::open(file_path).expect("File should exist");
            let mut archive = zip::ZipArchive::new(file).expect("Archive validated before-hand");

            loop {
                if !secret.lock().unwrap().is_empty() {
                    break;
                }

                match receive_password.recv() {
                    Err(_) => break,
                    Ok(password) => {
                        let res = archive.by_index_decrypt(0, password.as_bytes());
                        match res {
                            Ok(mut zip) => {
                                let mut buffer = Vec::new();
                                if zip.read_to_end(&mut buffer).is_ok() {
                                    let mut secret_guard = secret.lock().unwrap();
                                    if secret_guard.is_empty() {
                                        *secret_guard = String::from_utf8(buffer).unwrap();
                                        println!(
                                            "Worker-{} found the password: {}",
                                            index, password
                                        );
                                    }
                                    break;
                                }
                            }
                            Err(_) => (),
                        }
                    }
                }
            }
        })
        .unwrap()
}

pub fn password_finder(
    zip_path: &str,
    password_list_path: &str,
    workers: usize,
    secret: Arc<Mutex<String>>,
) {
    let zip_file_path = Path::new(zip_path).to_path_buf();
    let password_list_file_path = Path::new(password_list_path).to_path_buf();

    let (send_password, receive_password) = crossbeam_channel::bounded(workers * 10_000);

    let password_gen_handle = start_password_reader(
        password_list_file_path,
        send_password.clone(),
        secret.clone(),
    );

    let mut worker_handles = Vec::with_capacity(workers);
    for i in 1..=workers {
        let join_handle = password_checker(
            i,
            zip_file_path.clone(),
            receive_password.clone(),
            secret.clone(),
        );
        worker_handles.push(join_handle);
    }

    loop {
        if !secret.lock().unwrap().is_empty() {
            println!("\nPassword found! Shutting down all threads...");
            drop(send_password);
            break;
        }

        if password_gen_handle.is_finished() && receive_password.is_empty() {
            println!("\nPassword not found in the list.");
            drop(send_password);
            break;
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }

    for h in worker_handles {
        h.join().unwrap();
    }
    password_gen_handle.join().unwrap();
}
