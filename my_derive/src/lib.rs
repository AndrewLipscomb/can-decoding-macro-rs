extern crate proc_macro;
use proc_macro::TokenStream;

use virtue::prelude::*;

mod attribute;
mod derive_struct;

use attribute::ContainerAttributes;

#[proc_macro_derive(CanDecode, attributes(can_extract))]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_encode_inner(input).unwrap_or_else(|e| e.into_token_stream())
}

fn derive_encode_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(body) => {
            derive_struct::DeriveStruct {
                fields: body.fields,
                attributes,
            }
            .generate_encode(&mut generator)?;
        }
        Body::Enum(_body) => {
            // derive_enum::DeriveEnum {
            //     variants: body.variants,
            //     attributes,
            // }
            // .generate_encode(&mut generator)?;
        }
    }

    generator.export_to_file("proc_macro_crate", "Encode");

    generator.finish()
}
