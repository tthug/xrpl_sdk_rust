use crate::Amount;
use serde::Deserialize;

/// The object was placed as a passive offer
pub const LSF_PASSIVE: u32 = 0x00010000;

/// The object was placed as a sell offer
pub const LSF_SELL: u32 = 0x00020000;

/// An offer in the ledger.
///
/// <https://xrpl.org/offer.html>
///
/// {
///     "Account": "rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt",
///     "BookDirectory": "ACC27DE91DBA86FC509069EAF4BC511D73128B780F2E54BF5E07A369E2446000",
///     "BookNode": "0000000000000000",
///     "Flags": 131072,
///     "LedgerEntryType": "Offer",
///     "OwnerNode": "0000000000000000",
///     "PreviousTxnID": "F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF",
///     "PreviousTxnLgrSeq": 14524914,
///     "Sequence": 866,
///     "TakerGets": {
///         "currency": "XAG",
///         "issuer": "r9Dr5xwkeLegBeXq6ujinjSBLQzQ1zQGjH",
///         "value": "37"
///     },
///     "TakerPays": "79550000000",
///     "index": "96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797"
/// }
#[derive(Debug, Deserialize)]
pub struct Offer {
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "BookDirectory")]
    pub book_directory: String,

    #[serde(rename = "BookNode")]
    pub book_node: Option<String>,

    #[serde(rename = "Flags")]
    pub flags: u32,

    #[serde(rename = "Sequence")]
    pub sequence: u32,

    #[serde(rename = "TakerGets")]
    pub taker_gets: Amount,

    #[serde(rename = "TakerPays")]
    pub taker_pays: Amount,

    pub index: Option<String>,
}
