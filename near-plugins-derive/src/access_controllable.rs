use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(
    default,
    attributes(access_controllable),
    forward_attrs(allow, doc, cfg)
)]
struct Opts {
    storage_prefix: Option<String>,
    role_enum: Option<syn::Path>,
}

// TODO remove default
impl Default for Opts {
    fn default() -> Self {
        Self {
            storage_prefix: Some("__acl".to_string()),
            role_enum: None,
        }
    }
}

pub fn derive_access_controllable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let defaults = Opts::default();
    let storage_prefix = opts.storage_prefix.unwrap_or_else(|| {
        defaults
            .storage_prefix
            .expect("Default Opts should have storage prefix")
    });
    let role_enum = opts
        .role_enum
        .expect("Derive input should specify role_enum");

    let output = quote! {
        #[near_bindgen]
        impl AccessControllable for #ident {
            fn acl_storage_prefix(&self) -> &[u8] {
                (#storage_prefix).as_bytes()
            }

            fn acl_has_role(&self, role: u8, account_id: ::near_sdk::AccountId) -> bool {
                false // TODO
            }
        }
    };

    output.into()
}
