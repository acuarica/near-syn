#![deny(warnings)]

use chrono::Utc;
use clap::Clap;
use near_syn::{
    has_attr, is_init, is_mut, is_payable, is_public, join_path, parse_rust, ts::ts_sig,
    write_docs, Args,
};
use std::{env, io};
use syn::{ImplItem, Item::Impl, ItemImpl, Type};

Args!(env!("CARGO_BIN_NAME"));

fn main() {
    let mut args = Args::parse();

    println!("<!-- AUTOGENERATED doc, do not modify! {} -->", args.now());
    println!("# Contract\n");

    for file_name in &args.files {
        let ast = parse_rust(file_name);
        md(&ast);
    }

    println!("\n---\n\nReferences\n");
    println!("- :rocket: Initialization method. Needs to be called right after deployment.");
    println!("- :eyeglasses: View only method, *i.e.*, does not modify the contract state.");
    println!("- :writing_hand: Call method, i.e., does modify the contract state.");
    println!("- &#x24C3; Payable method, i.e., call needs to have an attached NEAR deposit.");

    println!(
        "\n---\n\n*This documentation was generated with* **{} v{}** <{}> *on {}*",
        env!("CARGO_BIN_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY"),
        args.now()
    );
}

fn md(syntax: &syn::File) {
    write_docs(&mut io::stdout(), &syntax.attrs, |l| l.trim().to_string());

    for item in &syntax.items {
        if let Impl(impl_item) = item {
            if has_attr(&impl_item.attrs, "near_bindgen") {
                if let Some((_, trait_path, _)) = &impl_item.trait_ {
                    println!("\n## Methods for `{}` interface", join_path(trait_path));
                } else {
                    if let Type::Path(type_path) = &*impl_item.self_ty {
                        println!("\n## Methods for {}", join_path(&type_path.path));
                    } else {
                        println!("\n## Methods for Contract");
                    }
                }

                methods(&impl_item);
            }
        }
    }
}

fn methods(input: &ItemImpl) {
    for impl_item in input.items.iter() {
        if let ImplItem::Method(method) = impl_item {
            if is_public(method) || input.trait_.is_some() {
                let mut mut_mod = if is_mut(&method) {
                    if is_payable(&method) {
                        "&#x24C3;"
                    } else {
                        ":writing_hand:"
                    }
                } else {
                    ":eyeglasses:"
                };
                let init_decl = if is_init(&method) {
                    mut_mod = ":rocket:";
                    " (*constructor*)"
                } else {
                    ""
                };
                println!("\n### {} `{}`{}\n", mut_mod, method.sig.ident, init_decl);
                println!("```typescript\n{}\n```\n", ts_sig(&method));
                write_docs(&mut io::stdout(), &method.attrs, |l| l.trim().to_string());
            }
        }
    }
}