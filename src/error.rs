pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub enum Error {
    BadInvoice,
    Http(reqwest::Error),
    Opennode(opennode::error::RequestError),
}
