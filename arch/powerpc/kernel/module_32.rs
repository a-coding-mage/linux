// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel module help for PPC. */

// C headers and build-time macros are supplied by the surrounding kernel
// translation unit.

unsafe fn count_relocs(rela: *const Elf32_Rela, num: c_uint) -> c_uint {
    let mut count = 0;
    let mut r_info = 0;
    let mut r_addend = 0;
    for i in 0..num {
        let r = &*rela.add(i as usize);
        if ELF32_R_TYPE(r.r_info) == R_PPC_REL24
            && (r_info != ELF32_R_SYM(r.r_info) || r_addend != r.r_addend)
        {
            count += 1;
            r_info = ELF32_R_SYM(r.r_info);
            r_addend = r.r_addend;
        }
    }
    #[cfg(CONFIG_DYNAMIC_FTRACE)]
    { count += 1; }
    count
}

unsafe extern "C" fn relacmp(a: *const c_void, b: *const c_void) -> c_int {
    let x = &*(b as *const Elf32_Rela);
    let y = &*(a as *const Elf32_Rela);
    if x.r_info < y.r_info { -1 }
    else if x.r_info > y.r_info { 1 }
    else if x.r_addend < y.r_addend { -1 }
    else if x.r_addend > y.r_addend { 1 }
    else { 0 }
}

unsafe fn get_plt_size(hdr: *const Elf32_Ehdr, sechdrs: *const Elf32_Shdr,
                       secstrings: *const c_char, is_init: c_int) -> c_ulong {
    let mut ret = 0;
    for i in 1..(*hdr).e_shnum {
        let sh = &*sechdrs.add(i as usize);
        if (strstr(secstrings.add(sh.sh_name as usize), b".init\0".as_ptr() as _).is_null()) as c_int != is_init { continue; }
        if !strstr(secstrings.add(sh.sh_name as usize), b".debug\0".as_ptr() as _).is_null() { continue; }
        if sh.sh_type == SHT_RELA {
            sort((hdr as *mut u8).add(sh.sh_offset as usize) as _,
                 (sh.sh_size as usize) / size_of::<Elf32_Rela>(),
                 size_of::<Elf32_Rela>(), Some(relacmp), core::ptr::null_mut());
            ret += count_relocs((hdr as *mut u8).add(sh.sh_offset as usize) as _,
                                (sh.sh_size as usize / size_of::<Elf32_Rela>()) as _) as c_ulong
                * size_of::<ppc_plt_entry>() as c_ulong;
        }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn module_frob_arch_sections(hdr: *mut Elf32_Ehdr, sechdrs: *mut Elf32_Shdr,
                                                    secstrings: *mut c_char, me: *mut module) -> c_int {
    for i in 0..(*hdr).e_shnum {
        let sh = &*sechdrs.add(i as usize);
        let name = secstrings.add(sh.sh_name as usize);
        if strcmp(name, b".init.plt\0".as_ptr() as _) == 0 { (*me).arch.init_plt_section = i; }
        else if strcmp(name, b".plt\0".as_ptr() as _) == 0 { (*me).arch.core_plt_section = i; }
    }
    if (*me).arch.core_plt_section == 0 || (*me).arch.init_plt_section == 0 { return -ENOEXEC; }
    (*sechdrs.add((*me).arch.core_plt_section as usize)).sh_size = get_plt_size(hdr, sechdrs, secstrings, 0);
    (*sechdrs.add((*me).arch.init_plt_section as usize)).sh_size = get_plt_size(hdr, sechdrs, secstrings, 1);
    0
}

unsafe fn entry_matches(entry: *const ppc_plt_entry, val: Elf32_Addr) -> c_int {
    if (*entry).jump[0] != PPC_RAW_LIS(_R12, PPC_HA(val)) { return 0; }
    if (*entry).jump[1] != PPC_RAW_ADDI(_R12, _R12, PPC_LO(val)) { return 0; }
    1
}

unsafe fn do_plt_call(location: *mut c_void, mut val: Elf32_Addr, sechdrs: *const Elf32_Shdr, mod_: *mut module) -> u32 {
    let mut entry: *mut ppc_plt_entry;
    if within_module_core(location as usize as c_ulong, mod_) { entry = (*sechdrs.add((*mod_).arch.core_plt_section as usize)).sh_addr as *mut _; }
    else { entry = (*sechdrs.add((*mod_).arch.init_plt_section as usize)).sh_addr as *mut _; }
    while (*entry).jump[0] != 0 {
        if entry_matches(entry, val) != 0 { return entry as u32; }
        entry = entry.add(1);
    }
    if patch_instruction(&mut (*entry).jump[0], ppc_inst(PPC_RAW_LIS(_R12, PPC_HA(val)))) != 0 { return 0; }
    if patch_instruction(&mut (*entry).jump[1], ppc_inst(PPC_RAW_ADDI(_R12, _R12, PPC_LO(val)))) != 0 { return 0; }
    if patch_instruction(&mut (*entry).jump[2], ppc_inst(PPC_RAW_MTCTR(_R12))) != 0 { return 0; }
    if patch_instruction(&mut (*entry).jump[3], ppc_inst(PPC_RAW_BCTR())) != 0 { return 0; }
    entry as u32
}

unsafe fn patch_location_16(mut loc: *mut u32, value: u16) -> c_int {
    loc = PTR_ALIGN_DOWN(loc, size_of::<u32>());
    patch_instruction(loc, ppc_inst((*loc & 0xffff0000) | value as u32))
}

#[no_mangle]
pub unsafe extern "C" fn apply_relocate_add(sechdrs: *mut Elf32_Shdr, _strtab: *const c_char,
    symindex: c_uint, relsec: c_uint, module: *mut module) -> c_int {
    let rela = (*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rela;
    for i in 0..((*sechdrs.add(relsec as usize)).sh_size as usize / size_of::<Elf32_Rela>()) {
        let r = &*rela.add(i);
        let location = ((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr + r.r_offset) as *mut u32;
        let sym = ((*sechdrs.add(symindex as usize)).sh_addr as *const Elf32_Sym).add(ELF32_R_SYM(r.r_info) as usize);
        let mut value = (*sym).st_value.wrapping_add(r.r_addend as u32);
        match ELF32_R_TYPE(r.r_info) {
            R_PPC_ADDR32 => *location = value,
            R_PPC_ADDR16_LO => if patch_location_16(location, PPC_LO(value)) != 0 { return -EFAULT; },
            R_PPC_ADDR16_HI => if patch_location_16(location, PPC_HI(value)) != 0 { return -EFAULT; },
            R_PPC_ADDR16_HA => if patch_location_16(location, PPC_HA(value)) != 0 { return -EFAULT; },
            R_PPC_REL24 => {
                let diff = value.wrapping_sub(location as u32) as i32;
                if diff < -0x02000000 || diff >= 0x02000000 { value = do_plt_call(location as _, value, sechdrs, module); if value == 0 { return -EFAULT; } }
                value = (*location & !PPC_LI_MASK) | PPC_LI(value.wrapping_sub(location as u32));
                if patch_instruction(location, ppc_inst(value)) != 0 { return -EFAULT; }
            },
            R_PPC_REL32 => *location = value.wrapping_sub(location as u32),
            _ => return -ENOEXEC,
        }
    }
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[no_mangle]
pub unsafe extern "C" fn module_trampoline_target(_mod: *mut module, mut addr: c_ulong,
                                                    target: *mut c_ulong) -> c_int {
    let mut jmp: [ppc_inst_t; 4] = core::mem::zeroed();
    if copy_inst_from_kernel_nofault(&mut jmp[0], addr as *const c_void) != 0 { return -EFAULT; }
    if __copy_inst_from_kernel_nofault(&mut jmp[1], (addr + 4) as *const c_void) != 0 { return -EFAULT; }
    if __copy_inst_from_kernel_nofault(&mut jmp[2], (addr + 8) as *const c_void) != 0 { return -EFAULT; }
    if __copy_inst_from_kernel_nofault(&mut jmp[3], (addr + 12) as *const c_void) != 0 { return -EFAULT; }
    if ppc_inst_val(jmp[0]) & 0xffff0000 != PPC_RAW_LIS(_R12, 0) { return -EINVAL; }
    if ppc_inst_val(jmp[1]) & 0xffff0000 != PPC_RAW_ADDI(_R12, _R12, 0) { return -EINVAL; }
    if ppc_inst_val(jmp[2]) != PPC_RAW_MTCTR(_R12) || ppc_inst_val(jmp[3]) != PPC_RAW_BCTR() { return -EINVAL; }
    addr = ((ppc_inst_val(jmp[1]) & 0xffff) | ((ppc_inst_val(jmp[0]) & 0xffff) << 16)) as c_ulong;
    if addr & 0x8000 != 0 { addr = addr.wrapping_sub(0x10000); }
    *target = addr;
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[no_mangle]
pub unsafe extern "C" fn module_finalize_ftrace(module: *mut module, sechdrs: *const Elf_Shdr) -> c_int {
    (*module).arch.tramp = do_plt_call((*module).mem[MOD_TEXT].base as _, ftrace_caller as usize as _, sechdrs as _, module);
    if (*module).arch.tramp == 0 { return -ENOENT; }
    #[cfg(CONFIG_DYNAMIC_FTRACE_WITH_REGS)]
    {
        (*module).arch.tramp_regs = do_plt_call((*module).mem[MOD_TEXT].base as _, ftrace_regs_caller as usize as _, sechdrs as _, module);
        if (*module).arch.tramp_regs == 0 { return -ENOENT; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
