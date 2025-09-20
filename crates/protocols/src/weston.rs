pub mod color_management_v1 {
    pub mod xx_color_manager_v4 {
        pub trait XxColorManagerV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_color_manager_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_color_management_output_v4 {
        pub trait XxColorManagementOutputV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_color_management_output_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_color_management_surface_v4 {
        pub trait XxColorManagementSurfaceV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_color_management_surface_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_color_management_feedback_surface_v4 {
        pub trait XxColorManagementFeedbackSurfaceV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_color_management_feedback_surface_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_image_description_creator_icc_v4 {
        pub trait XxImageDescriptionCreatorIccV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_image_description_creator_icc_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_image_description_creator_params_v4 {
        pub trait XxImageDescriptionCreatorParamsV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_image_description_creator_params_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_image_description_v4 {
        pub trait XxImageDescriptionV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_image_description_v4";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xx_image_description_info_v4 {
        pub trait XxImageDescriptionInfoV4<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xx_image_description_info_v4";
            const VERSION: u32 = 1u32;
        }
    }
}
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
pub mod ivi_hmi_controller {
    pub mod ivi_hmi_controller {
        pub trait IviHmiController<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ivi_hmi_controller";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod text_cursor_position {
    pub mod text_cursor_position {
        pub trait TextCursorPosition<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "text_cursor_position";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_content_protection {
    pub mod weston_content_protection {
        pub trait WestonContentProtection<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_content_protection";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_protected_surface {
        pub trait WestonProtectedSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_protected_surface";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_debug {
    pub mod weston_debug_v1 {
        pub trait WestonDebugV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_debug_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_debug_stream_v1 {
        pub trait WestonDebugStreamV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_debug_stream_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_desktop {
    pub mod weston_desktop_shell {
        pub trait WestonDesktopShell<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_desktop_shell";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_screensaver {
        pub trait WestonScreensaver<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_screensaver";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_direct_display {
    pub mod weston_direct_display_v1 {
        pub trait WestonDirectDisplayV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_direct_display_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_output_capture {
    pub mod weston_capture_v1 {
        pub trait WestonCaptureV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_capture_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_capture_source_v1 {
        pub trait WestonCaptureSourceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_capture_source_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_test {
    pub mod weston_test {
        pub trait WestonTest<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_test";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_test_runner {
        pub trait WestonTestRunner<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_test_runner";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod weston_touch_calibration {
    pub mod weston_touch_calibration {
        pub trait WestonTouchCalibration<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_touch_calibration";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_touch_calibrator {
        pub trait WestonTouchCalibrator<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_touch_calibrator";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod weston_touch_coordinate {
        pub trait WestonTouchCoordinate<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "weston_touch_coordinate";
            const VERSION: u32 = 1u32;
        }
    }
}
