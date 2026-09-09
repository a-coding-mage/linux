// SPDX-License-Identifier: GPL-2.0
/*
 * RISC-V Kexec image loader
 *
 */

// Dependency intent from the C source: linux/err.h, linux/errno.h,
// linux/kernel.h, linux/kexec.h, linux/pe.h, linux/string.h,
// asm/byteorder.h, and asm/image.h provide the types, constants, and
// functions referenced below.

use core::ffi::{c_char, c_void};

extern "C" {
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn kexec_add_buffer(kbuf: *mut kexec_buf) -> i32;
    fn load_extra_segments(
        image: *mut kimage,
        kernel_mem: u64,
        kernel_memsz: u64,
        initrd: *mut c_char,
        initrd_len: usize,
        cmdline: *mut c_char,
        cmdline_len: usize,
    ) -> i32;
    fn le64_to_cpu(value: u64) -> u64;
    fn riscv_image_flag_field(flags: u64, field: u64) -> bool;
    fn pr_err(fmt: *const c_char, ...) ;
    fn pr_info(fmt: *const c_char, ...) ;
}

// These declarations are supplied by the surrounding kernel translation.
// Build-time CONFIG_CPU_BIG_ENDIAN controls this value in the original code.
extern "C" {
    static CONFIG_CPU_BIG_ENDIAN: bool;
}

const EINVAL: i32 = 22;
const ULONG_MAX: u64 = u64::MAX;

unsafe fn image_probe(kernel_buf: *const c_char, kernel_len: usize) -> i32 {
    let h = kernel_buf as *const riscv_image_header;

    if h.is_null() || kernel_len < core::mem::size_of::<riscv_image_header>() {
        return -EINVAL;
    }

    /* According to Documentation/arch/riscv/boot-image-header.rst,
     * use "magic2" field to check when version >= 0.2.
     */
    if (*h).version >= RISCV_HEADER_VERSION
        && memcmp(
            &(*h).magic2 as *const _ as *const c_void,
            RISCV_IMAGE_MAGIC2.as_ptr() as *const c_void,
            core::mem::size_of_val(&(*h).magic2),
        ) != 0
    {
        return -EINVAL;
    }

    0
}

unsafe fn image_load(
    image: *mut kimage,
    kernel: *mut c_char,
    kernel_len: usize,
    initrd: *mut c_char,
    initrd_len: usize,
    cmdline: *mut c_char,
    cmdline_len: usize,
) -> *mut c_void {
    let h = kernel as *mut riscv_image_header;
    let mut flags: u64;
    let be_image: bool;
    let be_kernel: bool;
    let mut kbuf: kexec_buf = core::mem::zeroed();
    let ret: i32;

    /* Check Image header */
    if (*h).image_size == 0 {
        ret = -EINVAL;
        return (ret as isize) as *mut c_void;
    }

    /* Check endianness */
    flags = le64_to_cpu((*h).flags);
    be_image = riscv_image_flag_field(flags, RISCV_IMAGE_FLAG_BE);
    be_kernel = CONFIG_CPU_BIG_ENDIAN;
    if be_image != be_kernel {
        ret = -EINVAL;
        return (ret as isize) as *mut c_void;
    }

    /* Load the kernel image */
    kbuf.image = image;
    kbuf.buf_min = 0;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;

    kbuf.buffer = kernel;
    kbuf.bufsz = kernel_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = le64_to_cpu((*h).image_size);
    kbuf.buf_align = le64_to_cpu((*h).text_offset);

    ret = kexec_add_buffer(&mut kbuf);
    if ret != 0 {
        pr_err(b"Error add kernel image ret=%d\0".as_ptr() as *const c_char, ret);
        return (ret as isize) as *mut c_void;
    }

    (*image).start = kbuf.mem;

    pr_info(
        b"Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\0".as_ptr() as *const c_char,
        kbuf.mem,
        kbuf.bufsz,
        kbuf.memsz,
    );

    ret = load_extra_segments(
        image,
        kbuf.mem,
        kbuf.memsz,
        initrd,
        initrd_len,
        cmdline,
        cmdline_len,
    );

    if ret != 0 {
        (ret as isize) as *mut c_void
    } else {
        core::ptr::null_mut()
    }
}

#[repr(C)]
pub struct kexec_file_ops {
    pub probe: unsafe fn(*const c_char, usize) -> i32,
    pub load: unsafe fn(
        *mut kimage,
        *mut c_char,
        usize,
        *mut c_char,
        usize,
        *mut c_char,
        usize,
    ) -> *mut c_void,
}

pub static image_kexec_ops: kexec_file_ops = kexec_file_ops {
    probe: image_probe,
    load: image_load,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
