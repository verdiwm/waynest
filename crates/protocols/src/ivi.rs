pub mod ivi_application {
    pub mod ivi_surface {
        pub trait IviSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_surface";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ivi_application {
        pub trait IviApplication<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_application";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ivi_input {
    pub mod ivi_input {
        pub trait IviInput<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_input";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod ivi_wm {
    pub mod ivi_wm_screen {
        pub trait IviWmScreen<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_wm_screen";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod ivi_screenshot {
        pub trait IviScreenshot<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_screenshot";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod ivi_wm {
        pub trait IviWm<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_wm";
            const VERSION: u32 = 2u32;
        }
    }
}
