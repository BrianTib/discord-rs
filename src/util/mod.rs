use chrono::Local;
use colored::*;
use std::time::Duration;

pub mod rest;
pub use rest;

pub fn log_message(kind: &str, message: &str) {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let colored_message = match kind {
        "warning" => kind.to_uppercase().bold().yellow(),
        "error" => kind.to_uppercase().bold().red(),
        "success" => kind.to_uppercase().bold().green(),
        "event" => kind.to_uppercase().bold().purple(),
        _ => kind.to_uppercase().bold().white(),
    };

    let log_output = format!("[{}] {} - {}", current_time, colored_message, message);
    println!("{}", log_output);
}

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumArray)]
pub fn enum_array(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Get the name of the enum
    let name = &ast.ident;

    // Get the variants of the enum
    let variants = match ast.data {
        syn::Data::Enum(ref data) => &data.variants,
        _ => panic!("EnumArray macro can only be used on enums."),
    };

    // Generate the array of enum variants
    let array_tokens = variants.iter().map(|variant| &variant.ident);
    let array = quote! { [ #( #array_tokens ),* ] };

    // Generate the final code using the quote crate
    let expanded = quote! {
        impl #name {
            pub fn as_array() -> &'static [#name] {
                #array
            }
        }
    };

    // Return the generated code as a TokenStream
    expanded.into()
}