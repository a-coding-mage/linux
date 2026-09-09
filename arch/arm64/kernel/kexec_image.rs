// SPDX-License-Identifier: GPL-2.0
/*
 * Kexec image loader

 * Copyright (C) 2018 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 */

// pr_fmt(fmt) = "kexec_file(Image): " fmt

use core::ffi::{c_char, c_int, c_ulong, c_void};

// Types, constants, and functions supplied by the surrounding kernel code are
// intentionally referenced here rather than redefined.

unsafe extern "C" {
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn system_supports_mixed_endian() -> bool;
    fn system_supports_4kb_granule() -> bool;
    fn system_supports_64kb_granule() -> bool;
    fn system_supports_16kb_granule() -> bool;
    fn kexec_add_buffer(kbuf: *mut kexec_buf) -> c_int;
    fn load_other_segments(
        image: *mut kimage,
        kernel_mem: c_ulong,
        kernel_memsz: c_ulong,
        initrd: *mut c_char,
        initrd_len: c_ulong,
        cmdline: *mut c_char,
    ) -> c_int;
    fn kexec_kernel_verify_pe_sig(
        image: *mut kimage,
        kernel: *mut c_char,
        kernel_len: c_ulong,
    ) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn kexec_dprintk(fmt: *const c_char, ...);
}

unsafe fn image_probe(kernel_buf: *const c_char, kernel_len: c_ulong) -> c_int {
    let h = kernel_buf as *const arm64_image_header;

    if h.is_null() || kernel_len < core::mem::size_of::<arm64_image_header>() as c_ulong {
        return -EINVAL;
    }

    if memcmp(
        core::ptr::addr_of!((*h).magic) as *const c_void,
        ARM64_IMAGE_MAGIC.as_ptr() as *const c_void,
        core::mem::size_of_val(&(*h).magic),
    ) != 0
    {
        return -EINVAL;
    }

    0
}

unsafe fn image_load(
    image: *mut kimage,
    kernel: *mut c_char,
    kernel_len: c_ulong,
    initrd: *mut c_char,
    initrd_len: c_ulong,
    cmdline: *mut c_char,
    _cmdline_len: c_ulong,
) -> *mut c_void {
    let h = kernel as *mut arm64_image_header;
    let mut flags: u64;
    let mut value: u64;
    let be_image: bool;
    let be_kernel: bool;
    let mut kbuf: kexec_buf = core::mem::zeroed();
    let text_offset: c_ulong;
    let kernel_segment_number: c_ulong;
    let kernel_segment: *mut kexec_segment;
    let mut ret: c_int;

    /*
     * We require a kernel with an unambiguous Image header. Per
     * Documentation/arch/arm64/booting.rst, this is the case when image_size
     * is non-zero (practically speaking, since v3.17).
     */
    if (*h).image_size == 0 {
        return ERR_PTR(-EINVAL);
    }

    /* Check cpu features */
    flags = le64_to_cpu((*h).flags);
    be_image = arm64_image_flag_field(flags, ARM64_IMAGE_FLAG_BE);
    be_kernel = cfg!(CONFIG_CPU_BIG_ENDIAN);
    if be_image != be_kernel && !system_supports_mixed_endian() {
        return ERR_PTR(-EINVAL);
    }

    value = arm64_image_flag_field(flags, ARM64_IMAGE_FLAG_PAGE_SIZE);
    if (value == ARM64_IMAGE_FLAG_PAGE_SIZE_4K && !system_supports_4kb_granule())
        || (value == ARM64_IMAGE_FLAG_PAGE_SIZE_64K && !system_supports_64kb_granule())
        || (value == ARM64_IMAGE_FLAG_PAGE_SIZE_16K && !system_supports_16kb_granule())
    {
        return ERR_PTR(-EINVAL);
    }

    /* Load the kernel */
    kbuf.image = image;
    kbuf.buf_min = 0;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;
    kbuf.buffer = kernel;
    kbuf.bufsz = kernel_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = le64_to_cpu((*h).image_size);
    text_offset = le64_to_cpu((*h).text_offset);
    kbuf.buf_align = MIN_KIMG_ALIGN;

    /* Adjust kernel segment with TEXT_OFFSET */
    kbuf.memsz += text_offset;
    kernel_segment_number = (*image).nr_segments;

    /*
     * The location of the kernel segment may make it impossible to satisfy
     * the other segment requirements, so we try repeatedly to find a
     * location that will work.
     */
    while {
        ret = kexec_add_buffer(&mut kbuf);
        ret == 0
    } {
        /* Try to load additional data */
        kernel_segment = (*image).segment.add(kernel_segment_number as usize);
        ret = load_other_segments(
            image, (*kernel_segment).mem, (*kernel_segment).memsz,
            initrd, initrd_len, cmdline,
        );
        if ret == 0 {
            break;
        }

        /*
         * We couldn't find space for the other segments; erase the
         * kernel segment and try the next available hole.
         */
        (*image).nr_segments -= 1;
        kbuf.buf_min = (*kernel_segment).mem + (*kernel_segment).memsz;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    }

    if ret != 0 {
        pr_err(b"Could not find any suitable kernel location!\0".as_ptr() as *const c_char);
        return ERR_PTR(ret);
    }

    kernel_segment = (*image).segment.add(kernel_segment_number as usize);
    (*kernel_segment).mem += text_offset;
    (*kernel_segment).memsz -= text_offset;
    (*image).start = (*kernel_segment).mem;

    kexec_dprintk(b"Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\n\0".as_ptr() as *const c_char,
        (*kernel_segment).mem, kbuf.bufsz, (*kernel_segment).memsz);

    core::ptr::null_mut()
}

#[no_mangle]
pub static kexec_image_ops: kexec_file_ops = kexec_file_ops {
    probe: Some(image_probe),
    load: Some(image_load),
    #[cfg(CONFIG_KEXEC_IMAGE_VERIFY_SIG)]
    verify_sig: Some(kexec_kernel_verify_pe_sig),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
