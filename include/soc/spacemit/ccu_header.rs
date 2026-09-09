/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the corresponding Linux kernel interfaces:
// linux/auxiliary_bus.h, linux/regmap.h

/* Auxiliary device used to represent a CCU reset controller */
#[repr(C)]
pub struct spacemit_ccu_adev {
    pub adev: auxiliary_device,
    pub regmap: *mut regmap,
}

#[inline]
pub unsafe fn to_spacemit_ccu_adev(adev: *mut auxiliary_device) -> *mut spacemit_ccu_adev {
    // `adev` is the first field, so container_of is a direct pointer cast.
    adev as *mut spacemit_ccu_adev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
