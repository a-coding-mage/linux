// SPDX-License-Identifier: GPL-2.0
/*
 * Load EFI vmlinux file for the kexec_file_load syscall.
 *
 * Author: Youling Tang <tangyouling@kylinos.cn>
 * Copyright (C) 2025 KylinSoft Corporation.
 */

// pr_fmt(fmt) "kexec_file(EFI): " fmt
// Dependencies supplied by the surrounding kernel translation.

unsafe fn efi_kexec_probe(kernel_buf: *const core::ffi::c_char, kernel_len: usize) -> i32 {
    let h = kernel_buf as *const loongarch_image_header;

    if h.is_null() || (kernel_len < core::mem::size_of::<loongarch_image_header>()) {
        kexec_dprintk!("No LoongArch image header.\n");
        return -EINVAL;
    }

    if !loongarch_header_check_dos_sig(h) {
        kexec_dprintk!("No LoongArch PE image header.\n");
        return -EINVAL;
    }

    0
}

unsafe fn efi_kexec_load(
    image: *mut kimage,
    kernel: *mut core::ffi::c_char,
    kernel_len: usize,
    initrd: *mut core::ffi::c_char,
    initrd_len: usize,
    cmdline: *mut core::ffi::c_char,
    cmdline_len: usize,
) -> *mut core::ffi::c_void {
    let mut ret: i32;
    let mut text_offset: usize;
    let mut kernel_segment_number: usize;
    let mut kbuf: kexec_buf = core::mem::zeroed();
    let mut kernel_segment: *mut kexec_segment;
    let h = kernel as *mut loongarch_image_header;

    if (*h).kernel_asize == 0 {
        return ERR_PTR(-EINVAL);
    }

    /*
     * Load the kernel
     * FIXME: Non-relocatable kernel rejected for kexec_file (require CONFIG_RELOCATABLE)
     */
    (*(&mut kbuf)).image = image;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;

    kbuf.buffer = kernel;
    kbuf.bufsz = kernel_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = le64_to_cpu((*h).kernel_asize);
    text_offset = le64_to_cpu((*h).text_offset);
    kbuf.buf_min = text_offset;
    kbuf.buf_align = SZ_2M;

    kernel_segment_number = (*image).nr_segments;

    /*
     * The location of the kernel segment may make it impossible to
     * satisfy the other segment requirements, so we try repeatedly
     * to find a location that will work.
     */
    while {
        ret = kexec_add_buffer(&mut kbuf);
        ret == 0
    } {
        /* Try to load additional data */
        kernel_segment = (*image).segment.add(kernel_segment_number);
        ret = load_other_segments(
            image,
            (*kernel_segment).mem,
            (*kernel_segment).memsz,
            initrd,
            initrd_len,
            cmdline,
            cmdline_len,
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

    if ret < 0 {
        pr_err!("Could not find any suitable kernel location!");
        return ERR_PTR(ret);
    }

    kernel_segment = (*image).segment.add(kernel_segment_number);

    /* Make sure the second kernel jumps to the correct "kernel_entry" */
    (*image).start = (*kernel_segment).mem + (*h).kernel_entry - text_offset;

    kexec_dprintk!(
        "Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
        (*kernel_segment).mem,
        kbuf.bufsz,
        (*kernel_segment).memsz
    );

    core::ptr::null_mut()
}

pub static kexec_efi_ops: kexec_file_ops = kexec_file_ops {
    probe: Some(efi_kexec_probe),
    load: Some(efi_kexec_load),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
