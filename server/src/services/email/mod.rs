use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EMailService {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {

}