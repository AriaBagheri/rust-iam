use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr, Meta};

#[proc_macro_derive(Matches, attributes(wildcard_matching))]
pub fn derive_matches(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Check for the `#[wildcard_matching]` or `#[wildcard_matching(func)]` attribute
    let mut wildcard_function = None;
    for attr in input.attrs {
        if attr.path().is_ident("wildcard_matching") {
            if let Meta::List(m) = attr.meta {
                if let Ok(path) = m.parse_args::<LitStr>() {
                    wildcard_function = Some(quote! { #path });
                }
            } else {
                wildcard_function = Some(quote! { ToString::to_string });
            }
        }
    }

    // Generate the implementation
    let expanded = if let Some(func) = wildcard_function {
        quote! {
            impl MatchesTrait<bool> for #name {
                fn matches(&self, value: &Self) -> Result<bool, &'static str> {
                    use wildcard::Wildcard;

                    let self_str = #func(self);
                    let value_str = #func(value);

                    let pattern = Wildcard::new(self_str.as_bytes())
                        .map_err(|_| "Failed to compile wildcard pattern")?;

                    Ok(pattern.is_match(value_str.as_bytes()))
                }
            }
        }
    } else {
        quote! {
            impl MatchesTrait<bool> for #name {
                fn matches(&self, value: &Self) -> Result<bool, &'static str> {
                    Ok(self == value)
                }
            }
        }
    };

    TokenStream::from(expanded)
}