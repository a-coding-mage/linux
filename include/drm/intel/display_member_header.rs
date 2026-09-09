/* SPDX-License-Identifier: MIT */
/* Copyright © 2025 Intel Corporation */

// Translated from the C header. The included Linux/DRM declarations are
// expected to be supplied by the surrounding Rust translation environment.

/// Opaque forward declaration of `struct intel_display`.
#[repr(C)]
pub struct intel_display {
    _private: [u8; 0],
}

/// A dummy device struct used to define the relative offsets of DRM and
/// display members. With the members identically placed in the driver device
/// structs, this allows figuring out the `struct intel_display` pointer
/// without the definition of either driver-specific structure.
#[repr(C)]
pub struct __intel_generic_device {
    pub drm: drm_device,
    pub display: *mut intel_display,
}

/// Ensure correct placement of DRM and display members.
///
/// This is the Rust equivalent of the C static assertion. It requires the
/// `offset_of!` facility and the referenced type's fields to be available in
/// the surrounding translation environment.
#[macro_export]
macro_rules! INTEL_DISPLAY_MEMBER_STATIC_ASSERT {
    ($type:ty, $drm_member:tt, $display_member:tt) => {
        const _: () = {
            assert!(
                core::mem::offset_of!(__intel_generic_device, display)
                    - core::mem::offset_of!(__intel_generic_device, drm)
                    == core::mem::offset_of!($type, $display_member)
                        - core::mem::offset_of!($type, $drm_member),
                "invalid DRM/display member offsets"
            );
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
