use virtue::prelude::*;
use virtue::utils::{parse_tagged_attribute, ParsedAttribute};

use std::str::FromStr;

#[derive(Debug)]
pub struct ContainerAttributes {
    pub crate_name: String,
    pub bounds: Option<(String, Literal)>,
    pub decode_bounds: Option<(String, Literal)>,
    pub borrow_decode_bounds: Option<(String, Literal)>,
    pub encode_bounds: Option<(String, Literal)>,
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::can_extract".to_string(),
            bounds: None,
            decode_bounds: None,
            encode_bounds: None,
            borrow_decode_bounds: None,
        }
    }
}

impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "can_extract")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Self::default();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Property(key, val) if key.to_string() == "crate" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.crate_name = val_string[1..val_string.len() - 1].to_string();
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "decode_bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.decode_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "encode_bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.encode_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val)
                    if key.to_string() == "borrow_decode_bounds" =>
                {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.borrow_decode_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Tag(i) => {
                    return Err(Error::custom_at("Unknown field attribute", i.span()))
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown field attribute", key.span()))
                }
                _ => {}
            }
        }
        Ok(Some(result))
    }
}

#[derive(Default, Debug)]
pub struct FieldAttributes {
    pub offset: Option<u8>,
    pub extract_bytes: Option<u8>,
    pub use_big_endian: bool,
    pub use_decoder: Option<String>,
}

impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "can_extract")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Self::default();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(key) => match key.to_string().as_str() {
                    "use_big_endian" => {
                        result.use_big_endian = true;
                    }
                    _ => {
                        return Err(Error::custom_at("Unknown field attribute", key.span()));
                    }
                },
                ParsedAttribute::Property(key, value) => {
                    let str = value.to_string();
                    match key.to_string().as_str() {
                        "offset" => {
                            let Ok(offset_val) = u8::from_str(&str) else {
                                return Err(Error::custom_at("Invalid offset value", key.span()))
                            };
                            if offset_val >= 8 {
                                return Err(Error::custom_at(
                                    "Invalid offset, must be less than 8",
                                    key.span(),
                                ));
                            }
                            result.offset = Some(offset_val);
                        }
                        "use_decoder" => {
                            result.use_decoder = Some(str.replace("\"", ""));
                        }
                        "extract" => {
                            result.extract_bytes = u8::from_str(&str)
                                .map_err(|e| {
                                    Error::custom_at(
                                        format!("Invalid extract value - {}", e),
                                        key.span(),
                                    )
                                })?
                                .into();
                        }
                        _ => {
                            return Err(Error::custom_at("Unknown field attribute", key.span()));
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(Some(result))
    }
}
