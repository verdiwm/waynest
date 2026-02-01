use std::{collections::HashMap, ffi::OsStr, fmt::Write as _, fs::OpenOptions, io::Write as _};

use anyhow::{Context, Result, anyhow, bail};
use quote::quote;
use sap::{Argument, Parser};
use walkdir::WalkDir;
use waynest_gen::{ProtocolGenerator, parser::Protocol};

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

const PROTOCOLS: [(&str, &str); 15] = [
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
    ("mutter", "mutter/src/wayland/protocol"),
    ("river", "river/protocol"),
];

const SKIP: [(&str, &str); 5] = [
    ("core", "tests.xml"),
    ("cosmic", "cosmic-image-source-unstable-v1.xml"),
    // "treeland-personalization-manager-v1.xml",
    ("treeland", "treeland-capture-unstable-v1.xml"),
    ("river", "wlr-layer-shell-unstable-v1.xml"),
    ("river", "wlr-output-power-management-unstable-v1.xml"),
];

fn generate() -> Result<()> {
    let mut protocols = HashMap::new();

    for (module, protocol) in PROTOCOLS {
        let mut protos = Vec::new();

        for entry in WalkDir::new(format!("external/protocols/{protocol}")) {
            let entry = entry?;

            if entry.file_type().is_file()
                && let Some(extension) = entry.path().extension()
                && extension == OsStr::new("xml")
                && !SKIP
                    .map(|(p, f)| (p, OsStr::new(f)))
                    .contains(&(module, entry.file_name()))
            {
                protos.push(Protocol::from_path(entry.path())?);
            }
        }

        protocols.insert(module, protos);
    }

    let mut server_rs_content = "pub mod core;".to_string();
    let mut client_rs_content = "pub mod core;".to_string();

    for (module, xml) in &protocols {
        println!("Generating {module} protocols...");

        let server_modules =
            ProtocolGenerator::new(xml, &protocols).generate_protocols(false, true)?;
        let client_modules =
            ProtocolGenerator::new(xml, &protocols).generate_protocols(true, false)?;

        let server_module_content = quote! {
            #(#server_modules)*
        };

        let client_module_content = quote! {
            #(#client_modules)*
        };

        let mut server_module_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(format!("crates/protocols/src/server/{module}.rs"))?;

        let mut client_module_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(format!("crates/protocols/src/client/{module}.rs"))?;

        write!(&mut server_module_file, "{server_module_content}",)?;
        write!(&mut client_module_file, "{client_module_content}",)?;

        if *module != "core" {
            writeln!(
                &mut server_rs_content,
                r#"#[cfg(feature = "{module}")]
        #[cfg_attr(docsrs, doc(cfg(feature = "{module}")))]
        pub mod {module};"#
            )?;

            writeln!(
                &mut client_rs_content,
                r#"#[cfg(feature = "{module}")]
        #[cfg_attr(docsrs, doc(cfg(feature = "{module}")))]
        pub mod {module};"#
            )?;
        }
    }

    let mut server_rs = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("crates/protocols/src/server.rs")?;

    let mut client_rs = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("crates/protocols/src/client.rs")?;

    write!(&mut server_rs, "{server_rs_content}")?;
    write!(&mut client_rs, "{client_rs_content}")?;

    Ok(())
}
