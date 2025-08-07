use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_derive(SmartContract)]
pub fn smart_contract_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = input.ident;

    let field_idents = input.fields.iter().map(|f| f.ident.as_ref().unwrap());

    let props = input
        .fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string());

    let compile = quote! {
        impl SmartContract for #name {
            fn compile(&self) -> Artifact {
                let mut script = Vec::new();
                #(script.extend(self.#field_idents.to_script());)*
                Artifact { script, props: vec![#(#props.to_string(),)*] }
            }
        }
    };

    compile.into()
}
