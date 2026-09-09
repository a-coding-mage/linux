// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module kallsyms support
 *
 * Copyright (C) 2010 Rusty Russell
 */

/* Kernel dependencies supplied by the surrounding translation unit. */

unsafe fn lookup_exported_symbol(
    name: *const c_char,
    start: *const kernel_symbol,
    stop: *const kernel_symbol,
) -> *const kernel_symbol {
    unsafe { bsearch(name, start, stop.offset_from(start) as usize, core::mem::size_of::<kernel_symbol>(), cmp_name) }
}

unsafe fn is_exported(name: *const c_char, value: c_ulong, mod_: *const module) -> bool {
    let ks: *const kernel_symbol;
    if mod_.is_null() {
        ks = unsafe { lookup_exported_symbol(name, __start___ksymtab, __stop___ksymtab) };
    } else {
        ks = unsafe {
            lookup_exported_symbol(name, (*mod_).syms, (*mod_).syms.add((*mod_).num_syms as usize))
        };
    }
    !ks.is_null() && unsafe { kernel_symbol_value(ks) == value }
}

/* As per nm */
unsafe fn elf_type(sym: *const Elf_Sym, info: *const load_info) -> c_char {
    let sechdrs = unsafe { (*info).sechdrs };
    if unsafe { ELF_ST_BIND((*sym).st_info) } == STB_WEAK {
        if unsafe { ELF_ST_TYPE((*sym).st_info) } == STT_OBJECT { return b'v' as c_char; }
        return b'w' as c_char;
    }
    if unsafe { (*sym).st_shndx } == SHN_UNDEF { return b'U' as c_char; }
    if unsafe { (*sym).st_shndx == SHN_ABS || (*sym).st_shndx == (*info).index.pcpu } { return b'a' as c_char; }
    if unsafe { (*sym).st_shndx >= SHN_LORESERVE } { return b'?' as c_char; }
    let sec = unsafe { &*sechdrs.add((*sym).st_shndx as usize) };
    if sec.sh_flags & SHF_EXECINSTR != 0 { return b't' as c_char; }
    if sec.sh_flags & SHF_ALLOC != 0 && sec.sh_type != SHT_NOBITS {
        if sec.sh_flags & SHF_WRITE == 0 { return b'r' as c_char; }
        if sec.sh_flags & ARCH_SHF_SMALL != 0 { return b'g' as c_char; }
        return b'd' as c_char;
    }
    if sec.sh_type == SHT_NOBITS {
        if sec.sh_flags & ARCH_SHF_SMALL != 0 { return b's' as c_char; }
        return b'b' as c_char;
    }
    if unsafe { strstarts((*info).secstrings.add(sec.sh_name as usize), b".debug\0".as_ptr() as *const c_char) } { return b'n' as c_char; }
    b'?' as c_char
}

unsafe fn is_core_symbol(src: *const Elf_Sym, sechdrs: *const Elf_Shdr, shnum: c_uint, pcpundx: c_uint) -> bool {
    if unsafe { (*src).st_shndx == SHN_UNDEF || (*src).st_shndx >= shnum || (*src).st_name == 0 } { return false; }
    #[cfg(CONFIG_KALLSYMS_ALL)]
    if unsafe { (*src).st_shndx == pcpundx as _ } { return true; }
    let sec = unsafe { &*sechdrs.add((*src).st_shndx as usize) };
    let ty = sec.sh_entsize >> SH_ENTSIZE_TYPE_SHIFT;
    #[cfg(CONFIG_KALLSYMS_ALL)]
    let bad = sec.sh_flags & SHF_ALLOC == 0 || unsafe { mod_mem_type_is_init(ty) };
    #[cfg(not(CONFIG_KALLSYMS_ALL))]
    let bad = sec.sh_flags & SHF_ALLOC == 0 || sec.sh_flags & SHF_EXECINSTR == 0 || unsafe { mod_mem_type_is_init(ty) };
    !bad
}

/*
 * We only allocate and copy the strings needed by the parts of symtab
 * we keep.  This is simple, but has the effect of making multiple
 * copies of duplicates.  We could be more sophisticated, see
 * linux-kernel thread starting with
 * <73defb5e4bca04a6431392cc341112b1@localhost>.
 */
pub unsafe fn layout_symtab(mod_: *mut module, info: *mut load_info) {
    let symsect = unsafe { (*info).sechdrs.add((*info).index.sym as usize) };
    let strsect = unsafe { (*info).sechdrs.add((*info).index.str as usize) };
    let mut nsrc: c_uint;
    let mut ndst: usize;
    let mut strtab_size: usize = 0;
    let mod_mem_data = unsafe { &mut (*mod_).mem[MOD_DATA as usize] };
    let mod_mem_init_data = unsafe { &mut (*mod_).mem[MOD_INIT_DATA as usize] };
    unsafe { (*symsect).sh_flags |= SHF_ALLOC; (*symsect).sh_entsize = module_get_offset_and_type(mod_, MOD_INIT_DATA, symsect, (*info).index.sym); }
    unsafe { pr_debug(b"\t%s\n\0".as_ptr() as *const c_char, (*info).secstrings.add((*symsect).sh_name as usize)); }
    let src = unsafe { ((*info).hdr as *const u8).add((*symsect).sh_offset as usize) as *const Elf_Sym };
    nsrc = unsafe { ((*symsect).sh_size / core::mem::size_of::<Elf_Sym>()) as c_uint };
    ndst = 0;
    for i in 0..nsrc as usize {
        if i == 0 || unsafe { is_livepatch_module(mod_) } || unsafe { is_core_symbol(src.add(i), (*info).sechdrs, (*info).hdr.as_ref().unwrap().e_shnum as c_uint, (*info).index.pcpu as c_uint) } {
            strtab_size += unsafe { strlen((*info).strtab.add((*src.add(i)).st_name as usize)) } + 1;
            ndst += 1;
        }
    }
    unsafe { (*info).symoffs = ALIGN(mod_mem_data.size, if (*symsect).sh_addralign != 0 { (*symsect).sh_addralign } else { 1 }); }
    unsafe { (*info).stroffs = mod_mem_data.size = (*info).symoffs + ndst * core::mem::size_of::<Elf_Sym>(); }
    mod_mem_data.size += strtab_size;
    unsafe { (*info).core_typeoffs = mod_mem_data.size; }
    mod_mem_data.size += ndst;
    unsafe { (*strsect).sh_flags |= SHF_ALLOC; (*strsect).sh_entsize = module_get_offset_and_type(mod_, MOD_INIT_DATA, strsect, (*info).index.str); }
    unsafe { pr_debug(b"\t%s\n\0".as_ptr() as *const c_char, (*info).secstrings.add((*strsect).sh_name as usize)); }
    mod_mem_init_data.size = ALIGN(mod_mem_init_data.size, core::mem::align_of::<mod_kallsyms>());
    unsafe { (*info).mod_kallsyms_init_off = mod_mem_init_data.size; }
    mod_mem_init_data.size += core::mem::size_of::<mod_kallsyms>();
    unsafe { (*info).init_typeoffs = mod_mem_init_data.size; }
    mod_mem_init_data.size += nsrc as usize;
}

/* We use the full symtab and strtab arranged by layout_symtab. */
pub unsafe fn add_kallsyms(mod_: *mut module, info: *const load_info) {
    let mut ndst = 0usize;
    let symsec = unsafe { &*(*info).sechdrs.add((*info).index.sym as usize) };
    let mut strtab_size: usize;
    let data_base = unsafe { (*mod_).mem[MOD_DATA as usize].base };
    let init_data_base = unsafe { (*mod_).mem[MOD_INIT_DATA as usize].base };
    let kallsyms = unsafe { (init_data_base as *mut u8).add((*info).mod_kallsyms_init_off) as *mut mod_kallsyms };
    unsafe {
        (*kallsyms).symtab = symsec.sh_addr as *mut Elf_Sym;
        (*kallsyms).num_symtab = symsec.sh_size / core::mem::size_of::<Elf_Sym>();
        (*kallsyms).strtab = (*info).sechdrs[(*info).index.str as usize].sh_addr as *mut c_char;
        (*kallsyms).typetab = (init_data_base as *mut u8).add((*info).init_typeoffs) as *mut c_char;
        (*mod_).core_kallsyms.symtab = (data_base as *mut u8).add((*info).symoffs) as *mut Elf_Sym;
        (*mod_).core_kallsyms.strtab = (data_base as *mut u8).add((*info).stroffs) as *mut c_char;
        (*mod_).core_kallsyms.typetab = (data_base as *mut u8).add((*info).core_typeoffs) as *mut c_char;
        strtab_size = (*info).core_typeoffs - (*info).stroffs;
    }
    let src = unsafe { (*kallsyms).symtab };
    for i in 0..unsafe { (*kallsyms).num_symtab } {
        unsafe { *(*kallsyms).typetab.add(i) = elf_type(src.add(i), info); }
        if i == 0 || unsafe { is_livepatch_module(mod_) } || unsafe { is_core_symbol(src.add(i), (*info).sechdrs, (*info).hdr.as_ref().unwrap().e_shnum as c_uint, (*info).index.pcpu as c_uint) } {
            unsafe {
                *(*mod_).core_kallsyms.typetab.add(ndst) = *(*kallsyms).typetab.add(i);
                *(*mod_).core_kallsyms.symtab.add(ndst) = *src.add(i);
                let s = (*mod_).core_kallsyms.strtab.add((*info).stroffs);
                (*mod_).core_kallsyms.symtab.add(ndst).as_mut().unwrap().st_name = s.offset_from((*mod_).core_kallsyms.strtab) as _;
                let ret = strscpy(s, (*kallsyms).strtab.add((*src.add(i)).st_name as usize), strtab_size);
                if ret < 0 { break; }
                strtab_size -= ret as usize + 1;
            }
            ndst += 1;
        }
    }
    unsafe { rcu_assign_pointer((*mod_).kallsyms, kallsyms); (*mod_).core_kallsyms.num_symtab = ndst; }
}

#[cfg(CONFIG_STACKTRACE_BUILD_ID)]
pub unsafe fn init_build_id(mod_: *mut module, info: *const load_info) {
    for i in 0..unsafe { (*info).hdr.as_ref().unwrap().e_shnum } {
        let sechdr = unsafe { &*(*info).sechdrs.add(i as usize) };
        if unsafe { !sect_empty(sechdr) && sechdr.sh_type == SHT_NOTE && build_id_parse_buf(sechdr.sh_addr as *const c_void, (*mod_).build_id.as_mut_ptr(), sechdr.sh_size) == 0 } { break; }
    }
}
#[cfg(not(CONFIG_STACKTRACE_BUILD_ID))]
pub unsafe fn init_build_id(_mod_: *mut module, _info: *const load_info) {}

unsafe fn kallsyms_symbol_name(kallsyms: *mut mod_kallsyms, symnum: c_uint) -> *const c_char {
    unsafe { (*kallsyms).strtab.add((*(*kallsyms).symtab.add(symnum as usize)).st_name as usize) }
}

/* Given a module and address, find the corresponding symbol. */
unsafe fn find_kallsyms_symbol(mod_: *mut module, addr: c_ulong, size: *mut c_ulong, offset: *mut c_ulong) -> *const c_char {
    let mut best = 0usize;
    let mut nextval: c_ulong;
    let mut bestval: c_ulong;
    let kallsyms = unsafe { rcu_dereference((*mod_).kallsyms) };
    let mut mod_mem: *mut module_memory = core::ptr::null_mut();
    for_each_mod_mem_type!(type_ => {
        #[cfg(not(CONFIG_KALLSYMS_ALL))] if !unsafe { mod_mem_type_is_text(type_) } { continue; }
        if unsafe { within_module_mem_type(addr, mod_, type_) } { mod_mem = unsafe { &mut (*mod_).mem[type_ as usize] }; break; }
    });
    if mod_mem.is_null() { return core::ptr::null(); }
    nextval = unsafe { mod_mem as c_ulong + (*mod_mem).size as c_ulong };
    bestval = unsafe { mod_mem as c_ulong - 1 };
    for i in 1..unsafe { (*kallsyms).num_symtab } {
        let sym = unsafe { &*(*kallsyms).symtab.add(i as usize) };
        let thisval = unsafe { kallsyms_symbol_value(sym) };
        if sym.st_shndx == SHN_UNDEF { continue; }
        let name = unsafe { kallsyms_symbol_name(kallsyms, i) };
        if unsafe { *name == 0 || is_mapping_symbol(name) } { continue; }
        if thisval <= addr && thisval > bestval { best = i as usize; bestval = thisval; }
        if thisval > addr && thisval < nextval { nextval = thisval; }
    }
    if best == 0 { return core::ptr::null(); }
    if !size.is_null() { unsafe { *size = nextval - bestval; } }
    if !offset.is_null() { unsafe { *offset = addr - bestval; } }
    unsafe { kallsyms_symbol_name(kallsyms, best as c_uint) }
}

pub unsafe fn dereference_module_function_descriptor(_mod_: *mut module, ptr: *mut c_void) -> *mut c_void { ptr }

pub unsafe fn module_address_lookup(addr: c_ulong, size: *mut c_ulong, offset: *mut c_ulong, modname: *mut *mut c_char, modbuildid: *mut *const u8, namebuf: *mut c_char) -> c_int {
    let mut ret = 0;
    let mod_ = unsafe { __module_address(addr) };
    if !mod_.is_null() {
        if !modname.is_null() { unsafe { *modname = (*mod_).name; } }
        if !modbuildid.is_null() { unsafe { *modbuildid = module_buildid(mod_); } }
        let sym = unsafe { find_kallsyms_symbol(mod_, addr, size, offset) };
        if !sym.is_null() { ret = unsafe { strscpy(namebuf, sym, KSYM_NAME_LEN) }; }
    }
    ret
}

pub unsafe fn lookup_module_symbol_name(addr: c_ulong, symname: *mut c_char) -> c_int {
    list_for_each_entry_rcu!(mod_ in modules, list, {
        if unsafe { (*mod_).state == MODULE_STATE_UNFORMED } { continue; }
        if unsafe { within_module(addr, mod_) } {
            let sym = unsafe { find_kallsyms_symbol(mod_, addr, core::ptr::null_mut(), core::ptr::null_mut()) };
            if sym.is_null() { break; }
            unsafe { strscpy(symname, sym, KSYM_NAME_LEN); }
            return 0;
        }
    });
    -ERANGE
}

pub unsafe fn module_get_kallsym(mut symnum: c_uint, value: *mut c_ulong, type_: *mut c_char, name: *mut c_char, module_name: *mut c_char, exported: *mut c_int) -> c_int {
    list_for_each_entry_rcu!(mod_ in modules, list, {
        if unsafe { (*mod_).state == MODULE_STATE_UNFORMED } { continue; }
        let kallsyms = unsafe { rcu_dereference((*mod_).kallsyms) };
        if symnum < unsafe { (*kallsyms).num_symtab } {
            let sym = unsafe { &*(*kallsyms).symtab.add(symnum as usize) };
            unsafe { *value = kallsyms_symbol_value(sym); *type_ = *(*kallsyms).typetab.add(symnum as usize); strscpy(name, kallsyms_symbol_name(kallsyms, symnum), KSYM_NAME_LEN); strscpy(module_name, (*mod_).name, MODULE_NAME_LEN); *exported = is_exported(name, *value, mod_) as c_int; }
            return 0;
        }
        symnum -= unsafe { (*kallsyms).num_symtab };
    });
    -ERANGE
}

unsafe fn __find_kallsyms_symbol_value(mod_: *mut module, name: *const c_char) -> c_ulong {
    let kallsyms = unsafe { rcu_dereference((*mod_).kallsyms) };
    for i in 0..unsafe { (*kallsyms).num_symtab } {
        let sym = unsafe { &*(*kallsyms).symtab.add(i as usize) };
        if unsafe { strcmp(name, kallsyms_symbol_name(kallsyms, i)) == 0 && sym.st_shndx != SHN_UNDEF } { return unsafe { kallsyms_symbol_value(sym) }; }
    }
    0
}

unsafe fn __module_kallsyms_lookup_name(name: *const c_char) -> c_ulong {
    let colon = unsafe { strnchr(name, MODULE_NAME_LEN, b':' as c_int) };
    if !colon.is_null() {
        let mod_ = unsafe { find_module_all(name, colon.offset_from(name) as usize, false) };
        if !mod_.is_null() { return unsafe { __find_kallsyms_symbol_value(mod_, colon.add(1)) }; }
        return 0;
    }
    list_for_each_entry_rcu!(mod_ in modules, list, {
        if unsafe { (*mod_).state == MODULE_STATE_UNFORMED } { continue; }
        let ret = unsafe { __find_kallsyms_symbol_value(mod_, name) };
        if ret != 0 { return ret; }
    });
    0
}

pub unsafe fn module_kallsyms_lookup_name(name: *const c_char) -> c_ulong { unsafe { __module_kallsyms_lookup_name(name) } }
pub unsafe fn find_kallsyms_symbol_value(mod_: *mut module, name: *const c_char) -> c_ulong { unsafe { __find_kallsyms_symbol_value(mod_, name) } }

pub unsafe fn module_kallsyms_on_each_symbol(modname: *const c_char, fn_: Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_ulong) -> c_int>, data: *mut c_void) -> c_int {
    let mut ret = 0;
    unsafe { mutex_lock(&module_mutex); }
    list_for_each_entry!(mod_ in modules, list, {
        if unsafe { (*mod_).state == MODULE_STATE_UNFORMED } { continue; }
        if !modname.is_null() && unsafe { strcmp(modname, (*mod_).name) != 0 } { continue; }
        let kallsyms = unsafe { rcu_dereference_check((*mod_).kallsyms, lockdep_is_held(&module_mutex)) };
        for i in 0..unsafe { (*kallsyms).num_symtab } {
            let sym = unsafe { &*(*kallsyms).symtab.add(i as usize) };
            if sym.st_shndx == SHN_UNDEF { continue; }
            ret = unsafe { fn_.unwrap()(data, kallsyms_symbol_name(kallsyms, i), kallsyms_symbol_value(sym)) };
            if ret != 0 { break; }
        }
        if ret != 0 || !modname.is_null() { break; }
    });
    unsafe { mutex_unlock(&module_mutex); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
