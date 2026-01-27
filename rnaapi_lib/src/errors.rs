// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
use thiserror::Error;

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum NaApiError {
    #[error("{0}")]
    IpNotAllowed(String),
    #[error("{0}")]
    APIKeyInvalid(String),
    #[error("{0}")]
    UnknownError(String),
}
