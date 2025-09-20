pub mod alpha_modifier_v1 {
    pub mod wp_alpha_modifier_v1 {
        pub trait WpAlphaModifierV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_alpha_modifier_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_alpha_modifier_surface_v1 {
        pub trait WpAlphaModifierSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_alpha_modifier_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod color_management_v1 {
    pub mod wp_color_manager_v1 {
        pub trait WpColorManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_color_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_color_management_output_v1 {
        pub trait WpColorManagementOutputV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_color_management_output_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_color_management_surface_v1 {
        pub trait WpColorManagementSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_color_management_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_color_management_surface_feedback_v1 {
        pub trait WpColorManagementSurfaceFeedbackV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_color_management_surface_feedback_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_image_description_creator_icc_v1 {
        pub trait WpImageDescriptionCreatorIccV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_image_description_creator_icc_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_image_description_creator_params_v1 {
        pub trait WpImageDescriptionCreatorParamsV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_image_description_creator_params_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_image_description_v1 {
        pub trait WpImageDescriptionV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_image_description_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_image_description_info_v1 {
        pub trait WpImageDescriptionInfoV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_image_description_info_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod color_representation_v1 {
    pub mod wp_color_representation_manager_v1 {
        pub trait WpColorRepresentationManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_color_representation_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_color_representation_surface_v1 {
        pub trait WpColorRepresentationSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_color_representation_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod commit_timing_v1 {
    pub mod wp_commit_timing_manager_v1 {
        pub trait WpCommitTimingManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_commit_timing_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_commit_timer_v1 {
        pub trait WpCommitTimerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_commit_timer_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod content_type_v1 {
    pub mod wp_content_type_manager_v1 {
        pub trait WpContentTypeManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_content_type_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_content_type_v1 {
        pub trait WpContentTypeV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_content_type_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod cursor_shape_v1 {
    pub mod wp_cursor_shape_manager_v1 {
        pub trait WpCursorShapeManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_cursor_shape_manager_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod wp_cursor_shape_device_v1 {
        pub trait WpCursorShapeDeviceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_cursor_shape_device_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod drm_lease_v1 {
    pub mod wp_drm_lease_device_v1 {
        pub trait WpDrmLeaseDeviceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_drm_lease_device_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_drm_lease_connector_v1 {
        pub trait WpDrmLeaseConnectorV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_drm_lease_connector_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_drm_lease_request_v1 {
        pub trait WpDrmLeaseRequestV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_drm_lease_request_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_drm_lease_v1 {
        pub trait WpDrmLeaseV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_drm_lease_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_background_effect_v1 {
    pub mod ext_background_effect_manager_v1 {
        pub trait ExtBackgroundEffectManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_background_effect_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_background_effect_surface_v1 {
        pub trait ExtBackgroundEffectSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_background_effect_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_data_control_v1 {
    pub mod ext_data_control_manager_v1 {
        pub trait ExtDataControlManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_data_control_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_data_control_device_v1 {
        pub trait ExtDataControlDeviceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_data_control_device_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_data_control_source_v1 {
        pub trait ExtDataControlSourceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_data_control_source_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_data_control_offer_v1 {
        pub trait ExtDataControlOfferV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_data_control_offer_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_foreign_toplevel_list_v1 {
    pub mod ext_foreign_toplevel_list_v1 {
        pub trait ExtForeignToplevelListV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_foreign_toplevel_list_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_foreign_toplevel_handle_v1 {
        pub trait ExtForeignToplevelHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_foreign_toplevel_handle_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_idle_notify_v1 {
    pub mod ext_idle_notifier_v1 {
        pub trait ExtIdleNotifierV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_idle_notifier_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod ext_idle_notification_v1 {
        pub trait ExtIdleNotificationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_idle_notification_v1";
            const VERSION: u32 = 2u32;
        }
    }
}
pub mod ext_image_capture_source_v1 {
    pub mod ext_image_capture_source_v1 {
        pub trait ExtImageCaptureSourceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_image_capture_source_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_output_image_capture_source_manager_v1 {
        pub trait ExtOutputImageCaptureSourceManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_output_image_capture_source_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_foreign_toplevel_image_capture_source_manager_v1 {
        pub trait ExtForeignToplevelImageCaptureSourceManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_foreign_toplevel_image_capture_source_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_image_copy_capture_v1 {
    pub mod ext_image_copy_capture_manager_v1 {
        pub trait ExtImageCopyCaptureManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_image_copy_capture_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_image_copy_capture_session_v1 {
        pub trait ExtImageCopyCaptureSessionV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_image_copy_capture_session_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_image_copy_capture_frame_v1 {
        pub trait ExtImageCopyCaptureFrameV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_image_copy_capture_frame_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_image_copy_capture_cursor_session_v1 {
        pub trait ExtImageCopyCaptureCursorSessionV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_image_copy_capture_cursor_session_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_session_lock_v1 {
    pub mod ext_session_lock_manager_v1 {
        pub trait ExtSessionLockManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_session_lock_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_session_lock_v1 {
        pub trait ExtSessionLockV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_session_lock_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_session_lock_surface_v1 {
        pub trait ExtSessionLockSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_session_lock_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_transient_seat_v1 {
    pub mod ext_transient_seat_manager_v1 {
        pub trait ExtTransientSeatManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_transient_seat_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_transient_seat_v1 {
        pub trait ExtTransientSeatV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_transient_seat_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod ext_workspace_v1 {
    pub mod ext_workspace_manager_v1 {
        pub trait ExtWorkspaceManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_workspace_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_workspace_group_handle_v1 {
        pub trait ExtWorkspaceGroupHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_workspace_group_handle_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod ext_workspace_handle_v1 {
        pub trait ExtWorkspaceHandleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "ext_workspace_handle_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod fifo_v1 {
    pub mod wp_fifo_manager_v1 {
        pub trait WpFifoManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_fifo_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_fifo_v1 {
        pub trait WpFifoV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_fifo_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod fractional_scale_v1 {
    pub mod wp_fractional_scale_manager_v1 {
        pub trait WpFractionalScaleManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_fractional_scale_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_fractional_scale_v1 {
        pub trait WpFractionalScaleV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_fractional_scale_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod linux_drm_syncobj_v1 {
    pub mod wp_linux_drm_syncobj_manager_v1 {
        pub trait WpLinuxDrmSyncobjManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_linux_drm_syncobj_timeline_v1 {
        pub trait WpLinuxDrmSyncobjTimelineV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_timeline_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_linux_drm_syncobj_surface_v1 {
        pub trait WpLinuxDrmSyncobjSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_linux_drm_syncobj_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod pointer_warp_v1 {
    pub mod wp_pointer_warp_v1 {
        pub trait WpPointerWarpV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_pointer_warp_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod security_context_v1 {
    pub mod wp_security_context_manager_v1 {
        pub trait WpSecurityContextManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_security_context_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_security_context_v1 {
        pub trait WpSecurityContextV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_security_context_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod single_pixel_buffer_v1 {
    pub mod wp_single_pixel_buffer_manager_v1 {
        pub trait WpSinglePixelBufferManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_single_pixel_buffer_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod tearing_control_v1 {
    pub mod wp_tearing_control_manager_v1 {
        pub trait WpTearingControlManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_tearing_control_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod wp_tearing_control_v1 {
        pub trait WpTearingControlV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "wp_tearing_control_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_activation_v1 {
    pub mod xdg_activation_v1 {
        pub trait XdgActivationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_activation_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xdg_activation_token_v1 {
        pub trait XdgActivationTokenV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_activation_token_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_dialog_v1 {
    pub mod xdg_wm_dialog_v1 {
        pub trait XdgWmDialogV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_wm_dialog_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xdg_dialog_v1 {
        pub trait XdgDialogV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_dialog_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_system_bell_v1 {
    pub mod xdg_system_bell_v1 {
        pub trait XdgSystemBellV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_system_bell_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_toplevel_drag_v1 {
    pub mod xdg_toplevel_drag_manager_v1 {
        pub trait XdgToplevelDragManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_toplevel_drag_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xdg_toplevel_drag_v1 {
        pub trait XdgToplevelDragV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_toplevel_drag_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_toplevel_icon_v1 {
    pub mod xdg_toplevel_icon_manager_v1 {
        pub trait XdgToplevelIconManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_toplevel_icon_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xdg_toplevel_icon_v1 {
        pub trait XdgToplevelIconV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_toplevel_icon_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_toplevel_tag_v1 {
    pub mod xdg_toplevel_tag_manager_v1 {
        pub trait XdgToplevelTagManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_toplevel_tag_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xwayland_shell_v1 {
    pub mod xwayland_shell_v1 {
        pub trait XwaylandShellV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xwayland_shell_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xwayland_surface_v1 {
        pub trait XwaylandSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xwayland_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
