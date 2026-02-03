extern crate spidev;
use spidev::Spidev;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::{fs::File, io::prelude::*};
#[tokio::main]
async fn main() {
    let mut spidev = Spidev::open("/dev/spidev0.0").await.unwrap();
    spidev.devfile.write_all(&[0xAA, 0x00, 0x01, 0x02, 0x04]).await.unwrap();
    let mut buf: [u8; 10] = [0; 10];
    let read = spidev.devfile.read(&mut buf).await.unwrap(); // read 10
    println!("Read: {}, Data: {:?}", read, buf);
}
