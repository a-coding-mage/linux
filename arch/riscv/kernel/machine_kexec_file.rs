// SPDX-License-Identifier: GPL-2.0-only
/*
 * kexec_file for riscv, use vmlinux as the dump-capture kernel image.
 *
 * Copyright (C) 2021 Huawei Technologies Co, Ltd.
 *
 * Author: Liao Chang (liaochang1@huawei.com)
 */

// Dependencies supplied by the surrounding kernel translation.

pub static KEXEC_FILE_LOADERS: [*const KexecFileOps; 3] = [
    &ELF_KEXEC_OPS,
    &IMAGE_KEXEC_OPS,
    core::ptr::null(),
];

pub unsafe fn arch_kimage_file_post_load_cleanup(image: *mut Kimage) -> i32 {
    kvfree((*image).arch.fdt);
    (*image).arch.fdt = core::ptr::null_mut();

    vfree((*image).elf_headers);
    (*image).elf_headers = core::ptr::null_mut();
    (*image).elf_headers_sz = 0;

    kexec_image_post_load_cleanup_default(image)
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn get_nr_ram_ranges_callback(_res: *mut Resource, arg: *mut core::ffi::c_void) -> i32 {
    let nr_ranges = arg as *mut u32;
    *nr_ranges += 1;
    0
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn arch_get_system_nr_ranges() -> u32 {
    let mut nr_ranges: u32 = 2 + crashk_cma_cnt; // For exclusion of crashkernel region
    walk_system_ram_res(
        0,
        -1,
        &mut nr_ranges as *mut _ as *mut core::ffi::c_void,
        Some(get_nr_ram_ranges_callback),
    );
    nr_ranges
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn prepare_elf64_ram_headers_callback(
    res: *mut Resource,
    arg: *mut core::ffi::c_void,
) -> i32 {
    let cmem = arg as *mut CrashMem;
    (*cmem).ranges[(*cmem).nr_ranges].start = (*res).start;
    (*cmem).ranges[(*cmem).nr_ranges].end = (*res).end;
    (*cmem).nr_ranges += 1;
    0
}

#[cfg(CONFIG_CRASH_DUMP)]
pub unsafe fn arch_crash_populate_cmem(cmem: *mut CrashMem) -> i32 {
    walk_system_ram_res(
        0,
        -1,
        cmem as *mut core::ffi::c_void,
        Some(prepare_elf64_ram_headers_callback),
    )
}

#[cfg(CONFIG_CRASH_DUMP)]
unsafe fn setup_kdump_cmdline(
    image: *mut Kimage,
    cmdline: *mut u8,
    cmdline_len: usize,
) -> *mut u8 {
    let cmdline_ptr = kzalloc(COMMAND_LINE_SIZE, GFP_KERNEL);
    if cmdline_ptr.is_null() {
        return core::ptr::null_mut();
    }

    let elfcorehdr_strlen = sprintf(
        cmdline_ptr,
        c"elfcorehdr=0x%lx ".as_ptr(),
        (*image).elf_load_addr,
    );

    if elfcorehdr_strlen as usize + cmdline_len > COMMAND_LINE_SIZE {
        pr_err(c"Appending elfcorehdr=<addr> exceeds cmdline size\n".as_ptr());
        kfree(cmdline_ptr);
        return core::ptr::null_mut();
    }

    core::ptr::copy_nonoverlapping(
        cmdline,
        cmdline_ptr.add(elfcorehdr_strlen as usize),
        cmdline_len,
    );
    // Ensure it's nul terminated
    *cmdline_ptr.add(COMMAND_LINE_SIZE - 1) = 0;
    cmdline_ptr
}

const RISCV_IMM_BITS: i32 = 12;
const RISCV_IMM_REACH: i64 = 1i64 << RISCV_IMM_BITS;

macro_rules! riscv_const_high_part { ($x:expr) => { (($x + (RISCV_IMM_REACH >> 1)) & !(RISCV_IMM_REACH - 1)) }; }
macro_rules! riscv_const_low_part { ($x:expr) => { ($x - riscv_const_high_part!($x)) }; }
macro_rules! encode_itype_imm { ($x:expr) => { (rv_x!($x, 0, 12) << 20) }; }
macro_rules! encode_btype_imm { ($x:expr) => { (rv_x!($x, 1, 4) << 8) | (rv_x!($x, 5, 6) << 25) | (rv_x!($x, 11, 1) << 7) | (rv_x!($x, 12, 1) << 31) }; }
macro_rules! encode_utype_imm { ($x:expr) => { (rv_x!($x, 12, 20) << 12) }; }
macro_rules! encode_jtype_imm { ($x:expr) => { (rv_x!($x, 1, 10) << 21) | (rv_x!($x, 11, 1) << 20) | (rv_x!($x, 12, 8) << 12) | (rv_x!($x, 20, 1) << 31) }; }
macro_rules! encode_cbtype_imm { ($x:expr) => { (rv_x!($x, 1, 2) << 3) | (rv_x!($x, 3, 2) << 10) | (rv_x!($x, 5, 1) << 2) | (rv_x!($x, 6, 2) << 5) | (rv_x!($x, 8, 1) << 12) }; }
macro_rules! encode_cjtype_imm { ($x:expr) => { (rv_x!($x, 1, 3) << 3) | (rv_x!($x, 4, 1) << 11) | (rv_x!($x, 5, 1) << 2) | (rv_x!($x, 6, 1) << 7) | (rv_x!($x, 7, 1) << 6) | (rv_x!($x, 8, 2) << 9) | (rv_x!($x, 10, 1) << 8) | (rv_x!($x, 11, 1) << 12) }; }
macro_rules! encode_ujtype_imm { ($x:expr) => { encode_utype_imm!(riscv_const_high_part!($x)) | (encode_itype_imm!(riscv_const_low_part!($x)) << 32) }; }
macro_rules! encode_uitype_imm { ($x:expr) => { encode_utype_imm!($x) | (encode_itype_imm!($x) << 32) }; }
macro_rules! clean_imm { ($ty:ident, $x:expr) => { (!encode_##$ty##_imm(u64::MAX)) & $x }; }

pub unsafe fn arch_kexec_apply_relocations_add(
    pi: *mut PurgatoryInfo,
    section: *mut ElfShdr,
    relsec: *const ElfShdr,
    symtab: *const ElfShdr,
) -> i32 {
    let sechdrs = (*pi).ehdr as *mut u8;
    let sechdrs = sechdrs.add((*pi).ehdr_read().e_shoff as usize) as *const ElfShdr;
    let strtab = (*pi).ehdr as *mut u8;
    let strtab = strtab.add((*sechdrs.add((*symtab).sh_link as usize)).sh_offset as usize);
    let shstrtab = (*pi).ehdr as *mut u8;
    let shstrtab = shstrtab.add((*sechdrs.add((*pi).ehdr_read().e_shstrndx as usize)).sh_offset as usize);
    let relas = ((*pi).ehdr as *mut u8).add((*relsec).sh_offset as usize) as *mut Elf64Rela;
    let count = (*relsec).sh_size as usize / core::mem::size_of::<Elf64Rela>();

    for i in 0..count {
        let sym = ((*pi).ehdr as *mut u8).add((*symtab).sh_offset as usize) as *mut ElfSym;
        let sym = sym.add(elf64_r_sym((*relas.add(i)).r_info) as usize);
        let name = if (*sym).st_name != 0 { strtab.add((*sym).st_name as usize) } else { shstrtab.add((*sechdrs.add((*sym).st_shndx as usize)).sh_name as usize) };
        let loc = ((*pi).purgatory_buf as *mut u8).add((*section).sh_offset as usize + (*relas.add(i)).r_offset as usize);
        let sec_base = if (*sym).st_shndx == SHN_ABS { 0 } else if (*sym).st_shndx as usize >= (*pi).ehdr_read().e_shnum as usize { pr_err(c"Invalid section %d for symbol %s\n".as_ptr(), (*sym).st_shndx, name); return -ENOEXEC; } else { (*pi).sechdrs.add((*sym).st_shndx as usize).read().sh_addr };
        let val = (*sym).st_value.wrapping_add(sec_base).wrapping_add((*relas.add(i)).r_addend as u64);
        let addr = (*section).sh_addr.wrapping_add((*relas.add(i)).r_offset);
        match elf64_r_type((*relas.add(i)).r_info) {
            R_RISCV_BRANCH => *(loc as *mut u32) = (*(loc as *mut u32) & !encode_btype_imm!(u64::MAX) as u32) | encode_btype_imm!(val.wrapping_sub(addr)) as u32,
            R_RISCV_JAL => *(loc as *mut u32) = (*(loc as *mut u32) & !encode_jtype_imm!(u64::MAX) as u32) | encode_jtype_imm!(val.wrapping_sub(addr)) as u32,
            R_RISCV_PCREL_HI20 | R_RISCV_CALL_PLT | R_RISCV_CALL => *(loc as *mut u64) = (*(loc as *mut u64) & !encode_uitype_imm!(u64::MAX)) | encode_ujtype_imm!(val.wrapping_sub(addr)),
            R_RISCV_RVC_BRANCH => *(loc as *mut u32) = (*(loc as *mut u32) & !encode_cbtype_imm!(u64::MAX) as u32) | encode_cbtype_imm!(val.wrapping_sub(addr)) as u32,
            R_RISCV_RVC_JUMP => *(loc as *mut u32) = (*(loc as *mut u32) & !encode_cjtype_imm!(u64::MAX) as u32) | encode_cjtype_imm!(val.wrapping_sub(addr)) as u32,
            R_RISCV_ADD16 => *(loc as *mut u16) = (*((loc) as *mut u16)).wrapping_add(val as u16),
            R_RISCV_SUB16 => *(loc as *mut u16) = (*((loc) as *mut u16)).wrapping_sub(val as u16),
            R_RISCV_ADD32 => *(loc as *mut u32) = (*((loc) as *mut u32)).wrapping_add(val as u32),
            R_RISCV_SUB32 => *(loc as *mut u32) = (*((loc) as *mut u32)).wrapping_sub(val as u32),
            R_RISCV_PCREL_LO12_I | R_RISCV_ALIGN | R_RISCV_RELAX => {},
            R_RISCV_64 => *(loc as *mut u64) = val,
            _ => { pr_err(c"Unknown rela relocation: %d\n".as_ptr(), elf64_r_type((*relas.add(i)).r_info)); return -ENOEXEC; }
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
