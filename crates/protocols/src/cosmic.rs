pub mod cosmic_a11y_v1 {
    pub mod cosmic_a11y_manager_v1 {
        pub trait CosmicA11yManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "cosmic_a11y_manager_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod cosmic_atspi_v1 {
    pub mod cosmic_atspi_manager_v1 {
        pub trait CosmicAtspiManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "cosmic_atspi_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod cosmic_image_source_unstable_v1 {
    pub mod zcosmic_workspace_image_capture_source_manager_v1 {
        pub trait ZcosmicWorkspaceImageCaptureSourceManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_workspace_image_capture_source_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod cosmic_output_management_unstable_v1 {
    pub mod zcosmic_output_manager_v1 {
        pub trait ZcosmicOutputManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_output_manager_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zcosmic_output_head_v1 {
        pub trait ZcosmicOutputHeadV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_output_head_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zcosmic_output_configuration_v1 {
        pub trait ZcosmicOutputConfigurationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_output_configuration_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zcosmic_output_configuration_head_v1 {
        pub trait ZcosmicOutputConfigurationHeadV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_output_configuration_head_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod cosmic_overlap_notify_unstable_v1 {
    pub mod zcosmic_overlap_notify_v1 {
        pub trait ZcosmicOverlapNotifyV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_overlap_notify_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zcosmic_overlap_notification_v1 {
        pub trait ZcosmicOverlapNotificationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_overlap_notification_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod cosmic_screencopy_unstable_v2 {
    pub mod zcosmic_screencopy_manager_v2 {
        pub trait ZcosmicScreencopyManagerV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_screencopy_manager_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zcosmic_screencopy_session_v2 {
        pub trait ZcosmicScreencopySessionV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_screencopy_session_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zcosmic_screencopy_frame_v2 {
        pub trait ZcosmicScreencopyFrameV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_screencopy_frame_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zcosmic_screencopy_cursor_session_v2 {
        pub trait ZcosmicScreencopyCursorSessionV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_screencopy_cursor_session_v2";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod cosmic_toplevel_info_unstable_v1 {
    pub mod zcosmic_toplevel_info_v1 {
        pub trait ZcosmicToplevelInfoV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_toplevel_info_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zcosmic_toplevel_handle_v1 {
        pub trait ZcosmicToplevelHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_toplevel_handle_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod cosmic_toplevel_management_unstable_v1 {
    pub mod zcosmic_toplevel_manager_v1 {
        pub trait ZcosmicToplevelManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_toplevel_manager_v1";
            const VERSION: u32 = 4u32;
        }
    }
}
pub mod cosmic_workspace_unstable_v2 {
    pub mod zcosmic_workspace_manager_v2 {
        pub trait ZcosmicWorkspaceManagerV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_workspace_manager_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zcosmic_workspace_handle_v2 {
        pub trait ZcosmicWorkspaceHandleV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zcosmic_workspace_handle_v2";
            const VERSION: u32 = 2u32;
        }
    }
}
