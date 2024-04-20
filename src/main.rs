mod dhash;

use std::{sync::mpsc, thread, time::Duration};

use image::{io::Reader as ImageReader, ImageError};

use dhash::Dhash;

fn main() {
    let _ = read_image();
}

fn do_some_threading() {
    let (sender, receiver) = mpsc::channel();

    let threads = 5;

    for _ in 0..threads {
        let sender_clone = sender.clone();
        thread::spawn(move || {
            sender_clone
                .send(thread::current().id())
                .expect("thread id");
            thread::sleep(Duration::from_secs(1));
        });
    }

    let mut thread_ids = Vec::new();
    let mut received_cnt = 0;
    for received in receiver {
        println!("pushing {:?}", received);
        thread_ids.push(received);

        received_cnt += 1;
        if received_cnt == threads {
            break;
        }
    }
}

fn read_image() -> Result<Dhash, ImageError> {
    let image = ImageReader::open("test/test.png")?.decode();

    let dhash = Dhash::new(image?);

    Ok(dhash)
}
