use anyhow::Result;
use clap::Parser;
use parser::Protocol;
use std::{fs::OpenOptions, io::Write as _};
use tracing::info;

mod client;
mod parser;
mod server;
mod utils;

use client::generate_client_code;
use server::generate_server_code;

const PROTOCOLS: &[&str] = &[
    // Core protocol
    "protocols/wayland/protocol/wayland.xml",

    // Stable protocols
    "protocols/wayland-protocols/stable/linux-dmabuf/linux-dmabuf-v1.xml",
    "protocols/wayland-protocols/stable/presentation-time/presentation-time.xml",
    "protocols/wayland-protocols/stable/tablet/tablet-v2.xml",
    "protocols/wayland-protocols/stable/viewporter/viewporter.xml",
    "protocols/wayland-protocols/stable/xdg-shell/xdg-shell.xml",

    // Staging protocols
    "protocols/wayland-protocols/staging/alpha-modifier/alpha-modifier-v1.xml",
    "protocols/wayland-protocols/staging/content-type/content-type-v1.xml",
    "protocols/wayland-protocols/staging/cursor-shape/cursor-shape-v1.xml",
    "protocols/wayland-protocols/staging/drm-lease/drm-lease-v1.xml",
    "protocols/wayland-protocols/staging/ext-foreign-toplevel-list/ext-foreign-toplevel-list-v1.xml",
    "protocols/wayland-protocols/staging/ext-idle-notify/ext-idle-notify-v1.xml",
    "protocols/wayland-protocols/staging/ext-image-capture-source/ext-image-capture-source-v1.xml",
    "protocols/wayland-protocols/staging/ext-image-copy-capture/ext-image-copy-capture-v1.xml",
    "protocols/wayland-protocols/staging/ext-session-lock/ext-session-lock-v1.xml",
    "protocols/wayland-protocols/staging/ext-transient-seat/ext-transient-seat-v1.xml",
    "protocols/wayland-protocols/staging/fractional-scale/fractional-scale-v1.xml",
    "protocols/wayland-protocols/staging/linux-drm-syncobj/linux-drm-syncobj-v1.xml",
    "protocols/wayland-protocols/staging/security-context/security-context-v1.xml",
    "protocols/wayland-protocols/staging/single-pixel-buffer/single-pixel-buffer-v1.xml",
    "protocols/wayland-protocols/staging/tearing-control/tearing-control-v1.xml",
    "protocols/wayland-protocols/staging/xdg-activation/xdg-activation-v1.xml",
    "protocols/wayland-protocols/staging/xdg-dialog/xdg-dialog-v1.xml",
    "protocols/wayland-protocols/staging/xdg-toplevel-drag/xdg-toplevel-drag-v1.xml",
    "protocols/wayland-protocols/staging/xdg-toplevel-icon/xdg-toplevel-icon-v1.xml",

    // Unstable protocols
    "protocols/wayland-protocols/unstable/fullscreen-shell/fullscreen-shell-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/idle-inhibit/idle-inhibit-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/input-method/input-method-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/input-timestamps/input-timestamps-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/keyboard-shortcuts-inhibit/keyboard-shortcuts-inhibit-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/linux-dmabuf/linux-dmabuf-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/linux-explicit-synchronization/linux-explicit-synchronization-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/pointer-constraints/pointer-constraints-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/pointer-gestures/pointer-gestures-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/primary-selection/primary-selection-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/relative-pointer/relative-pointer-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/tablet/tablet-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/tablet/tablet-unstable-v2.xml",
    "protocols/wayland-protocols/unstable/text-input/text-input-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/text-input/text-input-unstable-v3.xml",
    "protocols/wayland-protocols/unstable/xdg-decoration/xdg-decoration-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/xdg-foreign/xdg-foreign-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/xdg-foreign/xdg-foreign-unstable-v2.xml",
    "protocols/wayland-protocols/unstable/xdg-output/xdg-output-unstable-v1.xml",
    "protocols/wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v5.xml",
    "protocols/wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v6.xml",
    "protocols/wayland-protocols/unstable/xwayland-keyboard-grab/xwayland-keyboard-grab-unstable-v1.xml",

    // Wlroots protocols
    "protocols/wlr-protocols/unstable/wlr-data-control-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-export-dmabuf-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-foreign-toplevel-management-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-gamma-control-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-input-inhibitor-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-layer-shell-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-output-management-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-output-power-management-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-screencopy-unstable-v1.xml",
    "protocols/wlr-protocols/unstable/wlr-virtual-pointer-unstable-v1.xml",
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let Args { json } = Args::parse();

    let protocols = PROTOCOLS
        .iter()
        .map(Protocol::from_path)
        .collect::<Result<Vec<Protocol>>>()?;

    if json {
        info!("Generating json file");

        let mut json_path = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("protocols.json")?;

        serde_json::to_writer(&mut json_path, &protocols)?;
    } else {
        let mut server_path = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("src/server/protocol.rs")?;

        write!(&mut server_path, "{}", generate_server_code(&protocols))?;

        let mut client_path = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("src/client/protocol.rs")?;

        write!(&mut client_path, "{}", generate_client_code(&protocols))?;
    }

    Ok(())
}
