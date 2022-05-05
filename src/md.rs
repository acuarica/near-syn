//! Markdown
use std::io::{self, Write};

use syn::{File, ImplItemMethod, Item};

use crate::{
    contract::{Contract, NearItemTrait},
    get_docs,
    ts::{ts_ret_type, ts_sig},
    write_docs, NearImpl, NearMethod,
};

///
pub fn md_prelude<W: Write>(buf: &mut W, now: String) -> io::Result<()> {
    writeln!(buf, "<!-- AUTOGENERATED doc{}, do not modify! -->", now)?;
    writeln!(buf, "# Contract\n")?;

    Ok(())
}

///
pub fn md_footer<W: Write>(buf: &mut W, bin: &str, now: String) -> io::Result<()> {
    writeln!(buf, "\n---\n\nReferences\n")?;
    writeln!(
        buf,
        "- :rocket: Initialization method. Needs to be called right after deployment."
    )?;
    writeln!(
        buf,
        "- :eyeglasses: View only method, *i.e.*, does not modify the contract state."
    )?;
    writeln!(
        buf,
        "- :writing_hand: Call method, i.e., does modify the contract state."
    )?;
    writeln!(
        buf,
        "- &#x24C3; Payable method, i.e., call needs to have an attached NEAR deposit."
    )?;

    writeln!(
        buf,
        "\n---\n\n*This documentation was generated with* **{} v{}** <{}>{}",
        bin,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY"),
        now
    )?;

    Ok(())
}

///
pub fn md_methods_table<W: Write>(
    buf: &mut W,
    asts: &Vec<File>,
    contract: &Contract,
) -> io::Result<()> {
    writeln!(buf, "| Method | Description | Return |")?;
    writeln!(buf, "| ------ | ----------- | ------ |")?;

    for ast in asts {
        for item in &ast.items {
            if let Item::Impl(impl_item) = item {
                if let Some(methods) = impl_item.bindgen_methods() {
                    let item_trait = if let Some(trait_name) = impl_item.get_trait_name() {
                        contract.traits.get(&trait_name)
                    } else {
                        None
                    };
                    md_methods_table_rows(buf, &methods, item_trait)?;
                }
            }
        }
    }

    writeln!(buf, "")?;

    Ok(())
}

///
pub fn md_methods_table_rows<W: Write>(
    buf: &mut W,
    methods: &Vec<&ImplItemMethod>,
    item_trait: Option<&NearItemTrait>,
) -> io::Result<()> {
    for method in methods {
        let (mut_mod, init_decl) = method.mods();
        let docs = get_docs(&method.join_attrs(item_trait)).join(" ");

        writeln!(
            buf,
            "| {} `{}`{} | {} | `{}` |",
            mut_mod,
            method.sig.ident,
            init_decl,
            docs,
            ts_ret_type(&method.sig.output).replace('|', "\\|"),
        )?;
    }

    Ok(())
}

///
pub fn md_items<W: Write>(buf: &mut W, syntax: &syn::File, contract: &Contract) -> io::Result<()> {
    write_docs(buf, &syntax.attrs, |l| l.trim().to_string())?;

    for item in &syntax.items {
        if let Item::Impl(impl_item) = item {
            if let Some(methods) = impl_item.bindgen_methods() {
                let item_trait = if let Some(trait_name) = impl_item.get_trait_name() {
                    writeln!(buf, "\n## Methods for `{}` interface", trait_name)?;

                    contract.traits.get(&trait_name)
                } else {
                    if let Some(impl_name) = impl_item.get_impl_name() {
                        writeln!(buf, "\n## Methods for {}", impl_name)?;
                    } else {
                        writeln!(buf, "\n## Methods for Contract")?;
                    }
                    None
                };

                md_methods(buf, methods, item_trait)?;
            }
        }
    }

    Ok(())
}

///
pub fn md_methods<W: Write>(
    buf: &mut W,
    methods: Vec<&ImplItemMethod>,
    item_trait: Option<&NearItemTrait>,
) -> io::Result<()> {
    for method in methods {
        let (mut_mod, init_decl) = method.mods();
        writeln!(
            buf,
            "\n### {} `{}`{}\n",
            mut_mod, method.sig.ident, init_decl
        )?;
        writeln!(buf, "```typescript\n{}\n```\n", ts_sig(&method))?;
        write_docs(buf, &method.join_attrs(item_trait), |l| {
            l.trim().to_string()
        })?;
    }

    Ok(())
}

///
pub trait MarkdownMethod {
    ///
    fn mut_mod(&self) -> &str;
    ///
    fn mods(&self) -> (&str, &str);
}

impl MarkdownMethod for ImplItemMethod {
    fn mut_mod(&self) -> &str {
        if self.is_mut() {
            if self.is_payable() {
                "&#x24C3;"
            } else {
                ":writing_hand:"
            }
        } else {
            ":eyeglasses:"
        }
    }

    fn mods(&self) -> (&str, &str) {
        if self.is_init() {
            (":rocket:", " (*constructor*)")
        } else {
            (self.mut_mod(), "")
        }
    }
}