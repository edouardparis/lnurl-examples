use crate::client::Client;
use bech32::ToBase32;
use image::{ImageOutputFormat, Luma};
use qrcode::QrCode;
use lightning_invoice::*;

use std::sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}};
use std::time::Duration;
use crate::error::Error;
use crate::client::create_withdrawal;

pub struct Faucet {
    pub lock_duration: usize,
    pub remained_time: AtomicUsize,
    pub locked: AtomicBool,
    pub amount_max_withdrawable: u64,
    pub amount_min_withdrawable: u64,
    pub lnurl: String,
    pub callback: String,
    pub client: Client,
    pub qrcode: Vec<u8>,
}

impl Faucet {
    pub fn new(url: &str, callback: &str, clt: Client, lock_duration: usize ) -> Faucet {
        let encoded = bech32::encode("lnurl", url.as_bytes().to_base32()).unwrap();
        let code = QrCode::new(encoded.to_string()).unwrap();
        let mut image: Vec<u8> = Vec::new();
        let img = image::DynamicImage::ImageLuma8(code.render::<Luma<u8>>().build());
        img.write_to(&mut image, ImageOutputFormat::PNG).unwrap();
        Faucet {
            amount_max_withdrawable: 1_000_000,
            amount_min_withdrawable: 10000,
            qrcode: image,
            lock_duration: lock_duration,
            remained_time: AtomicUsize::new(0),
            locked: AtomicBool::new(false),
            lnurl: url.to_string(),
            callback: callback.to_string(),
            client: clt,
        }
    }

    pub fn is_locked(&self) -> bool {
        return self.remained_time.load(Ordering::Relaxed) > 0;
    }

    pub fn lock(&self) {
        self.remained_time.fetch_add(self.lock_duration, Ordering::AcqRel);
    }

    pub async fn pay_invoice(&self, invoice: String) -> Result<(), Error> {
        if let Ok(signed) = invoice.parse::<SignedRawInvoice>() {
            if let Ok(i) = Invoice::from_signed(signed) {
                if let Some(amount) = i.amount_pico_btc() {
                    let converted_amount = amount/10;
                    let max_fee = self.amount_max_withdrawable/10;
                    if converted_amount >=  self.amount_min_withdrawable &&
                        converted_amount <= self.amount_max_withdrawable+max_fee{
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

pub async fn start(faucet: Arc<Faucet>) {
        loop {
           tokio::timer::delay_for(Duration::new(1,0)).await;
           if faucet.is_locked() {
                faucet.remained_time.fetch_sub(1, Ordering::AcqRel);
           }
        }
    }
