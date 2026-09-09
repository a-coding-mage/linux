/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux headers:
// linux/auxiliary_bus.h, linux/container_of.h

#[repr(C)]
pub struct NpcmClockAdev {
    pub base: *mut core::ffi::c_void,
    pub adev: auxiliary_device,
}

#[inline]
pub unsafe fn to_npcm_clock_adev(_adev: *mut auxiliary_device) -> *mut NpcmClockAdev {
    // Equivalent to container_of(_adev, struct npcm_clock_adev, adev).
    (_adev as *mut u8).sub(core::mem::offset_of!(NpcmClockAdev, adev))
        as *mut NpcmClockAdev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
