use std::{ffi::OsStr, fmt::Write as _, fs::OpenOptions, io::Write as _};

use anyhow::{Context, Result, anyhow, bail};
use quote::quote;
use sap::{Argument, Parser};
use walkdir::WalkDir;
use waynest_gen::{generate_module, utils::make_ident};

fn main() -> Result<()> {
    let mut parser = Parser::from_env().context("Failed to init sap")?;

    let mut command = None;

    while let Some(arg) = parser.forward()? {
        match arg {
            Argument::Value(c) => {
                command = Some(c.into_owned());
                break;
            }
            arg => Err(arg.into_error(None))?,
        }
    }

    match command.ok_or(anyhow!("Please specify a command"))?.as_str() {
        "generate" => generate(),
        command => bail!("Unknown command {command}"),
    }
}

const PROTOCOLS: [(&str, &str); 13] = [
    ("core", "wayland/protocol"),
    ("stable", "wayland-protocols/stable"),
    ("staging", "wayland-protocols/staging"),
    ("unstable", "wayland-protocols/unstable"),
    ("wlr", "wlr-protocols/unstable"),
    ("plasma", "plasma-wayland-protocols/src/protocols"),
    ("weston", "weston/protocol"),
    ("cosmic", "cosmic-protocols/unstable"),
    ("frog", "frog-protcols/frog-protocols"),
    ("ivi", "wayland-ivi-extension/protocol"),
    ("hyprland", "hyprland-protocols/protocols"),
    ("mesa", "mesa/src/egl/wayland/wayland-drm"),
    ("treeland", "treeland-protocols/xml"),
];

const SKIP: [&str; 5] = [
    "tests.xml",
    "cosmic-image-source-unstable-v1.xml",
    "cosmic-workspace-unstable-v1.xml",
    "treeland-personalization-manager-v1.xml",
    "treeland-capture-unstable-v1.xml",
];

fn generate() -> Result<()> {
    let mut lib_rs_contents = "pub mod core;".to_string();

    for (module, protocol) in PROTOCOLS {
        println!("Genearting {module}");

        let mut modules = Vec::new();

        for entry in WalkDir::new(format!("external/protocols/{protocol}")) {
            let entry = entry?;

            if entry.file_type().is_file()
                && let Some(extension) = entry.path().extension()
                && extension == OsStr::new("xml")
                && !SKIP.map(OsStr::new).contains(&entry.file_name())
            {
                modules.push(generate_module(module, entry.path())?);
            }
        }

        let module_name = make_ident(module);

        let module_content = quote! {
            mod #module_name {
                #(#modules)*
            }
        };

        let mut module_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(format!("crates/protocols/src/{module}.rs"))?;

        write!(&mut module_file, "{module_content}",)?;

        if module != "core" {
            writeln!(
                &mut lib_rs_contents,
                r#"#[cfg(feature = "{module}")]
        #[cfg_attr(docsrs, doc(cfg(feature = "{module}")))]
        pub mod {module};"#
            )?;
        }
    }

    let mut lib_rs = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(format!("crates/protocols/src/lib.rs"))?;

    write!(&mut lib_rs, "{lib_rs_contents}")?;

    Ok(())
}
