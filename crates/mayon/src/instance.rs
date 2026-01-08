use std::sync::Arc;

use crate::backend::Backend;

pub struct Instance {
    backend: Arc<dyn Backend>,
}
