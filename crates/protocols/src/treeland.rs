pub mod treeland_dde_shell_v1 {
    pub mod treeland_dde_shell_manager_v1 {
        pub trait TreelandDdeShellManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_dde_shell_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_window_overlap_checker {
        pub trait TreelandWindowOverlapChecker<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_window_overlap_checker";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_dde_shell_surface_v1 {
        pub trait TreelandDdeShellSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_dde_shell_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_dde_active_v1 {
        pub trait TreelandDdeActiveV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_dde_active_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_multitaskview_v1 {
        pub trait TreelandMultitaskviewV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_multitaskview_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_window_picker_v1 {
        pub trait TreelandWindowPickerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_window_picker_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_lockscreen_v1 {
        pub trait TreelandLockscreenV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_lockscreen_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_ddm {
    pub mod treeland_ddm {
        pub trait TreelandDdm<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_ddm";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_foreign_toplevel_manager_v1 {
    pub mod treeland_foreign_toplevel_manager_v1 {
        pub trait TreelandForeignToplevelManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_foreign_toplevel_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_foreign_toplevel_handle_v1 {
        pub trait TreelandForeignToplevelHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_foreign_toplevel_handle_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_dock_preview_context_v1 {
        pub trait TreelandDockPreviewContextV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_dock_preview_context_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_output_manager_v1 {
    pub mod treeland_output_manager_v1 {
        pub trait TreelandOutputManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_output_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_shortcut_manager_v1 {
    pub mod treeland_shortcut_manager_v1 {
        pub trait TreelandShortcutManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_shortcut_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_shortcut_context_v1 {
        pub trait TreelandShortcutContextV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_shortcut_context_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_virtual_output_manager_v1 {
    pub mod treeland_virtual_output_manager_v1 {
        pub trait TreelandVirtualOutputManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_virtual_output_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod treeland_virtual_output_v1 {
        pub trait TreelandVirtualOutputV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_virtual_output_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_wallpaper_color_v1 {
    pub mod treeland_wallpaper_color_manager_v1 {
        pub trait TreelandWallpaperColorManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_wallpaper_color_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod treeland_window_management_v1 {
    pub mod treeland_window_management_v1 {
        pub trait TreelandWindowManagementV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "treeland_window_management_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
