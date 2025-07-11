extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta, ItemImpl, Expr, Stmt, punctuated::Punctuated, token::Comma, Ident};
use serde::{Serialize, Deserialize};
use nprint_core::bsv_script;  // Import from core

/// sCrypt-like data types as traits/generics.
pub trait ScryptType: ToScript + Serialize {}
impl ScryptType for i128 {}  // BigInt
impl ScryptType for Vec<u8> {}  // ByteString
impl<T: ScryptType + Serialize, const N: usize> ScryptType for [T; N] {}  // FixedArray

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

    let (props, _prop_types) = match &input.data {
        Data::Struct(data) => if let Fields::Named(fields) = &data.fields {
            let props: Vec<_> = fields.named.iter().filter(|f| f.attrs.iter().any(|a| a.path().is_ident("prop"))).map(|f| f.ident.as_ref().unwrap()).collect();
            let types: Vec<_> = fields.named.iter().filter(|f| f.attrs.iter().any(|a| a.path().is_ident("prop"))).map(|f| &f.ty).collect();
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
            Meta::List(list) => {
                let tokens: Punctuated<Meta, Comma> = syn::parse2(list.tokens).unwrap();
                if tokens.len() == 1 {
                    if let Meta::NameValue(nv) = &tokens[0] {
                        if nv.path.is_ident("mutable") {
                            if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(b), .. }) = &nv.value {
                                b.value
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            _ => false,
        }
    } else {
        false
    };
    // Add mutable logic later (e.g., generate setter)
    item
}

/// #[method]: Compile method body to script.
#[proc_macro_attribute]
pub fn method(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let self_ty = &input.self_ty;
    let method = match &input.items[0] {
        syn::ImplItem::Fn(method) => method,
        _ => panic!("Expected a method in impl block"),
    };
    let method_name = &method.sig.ident;
    let script_method_name = format_ident!("{}_script", method_name);
    let body = &method.block;

    let mut script_tokens = quote! {};
    for stmt in &body.stmts {
        match stmt {
            // Map if to OP_IF
            Stmt::Expr(Expr::If(if_expr), _) => {
                let cond_script = expr_to_script(&if_expr.cond);
                let then_script = block_to_script(&if_expr.then_branch);
                script_tokens = quote! { #script_tokens nprint_core::bsv_script!(OP_IF) #then_script nprint_core::bsv_script!(OP_ENDIF) };
            }
            // Map assert_eq(a, b) to a b OP_EQUALVERIFY
            Stmt::Expr(Expr::Call(call), _) => {
                if let Expr::Path(path) = &*call.func {
                    if path.path.get_ident().map(|ident| ident.to_string()) == Some("assert_eq".to_string()) {
                        let a = expr_to_script(&call.args[0]);
                        let b = expr_to_script(&call.args[1]);
                        script_tokens = quote! { #script_tokens #a #b nprint_core::bsv_script!(OP_EQUALVERIFY) };
                    }
                }
            }
            // Add more: loops (unroll), arith, etc.
            _ => {},
        }
    }

    let expanded = quote! {
        #input
        impl #self_ty {
            pub fn #script_method_name(&self) -> Vec<u8> {
                nprint_core::bsv_script! { #script_tokens }
            }
        }
    };
    expanded.into()
}

/// Helper: Expr to script tokens (stub; expand for ops).
fn expr_to_script(expr: &Expr) -> proc_macro2::TokenStream {
    match expr {
        // Pseudo
        Expr::Path(p) => {
            let ident = p.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
            quote! { nprint_core::bsv_script! { self.#ident } }
        }
        _ => quote! {},
    }
}

// Recurse stmts
fn block_to_script(_block: &syn::Block) -> proc_macro2::TokenStream {
    quote! {}
}

// Tests: Add in dsl/tests/...
