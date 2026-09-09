// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel module help for PPC64. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// C headers and configuration symbols are supplied by the surrounding kernel
// translation.  Their names and conditional compilation intent are retained.

#[repr(C)]
pub struct ppc64_stub_entry {
    pub jump: [u32; 7],
    pub magic: u32,
    pub funcdata: func_desc_t,
}
#[repr(C)] pub struct ppc64_got_entry { pub addr: u64 }

extern "C" {
    pub fn sort(base: *mut core::ffi::c_void, n: usize, size: usize,
                cmp: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
                arg: *mut core::ffi::c_void);
    pub fn patch_instruction(p: *mut u32, i: u32) -> i32;
    pub fn ppc_inst(i: u32) -> u32;
    pub fn ppc_inst_prefix(a: u32, b: u32) -> u32;
    pub fn within_module_core(addr: usize, module: *mut module) -> bool;
    pub fn copy_from_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> i32;
    pub fn kernel_toc_addr() -> usize;
}

const STUB_MAGIC: u32 = 0x7374_7562;

pub unsafe fn module_elf_check_arch(hdr: *const Elf_Ehdr) -> bool {
    let abi_level = (*hdr).e_flags & 3;
    // IS_ENABLED(CONFIG_PPC64_ELF_ABI_V2)
    cfg!(CONFIG_PPC64_ELF_ABI_V2) && abi_level == 2 ||
        !cfg!(CONFIG_PPC64_ELF_ABI_V2) && abi_level < 2
}

#[cfg(CONFIG_PPC64_ELF_ABI_V2)]
unsafe fn func_desc(addr: usize) -> func_desc_t { let mut d = core::mem::zeroed(); d.addr = addr; d }
#[cfg(not(CONFIG_PPC64_ELF_ABI_V2))]
unsafe fn func_desc(addr: usize) -> func_desc_t { *(addr as *const func_desc_t) }
#[cfg(CONFIG_PPC64_ELF_ABI_V2)]
unsafe fn local_entry_offset(sym: *const Elf64_Sym) -> u32 { 1u32 << (((*sym).st_other & (7 << 5)) >> 5) >> 2 << 2 }
#[cfg(not(CONFIG_PPC64_ELF_ABI_V2))]
unsafe fn local_entry_offset(_: *const Elf64_Sym) -> u32 { 0 }
unsafe fn func_addr(addr: usize) -> usize { func_desc(addr).addr }
unsafe fn stub_func_addr(f: func_desc_t) -> usize { f.addr }

pub unsafe fn module_init_section(_: *const i8) -> bool { false }

unsafe fn relacmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let x = &*(a as *const Elf64_Rela); let y = &*(b as *const Elf64_Rela);
    if x.r_info < y.r_info { -1 } else if x.r_info > y.r_info { 1 }
    else if x.r_addend < y.r_addend { -1 } else if x.r_addend > y.r_addend { 1 } else { 0 }
}
unsafe fn count_relocs(r: *const Elf64_Rela, n: usize, typ: usize) -> usize {
    let mut count=0; let mut info=0; let mut addend=0;
    for i in 0..n { let x=&*r.add(i); if ELF64_R_TYPE(x.r_info)==typ && (info!=ELF64_R_SYM(x.r_info)||addend!=x.r_addend) { count+=1; info=ELF64_R_SYM(x.r_info); addend=x.r_addend; } } count
}

unsafe fn get_stubs_size(h: *const Elf64_Ehdr, s: *mut Elf64_Shdr, _ss: *mut i8, _m: *mut module) -> usize {
    let mut n=0; for i in 1..(*h).e_shnum as usize { if (*s.add(i)).sh_type==SHT_RELA { let r=(*s.add(i)).sh_addr as *mut Elf64_Rela; let c=(*s.add(i)).sh_size as usize/core::mem::size_of::<Elf64_Rela>(); sort(r as _,c,core::mem::size_of::<Elf64_Rela>(),relacmp,core::ptr::null_mut()); n+=count_relocs(r,c,R_PPC_REL24); #[cfg(CONFIG_PPC_KERNEL_PCREL)] { n+=count_relocs(r,c,R_PPC64_REL24_NOTOC); } } } n*core::mem::size_of::<ppc64_stub_entry>()
}

pub unsafe fn apply_relocate_add(sechdrs:*mut Elf64_Shdr,strtab:*const i8,symindex:u32,relsec:u32,me:*mut module)->i32 {
    let rela=(*sechdrs.add(relsec as usize)).sh_addr as *mut Elf64_Rela;
    let count=(*sechdrs.add(relsec as usize)).sh_size as usize/core::mem::size_of::<Elf64_Rela>();
    for i in 0..count {
        let r=&*rela.add(i); let sym=&*(((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf64_Sym).add(ELF64_R_SYM(r.r_info) as usize));
        let location=((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr+r.r_offset) as *mut u8;
        let mut value=sym.st_value.wrapping_add(r.r_addend as u64) as usize;
        match ELF64_R_TYPE(r.r_info) {
            R_PPC64_ADDR32 => *(location as *mut u32)=value as u32,
            R_PPC64_ADDR64 => *(location as *mut usize)=value,
            R_PPC64_REL64 => *(location as *mut usize)=value.wrapping_sub(location as usize),
            R_PPC64_REL32 => *(location as *mut u32)=value.wrapping_sub(location as usize) as u32,
            R_PPC64_REL16_HA => *(location as *mut u16)=((value.wrapping_sub(location as usize).wrapping_add(0x8000)>>16)&0xffff) as u16,
            R_PPC64_REL16_LO => *(location as *mut u16)=value.wrapping_sub(location as usize) as u16,
            _ => return -ENOEXEC,
        }
    } 0
}

pub unsafe fn module_frob_arch_sections(h:*mut Elf64_Ehdr,s:*mut Elf64_Shdr,strings:*const i8,me:*mut module)->i32 {
    for i in 1..(*h).e_shnum as usize { if (*s.add(i)).sh_type==SHT_RELA { (*s.add(i)).sh_size=get_stubs_size(h,s,strings,me) as u64; break; } } 0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
pub unsafe fn module_trampoline_target(_: *mut module, _: usize, _: *mut usize)->i32 { -EFAULT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
