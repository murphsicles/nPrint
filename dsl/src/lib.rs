extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, NestedMeta, ItemImpl, Expr, Stmt, Type};
use serde::{Serialize, Deserialize};
use nprint_core::{bsv_script, xswap, Stack};  // Import from core

/// sCrypt-like data types as traits/generics.
pub trait ScryptType: ToScript + Serialize {}
impl ScryptType for i128 {}  // BigInt
impl ScryptType for Vec<u8> {}  // ByteString
impl<T: ScryptType, const N: usize> ScryptType for [T; N] {}  // FixedArray

/// Trait to convert to BSV script pushes.
pub trait ToScript {
    fn to_script(&self) -> Vec<u8>;
}
impl ToScript for i128 {
    fn to_script(&self) -> Vec<u8> { bsv_script! { *self as i32 } }  // Simplify
}
impl ToScript for Vec<u8> {
    fn to_script(&self) -> Vec<u8> { self.clone() }
}
impl<T: ToScript, const N: usize> ToScript for [T; N] {
    fn to_script(&self) -> Vec<u8> {
        let mut script = Vec::new();
        for item in self { script.extend(item.to_script()); }
        script
    }
}

/// Artifact: Compiled contract output (JSON serializable).
#[derive(Serialize, Deserialize)]
pub struct Artifact {
    pub script: Vec<u8>,
    pub props: Vec<String>,  // Prop names
}

/// Trait for contracts.
pub trait SmartContract {
    fn compile(&self) -> Artifact;
}

/// #[contract]: Generates SmartContract impl, compiles props/methods.
#[proc_macro_attribute]
pub fn contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    let (props, prop_types) = match &input.data {
        Data::Struct(data) => if let Fields::Named(fields) = &data.fields {
            let props: Vec<_> = fields.named.iter().filter(|f| f.attrs.iter().any(|a| a.path.is_ident("prop"))).map(|f| f.ident.as_ref().unwrap()).collect();
            let types: Vec<_> = fields.named.iter().filter(|f| f.attrs.iter().any(|a| a.path.is_ident("prop"))).map(|f| &f.ty).collect();
            (props, types)
        } else { panic!("Named fields only"); },
        _ => panic!("Structs only"),
    };

    let expanded = quote! {
        #input

        impl #generics SmartContract for #name #generics {
            fn compile(&self) -> Artifact {
                let mut script = Vec::new();
                // Push props
                #(script.extend(self.#props.to_script());)*
                // Methods compiled separately; assume one main unlock for now
                Artifact { script, props: vec![#(stringify!(#props).to_string()),*] }
            }
        }
    };
    expanded.into()
}

/// #[prop(mutable = bool)]: Mark state.
#[proc_macro_attribute]
pub fn prop(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse mutable
    let is_mutable = if !attr.is_empty() {
        let meta = parse_macro_input!(attr as Meta);
        match meta {
            Meta::List(list) if list.nested.len() == 1 => {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = &list.nested[0] {
                    if nv.path.is_ident("mutable") { if let syn::Lit::Bool(b) = &nv.lit { b.value } else { false } } else { false }
                } else { false }
            },
            _ => false,
        }
    } else { false };
    // Add mutable logic later (e.g., generate setter)
    item
}

/// #[method]: Compile method body to script.
#[proc_macro_attribute]
pub fn method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let method_name = input.items[0].ident();  // Assume single fn
    let body = &input.items[0].block();  // Parse stmts

    let mut script_tokens = quote! {};
    for stmt in &body.stmts {
        match stmt {
            Stmt::Expr(Expr::If(if_expr)) => {
                // Map if to OP_IF
                let cond_script = expr_to_script(&if_expr.cond);
                let then_script = block_to_script(&if_expr.then_branch);
                script_tokens = quote! { #script_tokens OP_IF #then_script OP_ENDIF };
            }
            Stmt::Expr(Expr::Call(call)) if call.func.to_string() == "assert_eq" => {
                // Map assert_eq(a, b) to a b OP_EQUALVERIFY
                let a = expr_to_script(&call.args[0]);
                let b = expr_to_script(&call.args[1]);
                script_tokens = quote! { #script_tokens #a #b OP_EQUALVERIFY };
            }
            // Add more: loops (unroll), arith, etc.
            _ => {},
        }
    }

    let expanded = quote! {
        #input
        impl #input_self_ty {
            pub fn #method_name_script(&self) -> Vec<u8> {
                bsv_script! { #script_tokens }
            }
        }
    };
    expanded.into()
}

/// Helper: Expr to script tokens (stub; expand for ops).
fn expr_to_script(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Path(p) => quote! { OP_PUSH self.#p },  // Pseudo
        _ => quote! {},
    }
}

fn block_to_script(block: &syn::Block) -> proc_macro2::TokenStream {
    // Recurse stmts
    quote! {}
}

// Tests: Add in dsl/tests/...
