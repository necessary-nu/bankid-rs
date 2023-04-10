use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponse {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CancelPayload {
    // Used to collect the status of the order.
    pub order_ref: String,
}
