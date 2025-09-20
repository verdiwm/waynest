pub mod wlr_data_control_unstable_v1 {
    pub mod zwlr_data_control_manager_v1 {
        pub trait ZwlrDataControlManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_data_control_manager_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwlr_data_control_device_v1 {
        pub trait ZwlrDataControlDeviceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_data_control_device_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwlr_data_control_source_v1 {
        pub trait ZwlrDataControlSourceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_data_control_source_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwlr_data_control_offer_v1 {
        pub trait ZwlrDataControlOfferV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_data_control_offer_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod wlr_export_dmabuf_unstable_v1 {
    pub mod zwlr_export_dmabuf_manager_v1 {
        pub trait ZwlrExportDmabufManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_export_dmabuf_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwlr_export_dmabuf_frame_v1 {
        pub trait ZwlrExportDmabufFrameV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_export_dmabuf_frame_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod wlr_foreign_toplevel_management_unstable_v1 {
    pub mod zwlr_foreign_toplevel_manager_v1 {
        pub trait ZwlrForeignToplevelManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_foreign_toplevel_manager_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zwlr_foreign_toplevel_handle_v1 {
        pub trait ZwlrForeignToplevelHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_foreign_toplevel_handle_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod wlr_gamma_control_unstable_v1 {
    pub mod zwlr_gamma_control_manager_v1 {
        pub trait ZwlrGammaControlManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_gamma_control_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwlr_gamma_control_v1 {
        pub trait ZwlrGammaControlV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_gamma_control_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod wlr_input_inhibit_unstable_v1 {
    pub mod zwlr_input_inhibit_manager_v1 {
        pub trait ZwlrInputInhibitManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_input_inhibit_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwlr_input_inhibitor_v1 {
        pub trait ZwlrInputInhibitorV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_input_inhibitor_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod wlr_layer_shell_unstable_v1 {
    pub mod zwlr_layer_shell_v1 {
        pub trait ZwlrLayerShellV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_layer_shell_v1";
            const VERSION: u32 = 5u32;
        }
    }
    pub mod zwlr_layer_surface_v1 {
        pub trait ZwlrLayerSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_layer_surface_v1";
            const VERSION: u32 = 5u32;
        }
    }
}
pub mod wlr_output_management_unstable_v1 {
    pub mod zwlr_output_manager_v1 {
        pub trait ZwlrOutputManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_manager_v1";
            const VERSION: u32 = 4u32;
        }
    }
    pub mod zwlr_output_head_v1 {
        pub trait ZwlrOutputHeadV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_head_v1";
            const VERSION: u32 = 4u32;
        }
    }
    pub mod zwlr_output_mode_v1 {
        pub trait ZwlrOutputModeV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_mode_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zwlr_output_configuration_v1 {
        pub trait ZwlrOutputConfigurationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_configuration_v1";
            const VERSION: u32 = 4u32;
        }
    }
    pub mod zwlr_output_configuration_head_v1 {
        pub trait ZwlrOutputConfigurationHeadV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_configuration_head_v1";
            const VERSION: u32 = 4u32;
        }
    }
}
pub mod wlr_output_power_management_unstable_v1 {
    pub mod zwlr_output_power_manager_v1 {
        pub trait ZwlrOutputPowerManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_power_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwlr_output_power_v1 {
        pub trait ZwlrOutputPowerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_output_power_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod wlr_screencopy_unstable_v1 {
    pub mod zwlr_screencopy_manager_v1 {
        pub trait ZwlrScreencopyManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_screencopy_manager_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zwlr_screencopy_frame_v1 {
        pub trait ZwlrScreencopyFrameV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_screencopy_frame_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod wlr_virtual_pointer_unstable_v1 {
    pub mod zwlr_virtual_pointer_v1 {
        pub trait ZwlrVirtualPointerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_virtual_pointer_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwlr_virtual_pointer_manager_v1 {
        pub trait ZwlrVirtualPointerManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwlr_virtual_pointer_manager_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
