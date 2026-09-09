// SPDX-License-Identifier: GPL-2.0
/*
 * kexec_file for LoongArch
 *
 * Author: Youling Tang <tangyouling@kylinos.cn>
 * Copyright (C) 2025 KylinSoft Corporation.
 *
 * Most code is derived from LoongArch port of kexec-tools
 */

// C dependencies: linux/ioport.h, linux/kernel.h, linux/kexec.h,
// linux/memblock.h, linux/slab.h, linux/string.h, linux/types.h,
// linux/vmalloc.h, asm/bootinfo.h

pub static KEXEC_FILE_LOADERS: [*const KexecFileOps; 3] = unsafe {
    [
        &kexec_efi_ops as *const KexecFileOps,
        &kexec_elf_ops as *const KexecFileOps,
        core::ptr::null(),
    ]
};

pub unsafe fn arch_kimage_file_post_load_cleanup(image: *mut Kimage) -> i32 {
    vfree((*image).elf_headers);
    (*image).elf_headers = core::ptr::null_mut();
    (*image).elf_headers_sz = 0;

    kexec_image_post_load_cleanup_default(image)
}

/* Add the "kexec_file" command line parameter to command line. */
unsafe fn cmdline_add_loader(cmdline_tmplen: *mut libc::c_ulong, modified_cmdline: *mut libc::c_char) {
    let loader_strlen: libc::c_int = sprintf(
        modified_cmdline.add(*cmdline_tmplen as usize),
        b"kexec_file \0".as_ptr() as *const libc::c_char,
    );
    *cmdline_tmplen += loader_strlen as libc::c_ulong;
}

/* Add the "initrd=start,size" command line parameter to command line. */
unsafe fn cmdline_add_initrd(
    image: *mut Kimage,
    cmdline_tmplen: *mut libc::c_ulong,
    modified_cmdline: *mut libc::c_char,
    initrd: libc::c_ulong,
) {
    let initrd_strlen: libc::c_int = sprintf(
        modified_cmdline.add(*cmdline_tmplen as usize),
        b"initrd=0x%lx,0x%lx \0".as_ptr() as *const libc::c_char,
        initrd,
        (*image).initrd_buf_len,
    );
    *cmdline_tmplen += initrd_strlen as libc::c_ulong;
}

// The following declarations and definitions are enabled by CONFIG_CRASH_DUMP.
#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn arch_get_system_nr_ranges() -> libc::c_uint {
    let mut nr_ranges: libc::c_int = 2; /* for exclusion of crashkernel region */
    let mut start: PhysAddr = 0;
    let mut end: PhysAddr = 0;
    let mut i: u64 = 0;

    for_each_mem_range(&mut i, &mut start, &mut end);
    nr_ranges += i as libc::c_int;

    nr_ranges as libc::c_uint
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn arch_crash_populate_cmem(cmem: *mut CrashMem) -> libc::c_int {
    let mut start: PhysAddr = 0;
    let mut end: PhysAddr = 0;
    let mut i: u64 = 0;

    while for_each_mem_range(&mut i, &mut start, &mut end) {
        (*cmem).ranges[(*cmem).nr_ranges as usize].start = start;
        (*cmem).ranges[(*cmem).nr_ranges as usize].end = end - 1;
        (*cmem).nr_ranges += 1;
    }

    0
}

/*
 * Add the "mem=size@start" command line parameter to command line, indicating the
 * memory region the new kernel can use to boot into.
 */
#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn cmdline_add_mem(cmdline_tmplen: *mut libc::c_ulong, modified_cmdline: *mut libc::c_char) {
    let mut mem_strlen: libc::c_int = 0;

    mem_strlen = sprintf(
        modified_cmdline.add(*cmdline_tmplen as usize),
        b"mem=0x%llx@0x%llx \0".as_ptr() as *const libc::c_char,
        crashk_res.end - crashk_res.start + 1,
        crashk_res.start,
    );
    *cmdline_tmplen += mem_strlen as libc::c_ulong;

    if crashk_low_res.end != 0 {
        mem_strlen = sprintf(
            modified_cmdline.add(*cmdline_tmplen as usize),
            b"mem=0x%llx@0x%llx \0".as_ptr() as *const libc::c_char,
            crashk_low_res.end - crashk_low_res.start + 1,
            crashk_low_res.start,
        );
        *cmdline_tmplen += mem_strlen as libc::c_ulong;
    }
}

/* Add the "elfcorehdr=size@start" command line parameter to command line. */
#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn cmdline_add_elfcorehdr(
    image: *mut Kimage,
    cmdline_tmplen: *mut libc::c_ulong,
    modified_cmdline: *mut libc::c_char,
    elfcorehdr_sz: libc::c_ulong,
) {
    let elfcorehdr_strlen = sprintf(
        modified_cmdline.add(*cmdline_tmplen as usize),
        b"elfcorehdr=0x%lx@0x%lx \0".as_ptr() as *const libc::c_char,
        elfcorehdr_sz,
        (*image).elf_load_addr,
    );
    *cmdline_tmplen += elfcorehdr_strlen as libc::c_ulong;
}

/*
 * Try to add the initrd to the image. If it is not possible to find valid
 * locations, this function will undo changes to the image and return non zero.
 */
pub unsafe fn load_other_segments(
    image: *mut Kimage,
    kernel_load_addr: libc::c_ulong,
    kernel_size: libc::c_ulong,
    mut initrd: *mut libc::c_char,
    initrd_len: libc::c_ulong,
    mut cmdline: *mut libc::c_char,
    cmdline_len: libc::c_ulong,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut cmdline_tmplen: libc::c_ulong = 0;
    let mut initrd_load_addr: libc::c_ulong = 0;
    let orig_segments = (*image).nr_segments;
    let mut modified_cmdline: *mut libc::c_char = core::ptr::null_mut();
    let mut kbuf: KexecBuf = core::mem::zeroed();

    kbuf.image = image;
    /* Don't allocate anything below the kernel */
    kbuf.buf_min = kernel_load_addr + kernel_size;

    modified_cmdline = kzalloc(COMMAND_LINE_SIZE, GFP_KERNEL);
    if modified_cmdline.is_null() {
        return -EINVAL;
    }

    cmdline_add_loader(&mut cmdline_tmplen, modified_cmdline);
    /* Ensure it's null terminated */
    *modified_cmdline.add(COMMAND_LINE_SIZE - 1) = 0;

    #[cfg(CONFIG_CRASH_DUMP)]
    if (*image).type_ == KEXEC_TYPE_CRASH {
        let mut headers: *mut core::ffi::c_void = core::ptr::null_mut();
        let mut headers_sz: libc::c_ulong = 0;

        ret = crash_prepare_headers(true, &mut headers, &mut headers_sz, core::ptr::null_mut());
        if ret < 0 {
            pr_err!(b"Preparing elf core header failed\n\0");
            goto_out_err!(out_err);
        }

        kbuf.buffer = headers;
        kbuf.bufsz = headers_sz;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        kbuf.memsz = headers_sz;
        kbuf.buf_align = SZ_64K;
        kbuf.buf_max = ULONG_MAX;
        kbuf.top_down = true;

        ret = kexec_add_buffer(&mut kbuf);
        if ret < 0 {
            vfree(headers);
            goto_out_err!(out_err);
        }
        (*image).elf_headers = headers;
        (*image).elf_load_addr = kbuf.mem;
        (*image).elf_headers_sz = headers_sz;

        kexec_dprintk!(
            b"Loaded elf core header at 0x%lx bufsz=0x%lx memsz=0x%lx\n\0",
            (*image).elf_load_addr, kbuf.bufsz, kbuf.memsz
        );
        cmdline_add_mem(&mut cmdline_tmplen, modified_cmdline);
        cmdline_add_elfcorehdr(&mut *image, &mut cmdline_tmplen, modified_cmdline, headers_sz);
    }

    /* Load initrd */
    if !initrd.is_null() {
        kbuf.buffer = initrd as *mut core::ffi::c_void;
        kbuf.bufsz = initrd_len;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        kbuf.memsz = initrd_len;
        kbuf.buf_align = 0;
        /* within 1GB-aligned window of up to 32GB in size */
        kbuf.buf_max = round_down(kernel_load_addr, SZ_1G) + SZ_1G * 32;
        kbuf.top_down = false;

        ret = kexec_add_buffer(&mut kbuf);
        if ret < 0 {
            goto_out_err!(out_err);
        }
        initrd_load_addr = kbuf.mem;

        kexec_dprintk!(
            b"Loaded initrd at 0x%lx bufsz=0x%lx memsz=0x%lx\n\0",
            initrd_load_addr, kbuf.bufsz, kbuf.memsz
        );
        cmdline_add_initrd(&mut *image, &mut cmdline_tmplen, modified_cmdline, initrd_load_addr);
    }

    if cmdline_len + cmdline_tmplen > COMMAND_LINE_SIZE {
        pr_err!(b"Appending command line exceeds COMMAND_LINE_SIZE\n\0");
        ret = -EINVAL;
        goto_out_err!(out_err);
    }

    core::ptr::copy_nonoverlapping(
        cmdline,
        modified_cmdline.add(cmdline_tmplen as usize),
        cmdline_len as usize,
    );
    cmdline = modified_cmdline;
    (*image).arch.cmdline_ptr = cmdline as libc::c_ulong;
    return 0;

    // C goto target: restore the original segment count and free the command line.
    out_err: {
        (*image).nr_segments = orig_segments;
        kfree(modified_cmdline);
        ret
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
