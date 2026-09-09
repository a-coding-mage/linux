/*
 * Constant-time equality testing of memory regions.
 *
 * Authors:
 *
 *   James Yonan <james@openvpn.net>
 *   Daniel Borkmann <dborkman@redhat.com>
 *
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * The original license text is retained in the corresponding C source.
 */

/* Generic path for arbitrary size */
#[inline(always)]
unsafe fn __crypto_memneq_generic(a: *const core::ffi::c_void,
                                  b: *const core::ffi::c_void,
                                  mut size: usize) -> usize {
    let mut neq: usize = 0;
    let mut a = a as *const u8;
    let mut b = b as *const u8;

    #[cfg(feature = "CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS")]
    {
        while size >= core::mem::size_of::<usize>() {
            neq |= core::ptr::read_unaligned(a as *const usize)
                ^ core::ptr::read_unaligned(b as *const usize);
            // OPTIMIZER_HIDE_VAR(neq);
            a = a.add(core::mem::size_of::<usize>());
            b = b.add(core::mem::size_of::<usize>());
            size -= core::mem::size_of::<usize>();
        }
    }
    while size > 0 {
        neq |= *a ^ *b;
        // OPTIMIZER_HIDE_VAR(neq);
        a = a.add(1);
        b = b.add(1);
        size -= 1;
    }
    neq
}

/* Loop-free fast-path for frequently used 16-byte size */
#[inline(always)]
unsafe fn __crypto_memneq_16(a: *const core::ffi::c_void,
                             b: *const core::ffi::c_void) -> usize {
    let mut neq: usize = 0;
    let a = a as *const u8;
    let b = b as *const u8;

    #[cfg(feature = "CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS")]
    {
        if core::mem::size_of::<usize>() == 8 {
            neq |= core::ptr::read_unaligned(a as *const usize)
                ^ core::ptr::read_unaligned(b as *const usize);
            // OPTIMIZER_HIDE_VAR(neq);
            neq |= core::ptr::read_unaligned(a.add(8) as *const usize)
                ^ core::ptr::read_unaligned(b.add(8) as *const usize);
            // OPTIMIZER_HIDE_VAR(neq);
        } else if core::mem::size_of::<u32>() == 4 {
            neq |= core::ptr::read_unaligned(a as *const u32)
                as usize ^ core::ptr::read_unaligned(b as *const u32) as usize;
            // OPTIMIZER_HIDE_VAR(neq);
            neq |= core::ptr::read_unaligned(a.add(4) as *const u32)
                as usize ^ core::ptr::read_unaligned(b.add(4) as *const u32) as usize;
            // OPTIMIZER_HIDE_VAR(neq);
            neq |= core::ptr::read_unaligned(a.add(8) as *const u32)
                as usize ^ core::ptr::read_unaligned(b.add(8) as *const u32) as usize;
            // OPTIMIZER_HIDE_VAR(neq);
            neq |= core::ptr::read_unaligned(a.add(12) as *const u32)
                as usize ^ core::ptr::read_unaligned(b.add(12) as *const u32) as usize;
            // OPTIMIZER_HIDE_VAR(neq);
        } else {
            return __crypto_memneq_16_bytes(a, b, neq);
        }
    }
    #[cfg(not(feature = "CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS"))]
    {
        return __crypto_memneq_16_bytes(a, b, neq);
    }
    neq
}

#[inline(always)]
unsafe fn __crypto_memneq_16_bytes(a: *const u8, b: *const u8, mut neq: usize) -> usize {
    let mut i = 0;
    while i < 16 {
        neq |= *a.add(i) as usize ^ *b.add(i) as usize;
        // OPTIMIZER_HIDE_VAR(neq);
        i += 1;
    }
    neq
}

/* Compare two areas of memory without leaking timing information,
 * and with special optimizations for common sizes.  Users should
 * not call this function directly, but should instead use
 * crypto_memneq defined in crypto/utils.h.
 */
#[inline(never)]
pub unsafe fn __crypto_memneq(a: *const core::ffi::c_void,
                              b: *const core::ffi::c_void,
                              size: usize) -> usize {
    match size {
        16 => __crypto_memneq_16(a, b),
        _ => __crypto_memneq_generic(a, b, size),
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
