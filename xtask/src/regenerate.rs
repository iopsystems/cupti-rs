use std::path::PathBuf;

use anyhow::Context;
use bindgen::callbacks::{DeriveTrait, ImplementsTrait, ParseCallbacks};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Regenerate {
    #[arg(short, long)]
    pub output: PathBuf,

    pub input: String,

    #[arg(last = true)]
    pub rest: Vec<String>,
}

impl Regenerate {
    pub fn run(self) -> anyhow::Result<()> {
        let mut builder = bindgen::builder()
            .allowlist_item("[Cc][Uu][Pp][Tt][Ii].*")
            .parse_callbacks(Box::new(Callbacks))
            .impl_debug(true)
            .derive_default(true)
            .prepend_enum_name(false)
            .raw_line("#![rustfmt::skip]");

        builder = builder.blocklist_function("cu([^p]|p[^t]|pt[^i]).*");

        builder
            .clang_args(["-x", "c++"])
            .clang_args(&self.rest)
            .header(&self.input)
            .generate()?
            .write_to_file(&self.output)
            .context(format!("failed to write to {}", self.output.display()))?;

        let text = std::fs::read_to_string(&self.output)
            .context(format!("failed to read {}", self.output.display()))?;
        let ast = syn::parse_file(&text).context("failed to parse bindings")?;
        std::fs::write(&self.output, prettyplease::unparse(&ast))
            .context(format!("failed to write to {}", self.output.display()))?;

        Ok(())
    }
}

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn blocklisted_type_implements_trait(
        &self,
        name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        match derive_trait {
            DeriveTrait::Copy => Some(ImplementsTrait::Yes),
            DeriveTrait::Debug => Some(ImplementsTrait::Yes),
            DeriveTrait::Default => {
                if name.ends_with("_st") {
                    Some(ImplementsTrait::No)
                } else {
                    Some(ImplementsTrait::Yes)
                }
            }
            _ => None,
        }
    }
}
