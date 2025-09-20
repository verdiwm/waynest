pub mod fullscreen_shell_unstable_v1 {
    pub mod zwp_fullscreen_shell_v1 {
        pub trait ZwpFullscreenShellV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_fullscreen_shell_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_fullscreen_shell_mode_feedback_v1 {
        pub trait ZwpFullscreenShellModeFeedbackV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_fullscreen_shell_mode_feedback_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod idle_inhibit_unstable_v1 {
    pub mod zwp_idle_inhibit_manager_v1 {
        pub trait ZwpIdleInhibitManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_idle_inhibit_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_idle_inhibitor_v1 {
        pub trait ZwpIdleInhibitorV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_idle_inhibitor_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod input_method_unstable_v1 {
    pub mod zwp_input_method_context_v1 {
        pub trait ZwpInputMethodContextV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_input_method_context_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_input_method_v1 {
        pub trait ZwpInputMethodV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_input_method_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_input_panel_v1 {
        pub trait ZwpInputPanelV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_input_panel_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_input_panel_surface_v1 {
        pub trait ZwpInputPanelSurfaceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_input_panel_surface_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod input_timestamps_unstable_v1 {
    pub mod zwp_input_timestamps_manager_v1 {
        pub trait ZwpInputTimestampsManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_input_timestamps_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_input_timestamps_v1 {
        pub trait ZwpInputTimestampsV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_input_timestamps_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod keyboard_shortcuts_inhibit_unstable_v1 {
    pub mod zwp_keyboard_shortcuts_inhibit_manager_v1 {
        pub trait ZwpKeyboardShortcutsInhibitManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_keyboard_shortcuts_inhibit_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_keyboard_shortcuts_inhibitor_v1 {
        pub trait ZwpKeyboardShortcutsInhibitorV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_keyboard_shortcuts_inhibitor_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod linux_dmabuf_unstable_v1 {
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
pub mod zwp_linux_explicit_synchronization_unstable_v1 {
    pub mod zwp_linux_explicit_synchronization_v1 {
        pub trait ZwpLinuxExplicitSynchronizationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_linux_explicit_synchronization_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_linux_surface_synchronization_v1 {
        pub trait ZwpLinuxSurfaceSynchronizationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_linux_surface_synchronization_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_linux_buffer_release_v1 {
        pub trait ZwpLinuxBufferReleaseV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_linux_buffer_release_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod pointer_constraints_unstable_v1 {
    pub mod zwp_pointer_constraints_v1 {
        pub trait ZwpPointerConstraintsV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_pointer_constraints_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_locked_pointer_v1 {
        pub trait ZwpLockedPointerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_locked_pointer_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_confined_pointer_v1 {
        pub trait ZwpConfinedPointerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_confined_pointer_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod pointer_gestures_unstable_v1 {
    pub mod zwp_pointer_gestures_v1 {
        pub trait ZwpPointerGesturesV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_pointer_gestures_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zwp_pointer_gesture_swipe_v1 {
        pub trait ZwpPointerGestureSwipeV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_pointer_gesture_swipe_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_pointer_gesture_pinch_v1 {
        pub trait ZwpPointerGesturePinchV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_pointer_gesture_pinch_v1";
            const VERSION: u32 = 2u32;
        }
    }
    pub mod zwp_pointer_gesture_hold_v1 {
        pub trait ZwpPointerGestureHoldV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_pointer_gesture_hold_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod wp_primary_selection_unstable_v1 {
    pub mod zwp_primary_selection_device_manager_v1 {
        pub trait ZwpPrimarySelectionDeviceManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_primary_selection_device_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_primary_selection_device_v1 {
        pub trait ZwpPrimarySelectionDeviceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_primary_selection_device_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_primary_selection_offer_v1 {
        pub trait ZwpPrimarySelectionOfferV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_primary_selection_offer_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_primary_selection_source_v1 {
        pub trait ZwpPrimarySelectionSourceV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_primary_selection_source_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod relative_pointer_unstable_v1 {
    pub mod zwp_relative_pointer_manager_v1 {
        pub trait ZwpRelativePointerManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_relative_pointer_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_relative_pointer_v1 {
        pub trait ZwpRelativePointerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_relative_pointer_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod tablet_unstable_v1 {
    pub mod zwp_tablet_manager_v1 {
        pub trait ZwpTabletManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_seat_v1 {
        pub trait ZwpTabletSeatV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_seat_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_tool_v1 {
        pub trait ZwpTabletToolV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_tool_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_v1 {
        pub trait ZwpTabletV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod tablet_unstable_v2 {
    pub mod zwp_tablet_manager_v2 {
        pub trait ZwpTabletManagerV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_manager_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_seat_v2 {
        pub trait ZwpTabletSeatV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_seat_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_tool_v2 {
        pub trait ZwpTabletToolV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_tool_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_v2 {
        pub trait ZwpTabletV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_pad_ring_v2 {
        pub trait ZwpTabletPadRingV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_pad_strip_v2 {
        pub trait ZwpTabletPadStripV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_pad_group_v2 {
        pub trait ZwpTabletPadGroupV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_tablet_pad_v2 {
        pub trait ZwpTabletPadV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_tablet_pad_v2";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod text_input_unstable_v1 {
    pub mod zwp_text_input_v1 {
        pub trait ZwpTextInputV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_text_input_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_text_input_manager_v1 {
        pub trait ZwpTextInputManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_text_input_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod text_input_unstable_v3 {
    pub mod zwp_text_input_v3 {
        pub trait ZwpTextInputV3<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_text_input_v3";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_text_input_manager_v3 {
        pub trait ZwpTextInputManagerV3<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_text_input_manager_v3";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_decoration_unstable_v1 {
    pub mod zxdg_decoration_manager_v1 {
        pub trait ZxdgDecorationManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_decoration_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_toplevel_decoration_v1 {
        pub trait ZxdgToplevelDecorationV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_toplevel_decoration_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_foreign_unstable_v1 {
    pub mod zxdg_exporter_v1 {
        pub trait ZxdgExporterV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_exporter_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_importer_v1 {
        pub trait ZxdgImporterV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_importer_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_exported_v1 {
        pub trait ZxdgExportedV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_exported_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_imported_v1 {
        pub trait ZxdgImportedV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_imported_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_foreign_unstable_v2 {
    pub mod zxdg_exporter_v2 {
        pub trait ZxdgExporterV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_exporter_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_importer_v2 {
        pub trait ZxdgImporterV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_importer_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_exported_v2 {
        pub trait ZxdgExportedV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_exported_v2";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_imported_v2 {
        pub trait ZxdgImportedV2<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_imported_v2";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_output_unstable_v1 {
    pub mod zxdg_output_manager_v1 {
        pub trait ZxdgOutputManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_output_manager_v1";
            const VERSION: u32 = 3u32;
        }
    }
    pub mod zxdg_output_v1 {
        pub trait ZxdgOutputV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_output_v1";
            const VERSION: u32 = 3u32;
        }
    }
}
pub mod xdg_shell_unstable_v5 {
    pub mod xdg_shell {
        pub trait XdgShell<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_shell";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xdg_surface {
        pub trait XdgSurface<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_surface";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod xdg_popup {
        pub trait XdgPopup<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "xdg_popup";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xdg_shell_unstable_v6 {
    pub mod zxdg_shell_v6 {
        pub trait ZxdgShellV6<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_shell_v6";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_positioner_v6 {
        pub trait ZxdgPositionerV6<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_positioner_v6";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_surface_v6 {
        pub trait ZxdgSurfaceV6<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_surface_v6";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_toplevel_v6 {
        pub trait ZxdgToplevelV6<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_toplevel_v6";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zxdg_popup_v6 {
        pub trait ZxdgPopupV6<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zxdg_popup_v6";
            const VERSION: u32 = 1u32;
        }
    }
}
pub mod xwayland_keyboard_grab_unstable_v1 {
    pub mod zwp_xwayland_keyboard_grab_manager_v1 {
        pub trait ZwpXwaylandKeyboardGrabManagerV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_xwayland_keyboard_grab_manager_v1";
            const VERSION: u32 = 1u32;
        }
    }
    pub mod zwp_xwayland_keyboard_grab_v1 {
        pub trait ZwpXwaylandKeyboardGrabV1<
            C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>,
        >
        {
            const INTERFACE: &'static str = "zwp_xwayland_keyboard_grab_v1";
            const VERSION: u32 = 1u32;
        }
    }
}
