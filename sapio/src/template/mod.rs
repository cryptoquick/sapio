// Copyright Judica, Inc 2021
//
// This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! utilities for building Bitcoin transaction templates up programmatically
use bitcoin::hashes::sha256;
use bitcoin::util::amount::Amount;
use sapio_base::Clause;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod output;
pub use output::{Output, OutputMeta};

pub mod builder;
pub use builder::Builder;

/// Metadata Struct which has some standard defined fields
/// and can be extended via a hashmap
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct TemplateMetadata {
    /// A Label for this transaction
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub label: Option<String>,
    /// catch all map for future metadata....
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
    /// A Color to render this node.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub color: Option<String>,
}

impl TemplateMetadata {
    /// helps determine if a TemplateMetadata has anything worth serializing or not
    pub fn skip_serializing(&self) -> bool {
        *self == TemplateMetadata::new()
    }
    /// create a new `TemplateMetadata`
    pub fn new() -> Self {
        TemplateMetadata {
            color: None,
            label: None,
            extra: HashMap::new(),
        }
    }
}

/// Template holds the data needed to construct a Transaction for CTV Purposes, along with relevant
/// metadata
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Template {
    /// additional restrictions placed on this template
    #[serde(rename = "additional_preconditions")]
    pub guards: Vec<Clause>,
    /// the precomputed template hash for this Template
    #[serde(rename = "precomputed_template_hash")]
    pub ctv: sha256::Hash,
    /// the index used for the template hash. (TODO: currently always 0, although
    /// future version may support other indexes)
    #[serde(rename = "precomputed_template_hash_idx")]
    pub ctv_index: u32,
    /// the amount being sent to this Template (TODO: currently computed via tx.total_amount())
    #[serde(
        rename = "max_amount_sats",
        with = "bitcoin::util::amount::serde::as_sat"
    )]
    #[schemars(with = "i64")]
    pub max: Amount,
    /// the amount being sent to this Template (TODO: currently computed via tx.total_amount())
    #[serde(
        rename = "min_feerate_sats_vbyte",
        with = "bitcoin::util::amount::serde::as_sat::opt"
    )]
    #[schemars(with = "Option<i64>")]
    pub min_feerate_sats_vbyte: Option<Amount>,
    /// any metadata fields attached to this template
    #[serde(
        skip_serializing_if = "TemplateMetadata::skip_serializing",
        default = "TemplateMetadata::new"
    )]
    pub metadata_map_s2s: TemplateMetadata,
    /// The actual transaction this template will create
    #[serde(rename = "transaction_literal")]
    pub tx: bitcoin::Transaction,
    /// sapio specific information about all the outputs in the `tx`.
    #[serde(rename = "outputs_info")]
    pub outputs: Vec<Output>,
}

impl Template {
    /// Get the cached template hash of this Template
    pub fn hash(&self) -> sha256::Hash {
        self.ctv
    }

    /// recompute the total amount spent in this template. This is the total
    /// amount required to be sent to this template for this transaction to
    /// succeed.
    pub fn total_amount(&self) -> Amount {
        self.outputs
            .iter()
            .map(|o| o.amount)
            .fold(Amount::from_sat(0), |b, a| b + a)
    }
}
