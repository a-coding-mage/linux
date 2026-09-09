/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2014-2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 *
 * Copyright (C) 2018 Andes Technology Corporation <zong@andestech.com>
 */

// Kernel headers and build-time definitions are supplied by the surrounding
// Rust kernel environment.

pub unsafe fn module_emit_got_entry(mod_: *mut module, val: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let got_sec: *mut mod_section = &mut (*mod_).arch.got;
    let i = (*got_sec).num_entries as usize;
    let mut got: *mut got_entry = get_got_entry(val, got_sec);

    if !got.is_null() {
        return got as ::core::ffi::c_ulong;
    }

    /* There is no duplicate entry, create a new one */
    got = (*(*got_sec).shdr).sh_addr as *mut got_entry;
    *got.add(i) = emit_got_entry(val);

    (*got_sec).num_entries += 1;
    BUG_ON((*got_sec).num_entries > (*got_sec).max_entries);

    got.add(i) as ::core::ffi::c_ulong
}

pub unsafe fn module_emit_plt_entry(mod_: *mut module, val: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let got_plt_sec: *mut mod_section = &mut (*mod_).arch.got_plt;
    let mut got_plt: *mut got_entry;
    let plt_sec: *mut mod_section = &mut (*mod_).arch.plt;
    let mut plt: *mut plt_entry = get_plt_entry(val, plt_sec, got_plt_sec);
    let i = (*plt_sec).num_entries as usize;

    if !plt.is_null() {
        return plt as ::core::ffi::c_ulong;
    }

    /* There is no duplicate entry, create a new one */
    got_plt = (*(*got_plt_sec).shdr).sh_addr as *mut got_entry;
    *got_plt.add(i) = emit_got_entry(val);
    plt = (*(*plt_sec).shdr).sh_addr as *mut plt_entry;
    *plt.add(i) = emit_plt_entry(
        val,
        plt.add(i) as ::core::ffi::c_ulong,
        got_plt.add(i) as ::core::ffi::c_ulong,
    );

    (*plt_sec).num_entries += 1;
    (*got_plt_sec).num_entries += 1;
    BUG_ON((*plt_sec).num_entries > (*plt_sec).max_entries);

    plt.add(i) as ::core::ffi::c_ulong
}

unsafe extern "C" fn cmp_rela(a: *const ::core::ffi::c_void, b: *const ::core::ffi::c_void) -> i32 {
    let x = a as *const Elf_Rela;
    let y = b as *const Elf_Rela;

    /* sort by type, symbol index and addend */
    let mut i = cmp_int((*x).r_info, (*y).r_info);
    if i == 0 {
        i = cmp_int((*x).r_addend, (*y).r_addend);
    }
    i
}

unsafe fn duplicate_rela(rela: *const Elf_Rela, idx: i32) -> bool {
    /*
     * Entries are sorted by type, symbol index and addend. That means
     * that, if a duplicate entry exists, it must be in the preceding slot.
     */
    idx > 0 && cmp_rela(
        rela.add(idx as usize) as *const ::core::ffi::c_void,
        rela.add((idx - 1) as usize) as *const ::core::ffi::c_void,
    ) == 0
}

unsafe fn count_max_entries(
    relas: *const Elf_Rela,
    num: usize,
    plts: *mut u32,
    gots: *mut u32,
) {
    for i in 0..num {
        if duplicate_rela(relas, i as i32) {
            continue;
        }

        match ELF_R_TYPE((*relas.add(i)).r_info) {
            R_RISCV_CALL_PLT | R_RISCV_PLT32 => *plts += 1,
            R_RISCV_GOT_HI20 => *gots += 1,
            _ => unreachable!(),
        }
    }
}

unsafe fn rela_needs_plt_got_entry(rela: *const Elf_Rela) -> bool {
    match ELF_R_TYPE((*rela).r_info) {
        R_RISCV_CALL_PLT | R_RISCV_GOT_HI20 | R_RISCV_PLT32 => true,
        _ => false,
    }
}

pub unsafe fn module_frob_arch_sections(
    ehdr: *mut Elf_Ehdr,
    sechdrs: *mut Elf_Shdr,
    secstrings: *mut ::core::ffi::c_char,
    mod_: *mut module,
) -> i32 {
    let mut num_scratch_relas: usize = 0;
    let mut num_plts: u32 = 0;
    let mut num_gots: u32 = 0;
    let mut scratch: *mut Elf_Rela = core::ptr::null_mut();
    let mut scratch_size: usize = 0;

    /* Find the empty .got and .plt sections. */
    for i in 0..(*ehdr).e_shnum as usize {
        let name = secstrings.add((*sechdrs.add(i)).sh_name as usize);
        if strcmp(name, b".plt\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            (*mod_).arch.plt.shdr = sechdrs.add(i);
        } else if strcmp(name, b".got\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            (*mod_).arch.got.shdr = sechdrs.add(i);
        } else if strcmp(name, b".got.plt\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
            (*mod_).arch.got_plt.shdr = sechdrs.add(i);
        }
    }

    if (*mod_).arch.plt.shdr.is_null() {
        pr_err(b"%s: module PLT section(s) missing\n\0".as_ptr());
        return -ENOEXEC;
    }
    if (*mod_).arch.got.shdr.is_null() {
        pr_err(b"%s: module GOT section(s) missing\n\0".as_ptr());
        return -ENOEXEC;
    }
    if (*mod_).arch.got_plt.shdr.is_null() {
        pr_err(b"%s: module GOT.PLT section(s) missing\n\0".as_ptr());
        return -ENOEXEC;
    }

    /* Calculate the maximum number of entries */
    for i in 0..(*ehdr).e_shnum as usize {
        let num_relas = (*sechdrs.add(i)).sh_size as usize / core::mem::size_of::<Elf_Rela>();
        let relas = (ehdr as *mut u8).add((*sechdrs.add(i)).sh_offset as usize) as *mut Elf_Rela;
        let dst_sec = sechdrs.add((*sechdrs.add(i)).sh_info as usize);

        if (*sechdrs.add(i)).sh_type != SHT_RELA || ((*dst_sec).sh_flags & SHF_EXECINSTR) == 0 {
            continue;
        }

        let scratch_size_needed = (num_scratch_relas + num_relas) * core::mem::size_of::<Elf_Rela>();
        if scratch_size_needed > scratch_size {
            scratch_size = scratch_size_needed;
            let new_scratch = kvrealloc(scratch, scratch_size, GFP_KERNEL);
            if new_scratch.is_null() {
                kvfree(scratch as *mut ::core::ffi::c_void);
                return -ENOMEM;
            }
            scratch = new_scratch;
        }

        for j in 0..num_relas {
            if rela_needs_plt_got_entry(relas.add(j)) {
                *scratch.add(num_scratch_relas) = *relas.add(j);
                num_scratch_relas += 1;
            }
        }
    }

    if !scratch.is_null() {
        sort(scratch as *mut ::core::ffi::c_void, num_scratch_relas, core::mem::size_of::<Elf_Rela>(), Some(cmp_rela), core::ptr::null_mut());
        count_max_entries(scratch, num_scratch_relas, &mut num_plts, &mut num_gots);
        kvfree(scratch as *mut ::core::ffi::c_void);
    }

    (*(*mod_).arch.plt.shdr).sh_type = SHT_NOBITS;
    (*(*mod_).arch.plt.shdr).sh_flags = SHF_EXECINSTR | SHF_ALLOC;
    (*(*mod_).arch.plt.shdr).sh_addralign = L1_CACHE_BYTES;
    (*(*mod_).arch.plt.shdr).sh_size = (num_plts as usize + 1) * core::mem::size_of::<plt_entry>();
    (*mod_).arch.plt.num_entries = 0;
    (*mod_).arch.plt.max_entries = num_plts;

    (*(*mod_).arch.got.shdr).sh_type = SHT_NOBITS;
    (*(*mod_).arch.got.shdr).sh_flags = SHF_ALLOC;
    (*(*mod_).arch.got.shdr).sh_addralign = L1_CACHE_BYTES;
    (*(*mod_).arch.got.shdr).sh_size = (num_gots as usize + 1) * core::mem::size_of::<got_entry>();
    (*mod_).arch.got.num_entries = 0;
    (*mod_).arch.got.max_entries = num_gots;

    (*(*mod_).arch.got_plt.shdr).sh_type = SHT_NOBITS;
    (*(*mod_).arch.got_plt.shdr).sh_flags = SHF_ALLOC;
    (*(*mod_).arch.got_plt.shdr).sh_addralign = L1_CACHE_BYTES;
    (*(*mod_).arch.got_plt.shdr).sh_size = (num_plts as usize + 1) * core::mem::size_of::<got_entry>();
    (*mod_).arch.got_plt.num_entries = 0;
    (*mod_).arch.got_plt.max_entries = num_plts;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
