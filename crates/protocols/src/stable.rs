pub mod linux_dmabuf_v1 {
    pub mod zwp_linux_dmabuf_v1 {
        pub trait ZwpLinuxDmabufV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_v1";
            const VERSION: u32 = 5u32;
        }
    }
    pub mod zwp_linux_buffer_params_v1 {
        pub trait ZwpLinuxBufferParamsV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_linux_buffer_params_v1";
            const VERSION: u32 = 5u32;
        }
    }
    pub mod zwp_linux_dmabuf_feedback_v1 {
        pub trait ZwpLinuxDmabufFeedbackV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_linux_dmabuf_feedback_v1";
            const VERSION: u32 = 5u32;
        }
    }
}
pub mod presentation_time {
    pub mod wp_presentation {
        pub trait WpPresentation<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_presentation";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod wp_presentation_feedback {
        pub trait WpPresentationFeedback<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_presentation_feedback";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod tablet_v2 {
    pub mod zwp_tablet_manager_v2 {
        pub trait ZwpTabletManagerV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_manager_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_seat_v2 {
        pub trait ZwpTabletSeatV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_seat_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_tool_v2 {
        pub trait ZwpTabletToolV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_tool_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_v2 {
        pub trait ZwpTabletV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_pad_ring_v2 {
        pub trait ZwpTabletPadRingV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_pad_strip_v2 {
        pub trait ZwpTabletPadStripV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_pad_group_v2 {
        pub trait ZwpTabletPadGroupV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_pad_v2 {
        pub trait ZwpTabletPadV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_v2";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_tablet_pad_dial_v2 {
        pub trait ZwpTabletPadDialV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_dial_v2";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod viewporter {
    pub mod wp_viewporter {
        pub trait WpViewporter<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_viewporter";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_viewport {
        pub trait WpViewport<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_viewport";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_shell {
    pub mod xdg_wm_base {
        pub trait XdgWmBase<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_wm_base";
            const VERSION: u32 = 7u32;
        }
    }
    pub mod xdg_positioner {
        pub trait XdgPositioner<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_positioner";
            const VERSION: u32 = 7u32;
        }
    }
    pub mod xdg_surface {
        pub trait XdgSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_surface";
            const VERSION: u32 = 7u32;
        }
    }
    pub mod xdg_toplevel {
        pub trait XdgToplevel<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_toplevel";
            const VERSION: u32 = 7u32;
        }
    }
    pub mod xdg_popup {
        pub trait XdgPopup<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_popup";
            const VERSION: u32 = 7u32;
        }
    }
}
