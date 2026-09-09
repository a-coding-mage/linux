/* SPDX-License-Identifier: MIT */

/*
 * Translated from nova_drm.h.
 *
 * DISCLAIMER: Do not use, this is not a stable uAPI.
 *
 * This uAPI serves only testing purposes as long as this driver is still in
 * development. It is required to implement and test infrastructure which is
 * upstreamed in the context of this driver. See also [1].
 *
 * [1] https://lore.kernel.org/dri-devel/Zfsj0_tb-0-tNrJy@cassiopeiae/T/#u
 *
 * The ioctl encoding depends on the DRM definitions supplied by drm.h.
 */

/// NOVA_GETPARAM_VRAM_BAR_SIZE: query the VRAM BAR size in bytes.
pub const NOVA_GETPARAM_VRAM_BAR_SIZE: u64 = 0x1;

/// Query GPU and driver metadata.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nova_getparam {
    /// The identifier of the parameter to query.
    pub param: u64,
    /// The value for the specified parameter.
    pub value: u64,
}

/// Create a new DRM GEM object.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nova_gem_create {
    /// The handle of the new DRM GEM object.
    pub handle: u32,
    /// 32 bit padding, should be 0.
    pub pad: u32,
    /// The size of the new DRM GEM object.
    pub size: u64,
}

/// Query DRM GEM object metadata.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_nova_gem_info {
    /// The handle of the DRM GEM object to query.
    pub handle: u32,
    /// 32 bit padding, should be 0.
    pub pad: u32,
    /// The size of the DRM GEM obejct.
    pub size: u64,
}

pub const DRM_NOVA_GETPARAM: u64 = 0x00;
pub const DRM_NOVA_GEM_CREATE: u64 = 0x01;
pub const DRM_NOVA_GEM_INFO: u64 = 0x02;

/*
 * Note: these are represented as constants so that they correspond to the C
 * enum. DRM_COMMAND_BASE and DRM_IOWR are supplied by the DRM dependency.
 */
pub const DRM_IOCTL_NOVA_GETPARAM: u64 =
    DRM_IOWR(DRM_COMMAND_BASE + DRM_NOVA_GETPARAM, core::mem::size_of::<drm_nova_getparam>());
pub const DRM_IOCTL_NOVA_GEM_CREATE: u64 =
    DRM_IOWR(DRM_COMMAND_BASE + DRM_NOVA_GEM_CREATE, core::mem::size_of::<drm_nova_gem_create>());
pub const DRM_IOCTL_NOVA_GEM_INFO: u64 =
    DRM_IOWR(DRM_COMMAND_BASE + DRM_NOVA_GEM_INFO, core::mem::size_of::<drm_nova_gem_info>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
