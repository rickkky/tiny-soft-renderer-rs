use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

#[proc_macro_derive(Interpolate)]
pub fn derive_interpolate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse_macro_input!(input);

    let struct_name = &ast.ident;

    let expanded = match ast.data {
        Data::Struct(data) => interpolate_struct(struct_name, &data),
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    expanded.into()
}

fn interpolate_struct(struct_name: &Ident, data: &DataStruct) -> TokenStream {

    let field_linear_impls = match &data.fields {
        Fields::Named(fields) => {

        }
        _ => panic!("Only structs with named fields are supported"),
    }
}
