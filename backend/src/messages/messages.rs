use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessMessage {
    pub success: bool,
}

impl SuccessMessage {
    pub fn new(success: bool) -> Self {
        SuccessMessage { success }
    }
}
