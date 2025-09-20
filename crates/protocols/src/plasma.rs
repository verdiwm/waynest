pub mod appmenu {
    pub mod org_kde_kwin_appmenu_manager {
        pub trait OrgKdeKwinAppmenuManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_appmenu_manager";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod org_kde_kwin_appmenu {
        pub trait OrgKdeKwinAppmenu<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_appmenu";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod blur {
    pub mod org_kde_kwin_blur_manager {
        pub trait OrgKdeKwinBlurManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_blur_manager";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_blur {
        pub trait OrgKdeKwinBlur<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_blur";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod contrast {
    pub mod org_kde_kwin_contrast_manager {
        pub trait OrgKdeKwinContrastManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_contrast_manager";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod org_kde_kwin_contrast {
        pub trait OrgKdeKwinContrast<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_contrast";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod dpms {
    pub mod org_kde_kwin_dpms_manager {
        pub trait OrgKdeKwinDpmsManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_dpms_manager";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_dpms {
        pub trait OrgKdeKwinDpms<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_dpms";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod fake_input {
    pub mod org_kde_kwin_fake_input {
        pub trait OrgKdeKwinFakeInput<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_fake_input";
            const VERSION: u32 = 6u32;
        }
    }
}
pub mod fullscreen_shell {
    pub mod _wl_fullscreen_shell {
        pub trait WlFullscreenShell<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "_wl_fullscreen_shell";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod _wl_fullscreen_shell_mode_feedback {
        pub trait WlFullscreenShellModeFeedback<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "_wl_fullscreen_shell_mode_feedback";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod idle {
    pub mod org_kde_kwin_idle {
        pub trait OrgKdeKwinIdle<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_idle";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_idle_timeout {
        pub trait OrgKdeKwinIdleTimeout<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_idle_timeout";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod kde_external_brightness_v1 {
    pub mod kde_external_brightness_v1 {
        pub trait KdeExternalBrightnessV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_external_brightness_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod kde_external_brightness_device_v1 {
        pub trait KdeExternalBrightnessDeviceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_external_brightness_device_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod kde_lockscreen_overlay_v1 {
    pub mod kde_lockscreen_overlay_v1 {
        pub trait KdeLockscreenOverlayV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_lockscreen_overlay_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod kde_output_device_v2 {
    pub mod kde_output_device_v2 {
        pub trait KdeOutputDeviceV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_output_device_v2";
            const VERSION: u32 = 17u32;
        }
    }
    pub mod kde_output_device_mode_v2 {
        pub trait KdeOutputDeviceModeV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_output_device_mode_v2";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod kde_output_management_v2 {
    pub mod kde_output_management_v2 {
        pub trait KdeOutputManagementV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_output_management_v2";
            const VERSION: u32 = 17u32;
        }
    }
    pub mod kde_output_configuration_v2 {
        pub trait KdeOutputConfigurationV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_output_configuration_v2";
            const VERSION: u32 = 17u32;
        }
    }
}
pub mod kde_output_order_v1 {
    pub mod kde_output_order_v1 {
        pub trait KdeOutputOrderV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_output_order_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod kde_primary_output_v1 {
    pub mod kde_primary_output_v1 {
        pub trait KdePrimaryOutputV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_primary_output_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod kde_screen_edge_v1 {
    pub mod kde_screen_edge_manager_v1 {
        pub trait KdeScreenEdgeManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_screen_edge_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod kde_auto_hide_screen_edge_v1 {
        pub trait KdeAutoHideScreenEdgeV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "kde_auto_hide_screen_edge_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod keystate {
    pub mod org_kde_kwin_keystate {
        pub trait OrgKdeKwinKeystate<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_keystate";
            const VERSION: u32 = 5u32;
        }
    }
}
pub mod org_kde_plasma_virtual_desktop {
    pub mod org_kde_plasma_virtual_desktop_management {
        pub trait OrgKdePlasmaVirtualDesktopManagement<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_virtual_desktop_management";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod org_kde_plasma_virtual_desktop {
        pub trait OrgKdePlasmaVirtualDesktop<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_virtual_desktop";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod outputmanagement {
    pub mod org_kde_kwin_outputmanagement {
        pub trait OrgKdeKwinOutputmanagement<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_outputmanagement";
            const VERSION: u32 = 4u32;
        }
    }
    pub mod org_kde_kwin_outputconfiguration {
        pub trait OrgKdeKwinOutputconfiguration<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_outputconfiguration";
            const VERSION: u32 = 4u32;
        }
    }
}
pub mod org_kde_kwin_outputdevice {
    pub mod org_kde_kwin_outputdevice {
        pub trait OrgKdeKwinOutputdevice<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_outputdevice";
            const VERSION: u32 = 4u32;
        }
    }
}
pub mod plasma_shell {
    pub mod org_kde_plasma_shell {
        pub trait OrgKdePlasmaShell<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_shell";
            const VERSION: u32 = 8u32;
        }
    }
    pub mod org_kde_plasma_surface {
        pub trait OrgKdePlasmaSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_surface";
            const VERSION: u32 = 8u32;
        }
    }
}
pub mod plasma_window_management {
    pub mod org_kde_plasma_window_management {
        pub trait OrgKdePlasmaWindowManagement<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_window_management";
            const VERSION: u32 = 19u32;
        }
    }
    pub mod org_kde_plasma_window {
        pub trait OrgKdePlasmaWindow<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_window";
            const VERSION: u32 = 18u32;
        }
    }
    pub mod org_kde_plasma_activation_feedback {
        pub trait OrgKdePlasmaActivationFeedback<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_activation_feedback";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_plasma_activation {
        pub trait OrgKdePlasmaActivation<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_activation";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_plasma_stacking_order {
        pub trait OrgKdePlasmaStackingOrder<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_plasma_stacking_order";
            const VERSION: u32 = 17u32;
        }
    }
}
pub mod remote_access {
    pub mod org_kde_kwin_remote_access_manager {
        pub trait OrgKdeKwinRemoteAccessManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_remote_access_manager";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_remote_buffer {
        pub trait OrgKdeKwinRemoteBuffer<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_remote_buffer";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod server_decoration_palette {
    pub mod org_kde_kwin_server_decoration_palette_manager {
        pub trait OrgKdeKwinServerDecorationPaletteManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_palette_manager";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_server_decoration_palette {
        pub trait OrgKdeKwinServerDecorationPalette<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_palette";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod server_decoration {
    pub mod org_kde_kwin_server_decoration_manager {
        pub trait OrgKdeKwinServerDecorationManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration_manager";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_server_decoration {
        pub trait OrgKdeKwinServerDecoration<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_server_decoration";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod shadow {
    pub mod org_kde_kwin_shadow_manager {
        pub trait OrgKdeKwinShadowManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_shadow_manager";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod org_kde_kwin_shadow {
        pub trait OrgKdeKwinShadow<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_shadow";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod slide {
    pub mod org_kde_kwin_slide_manager {
        pub trait OrgKdeKwinSlideManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_slide_manager";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod org_kde_kwin_slide {
        pub trait OrgKdeKwinSlide<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "org_kde_kwin_slide";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod surface_extension {
    pub mod qt_surface_extension {
        pub trait QtSurfaceExtension<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "qt_surface_extension";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod qt_extended_surface {
        pub trait QtExtendedSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "qt_extended_surface";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod text_input_unstable_v2 {
    pub mod zwp_text_input_v2 {
        pub trait ZwpTextInputV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_text_input_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_text_input_manager_v2 {
        pub trait ZwpTextInputManagerV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_text_input_manager_v2";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod text {
    pub mod wl_text_input {
        pub trait WlTextInput<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_text_input";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_text_input_manager {
        pub trait WlTextInputManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_text_input_manager";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod wl_eglstream_controller {
    pub mod wl_eglstream_controller {
        pub trait WlEglstreamController<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_eglstream_controller";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod zkde_screencast_unstable_v1 {
    pub mod zkde_screencast_unstable_v1 {
        pub trait ZkdeScreencastUnstableV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zkde_screencast_unstable_v1";
            const VERSION: u32 = 5u32;
        }
    }
    pub mod zkde_screencast_stream_unstable_v1 {
        pub trait ZkdeScreencastStreamUnstableV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zkde_screencast_stream_unstable_v1";
            const VERSION: u32 = 5u32;
        }
    }
}
