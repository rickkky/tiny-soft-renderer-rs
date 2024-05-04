use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Interpolate)]
pub fn derive_interpolate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse_macro_input!(input);

    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let impl_fns = match ast.data {
        Data::Struct(data) => interpolate_struct(&data),
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    };

    let expanded = quote! {
        #[automatically_derived]
        impl #impl_generics interpolate::Interpolate for #name #ty_generics #where_clause {
            #impl_fns
        }
    };

    expanded.into()
}

fn interpolate_struct(data: &DataStruct) -> TokenStream {
    let linear_impls = match &data.fields {
        Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|field| &field.ident);
            let field_impls = fields.named.iter().map(|field| {
                let field_ty = &field.ty;
                let field_name = &field.ident;
                quote! {
                    <#field_ty as interpolate::Interpolate>::linear_interpolate(&v_0.#field_name, &v_1.#field_name, linear_coord)
                }
            });
            quote! {
                #(
                    #field_names: #field_impls,
                )*
            }
        }
        _ => panic!("Only structs with named fields are supported"),
    };

    let bary_impls = match &data.fields {
        Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|field| &field.ident);
            let field_impls = fields.named.iter().map(|field| {
                let field_ty = &field.ty;
                let field_name = &field.ident;
                quote! {
                    <#field_ty as interpolate::Interpolate>::barycentric_interpolate(&v_0.#field_name, &v_1.#field_name, &v_2.#field_name, bary_coord)
                }
            });
            quote! {
                #(
                    #field_names: #field_impls,
                )*
            }
        }
        _ => panic!("Only structs with named fields are supported"),
    };

    quote! {
        fn linear_interpolate<F: num_traits::Float>(v_0: &Self, v_1: &Self, linear_coord: &nalgebra::Vector2<F>) -> Self {
            Self {
                #linear_impls
            }
        }

        fn barycentric_interpolate<F: num_traits::Float>(v_0: &Self, v_1: &Self, v_2: &Self, bary_coord: &nalgebra::Vector3<F>) -> Self {
            Self {
                #bary_impls
            }
        }
    }
}
