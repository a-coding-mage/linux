// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boot/compressed/misc.c
 *
 * This is a collection of several routines from gzip-1.0.3
 * adapted for Linux.
 *
 * malloc by Hannu Savolainen 1993 and Matthias Urlichs 1994
 *
 * Adapted for SH by Stuart Menefy, Aug 1999
 *
 * Modified to use standard LinuxSH BIOS by Greg Banks 7Jul2000
 */

// C dependencies: <linux/uaccess.h>, <asm/addrspace.h>, <asm/page.h>, and "misc.h".
// The decompressor sources below are included by the corresponding build-time configuration.

// #define STATIC static
// #undef memset
// #undef memcpy
// #define memzero(s, n) memset((s), 0, (n))

extern "C" {
    static mut input_data: [core::ffi::c_char; 0];
    static mut input_len: core::ffi::c_int;
    static mut _text: core::ffi::c_int; // Defined in vmlinux.lds.S
    static mut _end: core::ffi::c_int;
    fn __decompress(
        input: *const core::ffi::c_char,
        input_len: core::ffi::c_int,
        fill: *const core::ffi::c_void,
        flush: *const core::ffi::c_void,
        output: *mut u8,
        output_len: usize,
        error: *const core::ffi::c_void,
        error_fn: unsafe extern "C" fn(*mut core::ffi::c_char),
    );
    fn __pa(address: usize) -> usize;
    static puts: unsafe extern "C" fn(*const core::ffi::c_char) -> core::ffi::c_int;
}

static mut output: *mut u8 = core::ptr::null_mut();
static mut free_mem_ptr: usize = 0;
static mut free_mem_end_ptr: usize = 0;

// #ifdef CONFIG_HAVE_KERNEL_BZIP2
const HEAP_SIZE: usize = 0x400000;
// #else
// const HEAP_SIZE: usize = 0x10000;
// #endif

// #ifdef CONFIG_KERNEL_GZIP
// include "../../../../lib/decompress_inflate.c"
// #endif
// #ifdef CONFIG_KERNEL_BZIP2
// include "../../../../lib/decompress_bunzip2.c"
// #endif
// #ifdef CONFIG_KERNEL_LZMA
// include "../../../../lib/decompress_unlzma.c"
// #endif
// #ifdef CONFIG_KERNEL_XZ
// include "../../../../lib/decompress_unxz.c"
// #endif
// #ifdef CONFIG_KERNEL_LZO
// include "../../../../lib/decompress_unlzo.c"
// #endif

#[no_mangle]
pub unsafe extern "C" fn puts(_s: *const core::ffi::c_char) -> core::ffi::c_int {
    // This should be updated to use the sh-sci routines
    0
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut core::ffi::c_void, c: core::ffi::c_int, n: usize) -> *mut core::ffi::c_void {
    let mut i: usize = 0;
    let ss = s as *mut core::ffi::c_char;
    while i < n {
        *ss.add(i) = c as core::ffi::c_char;
        i += 1;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(
    dest: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    n: usize,
) -> *mut core::ffi::c_void {
    let mut i: usize = 0;
    let d = dest as *mut core::ffi::c_char;
    let s = src as *const core::ffi::c_char;
    while i < n {
        *d.add(i) = *s.add(i);
        i += 1;
    }
    dest
}

unsafe fn error(x: *mut core::ffi::c_char) -> ! {
    puts(b"\n\n\0".as_ptr() as *const core::ffi::c_char);
    puts(x);
    puts(b"\n\n -- System halted\0".as_ptr() as *const core::ffi::c_char);
    loop {}
}

#[no_mangle]
pub static __stack_chk_guard: usize = 0x000a0dff;

#[no_mangle]
pub unsafe extern "C" fn __stack_chk_fail() {
    error(b"stack-protector: Kernel stack is corrupted\n\0".as_ptr() as *mut core::ffi::c_char);
}

// Needed because vmlinux.lds.h references this.
#[no_mangle]
pub extern "C" fn ftrace_stub() {}

#[no_mangle]
pub extern "C" fn arch_ftrace_ops_list_func() {}

const STACK_SIZE: usize = 4096;

#[repr(align(4))]
pub struct UserStack(pub [core::ffi::c_long; STACK_SIZE]);

#[no_mangle]
pub static mut user_stack: UserStack = UserStack([0; STACK_SIZE]);

#[no_mangle]
pub static mut stack_start: *mut core::ffi::c_long = unsafe {
    user_stack.0.as_ptr().add(STACK_SIZE) as *mut core::ffi::c_long
};

#[no_mangle]
pub unsafe extern "C" fn decompress_kernel() {
    let mut output_addr: usize;

    output_addr = __pa((&_text as *const _ as usize) + 4096);
    // #if defined(CONFIG_29BIT)
    // output_addr |= P2SEG;
    // #endif

    output = output_addr as *mut u8;
    free_mem_ptr = &_end as *const _ as usize;
    free_mem_end_ptr = free_mem_ptr.wrapping_add(HEAP_SIZE);

    puts(b"Uncompressing Linux... \0".as_ptr() as *const core::ffi::c_char);
    __decompress(
        input_data.as_ptr(),
        input_len,
        core::ptr::null(),
        core::ptr::null(),
        output,
        0,
        core::ptr::null(),
        error,
    );
    puts(b"Ok, booting the kernel.\n\0".as_ptr() as *const core::ffi::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
