use std::{sync::mpsc, thread, time::Duration};

use image::{io::Reader as ImageReader, DynamicImage, ImageError};

fn main() {
    let _ = read_image();
    do_some_threading();
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

fn read_image() -> Result<DynamicImage, ImageError> {
    let img = ImageReader::open("test/test.png")?.decode();

    img
}