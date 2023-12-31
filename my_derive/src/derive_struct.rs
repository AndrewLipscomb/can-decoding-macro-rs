#![allow(unused)]

use crate::attribute::{ContainerAttributes, FieldAttributes};
use virtue::generate::Generator;
use virtue::parse::{Fields, IdentOrIndex};
use virtue::prelude::*;

pub(crate) struct DeriveStruct {
    pub fields: Option<Fields>,
    pub attributes: ContainerAttributes,
}

impl DeriveStruct {
    pub fn generate_encode(self, generator: &mut Generator) -> Result<()> {
        let crate_name = &self.attributes.crate_name;
        generator
            .impl_for(&format!("{}::CanDecode", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) =
                    (self.attributes.encode_bounds.as_ref()).or(self.attributes.bounds.as_ref())
                {
                    where_constraints.clear();
                    where_constraints
                        .push_parsed_constraint(bounds)
                        .map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints
                            .push_constraint(g, format!("{}::CanDecode", crate_name))
                            .unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("from_socketcan")
            .with_self_arg(virtue::generate::FnSelfArg::None)
            .with_arg("frame", "[u8; 8]")
            .with_return_type(format!("core::result::Result<Self, {}::Error>", crate_name))
            .body(|fn_body| {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        if let Some(fields) = self.fields.as_ref() {
                            let Fields::Struct(fields) = fields else {
                                return Err(Error::Custom { error: "Can't use unnamed members. Use a named element struct".into(), span: None });
                            };
                            for (ident, field) in fields {
                                let attributes = field
                                    .attributes
                                    .get_attribute::<FieldAttributes>()?
                                    .unwrap_or_default();

                                let Some(offset) = attributes.offset else {
                                    return Err(Error::Custom { error: "Did not add an offset for struct member".into(), span: Some(ident.span()) });
                                };

                                let type_str = field.type_string();

                                let advance_token = match attributes.extract_bytes {
                                    Some(extract) => {
                                        format!("{0}::helper::extract_offset_by({1}, &frame, {2})", crate_name, offset, extract)
                                    }
                                    None => {
                                        format!("{0}::helper::extract_offset::<{2}>({1}, &frame)", crate_name, offset, type_str)
                                    }
                                };

                                if let Some(decoder) = attributes.use_decoder {
                                    struct_body
                                    .push_parsed(
                                        format!(
                                        "{1}: {3}( {2}?.try_into().map_err(|_| {0}::Error::InvalidBytesConversion)?,)?,",
                                        crate_name,
                                        ident.to_string(),
                                        advance_token,
                                        decoder,
                                    ))?;
                                }
                                else if attributes.use_big_endian {
                                    struct_body
                                    .push_parsed(format!(
                                        "{1}: {3}::from_be_bytes( {2}?.try_into().map_err(|_| {0}::Error::InvalidBytesConversion)?,),",
                                        crate_name,
                                        ident.to_string(),
                                        advance_token,
                                        type_str,
                                    ))?;
                                }
                                else {
                                    struct_body
                                    .push_parsed(format!(
                                        "{1}: {3}::from_le_bytes( {2}?.try_into().map_err(|_| {0}::Error::InvalidBytesConversion)?,),",
                                        crate_name,
                                        ident.to_string(),
                                        advance_token,
                                        type_str,
                                    ))?;
                                }

                                // fn_body.push_parsed(format!(
                                //     "{}::Encode::encode(&self.{}, encoder)?;",
                                //     crate_name, field
                                // ))?;
                                // if attributes.with_serde {
                                //     fn_body.push_parsed(format!(
                                //         "{0}::Encode::encode(&{0}::serde::Compat(&self.{1}), encoder)?;",
                                //         crate_name, field
                                //     ))?;
                                // } else {
                                //     fn_body.push_parsed(format!(
                                //         "{}::Encode::encode(&self.{}, encoder)?;",
                                //         crate_name, field
                                //     ))?;
                                // }
                            }
                        }

                        Ok(())
                    })?;
                    Ok(())
                });
                Ok(())
            })?;
        Ok(())
    }

    pub fn generate_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_borrow_decode
        let crate_name = &self.attributes.crate_name;

        generator
            .impl_for(format!("{}::Decode", crate_name))
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) =
                    (self.attributes.decode_bounds.as_ref()).or(self.attributes.bounds.as_ref())
                {
                    where_constraints.clear();
                    where_constraints
                        .push_parsed_constraint(bounds)
                        .map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints
                            .push_constraint(g, format!("{}::Decode", crate_name))
                            .unwrap();
                    }
                }
                Ok(())
            })?
            .generate_fn("decode")
            .with_generic_deps("__D", [format!("{}::de::Decoder", crate_name)])
            .with_arg("decoder", "&mut __D")
            .with_return_type(format!(
                "core::result::Result<Self, {}::error::DecodeError>",
                crate_name
            ))
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        // Fields
                        // {
                        //      a: proc_macro_crate::Decode::decode(decoder)?,
                        //      b: proc_macro_crate::Decode::decode(decoder)?,
                        //      ...
                        // }
                        if let Some(fields) = self.fields.as_ref() {
                            for field in fields.names() {
                                let attributes = field
                                    .attributes()
                                    .get_attribute::<FieldAttributes>()?
                                    .unwrap_or_default();
                                // if attributes.with_serde {
                                //     struct_body
                                //         .push_parsed(format!(
                                //             "{1}: (<{0}::serde::Compat<_> as {0}::Decode>::decode(decoder)?).0,",
                                //             crate_name,
                                //             field
                                //         ))?;
                                // } else {
                                //     struct_body
                                //         .push_parsed(format!(
                                //             "{1}: {0}::Decode::decode(decoder)?,",
                                //             crate_name,
                                //             field
                                //         ))?;
                                // }
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        self.generate_borrow_decode(generator)?;
        Ok(())
    }

    pub fn generate_borrow_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_decode
        let crate_name = self.attributes.crate_name;

        generator
            .impl_for_with_lifetimes(format!("{}::BorrowDecode", crate_name), ["__de"])
            .modify_generic_constraints(|generics, where_constraints| {
                if let Some((bounds, lit)) = (self.attributes.borrow_decode_bounds.as_ref())
                    .or(self.attributes.bounds.as_ref())
                {
                    where_constraints.clear();
                    where_constraints
                        .push_parsed_constraint(bounds)
                        .map_err(|e| e.with_span(lit.span()))?;
                } else {
                    for g in generics.iter_generics() {
                        where_constraints
                            .push_constraint(g, format!("{}::de::BorrowDecode<'__de>", crate_name))
                            .unwrap();
                    }
                    for lt in generics.iter_lifetimes() {
                        where_constraints
                            .push_parsed_constraint(format!("'__de: '{}", lt.ident))?;
                    }
                }
                Ok(())
            })?
            .generate_fn("borrow_decode")
            .with_generic_deps("__D", [format!("{}::de::BorrowDecoder<'__de>", crate_name)])
            .with_arg("decoder", "&mut __D")
            .with_return_type(format!(
                "core::result::Result<Self, {}::error::DecodeError>",
                crate_name
            ))
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        if let Some(fields) = self.fields.as_ref() {
                            for field in fields.names() {
                                let attributes = field
                                    .attributes()
                                    .get_attribute::<FieldAttributes>()?
                                    .unwrap_or_default();

                                // format!("{1}: ")
                                // struct_body.push_parsed(item)

                                // if attributes.with_serde {
                                //     struct_body
                                //         .push_parsed(format!(
                                //             "{1}: (<{0}::serde::BorrowCompat<_> as {0}::BorrowDecode>::borrow_decode(decoder)?).0,",
                                //             crate_name,
                                //             field
                                //         ))?;
                                // } else {
                                //     struct_body
                                //         .push_parsed(format!(
                                //             "{1}: {0}::BorrowDecode::borrow_decode(decoder)?,",
                                //             crate_name,
                                //             field
                                //         ))?;
                                // }
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok(())
    }
}
