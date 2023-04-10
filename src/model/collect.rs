use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "camelCase")]
pub(crate) enum CollectValue {
    #[serde(rename = "pending")]
    Pending {
        /// The orderRef in question.
        #[serde(rename = "orderRef")]
        order_ref: String,
        /// Describes the status of the order.
        #[serde(rename = "hintCode")]
        hint_code: String,
    },
    #[serde(rename = "failed")]
    Failed {
        /// The orderRef in question.
        #[serde(rename = "orderRef")]
        order_ref: String,
        /// Describes the status of the order.
        #[serde(rename = "hintCode")]
        hint_code: String,
    },
    #[serde(rename = "complete")]
    Complete {
        /// The orderRef in question.
        #[serde(rename = "orderRef")]
        order_ref: String,
        /// Only present for complete orders.
        #[serde(rename = "completionData")]
        completion_data: CompletionData,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CollectError {
    error_code: String,
    details: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CollectResponse {
    Pending {
        /// The orderRef in question.
        order_ref: String,
        /// Describes the status of the order.
        hint_code: String,
    },
    Failed {
        /// The orderRef in question.
        order_ref: String,
        /// Describes the status of the order.
        hint_code: String,
    },
    Complete {
        /// The orderRef in question.
        order_ref: String,
        /// Only present for complete orders.
        completion_data: CompletionData,
    },
    Error {
        error_code: String,
        details: String,
    },
}

impl From<CollectValue> for CollectResponse {
    fn from(value: CollectValue) -> Self {
        match value {
            CollectValue::Pending {
                order_ref,
                hint_code,
            } => CollectResponse::Pending {
                order_ref,
                hint_code,
            },
            CollectValue::Failed {
                order_ref,
                hint_code,
            } => CollectResponse::Failed {
                order_ref,
                hint_code,
            },
            CollectValue::Complete {
                order_ref,
                completion_data,
            } => CollectResponse::Complete {
                order_ref,
                completion_data,
            },
        }
    }
}

impl From<CollectError> for CollectResponse {
    fn from(value: CollectError) -> Self {
        CollectResponse::Error {
            error_code: value.error_code,
            details: value.details,
        }
    }
}

// #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
// #[serde(rename_all = "lowercase")]
// pub enum Status {
//     Pending,
//     Failed,
//     Complete,
// }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectPayload {
    /// Used to collect the status of the order.
    pub order_ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompletionData {
    /// Information related to the user.
    pub user: User,
    /// Information related to the device.
    pub device: Device,
    /// Information related to the user’s certificate.
    pub cert: Cert,
    /// The signature. Base64-encoded
    pub signature: String,
    pub ocsp_response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The personal number
    pub personal_number: String,
    /// The given name and surname of the user
    pub name: String,
    /// The given name of the user.
    pub given_name: String,
    /// The surname of the user.
    pub surname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// The IP address of the user agent as the BankID server discovers it
    pub ip_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    /// Start of validity of the users BankID
    pub not_before: String,
    /// End of validity of the Users BankID
    pub not_after: String,
}
