use anyhow::Result;
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::HashMap,
    fmt::Write as _,
    fs::OpenOptions,
    io::{stdout, Write as _},
};
use tracing::info;

use waynest_gen::{
    generate_client_code, generate_server_code,
    parser::{Error, Pair},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let Args { json } = Args::parse();

    let protocols_list: HashMap<String, Vec<String>> =
        serde_json::from_str(include_str!("protocols.json"))?;

    let mut protocols = Vec::new();
    let mut pairs = Vec::new();

    for (module, protos) in &protocols_list {
        let current = protos
            .into_iter()
            .map(|path| Pair::from_path(module, path))
            .collect::<Result<Vec<Pair>, Error>>()?;

        pairs.extend(current.clone());
        protocols.push((module, current))
    }

    if json {
        info!("Generating json file");

        let mut stdout = stdout().lock();

        serde_json::to_writer(&mut stdout, &protocols)?;
    } else {
        let mut module_content = "pub mod core;".to_string();

        for (module, _current) in &protocols {
            if module.as_str() != "core" {
                writeln!(
                    &mut module_content,
                    r#"#[cfg(feature = "{module}")]
        #[cfg_attr(docsrs, doc(cfg(feature = "{module}")))]
        pub mod {module};"#
                )?;
            }
        }

        protocols.par_iter().try_for_each(|(module, current)| {
            let mut server_path = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(format!("src/server/protocol/{module}.rs"))?;

            write!(
                &mut server_path,
                "{}",
                generate_server_code(&current, &pairs)
            )?;

            let mut client_path = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(format!("src/client/protocol/{module}.rs"))?;

            write!(
                &mut client_path,
                "{}",
                generate_client_code(&current, &pairs)
            )?;

            anyhow::Ok(())
        })?;

        let mut server_module = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("src/server/protocol.rs")?;

        write!(&mut server_module, "{module_content}",)?;

        let mut client_module = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("src/client/protocol.rs")?;

        write!(&mut client_module, "{module_content}",)?;
    }

    Ok(())
}
