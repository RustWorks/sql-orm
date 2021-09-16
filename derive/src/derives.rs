use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

pub fn expand_derive_table(input: DeriveInput) -> syn::Result<TokenStream> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = input.data
    {
        let ident = &input.ident;
        let fields = fields.named.iter().map(|field| {
            let name = field.ident.clone();
            quote! {stringify!(#name)}
        });

        let table_name = {
            let mut s = ident.to_string().to_lowercase();
            s.push('s');
            Ident::new(&s, Span::call_site())
        };

        let expanded = quote! {
          impl sql_orm::Table for #ident {
              const NAME: &'static str = stringify!(#table_name);
              const FIELDS: &'static [&'static str] = [#(#fields)*];
          }
        };

        Ok(TokenStream::from(expanded))
    } else {
        Err(syn::Error::new_spanned(
            input,
            "Only support named structure.",
        ))
    }
}
