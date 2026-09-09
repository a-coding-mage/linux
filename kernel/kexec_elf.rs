// SPDX-License-Identifier: GPL-2.0-only
/*
 * Load ELF vmlinux file for the kexec_file_load syscall.
 *
 * Copyright (C) 2004  Adam Litke (agl@us.ibm.com)
 * Copyright (C) 2004  IBM Corp.
 * Copyright (C) 2005  R Sharada (sharada@in.ibm.com)
 * Copyright (C) 2006  Mohan Kumar M (mohan@in.ibm.com)
 * Copyright (C) 2016  IBM Corporation
 *
 * Based on kexec-tools' kexec-elf-exec.c and kexec-elf-ppc64.c.
 * Heavily modified for the kernel by
 * Thiago Jung Bauermann <bauerman@linux.vnet.ibm.com>.
 */

// #define pr_fmt(fmt) "kexec_elf: " fmt
// Required definitions are supplied by the surrounding kernel translation.

unsafe fn elf_is_elf_file(ehdr: *const elfhdr) -> bool {
    core::slice::from_raw_parts((*ehdr).e_ident.as_ptr(), SELFMAG as usize)
        == ELFMAG
}

unsafe fn elf64_to_cpu(ehdr: *const elfhdr, mut value: u64) -> u64 {
    if (*ehdr).e_ident[EI_DATA] as usize == ELFDATA2LSB as usize {
        value = value.to_le();
    } else if (*ehdr).e_ident[EI_DATA] as usize == ELFDATA2MSB as usize {
        value = value.to_be();
    }
    value
}

unsafe fn elf32_to_cpu(ehdr: *const elfhdr, mut value: u32) -> u32 {
    if (*ehdr).e_ident[EI_DATA] as usize == ELFDATA2LSB as usize {
        value = value.to_le();
    } else if (*ehdr).e_ident[EI_DATA] as usize == ELFDATA2MSB as usize {
        value = value.to_be();
    }
    value
}

unsafe fn elf16_to_cpu(ehdr: *const elfhdr, mut value: u16) -> u16 {
    if (*ehdr).e_ident[EI_DATA] as usize == ELFDATA2LSB as usize {
        value = value.to_le();
    } else if (*ehdr).e_ident[EI_DATA] as usize == ELFDATA2MSB as usize {
        value = value.to_be();
    }
    value
}

/**
 * elf_is_ehdr_sane - check that it is safe to use the ELF header
 * @buf_len: size of the buffer in which the ELF file is loaded.
 */
unsafe fn elf_is_ehdr_sane(ehdr: *const elfhdr, buf_len: usize) -> bool {
    if (*ehdr).e_phnum > 0 && (*ehdr).e_phentsize as usize != core::mem::size_of::<elf_phdr>() {
        pr_debug!("Bad program header size.\n"); return false;
    } else if (*ehdr).e_shnum > 0 && (*ehdr).e_shentsize as usize != core::mem::size_of::<elf_shdr>() {
        pr_debug!("Bad section header size.\n"); return false;
    } else if (*ehdr).e_ident[EI_VERSION] != EV_CURRENT || (*ehdr).e_version != EV_CURRENT {
        pr_debug!("Unknown ELF version.\n"); return false;
    }
    if (*ehdr).e_phoff > 0 && (*ehdr).e_phnum > 0 {
        let phdr_size = core::mem::size_of::<elf_phdr>() * (*ehdr).e_phnum as usize;
        let end = (*ehdr).e_phoff as usize + phdr_size;
        if end < (*ehdr).e_phoff as usize { pr_debug!("Program headers at invalid location.\n"); return false; }
        else if end > buf_len { pr_debug!("Program headers truncated.\n"); return false; }
    }
    if (*ehdr).e_shoff > 0 && (*ehdr).e_shnum > 0 {
        let shdr_size = core::mem::size_of::<elf_shdr>() * (*ehdr).e_shnum as usize;
        let end = (*ehdr).e_shoff as usize + shdr_size;
        if end < (*ehdr).e_shoff as usize { pr_debug!("Section headers at invalid location.\n"); return false; }
        else if end > buf_len { pr_debug!("Section headers truncated.\n"); return false; }
    }
    true
}

unsafe fn elf_read_ehdr(buf: *const i8, len: usize, ehdr: *mut elfhdr) -> i32 {
    if len < core::mem::size_of::<elfhdr>() { pr_debug!("Buffer is too small to hold ELF header.\n"); return -ENOEXEC; }
    core::ptr::write_bytes(ehdr, 0, 1);
    core::ptr::copy_nonoverlapping(buf as *const u8, (*ehdr).e_ident.as_mut_ptr(), (*ehdr).e_ident.len());
    if !elf_is_elf_file(ehdr) { pr_debug!("No ELF header magic.\n"); return -ENOEXEC; }
    if (*ehdr).e_ident[EI_CLASS] != ELF_CLASS { pr_debug!("Not a supported ELF class.\n"); return -ENOEXEC; }
    else if (*ehdr).e_ident[EI_DATA] != ELFDATA2LSB && (*ehdr).e_ident[EI_DATA] != ELFDATA2MSB { pr_debug!("Not a supported ELF data format.\n"); return -ENOEXEC; }
    let b = buf as *const elfhdr;
    if elf16_to_cpu(ehdr, (*b).e_ehsize) as usize != core::mem::size_of::<elfhdr>() { pr_debug!("Bad ELF header size.\n"); return -ENOEXEC; }
    (*ehdr).e_type = elf16_to_cpu(ehdr, (*b).e_type); (*ehdr).e_machine = elf16_to_cpu(ehdr, (*b).e_machine);
    (*ehdr).e_version = elf32_to_cpu(ehdr, (*b).e_version); (*ehdr).e_flags = elf32_to_cpu(ehdr, (*b).e_flags);
    (*ehdr).e_phentsize = elf16_to_cpu(ehdr, (*b).e_phentsize); (*ehdr).e_phnum = elf16_to_cpu(ehdr, (*b).e_phnum);
    (*ehdr).e_shentsize = elf16_to_cpu(ehdr, (*b).e_shentsize); (*ehdr).e_shnum = elf16_to_cpu(ehdr, (*b).e_shnum); (*ehdr).e_shstrndx = elf16_to_cpu(ehdr, (*b).e_shstrndx);
    match (*ehdr).e_ident[EI_CLASS] {
        ELFCLASS64 => { (*ehdr).e_entry = elf64_to_cpu(ehdr, (*b).e_entry); (*ehdr).e_phoff = elf64_to_cpu(ehdr, (*b).e_phoff); (*ehdr).e_shoff = elf64_to_cpu(ehdr, (*b).e_shoff); }
        ELFCLASS32 => { (*ehdr).e_entry = elf32_to_cpu(ehdr, (*b).e_entry) as _; (*ehdr).e_phoff = elf32_to_cpu(ehdr, (*b).e_phoff) as _; (*ehdr).e_shoff = elf32_to_cpu(ehdr, (*b).e_shoff) as _; }
        _ => { pr_debug!("Unknown ELF class.\n"); return -EINVAL; }
    }
    if elf_is_ehdr_sane(ehdr, len) { 0 } else { -ENOEXEC }
}

/** elf_is_phdr_sane - check that it is safe to use the program header */
unsafe fn elf_is_phdr_sane(phdr: *const elf_phdr, buf_len: usize) -> bool {
    if (*phdr).p_offset + (*phdr).p_filesz < (*phdr).p_offset { pr_debug!("ELF segment location wraps around.\n"); false }
    else if (*phdr).p_offset + (*phdr).p_filesz > buf_len as _ { pr_debug!("ELF segment not in file.\n"); false }
    else if (*phdr).p_paddr + (*phdr).p_memsz < (*phdr).p_paddr { pr_debug!("ELF segment address wraps around.\n"); false } else { true }
}

unsafe fn elf_read_phdr(buf: *const i8, len: usize, elf_info: *mut kexec_elf_info, idx: usize) -> i32 {
    let phdr = &mut (*elf_info).proghdrs[idx];
    let ehdr = (*elf_info).ehdr;
    let buf_phdr = (buf as *const u8).add((*ehdr).e_phoff as usize + idx * core::mem::size_of::<elf_phdr>()) as *const elf_phdr;
    phdr.p_type = elf32_to_cpu(ehdr, (*buf_phdr).p_type); phdr.p_flags = elf32_to_cpu(ehdr, (*buf_phdr).p_flags);
    match (*ehdr).e_ident[EI_CLASS] {
        ELFCLASS64 => { phdr.p_offset=elf64_to_cpu(ehdr,(*buf_phdr).p_offset); phdr.p_paddr=elf64_to_cpu(ehdr,(*buf_phdr).p_paddr); phdr.p_vaddr=elf64_to_cpu(ehdr,(*buf_phdr).p_vaddr); phdr.p_filesz=elf64_to_cpu(ehdr,(*buf_phdr).p_filesz); phdr.p_memsz=elf64_to_cpu(ehdr,(*buf_phdr).p_memsz); phdr.p_align=elf64_to_cpu(ehdr,(*buf_phdr).p_align); }
        ELFCLASS32 => { phdr.p_offset=elf32_to_cpu(ehdr,(*buf_phdr).p_offset) as _; phdr.p_paddr=elf32_to_cpu(ehdr,(*buf_phdr).p_paddr) as _; phdr.p_vaddr=elf32_to_cpu(ehdr,(*buf_phdr).p_vaddr) as _; phdr.p_filesz=elf32_to_cpu(ehdr,(*buf_phdr).p_filesz) as _; phdr.p_memsz=elf32_to_cpu(ehdr,(*buf_phdr).p_memsz) as _; phdr.p_align=elf32_to_cpu(ehdr,(*buf_phdr).p_align) as _; }
        _ => { pr_debug!("Unknown ELF class.\n"); return -EINVAL; }
    }
    if elf_is_phdr_sane(phdr, len) { 0 } else { -ENOEXEC }
}

unsafe fn elf_read_phdrs(buf: *const i8, len: usize, elf_info: *mut kexec_elf_info) -> i32 {
    let phdr_size = core::mem::size_of::<elf_phdr>() * (*(*elf_info).ehdr).e_phnum as usize;
    (*elf_info).proghdrs = kzalloc(phdr_size, GFP_KERNEL);
    if (*elf_info).proghdrs.is_null() { return -ENOMEM; }
    for i in 0..(*(*elf_info).ehdr).e_phnum as usize { let ret = elf_read_phdr(buf, len, elf_info, i); if ret != 0 { kfree((*elf_info).proghdrs); (*elf_info).proghdrs = core::ptr::null_mut(); return ret; } }
    0
}

unsafe fn elf_read_from_buffer(buf: *const i8, len: usize, ehdr: *mut elfhdr, elf_info: *mut kexec_elf_info) -> i32 {
    let ret = elf_read_ehdr(buf, len, ehdr); if ret != 0 { return ret; }
    (*elf_info).buffer = buf; (*elf_info).ehdr = ehdr;
    if (*ehdr).e_phoff > 0 && (*ehdr).e_phnum > 0 { let ret = elf_read_phdrs(buf, len, elf_info); if ret != 0 { return ret; } }
    0
}

pub unsafe fn kexec_free_elf_info(elf_info: *mut kexec_elf_info) { kfree((*elf_info).proghdrs); core::ptr::write_bytes(elf_info, 0, 1); }

pub unsafe fn kexec_build_elf_info(buf: *const i8, len: usize, ehdr: *mut elfhdr, elf_info: *mut kexec_elf_info) -> i32 {
    let ret = elf_read_from_buffer(buf, len, ehdr, elf_info); if ret != 0 { return ret; }
    if (*ehdr).e_type != ET_EXEC && (*ehdr).e_type != ET_DYN { pr_err!("Not an ELF executable.\n"); }
    else if (*elf_info).proghdrs.is_null() { pr_err!("No ELF program header.\n"); }
    else { for i in 0..(*ehdr).e_phnum as usize { if (*elf_info).proghdrs[i].p_type == PT_INTERP { pr_err!("Requires an ELF interpreter.\n"); break; } if i + 1 == (*ehdr).e_phnum as usize { return 0; } } }
    kexec_free_elf_info(elf_info); -ENOEXEC
}

pub unsafe fn kexec_elf_probe(buf: *const i8, len: c_ulong) -> i32 {
    let mut ehdr: elfhdr = core::mem::zeroed(); let mut elf_info: kexec_elf_info = core::mem::zeroed();
    let ret = kexec_build_elf_info(buf, len as usize, &mut ehdr, &mut elf_info); if ret != 0 { return ret; }
    kexec_free_elf_info(&mut elf_info); if elf_check_arch(&ehdr) { 0 } else { -ENOEXEC }
}

/** kexec_elf_load - load ELF executable image */
pub unsafe fn kexec_elf_load(image: *mut kimage, ehdr: *mut elfhdr, elf_info: *mut kexec_elf_info, kbuf: *mut kexec_buf, lowest_load_addr: *mut c_ulong) -> i32 {
    let mut lowest_addr = ULONG_MAX; let mut ret: i32 = 0;
    for i in 0..(*ehdr).e_phnum as usize {
        let phdr = &(*elf_info).proghdrs[i]; if phdr.p_type != PT_LOAD { continue; }
        let size = core::cmp::min(phdr.p_filesz, phdr.p_memsz);
        (*kbuf).buffer = ((*elf_info).buffer as *mut u8).add(phdr.p_offset as usize) as *mut c_void;
        (*kbuf).bufsz=size as _; (*kbuf).memsz=phdr.p_memsz as _; (*kbuf).buf_align=phdr.p_align as _; (*kbuf).buf_min=phdr.p_paddr as _; (*kbuf).mem=KEXEC_BUF_MEM_UNKNOWN;
        ret=kexec_add_buffer(kbuf); if ret != 0 { break; } let load_addr=(*kbuf).mem; if load_addr<lowest_addr { lowest_addr=load_addr; }
    }
    *lowest_load_addr=lowest_addr; ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
