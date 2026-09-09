// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel module help for x86. */

// Kernel headers, ELF definitions, and configuration symbols are supplied by
// the surrounding translation unit.

#[cfg(target_pointer_width = "32")]
pub unsafe fn apply_relocate(s: *mut Elf32_Shdr, _t: *const core::ffi::c_char, n: u32, r: u32, m: *mut module) -> i32 {
    let rel = (*s.add(r as usize)).sh_addr as *mut Elf32_Rel;
    let mut i = 0u32;
    while i < (*s.add(r as usize)).sh_size / core::mem::size_of::<Elf32_Rel>() {
        let e = &*rel.add(i as usize);
        let loc = ((*s.add((*s.add(r as usize)).sh_info as usize)).sh_addr + e.r_offset) as *mut u32;
        let sym = ((*s.add(n as usize)).sh_addr as *mut Elf32_Sym).add((e.r_info >> 8) as usize);
        match e.r_info & 0xff { R_386_32 => *loc = (*loc).wrapping_add((*sym).st_value), R_386_PC32 | R_386_PLT32 => *loc = (*loc).wrapping_add((*sym).st_value.wrapping_sub(loc as u32)), _ => { pr_err_unknown(m, e.r_info & 0xff); return -8; } }
        i += 1;
    }
    0
}

#[cfg(not(target_pointer_width = "32"))]
unsafe fn __write_relocate_add(s: *mut Elf64_Shdr, _t: *const core::ffi::c_char, n: u32, r: u32, m: *mut module, write: unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize) -> *mut core::ffi::c_void, apply: bool) -> i32 {
    let rel = (*s.add(r as usize)).sh_addr as *mut Elf64_Rela; let mut i=0u32;
    while i < (*s.add(r as usize)).sh_size / core::mem::size_of::<Elf64_Rela>() { let e=&*rel.add(i as usize); let loc=((*s.add((*s.add(r as usize)).sh_info as usize)).sh_addr+e.r_offset) as *mut u8; let sym=((*s.add(n as usize)).sh_addr as *mut Elf64_Sym).add((e.r_info>>32) as usize); let mut val=(*sym).st_value.wrapping_add(e.r_addend as u64); let size=match e.r_info&0xffffffff { R_X86_64_NONE=>{i+=1;continue}, R_X86_64_64=>8, R_X86_64_32=>{if val!=val as u32 as u64{return relocation_overflow(m,e.r_info,val,r,i)}4}, R_X86_64_32S=>{if val as i64!=val as u32 as i32 as i64{return relocation_overflow(m,e.r_info,val,r,i)}4}, R_X86_64_PC32|R_X86_64_PLT32=>{val=val.wrapping_sub(loc as u64);4}, R_X86_64_PC64=>{val=val.wrapping_sub(loc as u64);8}, _=>{pr_err_unknown_rela(m,e.r_info);return -8}}; let zero=0u64; if apply {write(loc as *mut _,&zero as *const _ as *const _,size)} else {write(loc as *mut _,&zero as *const _ as *const _,size)}; i+=1; } 0
}
#[cfg(not(target_pointer_width = "32"))] unsafe fn write_relocate_add(s:*mut Elf64_Shdr,t:*const core::ffi::c_char,n:u32,r:u32,m:*mut module,a:bool)->i32 { __write_relocate_add(s,t,n,r,m,core::ptr::copy_nonoverlapping,a) }
#[cfg(not(target_pointer_width = "32"))] pub unsafe fn apply_relocate_add(s:*mut Elf64_Shdr,t:*const core::ffi::c_char,n:u32,r:u32,m:*mut module)->i32 {write_relocate_add(s,t,n,r,m,true)}
#[cfg(feature="CONFIG_LIVEPATCH")] pub unsafe fn clear_relocate_add(s:*mut Elf64_Shdr,t:*const core::ffi::c_char,n:u32,r:u32,m:*mut module){write_relocate_add(s,t,n,r,m,false);}
pub unsafe fn module_finalize(_h:*const Elf_Ehdr,_s:*const Elf_Shdr,m:*mut module)->i32 {its_init_mod(m);its_fini_mod(m);0}
pub unsafe fn module_arch_cleanup(m:*mut module){its_free_mod(m)}
extern "C" { type Elf32_Shdr;type Elf32_Rel;type Elf32_Sym;type Elf64_Shdr;type Elf64_Rela;type Elf64_Sym;type Elf_Ehdr;type module;fn its_init_mod(*mut module);fn its_fini_mod(*mut module);fn its_free_mod(*mut module);fn pr_err_unknown(*mut module,u32);fn pr_err_unknown_rela(*mut module,u64);fn relocation_overflow(*mut module,u64,u64,u32,u32)->i32; }
const R_386_32:u32=1;const R_386_PC32:u32=2;const R_386_PLT32:u32=4;const R_X86_64_NONE:u64=0;const R_X86_64_64:u64=1;const R_X86_64_PC32:u64=2;const R_X86_64_PLT32:u64=4;const R_X86_64_32:u64=10;const R_X86_64_32S:u64=11;const R_X86_64_PC64:u64=24;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
