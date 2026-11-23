// SPDX-License-Identifier: GPL-2.0

// Dependencies: linux/io.h, linux/export.h.

use core::ffi::c_void;
use kernel::bindings::readb;
use kernel::ffi::c_int;

/**
 *	check_signature		-	find BIOS signatures
 *	@io_addr: mmio address to check
 *	@signature:  signature block
 *	@length: length of signature
 *
 *	Perform a signature comparison with the mmio address io_addr. This
 *	address should have been obtained by ioremap.
 *	Returns 1 on a match.
 */
/// Compares `length` MMIO bytes at `io_addr` with `signature`.
///
/// # Safety
/// `io_addr` is a mapped MMIO range and `signature` a readable buffer, both
/// covering `length` bytes.
#[no_mangle]
pub unsafe extern "C" fn check_signature(
    mut io_addr: *const c_void,
    mut signature: *const u8,
    mut length: c_int,
) -> c_int {
    // `while (length--)`, with the int decrement wrapping as in C.
    loop {
        let remaining = length;
        length = length.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
        // SAFETY: both ranges cover the bytes compared; readb() is the
        // configured architecture's MMIO accessor.
        unsafe {
            if readb(io_addr) != *signature {
                return 0;
            }
        }
        io_addr = io_addr.wrapping_byte_add(1);
        signature = signature.wrapping_add(1);
    }
    1
}
// EXPORT_SYMBOL(check_signature);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
