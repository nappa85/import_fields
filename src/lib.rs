use std::fs::File;
use std::io::Read;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, punctuated::Punctuated, ItemStruct, Item, Token};
use proc_macro2::Literal;

#[derive(Debug)]
struct Args {
    pub vars: Vec<Literal>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Literal, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect::<Vec<Literal>>(),
        })
    }
}

#[proc_macro_attribute]
pub fn import_fields(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(args as Args);

    let extra_fields = args
        .vars
        .iter()
        .flat_map(|arg| {
            let s_arg = format!("{}", arg);
            let pathname = if s_arg.starts_with('"') && s_arg.ends_with('"') {
                &s_arg[1..s_arg.len() - 1]
            }
            else {
                &s_arg
            };
            let (path, name) = if let Some(pos) = pathname.find("::") {
                (&pathname[0..pos], Some(&pathname[(pos + 2)..]))
            }
            else {
                (pathname, None)
            };
            let mut file = File::open(path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
        
            let ast = syn::parse_file(&content).unwrap();
            let mut fields = Vec::new();
            for item in ast.items {
                if let Item::Struct(s) = item {
                    if name.is_none() || name == Some(&s.ident.to_string()) {
                        for f in s.fields {
                            fields.push(f);
                        }
                    }
                }
            }
            fields
        })
        .collect::<Vec<_>>();

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in extra_fields {
            fields.named.push(field);
        }
    }

    return quote! {
        #item_struct
    }
    .into();
}
