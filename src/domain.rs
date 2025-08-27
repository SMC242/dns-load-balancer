use std::{collections::HashSet, str::FromStr};

/// Get the DNS zones in top-down order (I.E TLD first)
pub fn to_zones(domain: &str) -> impl Iterator<Item = &str> {
    domain.split('.').rev()
}

pub struct Domain {
    name: String,
    zones: Vec<String>,
}

impl Domain {
    pub fn new(domain: &str) -> Self {
        Self {
            name: domain.to_string(),
            zones: to_zones(domain).map(str::to_string).collect(),
        }
    }
}

pub enum DomainParseError {
    EmptyString,
    InvalidFormat,
}

impl FromStr for Domain {
    type Err = DomainParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<String> = to_zones(s).map(str::to_string).collect();

        match segments.len() {
            0 => Err(DomainParseError::EmptyString),
            1 => Err(DomainParseError::InvalidFormat),
            _ => Ok(Self {
                name: s.to_string(),
                zones: segments,
            }),
        }
    }
}
