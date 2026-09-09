/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <uapi/linux/vt.h>

/* Virtual Terminal events. */
pub const VT_ALLOCATE: i32 = 0x0001; // Console got allocated
pub const VT_DEALLOCATE: i32 = 0x0002; // Console will be deallocated
pub const VT_WRITE: i32 = 0x0003; // A char got output
pub const VT_UPDATE: i32 = 0x0004; // A bigger update occurred
pub const VT_PREWRITE: i32 = 0x0005; // A char is about to be written to the console

// Under CONFIG_VT_CONSOLE, this function is provided externally by the
// corresponding implementation.
#[cfg(CONFIG_VT_CONSOLE)]
unsafe extern "C" {
    pub fn vt_kmsg_redirect(r#new: i32) -> i32;
}

// When CONFIG_VT_CONSOLE is disabled, the header provides this inline stub.
#[cfg(not(CONFIG_VT_CONSOLE))]
#[inline]
pub fn vt_kmsg_redirect(_new: i32) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
