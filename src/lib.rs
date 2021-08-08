//! Unofficial Rust library for the [Clicksign API](https://developers.clicksign.com/)
//! =======
//! ---
//! **Disclaimer**
//!
//! All person names and document numbers in the examples of this documentation are
//! fake. Any resemblance to real people is purely coincidental.
//!
//! **Work in progress...**
//! This crate isn't production-ready yet. A lot of changes can bring to it before
//! becoming stable. (Your PR is welcome)
//!
//! ---
//! # Features
//! ### Documents
//! - [ ] Create document by upload
//! - [x] Create document by template
//! - [ ] View document
//! - [ ] View all documents
//! - [ ] Configure document
//! - [ ] Finish document
//! - [ ] Cancel document
//! - [ ] Duplicate document
//! - [ ] Delete document
//!
//! ### Signers
//! - [x] Create signer
//! - [ ] View Signer
//! - [x] Add signer to document
//! - [ ] Remove signer from document
//! - [ ] Delete signer
//!
//! ### Batches
//! - [ ] Create batches
//!
//! ### Notifications
//! - [x] Request signing by email
//! - [ ] Request signing by Whatsapp
//! - [ ] Request signing by SMS
//!
//! ### Sign documents via API
//! - [ ] Sign documents via API
//!
//! # Usage
//! ## Instantiating a client
//! ```rust
//! use clicksign::client::Client;
//!
//! let client = Client::new(
//!    "c9d91ece-9b3b-4def-abac-25b645cb083c",
//!    Some("https://api.example.com"),
//! );
//! assert_eq!("https://api.example.com", client.host);
//! assert_eq!("c9d91ece-9b3b-4def-abac-25b645cb083c", client.access_token);
//! ```
//! ## Creating a new document for sign
//! ```no_run
//! async {
//!  use clicksign::client::Client;
//!
//!  let client = Client::new(
//!     "some_access_token",
//!     Some("https://api.example.com/"),
//!  );
//!  let template_body = r#"
//!     {
//!       "document": {
//!         "path": "/Modelos/Teste-123.docx",
//!         "template": {
//!           "data": {
//!             "Company Name": "Clicksign Gestão de Documentos S.A.",
//!             "Address": "R. Teodoro Sampaio 2767, 10° andar",
//!             "Phone": "(11) 3145-2570",
//1             "Website": "https://www.clicksign.com"
//!           }
//!         }
//!       }
//!     }
//!  "#;
//!
//!  let document = client.create_document_by_model("template_id", template_body)
//!      .await
//!      .unwrap();
//! };
//! ```
//! # License
//!
//! The MIT License (MIT)
//!
//! Copyright (c) 2021 Solaris Capital <bptech at solcap dot com dot br>
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//! THE SOFTWARE.

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

extern crate error_chain;

/// Implementing a clicksign API client
pub mod client;
/// Request/Response models for clicksign API
pub mod models;
