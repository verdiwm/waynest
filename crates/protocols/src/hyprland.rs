pub mod hyprland_ctm_control_v1 {
    pub mod hyprland_ctm_control_manager_v1 {
        pub trait HyprlandCtmControlManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_ctm_control_manager_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod hyprland_focus_grab_v1 {
    pub mod hyprland_focus_grab_manager_v1 {
        pub trait HyprlandFocusGrabManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_focus_grab_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod hyprland_focus_grab_v1 {
        pub trait HyprlandFocusGrabV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_focus_grab_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod hyprland_global_shortcuts_v1 {
    pub mod hyprland_global_shortcuts_manager_v1 {
        pub trait HyprlandGlobalShortcutsManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_global_shortcuts_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod hyprland_global_shortcut_v1 {
        pub trait HyprlandGlobalShortcutV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_global_shortcut_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod hyprland_lock_notify_v1 {
    pub mod hyprland_lock_notifier_v1 {
        pub trait HyprlandLockNotifierV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_lock_notifier_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod hyprland_lock_notification_v1 {
        pub trait HyprlandLockNotificationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_lock_notification_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod hyprland_surface_v1 {
    pub mod hyprland_surface_manager_v1 {
        pub trait HyprlandSurfaceManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_surface_manager_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod hyprland_surface_v1 {
        pub trait HyprlandSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_surface_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod hyprland_toplevel_export_v1 {
    pub mod hyprland_toplevel_export_manager_v1 {
        pub trait HyprlandToplevelExportManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_toplevel_export_manager_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod hyprland_toplevel_export_frame_v1 {
        pub trait HyprlandToplevelExportFrameV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_toplevel_export_frame_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod hyprland_toplevel_mapping_v1 {
    pub mod hyprland_toplevel_mapping_manager_v1 {
        pub trait HyprlandToplevelMappingManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_toplevel_mapping_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod hyprland_toplevel_window_mapping_handle_v1 {
        pub trait HyprlandToplevelWindowMappingHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "hyprland_toplevel_window_mapping_handle_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
