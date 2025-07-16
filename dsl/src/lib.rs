use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Attribute};

#[proc_macro_derive(SmartContract)]
pub fn smart_contract_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident;
    let fields = input.fields;

    let props = fields.iter().map(|f| f.ident.as_ref().unwrap().to_string());

    let compile = quote! {
        impl SmartContract for #name {
            fn compile(&self) -> Artifact {
                let mut script = Vec::new();
                # (script.extend(self.#fields.to_script()); )*
                Artifact { script, props: vec![#( #props.to_string(), )*] }
            }
        }
    };

    compile.into()
}
