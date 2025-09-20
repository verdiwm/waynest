pub mod wayland {
    pub mod wl_display {
        pub trait WlDisplay<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_display";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_registry {
        pub trait WlRegistry<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_registry";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_callback {
        pub trait WlCallback<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_callback";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_compositor {
        pub trait WlCompositor<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_compositor";
            const VERSION: u32 = 6u32;
        }
    }
    pub mod wl_shm_pool {
        pub trait WlShmPool<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_shm_pool";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod wl_shm {
        pub trait WlShm<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_shm";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod wl_buffer {
        pub trait WlBuffer<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_buffer";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_data_offer {
        pub trait WlDataOffer<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_data_offer";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod wl_data_source {
        pub trait WlDataSource<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_data_source";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod wl_data_device {
        pub trait WlDataDevice<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_data_device";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod wl_data_device_manager {
        pub trait WlDataDeviceManager<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_data_device_manager";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod wl_shell {
        pub trait WlShell<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_shell";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_shell_surface {
        pub trait WlShellSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_shell_surface";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_surface {
        pub trait WlSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_surface";
            const VERSION: u32 = 6u32;
        }
    }
    pub mod wl_seat {
        pub trait WlSeat<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_seat";
            const VERSION: u32 = 10u32;
        }
    }
    pub mod wl_pointer {
        pub trait WlPointer<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_pointer";
            const VERSION: u32 = 10u32;
        }
    }
    pub mod wl_keyboard {
        pub trait WlKeyboard<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_keyboard";
            const VERSION: u32 = 10u32;
        }
    }
    pub mod wl_touch {
        pub trait WlTouch<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_touch";
            const VERSION: u32 = 10u32;
        }
    }
    pub mod wl_output {
        pub trait WlOutput<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_output";
            const VERSION: u32 = 4u32;
        }
    }
    pub mod wl_region {
        pub trait WlRegion<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_region";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_subcompositor {
        pub trait WlSubcompositor<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_subcompositor";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_subsurface {
        pub trait WlSubsurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_subsurface";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wl_fixes {
        pub trait WlFixes<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_fixes";
            const VERSION: u32 = 1u32;
        }
    }
}
