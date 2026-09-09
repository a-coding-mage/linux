// SPDX-License-Identifier: GPL-2.0-only
/*
 * Load ELF vmlinux file for the kexec_file_load syscall.
 *
 * Copyright (C) 2021 Huawei Technologies Co, Ltd.
 *
 * Author: Liao Chang (liaochang1@huawei.com)
 *
 * Based on kexec-tools' kexec-elf-riscv.c, heavily modified
 * for kernel.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn riscv_kexec_elf_load(
    image: *mut kimage,
    ehdr: *mut elfhdr,
    elf_info: *mut kexec_elf_info,
    old_pbase: c_ulong,
    new_pbase: c_ulong,
) -> c_int {
    let mut ret: c_int = 0;
    let mut kbuf: kexec_buf = core::mem::zeroed();
    let mut phdr: *const elf_phdr;

    kbuf.image = image;

    for i in 0..(*ehdr).e_phnum {
        phdr = &(*elf_info).proghdrs[i as usize];
        if (*phdr).p_type != PT_LOAD {
            continue;
        }

        kbuf.buffer = ((*elf_info).buffer as *mut u8).add((*phdr).p_offset as usize)
            as *mut core::ffi::c_void;
        kbuf.bufsz = core::cmp::min((*phdr).p_filesz, (*phdr).p_memsz);
        kbuf.buf_align = (*phdr).p_align;
        kbuf.mem = (*phdr).p_paddr.wrapping_sub(old_pbase).wrapping_add(new_pbase);
        kbuf.memsz = (*phdr).p_memsz;
        kbuf.top_down = false;
        ret = kexec_add_buffer(&mut kbuf);
        if ret != 0 {
            break;
        }
    }

    ret
}

/*
 * Go through the available phsyical memory regions and find one that hold
 * an image of the specified size.
 */
unsafe fn elf_find_pbase(
    image: *mut kimage,
    kernel_len: c_ulong,
    ehdr: *mut elfhdr,
    elf_info: *mut kexec_elf_info,
    old_pbase: *mut c_ulong,
    new_pbase: *mut c_ulong,
) -> c_int {
    let mut ret: c_int;
    let mut kbuf: kexec_buf = core::mem::zeroed();
    let mut lowest_paddr: c_ulong = ULONG_MAX;
    let mut lowest_vaddr: c_ulong = ULONG_MAX;

    for i in 0..(*ehdr).e_phnum {
        let phdr = &(*elf_info).proghdrs[i as usize];
        if phdr.p_type != PT_LOAD {
            continue;
        }

        if lowest_paddr > phdr.p_paddr {
            lowest_paddr = phdr.p_paddr;
        }

        if lowest_vaddr > phdr.p_vaddr {
            lowest_vaddr = phdr.p_vaddr;
        }
    }

    kbuf.image = image;
    kbuf.buf_min = lowest_paddr;
    kbuf.buf_max = ULONG_MAX;

    /*
     * Current riscv boot protocol requires 2MB alignment for
     * RV64 and 4MB alignment for RV32
     *
     */
    kbuf.buf_align = PMD_SIZE;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = ALIGN(kernel_len, PAGE_SIZE);
    kbuf.cma = core::ptr::null_mut();
    kbuf.top_down = false;
    ret = arch_kexec_locate_mem_hole(&mut kbuf);
    if ret == 0 {
        *old_pbase = lowest_paddr;
        *new_pbase = kbuf.mem;
        (*image).start = (*ehdr).e_entry.wrapping_sub(lowest_vaddr).wrapping_add(kbuf.mem);
    }
    ret
}

unsafe fn elf_kexec_load(
    image: *mut kimage,
    kernel_buf: *mut c_char,
    kernel_len: c_ulong,
    initrd: *mut c_char,
    initrd_len: c_ulong,
    cmdline: *mut c_char,
    cmdline_len: c_ulong,
) -> *mut core::ffi::c_void {
    let mut ret: c_int;
    let mut old_kernel_pbase: c_ulong = ULONG_MAX;
    let mut new_kernel_pbase: c_ulong = 0;
    let mut ehdr: elfhdr = core::mem::zeroed();
    let mut elf_info: kexec_elf_info = core::mem::zeroed();

    ret = kexec_build_elf_info(kernel_buf, kernel_len, &mut ehdr, &mut elf_info);
    if ret != 0 {
        return ERR_PTR(ret);
    }

    ret = elf_find_pbase(
        image,
        kernel_len,
        &mut ehdr,
        &mut elf_info,
        &mut old_kernel_pbase,
        &mut new_kernel_pbase,
    );
    if ret != 0 {
        kexec_free_elf_info(&mut elf_info);
        return ERR_PTR(ret);
    }

    /* Add the kernel binary to the image */
    ret = riscv_kexec_elf_load(
        image,
        &mut ehdr,
        &mut elf_info,
        old_kernel_pbase,
        new_kernel_pbase,
    );
    if ret != 0 {
        kexec_free_elf_info(&mut elf_info);
        return ERR_PTR(ret);
    }

    ret = load_extra_segments(
        image,
        (*image).start,
        kernel_len,
        initrd,
        initrd_len,
        cmdline,
        cmdline_len,
    );
    kexec_free_elf_info(&mut elf_info);
    if ret != 0 { ERR_PTR(ret) } else { core::ptr::null_mut() }
}

const elf_kexec_ops: kexec_file_ops = kexec_file_ops {
    probe: kexec_elf_probe,
    load: elf_kexec_load,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
