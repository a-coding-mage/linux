// SPDX-License-Identifier: GPL-2.0-only
/*
 * Load ELF vmlinux file for the kexec_file_load syscall.
 *
 * Author: Youling Tang <tangyouling@kylinos.cn>
 * Copyright (C) 2025 KylinSoft Corporation.
 */

// C includes: linux/elf.h, linux/kexec.h, linux/slab.h, linux/types.h,
// linux/memblock.h, and asm/setup.h.

// #define elf_kexec_probe kexec_elf_probe

unsafe fn _elf_kexec_load(
    image: *mut kimage,
    ehdr: *mut elfhdr,
    elf_info: *mut kexec_elf_info,
    kbuf: *mut kexec_buf,
    text_offset: *mut c_ulong,
) -> c_int {
    let mut ret: c_int = -1;

    /* Read in the PT_LOAD segments. */
    let mut i: c_int = 0;
    while i < (*ehdr).e_phnum as c_int {
        let phdr: *const elf_phdr = (*elf_info).proghdrs.add(i as usize);
        if (*phdr).p_type != PT_LOAD {
            i += 1;
            continue;
        }

        let mut size: size_t = (*phdr).p_filesz as size_t;
        if size > (*phdr).p_memsz as size_t {
            size = (*phdr).p_memsz as size_t;
        }

        (*kbuf).buffer = ((*elf_info).buffer as *mut u8).add((*phdr).p_offset as usize) as *mut c_void;
        (*kbuf).bufsz = size;
        (*kbuf).buf_align = (*phdr).p_align;
        *text_offset = __pa((*phdr).p_paddr);
        (*kbuf).buf_min = *text_offset;
        (*kbuf).memsz = ALIGN((*phdr).p_memsz, SZ_64K);
        (*kbuf).mem = KEXEC_BUF_MEM_UNKNOWN;
        ret = kexec_add_buffer(kbuf);
        if ret < 0 {
            break;
        }
        i += 1;
    }

    ret
}

unsafe fn elf_kexec_load(
    image: *mut kimage,
    kernel: *mut c_char,
    kernel_len: c_ulong,
    initrd: *mut c_char,
    initrd_len: c_ulong,
    cmdline: *mut c_char,
    cmdline_len: c_ulong,
) -> *mut c_void {
    let mut ret: c_int;
    let mut text_offset: c_ulong = 0;
    let mut kernel_segment_number: c_ulong;
    let mut ehdr: elfhdr = core::mem::zeroed();
    let mut kbuf: kexec_buf = core::mem::zeroed();
    let mut elf_info: kexec_elf_info = core::mem::zeroed();
    let kernel_segment: *mut kexec_segment;

    ret = kexec_build_elf_info(kernel, kernel_len, &mut ehdr, &mut elf_info);
    if ret < 0 {
        return ERR_PTR(ret);
    }

    /*
     * Load the kernel
     * FIXME: Non-relocatable kernel rejected for kexec_file (require CONFIG_RELOCATABLE)
     */
    kbuf.image = image;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;

    kernel_segment_number = (*image).nr_segments;

    ret = _elf_kexec_load(image, &mut ehdr, &mut elf_info, &mut kbuf, &mut text_offset);
    if ret < 0 {
        kexec_free_elf_info(&mut elf_info);
        return ERR_PTR(ret);
    }

    /* Load additional data */
    kernel_segment = (*image).segment.add(kernel_segment_number as usize);
    ret = load_other_segments(
        image,
        (*kernel_segment).mem,
        (*kernel_segment).memsz,
        initrd,
        initrd_len,
        cmdline,
        cmdline_len,
    );
    if ret < 0 {
        kexec_free_elf_info(&mut elf_info);
        return ERR_PTR(ret);
    }

    /* Make sure the second kernel jumps to the correct "kernel_entry". */
    (*image).start = (*kernel_segment).mem + __pa(ehdr.e_entry) - text_offset;

    kexec_dprintk(
        "Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
        (*kernel_segment).mem,
        kbuf.bufsz,
        (*kernel_segment).memsz,
    );

    kexec_free_elf_info(&mut elf_info);
    if ret != 0 { ERR_PTR(ret) } else { core::ptr::null_mut() }
}

pub static kexec_elf_ops: kexec_file_ops = kexec_file_ops {
    probe: kexec_elf_probe,
    load: elf_kexec_load,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
