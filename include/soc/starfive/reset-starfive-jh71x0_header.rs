/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
//   linux/auxiliary_bus.h
//   linux/compiler_types.h
//   linux/container_of.h

#[repr(C)]
pub struct jh71x0_reset_adev {
    pub base: *mut core::ffi::c_void,
    pub adev: auxiliary_device,
}

/// Equivalent to the C `container_of((_adev), struct jh71x0_reset_adev, adev)` macro.
#[inline]
pub unsafe fn to_jh71x0_reset_adev(_adev: *mut auxiliary_device) -> *mut jh71x0_reset_adev {
    (_adev as *mut u8).sub(core::mem::offset_of!(jh71x0_reset_adev, adev))
        as *mut jh71x0_reset_adev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
