// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use serde::{Deserialize, Serialize};

use crate::{EndpointGetAll, EndpointGetOne};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    EndpointGetOne,
    EndpointGetAll,
)]
#[serde(rename_all = "snake_case")]
#[getone(path = "support/tickets/{}", id: String)]
#[getall(path = "support/tickets")]
pub struct Ticket {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub department: String,
    #[serde(default)]
    pub urgency: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "support/legacy-tickets")]
pub struct LegacyTickets(pub Vec<Ticket>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "support/tickets-old")]
pub struct OldTickets(pub Vec<Ticket>);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(transparent)]
#[getone(path = "support/tickets-old/{}", id: String)]
pub struct OldTicket(pub Ticket);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "support/tickets/{}/replies", id: String)]
pub struct TicketReply {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetAll)]
#[serde(rename_all = "snake_case")]
#[getall(path = "support/tickets/departments")]
pub struct TicketDepartment {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EndpointGetOne)]
#[serde(rename_all = "snake_case")]
#[getone(
    path = "support/tickets/{}/attachment/{}/{}/{}",
    id: String,
    attachment_type: String,
    rel_id: String,
    index: u32
)]
pub struct TicketAttachment {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub size: i64,
    #[serde(default)]
    pub data: String,
}
