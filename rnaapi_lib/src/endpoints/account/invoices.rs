// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::super::flex;
use crate::EndpointGetAll;
use crate::EndpointGetOne;

/// Account invoices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "account/invoices")]
pub struct Invoices {
    pub id: u32,
    pub userid: u32,
    pub date: String,
    pub duedate: String,
    pub subtotal: String,
    pub credit: String,
    pub status: String,
    pub paymentmethod: String,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub datepaid: NaiveDateTime,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub created_at: NaiveDateTime,
    // #[serde(with = "custom_datetime_format_seconds")]
    // pub updated_at: NaiveDateTime,
    //
    // pub datepaid: String,
    // pub created_at: String,
    // pub updated_at: String,
}

/// `GET account/invoices/{id}`. The list row is `Invoices`; this call returns
/// the WHMCS invoice, including its line items.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "UPPERCASE")]
#[getone(path = "account/invoices/{}", id: u32)]
pub struct Invoice {
    #[serde(rename = "INVOICEID", deserialize_with = "flex::int")]
    pub id: i64,
    #[serde(default)]
    pub invoicenum: Option<String>,
    #[serde(deserialize_with = "flex::int")]
    pub userid: i64,
    pub date: String,
    pub duedate: String,
    #[serde(default)]
    pub datepaid: Option<String>,
    #[serde(default, rename = "LASTCAPTUREATTEMPT")]
    pub last_capture_attempt: Option<String>,
    pub subtotal: String,
    pub credit: String,
    pub tax: String,
    pub tax2: String,
    pub total: String,
    pub balance: String,
    pub taxrate: String,
    pub taxrate2: String,
    pub status: String,
    pub paymentmethod: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub ccgateway: Option<String>,
    #[serde(default, deserialize_with = "invoice_items")]
    pub items: Vec<InvoiceItem>,
    #[serde(default)]
    pub transactions: Option<Value>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct InvoiceItem {
    #[serde(deserialize_with = "flex::int")]
    pub id: i64,
    #[serde(default)]
    pub r#type: String,
    #[serde(default, deserialize_with = "flex::int")]
    pub relid: i64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub taxed: String,
}

/// WHMCS sends one line item as an object and several as a list.
fn invoice_items<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<InvoiceItem>, D::Error> {
    let value = Option::<Value>::deserialize(deserializer)?;
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let item = match value {
        Value::Object(mut map) => map.remove("ITEM").unwrap_or(Value::Null),
        other => other,
    };
    match item {
        Value::Null => Ok(Vec::new()),
        Value::Array(rows) => rows
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(de::Error::custom),
        Value::Object(_) => serde_json::from_value(item)
            .map(|item| vec![item])
            .map_err(de::Error::custom),
        _ => Err(de::Error::custom(
            "invoice ITEMS.ITEM was not an object or a list",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn invoice(items: Value) -> Value {
        json!({
            "INVOICEID": "10",
            "USERID": "4",
            "DATE": "2026-01-01",
            "DUEDATE": "2026-01-15",
            "SUBTOTAL": "1.00",
            "CREDIT": "0.00",
            "TAX": "0.00",
            "TAX2": "0.00",
            "TOTAL": "1.00",
            "BALANCE": "1.00",
            "TAXRATE": "0.00",
            "TAXRATE2": "0.00",
            "STATUS": "Unpaid",
            "PAYMENTMETHOD": "card",
            "ITEMS": items
        })
    }

    #[test]
    fn one_line_item_is_a_single_object() {
        let invoice: Invoice = serde_json::from_value(invoice(json!({
            "ITEM": {
                "ID": "7",
                "TYPE": "Hosting",
                "RELID": "3",
                "DESCRIPTION": "server",
                "AMOUNT": "1.00",
                "TAXED": "0"
            }
        })))
        .unwrap();
        assert_eq!(invoice.id, 10);
        assert_eq!(invoice.items.len(), 1);
        assert_eq!(invoice.items[0].id, 7);
        assert_eq!(invoice.items[0].r#type, "Hosting");
    }

    #[test]
    fn several_line_items_are_a_list() {
        let invoice: Invoice = serde_json::from_value(invoice(json!({
            "ITEM": [
                {"ID": "7", "TYPE": "Hosting", "RELID": "3", "DESCRIPTION": "a", "AMOUNT": "1.00", "TAXED": "0"},
                {"ID": "8", "TYPE": "Addon", "RELID": "3", "DESCRIPTION": "b", "AMOUNT": "2.00", "TAXED": "1"}
            ]
        })))
        .unwrap();
        assert_eq!(invoice.items.len(), 2);
        assert_eq!(invoice.items[1].id, 8);
    }
}
