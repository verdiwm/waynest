use anyhow::Result;
use clap::Parser;
use parser::Pair;
use std::{fs::OpenOptions, io::Write as _};
use tracing::info;

mod client;
mod parser;
mod server;
mod utils;

use client::generate_client_code;
use server::generate_server_code;

const PROTOCOLS: [(&str, &[&str]); 8] = [
    ("core", &["protocols/wayland/protocol/wayland.xml"]),
    (
        "stable",
        &[
            "protocols/wayland-protocols/stable/linux-dmabuf/linux-dmabuf-v1.xml",
            "protocols/wayland-protocols/stable/presentation-time/presentation-time.xml",
            "protocols/wayland-protocols/stable/tablet/tablet-v2.xml",
            "protocols/wayland-protocols/stable/viewporter/viewporter.xml",
            "protocols/wayland-protocols/stable/xdg-shell/xdg-shell.xml",
        ],
    ),
    (
        "staging", 
        &[
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
            "protocols/wayland-protocols/staging/xdg-toplevel-icon/xdg-toplevel-icon-v1.xml"
        ]
    ),
    (
        "unstable", 
        &[
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
            "protocols/wayland-protocols/unstable/xwayland-keyboard-grab/xwayland-keyboard-grab-unstable-v1.xml"
        ]
    ),
    (
        "wlr",
        &[
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
        ],
    ),
    (
        "plasma",
        &[
            "protocols/plasma-wayland-protocols/src/protocols/appmenu.xml",
            "protocols/plasma-wayland-protocols/src/protocols/blur.xml",
            "protocols/plasma-wayland-protocols/src/protocols/contrast.xml",
            "protocols/plasma-wayland-protocols/src/protocols/fullscreen-shell.xml",
            "protocols/plasma-wayland-protocols/src/protocols/idle.xml",
            "protocols/plasma-wayland-protocols/src/protocols/keystate.xml",
            "protocols/plasma-wayland-protocols/src/protocols/output-management.xml",
            "protocols/plasma-wayland-protocols/src/protocols/outputdevice.xml",
            "protocols/plasma-wayland-protocols/src/protocols/remote-access.xml",
            "protocols/plasma-wayland-protocols/src/protocols/server-decoration-palette.xml",
            "protocols/plasma-wayland-protocols/src/protocols/server-decoration.xml",
            "protocols/plasma-wayland-protocols/src/protocols/shadow.xml",
            "protocols/plasma-wayland-protocols/src/protocols/slide.xml",
            "protocols/plasma-wayland-protocols/src/protocols/surface-extension.xml",
            "protocols/plasma-wayland-protocols/src/protocols/text-input-unstable-v2.xml",
            "protocols/plasma-wayland-protocols/src/protocols/text-input.xml",
            "protocols/plasma-wayland-protocols/src/protocols/wayland-eglstream-controller.xml",
            "protocols/plasma-wayland-protocols/src/protocols/dpms.xml",
            "protocols/plasma-wayland-protocols/src/protocols/fake-input.xml",
            "protocols/plasma-wayland-protocols/src/protocols/kde-lockscreen-overlay-v1.xml",
            "protocols/plasma-wayland-protocols/src/protocols/kde-output-device-v2.xml",
            "protocols/plasma-wayland-protocols/src/protocols/kde-output-management-v2.xml",
            "protocols/plasma-wayland-protocols/src/protocols/kde-output-order-v1.xml",
            "protocols/plasma-wayland-protocols/src/protocols/kde-primary-output-v1.xml",
            "protocols/plasma-wayland-protocols/src/protocols/kde-screen-edge-v1.xml",
            "protocols/plasma-wayland-protocols/src/protocols/org-kde-plasma-virtual-desktop.xml",
            "protocols/plasma-wayland-protocols/src/protocols/plasma-shell.xml",
            "protocols/plasma-wayland-protocols/src/protocols/plasma-window-management.xml",
            "protocols/plasma-wayland-protocols/src/protocols/zkde-screencast-unstable-v1.xml",
        ],
    ),
    (
        "weston",
        &[
            "protocols/weston/protocol/color-management-v1.xml",
            "protocols/weston/protocol/ivi-application.xml",
            "protocols/weston/protocol/ivi-hmi-controller.xml",
            "protocols/weston/protocol/text-cursor-position.xml",
            "protocols/weston/protocol/weston-content-protection.xml",
            "protocols/weston/protocol/weston-debug.xml",
            "protocols/weston/protocol/weston-desktop-shell.xml",
            "protocols/weston/protocol/weston-direct-display.xml",
            "protocols/weston/protocol/weston-output-capture.xml",
            "protocols/weston/protocol/weston-test.xml",
            "protocols/weston/protocol/weston-touch-calibration.xml",
        ],
    ),
    (
        "cosmic",
        &[
            "protocols/cosmic-protocols/unstable/cosmic-image-source-unstable-v1.xml",
            "protocols/cosmic-protocols/unstable/cosmic-output-management-unstable-v1.xml",
            "protocols/cosmic-protocols/unstable/cosmic-overlap-notify-unstable-v1.xml",
            "protocols/cosmic-protocols/unstable/cosmic-screencopy-unstable-v2.xml",
            "protocols/cosmic-protocols/unstable/cosmic-toplevel-info-unstable-v1.xml",
            "protocols/cosmic-protocols/unstable/cosmic-toplevel-management-unstable-v1.xml",
            "protocols/cosmic-protocols/unstable/cosmic-workspace-unstable-v1.xml",
        ],
    ),
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

    let mut protocols = Vec::new();
    let mut pairs = Vec::new();

    for (module, protos) in PROTOCOLS {
        let current = protos
            .into_iter()
            .map(|path| Pair::from_path(module, path))
            .collect::<Result<Vec<Pair>>>()?;

        pairs.extend(current.clone());
        protocols.push((module, current))
    }

    if json {
        info!("Generating json file");

        let mut json_path = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open("protocols.json")?;

        serde_json::to_writer(&mut json_path, &protocols)?;
    } else {
        for (module, current) in protocols {
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
        }

        let module_content = r#"
mod core;
#[cfg(feature = "stable")]
mod stable;
#[cfg(feature = "staging")]
mod staging;
#[cfg(feature = "unstable")]
mod unstable;
#[cfg(feature = "wlr")]
mod wlr;
#[cfg(feature = "plasma")]
mod plasma;
#[cfg(feature = "weston")]
mod weston;
#[cfg(feature = "cosmic")]
mod cosmic;
"#;

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
