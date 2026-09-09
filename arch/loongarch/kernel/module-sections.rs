// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    fn get_got_entry(val: Elf_Addr, sechdrs: *mut Elf_Shdr, got_sec: *mut mod_section) -> *mut got_entry;
    fn emit_got_entry(val: Elf_Addr) -> got_entry;
    fn get_plt_entry(val: Elf_Addr, sechdrs: *mut Elf_Shdr, plt_sec: *mut mod_section,
                     plt_idx_sec: *mut mod_section) -> *mut plt_entry;
    fn emit_plt_entry(val: Elf_Addr) -> plt_entry;
    fn emit_plt_idx_entry(val: Elf_Addr) -> plt_idx_entry;
    fn sort(base: *mut c_void, num: usize, size: usize,
            cmp: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
            swap: *mut c_void);
    fn cmp_int(x: u64, y: u64) -> c_int;
    fn strcmp(x: *const c_char, y: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn bug_on(condition: bool);
}

pub unsafe extern "C" fn module_emit_got_entry(
    mod_: *mut module,
    sechdrs: *mut Elf_Shdr,
    val: Elf_Addr,
) -> Elf_Addr {
    let got_sec = &mut (*mod_).arch.got;
    let i = got_sec.num_entries;
    let mut got = get_got_entry(val, sechdrs, got_sec);

    if !got.is_null() {
        return got as Elf_Addr;
    }

    /* There is no GOT entry for val yet, create a new one. */
    got = (*sechdrs.add(got_sec.shndx as usize)).sh_addr as *mut got_entry;
    *got.add(i as usize) = emit_got_entry(val);

    got_sec.num_entries += 1;
    if got_sec.num_entries > got_sec.max_entries {
        /*
         * This may happen when the module contains a GOT_HI20 without
         * a paired GOT_LO12. Such a module is broken, reject it.
         */
        pr_err(b"%s: module contains bad GOT relocation\0".as_ptr() as *const c_char, (*mod_).name);
        return 0;
    }

    &*got.add(i as usize) as *const got_entry as Elf_Addr
}

pub unsafe extern "C" fn module_emit_plt_entry(
    mod_: *mut module,
    sechdrs: *mut Elf_Shdr,
    val: Elf_Addr,
) -> Elf_Addr {
    let plt_sec = &mut (*mod_).arch.plt;
    let plt_idx_sec = &mut (*mod_).arch.plt_idx;
    let mut plt = get_plt_entry(val, sechdrs, plt_sec, plt_idx_sec);
    let plt_idx: *mut plt_idx_entry;

    if !plt.is_null() {
        return plt as Elf_Addr;
    }

    let nr = plt_sec.num_entries;

    /* There is no duplicate entry, create a new one */
    plt = (*sechdrs.add(plt_sec.shndx as usize)).sh_addr as *mut plt_entry;
    *plt.add(nr as usize) = emit_plt_entry(val);
    plt_idx = (*sechdrs.add(plt_idx_sec.shndx as usize)).sh_addr as *mut plt_idx_entry;
    *plt_idx.add(nr as usize) = emit_plt_idx_entry(val);

    plt_sec.num_entries += 1;
    plt_idx_sec.num_entries += 1;
    bug_on(plt_sec.num_entries > plt_sec.max_entries);

    &*plt.add(nr as usize) as *const plt_entry as Elf_Addr
}

unsafe extern "C" fn compare_rela(x: *const c_void, y: *const c_void) -> c_int {
    let rela_x = x as *const Elf_Rela;
    let rela_y = y as *const Elf_Rela;

    let mut ret = cmp_int((*rela_x).r_info, (*rela_y).r_info);
    if ret == 0 {
        ret = cmp_int((*rela_x).r_addend, (*rela_y).r_addend);
    }
    ret
}

unsafe fn count_max_entries(
    relas: *mut Elf_Rela,
    num: c_int,
    plts: *mut c_uint,
    gots: *mut c_uint,
) {
    sort(relas as *mut c_void, num as usize, core::mem::size_of::<Elf_Rela>(), compare_rela, core::ptr::null_mut());

    for i in 0..num as usize {
        if i != 0 && compare_rela(relas.add(i - 1) as *const c_void, relas.add(i) as *const c_void) == 0 {
            continue;
        }

        match ELF_R_TYPE((*relas.add(i)).r_info) {
            R_LARCH_SOP_PUSH_PLT_PCREL | R_LARCH_B26 => *plts += 1,
            R_LARCH_GOT_PC_HI20 | R_LARCH_GOT_PCADD_HI20 => *gots += 1,
            _ => { /* Do nothing. */ }
        }
    }
}

pub unsafe extern "C" fn module_frob_arch_sections(
    ehdr: *mut Elf_Ehdr,
    sechdrs: *mut Elf_Shdr,
    secstrings: *mut c_char,
    mod_: *mut module,
) -> c_int {
    let mut num_plts: c_uint = 0;
    let mut num_gots: c_uint = 0;
    let mut got_sec: *mut Elf_Shdr;
    let mut plt_sec: *mut Elf_Shdr;
    let mut plt_idx_sec: *mut Elf_Shdr;
    let mut tramp: *mut Elf_Shdr = core::ptr::null_mut();

    /* Find the empty .plt sections. */
    for i in 0..(*ehdr).e_shnum as usize {
        let name = secstrings.add((*sechdrs.add(i)).sh_name as usize);
        if strcmp(name, b".got\0".as_ptr() as *const c_char) == 0 {
            (*mod_).arch.got.shndx = i as _;
        } else if strcmp(name, b".plt\0".as_ptr() as *const c_char) == 0 {
            (*mod_).arch.plt.shndx = i as _;
        } else if strcmp(name, b".plt.idx\0".as_ptr() as *const c_char) == 0 {
            (*mod_).arch.plt_idx.shndx = i as _;
        } else if strcmp(name, b".ftrace_trampoline\0".as_ptr() as *const c_char) == 0 {
            tramp = sechdrs.add(i);
        }
    }

    if (*mod_).arch.got.shndx == 0 { pr_err(b"%s: module GOT section(s) missing\0".as_ptr() as *const c_char, (*mod_).name); return -ENOEXEC; }
    if (*mod_).arch.plt.shndx == 0 { pr_err(b"%s: module PLT section(s) missing\0".as_ptr() as *const c_char, (*mod_).name); return -ENOEXEC; }
    if (*mod_).arch.plt_idx.shndx == 0 { pr_err(b"%s: module PLT.IDX section(s) missing\0".as_ptr() as *const c_char, (*mod_).name); return -ENOEXEC; }

    /* Calculate the maxinum number of entries */
    for i in 0..(*ehdr).e_shnum as usize {
        if (*sechdrs.add(i)).sh_type != SHT_RELA { continue; }
        let dst_sec = sechdrs.add((*sechdrs.add(i)).sh_info as usize);
        if (*dst_sec).sh_flags & SHF_EXECINSTR == 0 { continue; }
        let relas = (ehdr as *mut u8).add((*sechdrs.add(i)).sh_offset as usize) as *mut Elf_Rela;
        count_max_entries(relas, ((*sechdrs.add(i)).sh_size as usize / core::mem::size_of::<Elf_Rela>()) as c_int, &mut num_plts, &mut num_gots);
    }

    got_sec = sechdrs.add((*mod_).arch.got.shndx as usize);
    (*got_sec).sh_type = SHT_NOBITS; (*got_sec).sh_flags = SHF_ALLOC; (*got_sec).sh_addralign = L1_CACHE_BYTES; (*got_sec).sh_size = (num_gots + 1) as _ * core::mem::size_of::<got_entry>() as _;
    (*mod_).arch.got.num_entries = 0; (*mod_).arch.got.max_entries = num_gots;
    plt_sec = sechdrs.add((*mod_).arch.plt.shndx as usize);
    (*plt_sec).sh_type = SHT_NOBITS; (*plt_sec).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*plt_sec).sh_addralign = L1_CACHE_BYTES; (*plt_sec).sh_size = (num_plts + 1) as _ * core::mem::size_of::<plt_entry>() as _;
    (*mod_).arch.plt.num_entries = 0; (*mod_).arch.plt.max_entries = num_plts;
    plt_idx_sec = sechdrs.add((*mod_).arch.plt_idx.shndx as usize);
    (*plt_idx_sec).sh_type = SHT_NOBITS; (*plt_idx_sec).sh_flags = SHF_ALLOC; (*plt_idx_sec).sh_addralign = L1_CACHE_BYTES; (*plt_idx_sec).sh_size = (num_plts + 1) as _ * core::mem::size_of::<plt_idx_entry>() as _;
    (*mod_).arch.plt_idx.num_entries = 0; (*mod_).arch.plt_idx.max_entries = num_plts;
    if !tramp.is_null() { (*tramp).sh_type = SHT_NOBITS; (*tramp).sh_flags = SHF_EXECINSTR | SHF_ALLOC; (*tramp).sh_addralign = core::mem::align_of::<plt_entry>() as _; (*tramp).sh_size = NR_FTRACE_PLTS as _ * core::mem::size_of::<plt_entry>() as _; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
