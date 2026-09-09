/* SPDX-License-Identifier: GPL-2.0 */

// When CONFIG_ARCH_HAS_SYNC_CORE_BEFORE_USERMODE is enabled, the architecture
// supplies sync_core_before_usermode() through <asm/sync_core.h>.
// The architecture-provided declaration is an external dependency.

// Opaque declaration supplied by the surrounding kernel translation.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[cfg(not(CONFIG_ARCH_HAS_SYNC_CORE_BEFORE_USERMODE))]
#[inline]
pub fn sync_core_before_usermode() {
    // This is a dummy sync_core_before_usermode() implementation that can be
    // used on all architectures which return to user-space through core
    // serializing instructions.
    // If your architecture returns to user-space through non-core-serializing
    // instructions, you need to write your own functions.
}

// When CONFIG_ARCH_HAS_PREPARE_SYNC_CORE_CMD is enabled, the architecture
// supplies prepare_sync_core_cmd() through <asm/sync_core.h>.
// The architecture-provided declaration is an external dependency.

#[cfg(not(CONFIG_ARCH_HAS_PREPARE_SYNC_CORE_CMD))]
#[inline]
pub fn prepare_sync_core_cmd(_mm: *mut mm_struct) {
    // This is a dummy prepare_sync_core_cmd() implementation that can be used
    // on all architectures which provide unconditional core serializing
    // instructions in switch_mm().
    // If your architecture doesn't provide such core serializing instructions
    // in switch_mm(), you may need to write your own functions.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
