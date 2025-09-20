pub mod frog_color_management_v1 {
    pub mod frog_color_management_factory_v1 {
        pub trait FrogColorManagementFactoryV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "frog_color_management_factory_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod frog_color_managed_surface {
        pub trait FrogColorManagedSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "frog_color_managed_surface";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod frog_fifo_v1 {
    pub mod frog_fifo_manager_v1 {
        pub trait FrogFifoManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "frog_fifo_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod frog_fifo_surface_v1 {
        pub trait FrogFifoSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "frog_fifo_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
