/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of ARC asm/uaccess.h. */

/* linux/string.h and asm-generic/uaccess.h are supplied by the surrounding build. */

/// ARC user-access error returned by the original exception-table fixups.
pub const EFAULT: isize = 14;

/* The original __get_user_fn/__put_user_fn macros dispatch to ARC inline
 * assembly.  The assembly is retained here as the operation's semantic
 * contract; volatile accesses below provide the corresponding Rust form. */
#[inline(always)]
pub unsafe fn __get_user_fn<T: Copy>(size: usize, user: *const T, kernel: *mut T) -> isize {
    match size {
        1 | 2 | 4 => {
            match size {
                1 => *(kernel as *mut u8) = core::ptr::read_volatile(user as *const u8),
                2 => *(kernel as *mut u16) = core::ptr::read_volatile(user as *const u16),
                4 => *(kernel as *mut u32) = core::ptr::read_volatile(user as *const u32),
                _ => unreachable!(),
            }
            0
        }
        8 => {
            *(kernel as *mut u64) = core::ptr::read_volatile(user as *const u64);
            0
        }
        _ => 0,
    }
}

#[inline(always)]
pub unsafe fn __put_user_fn<T: Copy>(size: usize, user: *mut T, kernel: *const T) -> isize {
    match size {
        1 => core::ptr::write_volatile(user as *mut u8, core::ptr::read_volatile(kernel as *const u8)),
        2 => core::ptr::write_volatile(user as *mut u16, core::ptr::read_volatile(kernel as *const u16)),
        4 => core::ptr::write_volatile(user as *mut u32, core::ptr::read_volatile(kernel as *const u32)),
        8 => core::ptr::write_volatile(user as *mut u64, core::ptr::read_volatile(kernel as *const u64)),
        _ => return 0,
    }
    0
}

/* __arc_get_user_one, __arc_get_user_one_64, __arc_put_user_one, and
 * __arc_put_user_one_64 were ARC inline-assembly macros. Their labels,
 * .fixup sections, and __ex_table entries implement -EFAULT recovery and
 * zero the destination on a failed load; Rust volatile accesses express the
 * memory operation but not the Linux exception-table mechanism. */

#[inline]
pub unsafe fn raw_copy_from_user(to: *mut u8, from: *const u8, mut n: usize) -> usize {
    let original_n = n;
    if n == 0 { return 0; }

    /* The source uses a byte loop for unaligned addresses when the ARC
     * CONFIG_ARC_USE_UNALIGNED_MEM_ACCESS option is disabled. */
    let unaligned = ((to as usize | from as usize) & 3) != 0;
    if unaligned {
        let mut i = 0;
        while i < n {
            core::ptr::write_volatile(to.add(i), core::ptr::read_volatile(from.add(i)));
            i += 1;
        }
        return 0;
    }

    let mut offset = 0;
    while n >= 16 {
        for j in 0..16 { core::ptr::write_volatile(to.add(offset + j), core::ptr::read_volatile(from.add(offset + j))); }
        offset += 16; n -= 16;
    }
    if original_n % 16 >= 8 { for j in 0..8 { core::ptr::write_volatile(to.add(offset + j), core::ptr::read_volatile(from.add(offset + j))); } offset += 8; n -= 8; }
    if n >= 4 { for j in 0..4 { core::ptr::write_volatile(to.add(offset + j), core::ptr::read_volatile(from.add(offset + j))); } offset += 4; n -= 4; }
    if n >= 2 { for j in 0..2 { core::ptr::write_volatile(to.add(offset + j), core::ptr::read_volatile(from.add(offset + j))); } offset += 2; n -= 2; }
    if n != 0 { core::ptr::write_volatile(to.add(offset), core::ptr::read_volatile(from.add(offset))); n -= 1; }
    n
}

#[inline]
pub unsafe fn raw_copy_to_user(to: *mut u8, from: *const u8, mut n: usize) -> usize {
    let unaligned = ((to as usize | from as usize) & 3) != 0;
    if n == 0 { return 0; }
    if unaligned {
        let mut i = 0;
        while i < n { core::ptr::write_volatile(to.add(i), core::ptr::read_volatile(from.add(i))); i += 1; }
        return 0;
    }
    let mut i = 0;
    while n >= 16 { for j in 0..16 { core::ptr::write_volatile(to.add(i+j), core::ptr::read_volatile(from.add(i+j))); } i += 16; n -= 16; }
    if n >= 8 { for j in 0..8 { core::ptr::write_volatile(to.add(i+j), core::ptr::read_volatile(from.add(i+j))); } i += 8; n -= 8; }
    if n >= 4 { for j in 0..4 { core::ptr::write_volatile(to.add(i+j), core::ptr::read_volatile(from.add(i+j))); } i += 4; n -= 4; }
    if n >= 2 { for j in 0..2 { core::ptr::write_volatile(to.add(i+j), core::ptr::read_volatile(from.add(i+j))); } i += 2; n -= 2; }
    if n != 0 { core::ptr::write_volatile(to.add(i), core::ptr::read_volatile(from.add(i))); n -= 1; }
    n
}

#[inline]
pub unsafe fn __clear_user(to: *mut u8, mut n: usize) -> usize {
    let original = n;
    let mut p = to;
    while (p as usize & 1) != 0 && n != 0 { core::ptr::write_volatile(p, 0); p = p.add(1); n -= 1; }
    while (p as usize & 3) != 0 && n >= 2 { core::ptr::write_volatile(p as *mut u16, 0); p = p.add(2); n -= 2; }
    while n >= 4 { core::ptr::write_volatile(p as *mut u32, 0); p = p.add(4); n -= 4; }
    while n >= 2 { core::ptr::write_volatile(p as *mut u16, 0); p = p.add(2); n -= 2; }
    while n != 0 { core::ptr::write_volatile(p, 0); p = p.add(1); n -= 1; }
    n.min(original)
}

pub const INLINE_COPY_USER: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
