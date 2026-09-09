// SPDX-License-Identifier: GPL-2.0
//
// The Linux <linux/io.h> and <linux/export.h> dependencies are supplied by
// the surrounding environment. `readb` is the corresponding MMIO byte read.

unsafe extern "C" {
    fn readb(addr: *const core::ffi::c_void) -> u8;
}

/**
 * check_signature - find BIOS signatures
 * @io_addr: mmio address to check
 * @signature: signature block
 * @length: length of signature
 *
 * Perform a signature comparison with the mmio address io_addr. This
 * address should have been obtained by ioremap.
 * Returns 1 on a match.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_signature(
    mut io_addr: *const core::ffi::c_void,
    mut signature: *const u8,
    mut length: i32,
) -> i32 {
    while length != 0 {
        if unsafe { readb(io_addr) } != unsafe { *signature } {
            return 0;
        }
        io_addr = unsafe { (io_addr as *const u8).add(1) as *const core::ffi::c_void };
        signature = unsafe { signature.add(1) };
        length = length.wrapping_sub(1);
    }
    1
}

// EXPORT_SYMBOL(check_signature)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
