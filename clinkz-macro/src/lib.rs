use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Item};

#[proc_macro_attribute]
pub fn http_status(attr: TokenStream, item: TokenStream) -> TokenStream {
    let status_expr = if attr.is_empty() {
        quote! { axum::http::StatusCode::INTERNAL_SERVER_ERROR }
    } else {
        match syn::parse::<Expr>(attr.into()) {
            Ok(expr) => quote! { #expr },
            Err(e) => return e.to_compile_error().into(),
        }
    };

    let parsed_item = match syn::parse::<Item>(item) {
        Ok(i) => i,
        Err(e) => return e.to_compile_error().into(),
    };

    let name = match &parsed_item {
        Item::Struct(s) => &s.ident,
        Item::Enum(e) => &e.ident,
        _ => {
            return syn::Error::new_spanned(&parsed_item, "only struct/enum")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        #parsed_item

        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                use axum::Json;
                use serde_json::json;

                let status = #status_expr;
                let body = json!({"error": format!("{}", self)});
                (status, Json(body)).into_response()
            }
        }
    };

    TokenStream::from(expanded)
}
