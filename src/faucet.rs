use crate::client::Client;
use bech32::ToBase32;
use image::{ImageOutputFormat, Luma};
use qrcode::QrCode;
use lightning_invoice::*;

use std::sync::atomic::{AtomicUsize, Ordering};
use crate::error::Error;
use crate::client::create_withdrawal;

pub struct Faucet {
    pub remain_counter: AtomicUsize,
    pub amount_max_withdrawable: u64,
    pub amount_min_withdrawable: u64,
    pub lnurl: String,
    pub callback: String,
    pub client: Client,
    pub qrcode: Vec<u8>,
}

impl Faucet {
    pub fn new(url: &str, callback: &str, clt: Client, remain_counter: AtomicUsize) -> Faucet {
        let encoded = bech32::encode("lnurl", url.as_bytes().to_base32()).unwrap();
        let code = QrCode::new(encoded.to_string()).unwrap();
        let mut image: Vec<u8> = Vec::new();
        let img = image::DynamicImage::ImageLuma8(code.render::<Luma<u8>>().build());
        img.write_to(&mut image, ImageOutputFormat::PNG).unwrap();
        Faucet {
            amount_max_withdrawable: 1_000_000,
            amount_min_withdrawable: 10,
            qrcode: image,
            remain_counter: remain_counter,
            lnurl: url.to_string(),
            callback: callback.to_string(),
            client: clt,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.remain_counter.load(Ordering::Relaxed) == 0;
    }

    pub async fn pay_invoice(&self, invoice: String) -> Result<(), Error> {
        if let Ok(signed) = invoice.parse::<SignedRawInvoice>() {
            if let Ok(i) = Invoice::from_signed(signed) {
                if let Some(amount) = i.amount_pico_btc() {
                    if amount <  self.amount_min_withdrawable || amount >= self.amount_max_withdrawable {
                        return match create_withdrawal(&self.client, &invoice).await {
                            Ok(w) => {
                                info!("withdrawal: {}", w.id);
                                Ok(())
                            }
                            Err(e) => Err(e),
                        };
                    }
                }
            }
        }
        Err(Error::BadInvoice)
    }
}
