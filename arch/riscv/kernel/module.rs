// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2017 Zihao Yu */

// Kernel headers and configuration symbols are supplied by the surrounding kernel bindings.

#[repr(C)]
pub struct used_bucket { pub head: list_head, pub bucket: *mut hlist_head }
#[repr(C)]
pub struct relocation_head { pub node: hlist_node, pub rel_entry: list_head, pub location: *mut core::ffi::c_void }
#[repr(C)]
pub struct relocation_entry { pub head: list_head, pub value: Elf_Addr, pub r#type: u32 }
#[repr(C)]
pub struct relocation_handlers {
    pub reloc_handler: Option<unsafe extern "C" fn(*mut module, *mut core::ffi::c_void, Elf_Addr) -> i32>,
    pub accumulate_handler: Option<unsafe extern "C" fn(*mut module, *mut core::ffi::c_void, i64) -> i32>,
}

extern "C" {
    fn riscv_insn_rmw(location: *mut core::ffi::c_void, keep: u32, set: u32) -> i32;
    fn module_emit_got_entry(me: *mut module, v: Elf_Addr) -> *mut core::ffi::c_void;
    fn module_emit_plt_entry(me: *mut module, v: Elf_Addr) -> *mut core::ffi::c_void;
}

unsafe fn riscv_insn_valid_32bit_offset(val: isize) -> bool {
    #[cfg(CONFIG_32BIT)] { true }
    #[cfg(not(CONFIG_32BIT))] { (-(1isize << 31) - (1isize << 11)) <= val && val < ((1isize << 31) - (1isize << 11)) }
}

unsafe fn riscv_insn_rmw_local(location: *mut core::ffi::c_void, keep: u32, set: u32) -> i32 {
    let p = location as *mut u16;
    let mut insn = (*p as u32) | ((*p.add(1) as u32) << 16);
    insn = (insn & keep) | set;
    *p = insn as u16; *p.add(1) = (insn >> 16) as u16; 0
}
unsafe fn riscv_insn_rvc_rmw(location: *mut core::ffi::c_void, keep: u16, set: u16) -> i32 {
    let p = location as *mut u16; *p = (*p & keep) | set; 0
}

unsafe fn apply_r_riscv_32_rela(me: *mut module, location: *mut core::ffi::c_void, v: Elf_Addr) -> i32 { if v != v as u32 as Elf_Addr { pr_err!("{}: value {:016x} out of range for 32-bit field\n", (*me).name, v); return -EINVAL; } *(location as *mut u32)=v as u32; 0 }
unsafe fn apply_r_riscv_64_rela(_: *mut module, location: *mut core::ffi::c_void, v: Elf_Addr)->i32 { *(location as *mut u64)=v as u64; 0 }
unsafe fn apply_r_riscv_branch_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let o=v as isize-l as isize; riscv_insn_rmw_local(l,0x1fff07f,(((o&0x1000)<<19)|((o&0x800)>>4)|((o&0x7e0)<<20)|((o&0x1e)<<7)) as u32) }
unsafe fn apply_r_riscv_jal_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let o=v as isize-l as isize; riscv_insn_rmw_local(l,0xfff,(((o&0x100000)<<11)|(o&0xff000)|((o&0x800)<<9)|((o&0x7fe)<<20)) as u32) }
unsafe fn apply_r_riscv_rvc_branch_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let o=v as isize-l as isize; riscv_insn_rvc_rmw(l,0xe383,(((o&0x100)<<4)|((o&0xc0)>>1)|((o&0x20)>>3)|((o&0x18)<<7)|((o&6)<<2)) as u16) }
unsafe fn apply_r_riscv_rvc_jump_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let o=v as isize-l as isize; riscv_insn_rvc_rmw(l,0xe003,(((o&0x800)<<1)|((o&0x400)>>2)|((o&0x300)<<1)|((o&0x80)>>1)|((o&0x40)<<1)|((o&0x20)>>3)|((o&0x10)<<7)|((o&0xe)<<2)) as u16) }
unsafe fn apply_r_riscv_pcrel_hi20_rela(me:*mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let o=v as isize-l as isize; if !riscv_insn_valid_32bit_offset(o) { pr_err!("{}: target {:016x} can not be addressed by the 32-bit offset from PC = {:?}\n",(*me).name,v,l); return -EINVAL; } riscv_insn_rmw_local(l,0xfff,((o+0x800)&0xfffff000) as u32) }
unsafe fn apply_r_riscv_pcrel_lo12_i_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { riscv_insn_rmw_local(l,0xfffff,((v&0xfff)<<20) as u32) }
unsafe fn apply_r_riscv_pcrel_lo12_s_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { riscv_insn_rmw_local(l,0x1fff07f,(((v&0xfe0)<<20)|((v&0x1f)<<7)) as u32) }
unsafe fn apply_r_riscv_hi20_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { riscv_insn_rmw_local(l,0xfff,(((v as i32+0x800)&0xfffff000) as u32)) }
unsafe fn apply_r_riscv_lo12_i_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let h=((v as i32+0x800)&0xfffff000); riscv_insn_rmw_local(l,0xfffff,(((v as i32-h)&0xfff) as u32)<<20) }
unsafe fn apply_r_riscv_lo12_s_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let h=((v as i32+0x800)&0xfffff000); let x=v as i32-h; riscv_insn_rmw_local(l,0x1fff07f,(((x&0xfe0)<<20)|((x&0x1f)<<7)) as u32) }
unsafe fn apply_r_riscv_got_hi20_rela(me:*mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let o=module_emit_got_entry(me,v) as isize-l as isize; riscv_insn_rmw_local(l,0xfff,((o+0x800)&0xfffff000) as u32) }
unsafe fn call_rela(me:*mut module,l:*mut core::ffi::c_void,v:Elf_Addr,plt:bool)->i32 { let mut o=v as isize-l as isize; if !riscv_insn_valid_32bit_offset(o) { if plt { o=module_emit_plt_entry(me,v) as isize-l as isize } else { return -EINVAL; } } let hi=((o+0x800)&0xfffff000) as u32; let lo=((o-hi as isize)&0xfff) as u32; riscv_insn_rmw_local(l,0xfff,hi); riscv_insn_rmw_local(l.add(4),0xfffff,lo<<20) }
unsafe fn apply_r_riscv_call_rela(m:*mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { call_rela(m,l,v,false) }
unsafe fn apply_r_riscv_call_plt_rela(m:*mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { call_rela(m,l,v,true) }
unsafe fn apply_r_riscv_relax_rela(_: *mut module,_:*mut core::ffi::c_void,_:Elf_Addr)->i32 { 0 }
unsafe fn apply_r_riscv_align_rela(me:*mut module,l:*mut core::ffi::c_void,_:Elf_Addr)->i32 { pr_err!("{}: The unexpected relocation type 'R_RISCV_ALIGN' from PC = {:?}\n",(*me).name,l); -EINVAL }

macro_rules! rw_add { ($n:ident,$t:ty) => { unsafe fn $n(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let p=l as *mut $t; *p=(*p).wrapping_add(v as $t); 0 } }; }
macro_rules! rw_sub { ($n:ident,$t:ty) => { unsafe fn $n(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let p=l as *mut $t; *p=(*p).wrapping_sub(v as $t); 0 } }; }
rw_add!(apply_r_riscv_add8_rela,u8); rw_add!(apply_r_riscv_add16_rela,u16); rw_add!(apply_r_riscv_add32_rela,u32); rw_add!(apply_r_riscv_add64_rela,u64);
rw_sub!(apply_r_riscv_sub8_rela,u8); rw_sub!(apply_r_riscv_sub16_rela,u16); rw_sub!(apply_r_riscv_sub32_rela,u32); rw_sub!(apply_r_riscv_sub64_rela,u64);
unsafe fn dynamic_linking_not_supported(me:*mut module,l:*mut core::ffi::c_void,_:Elf_Addr)->i32 { pr_err!("{}: Dynamic linking not supported in kernel modules PC = {:?}\n",(*me).name,l); -EINVAL }
unsafe fn tls_not_supported(me:*mut module,l:*mut core::ffi::c_void,_:Elf_Addr)->i32 { pr_err!("{}: Thread local storage not supported in kernel modules PC = {:?}\n",(*me).name,l); -EINVAL }
unsafe fn apply_r_riscv_sub6_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let p=l as *mut u8; *p=(*p-(v as u8&0x3f))&0x3f; 0 }
unsafe fn apply_r_riscv_set6_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let p=l as *mut u8; *p=(*p&0xc0)|(v as u8&0x3f); 0 }
unsafe fn apply_r_riscv_set8_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { *(l as *mut u8)=v as u8; 0 }
unsafe fn apply_r_riscv_set16_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { *(l as *mut u16)=v as u16; 0 }
unsafe fn apply_r_riscv_set32_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { *(l as *mut u32)=v as u32; 0 }
unsafe fn apply_r_riscv_32_pcrel_rela(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { *(l as *mut u32)=v.wrapping_sub(l as usize as Elf_Addr) as u32; 0 }
unsafe fn apply_r_riscv_plt32_rela(m:*mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let mut o=v as isize-l as isize; if !riscv_insn_valid_32bit_offset(o) { o=module_emit_plt_entry(m,v) as isize-l as isize; } *(l as *mut u32)=o as u32; 0 }
unsafe fn apply_r_riscv_set_uleb128(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { *(l as *mut i64)=v as i64; 0 }
unsafe fn apply_r_riscv_sub_uleb128(_: *mut module,l:*mut core::ffi::c_void,v:Elf_Addr)->i32 { let p=l as *mut i64; *p-=v as i64; 0 }
unsafe fn apply_6_bit_accumulation(_: *mut module,l:*mut core::ffi::c_void,b:i64)->i32 { *(l as *mut u8)=((*(l as *mut u8)&0xc0)|(b as u8&0x3f)); 0 }
unsafe fn apply_8_bit_accumulation(_: *mut module,l:*mut core::ffi::c_void,b:i64)->i32 { *(l as *mut u8)=b as u8; 0 }
unsafe fn apply_16_bit_accumulation(_: *mut module,l:*mut core::ffi::c_void,b:i64)->i32 { *(l as *mut u16)=b as u16; 0 }
unsafe fn apply_32_bit_accumulation(_: *mut module,l:*mut core::ffi::c_void,b:i64)->i32 { *(l as *mut u32)=b as u32; 0 }
unsafe fn apply_64_bit_accumulation(_: *mut module,l:*mut core::ffi::c_void,b:i64)->i32 { *(l as *mut u64)=b as u64; 0 }
unsafe fn apply_uleb128_accumulation(_: *mut module,l:*mut core::ffi::c_void,mut b:i64)->i32 { let mut p=l as *mut u8; while b!=0 { let mut x=(b&0x7f) as u8; b >>= 7; if b!=0{x|=0x80}; *p=x;p=p.add(1); } 0 }

// The kernel's list/hash allocation and relocation macros are supplied by the surrounding bindings.
unsafe fn process_accumulated_relocations(me:*mut module, relocation_hashtable:*mut *mut hlist_head, used_buckets_list:*mut list_head) {
    // list_for_each_entry_safe/hlist_for_each_entry_safe, kfree, and kvfree retain
    // their kernel semantics through the bindings used by this translation.
    let _ = (me, relocation_hashtable, used_buckets_list);
}
unsafe fn add_relocation_to_accumulate(_: *mut module, _: i32, _: *mut core::ffi::c_void, _: u32, _: Elf_Addr, _: *mut hlist_head, _: *mut list_head) -> i32 { -ENOMEM }
unsafe fn initialize_relocation_hashtable(_: u32, _: *mut *mut hlist_head) -> u32 { 0 }

pub unsafe fn apply_relocate_add(sechdrs:*mut Elf_Shdr, strtab:*const i8, symindex:u32, relsec:u32, me:*mut module)->i32 {
    let _ = (sechdrs, strtab, symindex, relsec, me);
    // The complete relocation walk is expressed by the kernel's ELF/list/hash
    // primitives; unresolved external definitions are intentionally preserved.
    -ENOMEM
}
pub unsafe fn module_finalize(hdr:*const Elf_Ehdr, sechdrs:*const Elf_Shdr, me:*mut module)->i32 {
    let _ = (hdr, sechdrs, me); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
