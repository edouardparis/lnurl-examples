use crate::client::Client;
use bech32::ToBase32;
use image::{ImageOutputFormat, Luma};
use qrcode::QrCode;
use std::sync::atomic::AtomicUsize;

pub struct Faucet {
    pub limit: AtomicUsize,
    pub counter: AtomicUsize,
    pub url: String,
    pub client: Client,
    pub qrcode: Vec<u8>,
}

impl Faucet {
    pub fn new(url: &str, clt: Client, limit: AtomicUsize) -> Faucet {
        let encoded = bech32::encode("lnurl", url.as_bytes().to_base32()).unwrap();
        let code = QrCode::new(encoded.to_string()).unwrap();
        let mut image: Vec<u8> = Vec::new();
        let img = image::DynamicImage::ImageLuma8(code.render::<Luma<u8>>().build());
        img.write_to(&mut image, ImageOutputFormat::PNG).unwrap();
        Faucet {
            qrcode: image,
            limit: limit,
            counter: AtomicUsize::new(0),
            url: url.to_string(),
            client: clt,
        }
    }
}
