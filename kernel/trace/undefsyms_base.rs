// SPDX-License-Identifier: GPL-2.0

/*
 * simple_ring_buffer is used by the pKVM hypervisor which does not have access
 * to all kernel symbols. Whatever is undefined when compiling this file is
 * compiler and tooling-generated symbols that can safely be ignored for
 * simple_ring_buffer.
 */

use core::ffi::c_void;

// PAGE_SIZE and the kernel warning/atomic facilities are supplied by the
// surrounding kernel environment.
extern "C" {
    fn WARN_ON(condition: bool) -> i32;
}

#[repr(align(4096))]
struct Page([u8; PAGE_SIZE]);

static mut page: Page = Page([0; PAGE_SIZE]);

pub unsafe fn undefsyms_base(p: *mut c_void, n: i32) {
    let mut buffer = [0u8; 256];

    let mut u: u32 = 0;
    core::ptr::write_bytes(
        page.0.as_mut_ptr().cast::<u8>(),
        8,
        PAGE_SIZE,
    );
    core::ptr::write_bytes(buffer.as_mut_ptr(), 8, core::mem::size_of_val(&buffer));
    core::ptr::copy_nonoverlapping(
        buffer.as_ptr(),
        p.cast::<u8>(),
        core::mem::size_of_val(&buffer),
    );
    let _ = (&mut u as *mut u32).compare_exchange(0, 8, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst);
    WARN_ON(n == 0xdeadbeef_u32 as i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
