pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub enum Error {
    BadInvoice,
    Http(reqwest::Error),
    Opennode(opennode::error::RequestError),
}
