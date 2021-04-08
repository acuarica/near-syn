use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::process;
use std::{env, ops::Deref};

use chrono::Utc;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{
    Attribute, FnArg, ImplItem, ImplItemMethod, Item::Impl, ItemImpl, Pat, Path, Type, Visibility,
};
use TokenTree::Literal;

fn main() {
    let mut args = env::args();
    let _ = args.next(); // executable name

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename,
        _ => {
            eprintln!("Usage: dump-syntax path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut file = File::open(&filename).expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    let now = Utc::now();
    println!("<!-- AUTOGENERATED doc, do not modify! {} -->", now);
    println!("# Contract\n");
    extract_docs(&syntax.attrs);

    for item in syntax.items {
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

    println!("\n---\n\nReferences\n");
    println!("- :bricks: Initialization method. Needs to be called right after deployment.");
    println!("- :eyeglasses: View only method, *i.e.*, does not modify the contract state.");
    println!("- :writing_hand: Call method, i.e., does modify the contract state.");

    let name = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    let repo = env!("CARGO_PKG_REPOSITORY");
    println!(
        "\n---\n\n*This documentation was generated with* **{} v{}** <{}> *on {}*",
        name, ver, repo, now
    );
}

fn methods(input: &ItemImpl) {
    for impl_item in input.items.iter() {
        if let ImplItem::Method(method) = impl_item {
            if is_public(method) || input.trait_.is_some() {
                let mut mut_mod = if is_mut(&method) {
                    ":writing_hand:"
                } else {
                    ":eyeglasses:"
                };
                let init_decl = if has_attr(&method.attrs, "init") {
                    mut_mod = ":bricks:";
                    " (*constructor*)"
                } else {
                    ""
                };
                println!("\n### {} `{}`{}\n", mut_mod, method.sig.ident, init_decl);
                let sig = extract_sig(&method);
                println!("```typescript\n{}\n```\n", sig);
                extract_docs(&method.attrs);
            }
        }
    }
}

fn is_mut(method: &ImplItemMethod) -> bool {
    if let Some(FnArg::Receiver(r)) = method.sig.inputs.iter().next() {
        r.mutability.is_some()
    } else {
        false
    }
}

fn extract_sig(method: &ImplItemMethod) -> String {
    let mut args = Vec::new();
    for arg in method.sig.inputs.iter() {
        match arg {
            FnArg::Typed(pat_type) => {
                if let Pat::Ident(pat_ident) = pat_type.pat.deref() {
                    let type_name = if let Type::Path(type_path) = &*pat_type.ty {
                        join_path(&type_path.path)
                    } else {
                        "?".to_string()
                    };
                    let arg_ident = &pat_ident.ident;
                    args.push(format!("{}: {}", arg_ident, type_name));
                }
            }
            _ => {}
        }
    }

    let ret_type = match &method.sig.output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, typ) => {
            let typ = typ.deref();
            let type_name = proc_macro2::TokenStream::from(quote! { #typ }).to_string();
            if type_name == "Self" {
                "void".to_string()
            } else {
                type_name
            }
        }
    };

    let mut fmt = String::new();
    write!(fmt, "{}(", method.sig.ident).unwrap();
    write!(fmt, "{}", args.join(", ")).unwrap();
    write!(fmt, "): {}", ret_type).unwrap();
    fmt
}

fn is_public(method: &ImplItemMethod) -> bool {
    match method.vis {
        Visibility::Public(_) => true,
        _ => false,
    }
}

fn join_path(path: &Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

fn has_attr(attrs: &Vec<Attribute>, attr_name: &str) -> bool {
    for attr in attrs {
        if attr.path.is_ident(attr_name) {
            return true;
        }
    }
    false
}

fn extract_docs(attrs: &Vec<Attribute>) {
    for attr in attrs {
        if attr.path.is_ident("doc") {
            for token in attr.tokens.clone() {
                if let Literal(lit) = token {
                    if let Some(line) = lit
                        .to_string()
                        .strip_prefix('"')
                        .and_then(|s| s.strip_suffix('"'))
                    {
                        println!("{}", line.trim());
                    }
                }
            }
        }
    }
}