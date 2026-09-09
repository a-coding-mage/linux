// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by boot.h and drivers/s390/char/sclp_early_core.c

// SCLP early buffer must stay page-aligned and below 2GB.
// The alignment is provided by the surrounding build environment's PAGE_SIZE.
#[repr(align(4096))]
struct SclpEarlySccb([u8; EXT_SCCB_READ_SCP]);

static mut __SCLP_EARLY_SCCB: SclpEarlySccb = SclpEarlySccb([0; EXT_SCCB_READ_SCP]);

extern "C" {
    fn sclp_early_set_buffer(buffer: *mut core::ffi::c_void);
}

pub unsafe extern "C" fn sclp_early_setup_buffer() {
    sclp_early_set_buffer(
        (&raw mut __SCLP_EARLY_SCCB.0).cast::<core::ffi::c_void>(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
