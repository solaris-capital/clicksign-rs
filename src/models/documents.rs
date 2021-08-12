use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines a document template inside the Clicksign
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentTemplate {
    /// Unique key within Clicksign
    pub key: String,
    /// Data to fill in the template placeholders
    pub data: HashMap<String, String>,
}

/// Defines the field "data" of a document event.
#[derive(Debug, Serialize, Deserialize)]
pub struct EventData {
    /// Information of the user who created the document
    pub user: HashMap<String, String>,
    /// Information about the Clicksign account in whinch the document was created
    pub account: HashMap<String, String>,
    // TODO: I'm not sure about the difference between this deadline_at and the Document.deadline_at
    deadline_at: String,
    // TODO: I'm not sure about the difference between this auto_close and the Document.auto_close
    auto_close: bool,
    // TODO: I'm not sure about the difference between this locale and the Document.locale
    locale: String,
}

/// This struct models a document event
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentEvent {
    /// Event name
    pub name: String,
    /// Event data
    pub data: EventData,
    /// Datetime the event occurred
    pub occurred_at: String
}

/// This struct defines a metainformation about the document
#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    /// Unique key within Clicksign
    pub key: Option<String>,
    /// Full path for the document within Clicksign
    pub path: String,
    /// Name of generated file
    pub filename: Option<String>,
    /// Datetime for the last update in the document
    pub updated_at: Option<String>,
    /// Document finalization datetime
    pub finished_at: Option<String>,
    /// Document deadline
    pub deadline_at: Option<String>,
    /// Status of the document
    pub status: Option<String>,
    /// Indicates whether the document will be automatically finalized when all the signers sign.
    pub auto_close: Option<bool>,
    /// Indicates the document's locale
    pub locale: Option<String>,
    /// Metinformation about the document
    pub metadata: Option<HashMap<String, String>>,
    /// Missing information in the clicksign documentation
    pub sequence_enabled: Option<bool>,
    /// Missing information in the clicksign documentation
    pub signable_group: Option<String>,
    /// Missing information in the clicksign documentation
    pub remind_interval: Option<String>,
    /// Document download information
    pub downloads: Option<HashMap<String, String>>,
    /// Document template data
    pub template: DocumentTemplate,
    /// List of signers in the document
    pub signers: Option<Vec<String>>,
    /// Lists of events that occurred in the document
    pub events: Option<Vec<DocumentEvent>>
}
