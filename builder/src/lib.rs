use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::DeriveInput;
use syn::Ident;

macro_rules! builder_name {
    ($t:expr) => {{
        let ident: &Ident = &$t;
        ident.to_string() + "Builder"
    }};
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse::<DeriveInput>(input).unwrap();
    if let Struct(data) = ast.data {
        // println!("ident: {:?}", ast.ident);
        // for field in data.fields {
        //     println!("field: {:?}", field.ident);
        // }
        let ident = ast.ident;
        let builder_name = builder_name!(ident);
        let new_ident = Ident::new(&builder_name, Span::call_site().into());
        quote! {
            pub struct #new_ident {
                executable: Option<String>,
                args: Option<Vec<String>>,
                env: Option<Vec<String>>,
                current_dir: Option<String>,
            }
            impl #ident {
                pub fn builder() -> #new_ident {
                    #new_ident {
                        executable: None,
                        args: None,
                        env: None,
                        current_dir: None,
                    }
                }
            }
        }
        .into()
    } else {
        unimplemented!()
    }
}
