pub mod drm {
    pub mod wl_drm {
        pub trait WlDrm<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wl_drm";
            const VERSION: u32 = 2u32;
        }
    }
}
