mod derives;


#[proc_macro_derive(Table)]
pub fn derive_table(tokenstream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokenstream as syn::DeriveInput);

    match derives::expand_derive_table(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }

}



