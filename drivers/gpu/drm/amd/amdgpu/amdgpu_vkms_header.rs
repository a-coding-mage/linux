/* SPDX-License-Identifier: GPL-2.0+ */

pub const XRES_DEF: u32 = 1024;
pub const YRES_DEF: u32 = 768;

pub const XRES_MAX: u32 = 16384;
pub const YRES_MAX: u32 = 16384;

/*
 * C equivalent:
 * container_of(target, struct amdgpu_vkms_output, crtc.base)
 *
 * The containing-object calculation depends on the C layout and the
 * externally supplied `container_of` implementation.
 */
#[macro_export]
macro_rules! drm_crtc_to_amdgpu_vkms_output {
    ($target:expr) => {
        container_of!($target, amdgpu_vkms_output, crtc.base)
    };
}

extern "C" {
    pub static amdgpu_vkms_ip_block: amdgpu_ip_block_version;
}

#[repr(C)]
pub struct amdgpu_vkms_output {
    pub crtc: amdgpu_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub period_ns: ktime_t,
    pub event: *mut drm_pending_vblank_event,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
