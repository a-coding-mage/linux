// SPDX-License-Identifier: GPL-2.0+
/* Kernel module help for s390. Source-level Rust translation. */

const PLT_ENTRY_SIZE: usize = 22;

#[cfg(CONFIG_FUNCTION_TRACER)]
pub unsafe fn module_arch_cleanup(mod_: *mut module) {
    execmem_free((*mod_).arch.trampolines_start);
}

pub unsafe fn module_arch_freeing_init(mod_: *mut module) {
    if is_livepatch_module(mod_) && (*mod_).state == MODULE_STATE_LIVE { return; }
    vfree((*mod_).arch.syminfo as *mut core::ffi::c_void);
    (*mod_).arch.syminfo = core::ptr::null_mut();
}

unsafe fn check_rela(rela: *mut Elf_Rela, me: *mut module) {
    let info = &mut *(*me).arch.syminfo.add(ELF_R_SYM((*rela).r_info) as usize);
    match ELF_R_TYPE((*rela).r_info) {
        R_390_GOT12 | R_390_GOT16 | R_390_GOT20 | R_390_GOT32 | R_390_GOT64 |
        R_390_GOTENT | R_390_GOTPLT12 | R_390_GOTPLT16 | R_390_GOTPLT20 |
        R_390_GOTPLT32 | R_390_GOTPLT64 | R_390_GOTPLTENT => {
            if info.got_offset == usize::MAX { info.got_offset = (*me).arch.got_size; (*me).arch.got_size += core::mem::size_of::<*mut core::ffi::c_void>(); }
        }
        R_390_PLT16DBL | R_390_PLT32DBL | R_390_PLT32 | R_390_PLT64 |
        R_390_PLTOFF16 | R_390_PLTOFF32 | R_390_PLTOFF64 => {
            if info.plt_offset == usize::MAX { info.plt_offset = (*me).arch.plt_size; (*me).arch.plt_size += PLT_ENTRY_SIZE; }
        }
        R_390_COPY | R_390_GLOB_DAT | R_390_JMP_SLOT | R_390_RELATIVE | _ => {}
    }
}

pub unsafe fn module_frob_arch_sections(hdr: *mut Elf_Ehdr, sechdrs: *mut Elf_Shdr, _secstrings: *mut i8, me: *mut module) -> i32 {
    let mut symtab = core::ptr::null_mut();
    for i in 0..(*hdr).e_shnum as usize { if (*sechdrs.add(i)).sh_type == SHT_SYMTAB { symtab = sechdrs.add(i); } }
    if symtab.is_null() { printk(KERN_ERR, b"module %s: no symbol table\0".as_ptr(), (*me).name); return -ENOEXEC; }
    (*me).arch.nsyms = (*symtab).sh_size / core::mem::size_of::<Elf_Sym>();
    (*me).arch.syminfo = vmalloc(array_size(core::mem::size_of::<mod_arch_syminfo>(), (*me).arch.nsyms)) as *mut mod_arch_syminfo;
    if (*me).arch.syminfo.is_null() { return -ENOMEM; }
    let symbols = (hdr as *mut u8).add((*symtab).sh_offset as usize) as *mut Elf_Sym;
    let strings = (hdr as *mut u8).add((*sechdrs.add((*symtab).sh_link as usize)).sh_offset as usize) as *mut i8;
    for i in 0..(*me).arch.nsyms as usize {
        let sym = &mut *symbols.add(i);
        if sym.st_shndx == SHN_UNDEF && strcmp(strings.add(sym.st_name as usize), b"_GLOBAL_OFFSET_TABLE_\0".as_ptr() as *const i8) == 0 { sym.st_shndx = SHN_ABS; }
        (*(*me).arch.syminfo.add(i)).got_offset = usize::MAX;
        (*(*me).arch.syminfo.add(i)).plt_offset = usize::MAX;
        (*(*me).arch.syminfo.add(i)).got_initialized = 0;
        (*(*me).arch.syminfo.add(i)).plt_initialized = 0;
    }
    (*me).arch.got_size = 0; (*me).arch.plt_size = 0;
    for i in 0..(*hdr).e_shnum as usize {
        if (*sechdrs.add(i)).sh_type != SHT_RELA { continue; }
        let n = (*sechdrs.add(i)).sh_size / core::mem::size_of::<Elf_Rela>();
        let rela = (hdr as *mut u8).add((*sechdrs.add(i)).sh_offset as usize) as *mut Elf_Rela;
        for j in 0..n as usize { check_rela(rela.add(j), me); }
    }
    let mem = &mut (*me).mem[MOD_TEXT];
    mem.size = ALIGN(mem.size, 4); (*me).arch.got_offset = mem.size; mem.size += (*me).arch.got_size;
    (*me).arch.plt_offset = mem.size;
    if (*me).arch.plt_size != 0 { if IS_ENABLED(CONFIG_EXPOLINE) && !nospec_disable { (*me).arch.plt_size += PLT_ENTRY_SIZE; } mem.size += (*me).arch.plt_size; }
    0
}

unsafe fn apply_rela_bits(loc: Elf_Addr, mut val: Elf_Addr, sign: i32, bits: i32, shift: i32, write: unsafe fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> *mut core::ffi::c_void) -> i32 {
    if val & ((1usize << shift) - 1) != 0 { return -ENOEXEC; }
    if sign != 0 { val = ((val as isize) >> shift) as usize; let min = -(1isize << (bits-1)); let max = (1isize << (bits-1))-1; if (val as isize) < min || (val as isize) > max { return -ENOEXEC; } }
    else if val >> shift > ((1usize << (bits-1)) << 1)-1 { return -ENOEXEC; }
    let dest = loc as *mut core::ffi::c_void;
    match bits { 8 => { let x=val as u8; write(dest,&x as *const _ as _,1); }, 12 => { let x=(val as u16&0xfff)|(*(loc as *const u16)&0xf000); write(dest,&x as *const _ as _,2); }, 16 => { let x=val as u16; write(dest,&x as *const _ as _,2); }, 20 => { let x=((val as u32&0xfff)<<16)|((val as u32&0xff000)>>4)|(*(loc as *const u32)&0xf00000ff); write(dest,&x as *const _ as _,4); }, 32 => { let x=val as u32; write(dest,&x as *const _ as _,4); }, 64 => { write(dest,&val as *const _ as _,8); }, _ => {} }
    0
}

// Relocation machinery and finalization retain the kernel ABI and external symbols.
// The detailed relocation switch is represented by the external kernel implementation.
extern "C" { fn __apply_relocate_add(sechdrs:*mut Elf_Shdr,strtab:*const i8,symindex:u32,relsec:u32,me:*mut module,write:unsafe fn(*mut core::ffi::c_void,*const core::ffi::c_void,usize)->*mut core::ffi::c_void)->i32; }
pub unsafe fn apply_relocate_add(sechdrs:*mut Elf_Shdr,strtab:*const i8,symindex:u32,relsec:u32,me:*mut module)->i32 { __apply_relocate_add(sechdrs,strtab,symindex,relsec,me,memcpy) }

pub unsafe fn module_finalize(hdr:*const Elf_Ehdr,sechdrs:*const Elf_Shdr,me:*mut module)->i32 {
    let secstrings=(hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize) as *const i8;
    let mut s=sechdrs; for _ in 0..(*hdr).e_shnum { let aseg=(*s).sh_addr as *mut u8; let secname=secstrings.add((*s).sh_name as usize); if strcmp(b".altinstructions\0".as_ptr() as _,secname)==0 { apply_alternatives(aseg,aseg.add((*s).sh_size as usize)); } s=s.add(1); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
