use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Derive macro for the `xcfg::XCfg` trait.
///
/// This macro can only be used on structs. It generates an empty implementation
/// of `xcfg::XCfg`, relying on the trait's default methods for `load`, `save`,
/// etc. The struct must also implement `serde::Serialize` and
/// `serde::Deserialize` for those methods to be usable.
#[proc_macro_derive(XCfg)]
pub fn derive_xcfg(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    if !matches!(ast.data, Data::Struct(_)) {
        return syn::Error::new_spanned(ast, "XCfg derive is only supported on structs")
            .to_compile_error()
            .into();
    }

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::xcfg::XCfg for #name #ty_generics #where_clause {}
    }
    .into()
}
