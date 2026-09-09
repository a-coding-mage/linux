// SPDX-License-Identifier: GPL-2.0-only
// Translation of linux/arch/arm/kernel/module.c.

use core::ffi::{c_char, c_int, c_void};

type U8 = u8; type U16 = u16; type U32 = u32; type S32 = i32;

#[repr(C)] pub struct Elf32_Shdr { pub sh_name:u32,pub sh_type:u32,pub sh_flags:u32,pub sh_addr:usize,pub sh_offset:usize,pub sh_size:usize,pub sh_link:u32,pub sh_info:u32,pub sh_addralign:usize,pub sh_entsize:usize }
#[repr(C)] pub struct Elf32_Ehdr { pub e_shstrndx:u16,pub e_shnum:u16 }
#[repr(C)] pub struct Elf32_Rel { pub r_offset:u32,pub r_info:u32 }
#[repr(C)] pub struct Elf32_Sym { pub st_name:u32,pub st_value:u32,pub st_info:u8 }
#[repr(C)] pub struct ListHead { pub next:*mut ListHead,pub prev:*mut ListHead }
#[repr(C)] pub struct Arch { pub unwind_list:ListHead,pub init_table:*mut c_void }
#[repr(C)] pub struct Module { pub name:*const c_char,pub arch:Arch }

extern "C" {
    fn strstarts(s:*const c_char, prefix:*const c_char)->bool;
    fn strcmp(a:*const c_char,b:*const c_char)->c_int;
    fn strlen(s:*const c_char)->usize;
    fn __fls(x:u32)->u32;
    fn sign_extend32(x:u32, index:u32)->i32;
    fn ror32(x:u32, shift:u32)->u32;
    fn __mem_to_opcode_arm(x:u32)->u32; fn __opcode_to_mem_arm(x:u32)->u32;
    fn __mem_to_opcode_thumb16(x:u16)->u16; fn __opcode_to_mem_thumb16(x:u16)->u16;
    fn get_module_plt(m:*mut Module, loc:usize, target:i32)->usize;
    fn is_smp()->bool; fn fixup_pv_table(p:*const c_void, size:usize); fn fixup_smp(p:*const c_void,size:usize);
    fn unwind_table_add(a:usize,b:usize,c:usize,d:usize)->*mut c_void; fn unwind_table_del(p:*mut c_void);
    fn INIT_LIST_HEAD(p:*mut ListHead); fn list_add(a:*mut ListHead,b:*mut ListHead);
    fn list_del(a:*mut ListHead); fn pr_err(fmt:*const c_char,...);
}

pub unsafe fn module_init_section(name:*const c_char)->bool { strstarts(name,b".init\0".as_ptr() as _) || strstarts(name,b".ARM.extab.init\0".as_ptr() as _) || strstarts(name,b".ARM.exidx.init\0".as_ptr() as _) }
pub unsafe fn module_exit_section(name:*const c_char)->bool { strstarts(name,b".exit\0".as_ptr() as _) || strstarts(name,b".ARM.extab.exit\0".as_ptr() as _) || strstarts(name,b".ARM.exidx.exit\0".as_ptr() as _) }

#[cfg(feature="CONFIG_ARM_HAS_GROUP_RELOCS")]
unsafe fn get_group_rem(group:&mut u32, offset:&mut u32)->u32 { let mut val=*offset; let shift; loop { shift=if val!=0 {(31-__fls(val))&!1} else {32}; *offset=val; if val==0 {break} val &= 0xffffff >> shift; if *group==0 {break} *group-=1; } shift }

const R_ARM_NONE:u32=0; const R_ARM_ABS32:u32=2; const R_ARM_TARGET1:u32=38; const R_ARM_PC24:u32=1; const R_ARM_CALL:u32=28; const R_ARM_JUMP24:u32=29; const R_ARM_V4BX:u32=40; const R_ARM_PREL31:u32=42; const R_ARM_REL32:u32=3;
const R_ARM_MOVW_ABS_NC:u32=43; const R_ARM_MOVT_ABS:u32=44; const R_ARM_MOVW_PREL_NC:u32=45; const R_ARM_MOVT_PREL:u32=46;
#[cfg(feature="CONFIG_ARM_HAS_GROUP_RELOCS")] const R_ARM_ALU_PC_G0_NC:u32=57;
#[cfg(feature="CONFIG_ARM_HAS_GROUP_RELOCS")] const R_ARM_ALU_PC_G1_NC:u32=58;
#[cfg(feature="CONFIG_ARM_HAS_GROUP_RELOCS")] const R_ARM_LDR_PC_G2:u32=59;
extern "C" { fn ELF32_R_SYM(x:u32)->i32; fn ELF32_R_TYPE(x:u32)->u32; fn ELF32_ST_TYPE(x:u8)->u32; }
const STT_FUNC:u32=2; const ENOEXEC:c_int=8; const EINVAL:c_int=22; const SHF_ALLOC:u32=2; const ELF_SECTION_UNWIND:u32=0x70000001;

pub unsafe fn apply_relocate(sechdrs:*mut Elf32_Shdr,strtab:*const c_char,symindex:u32,relindex:u32,module:*mut Module)->c_int {
 let symsec=sechdrs.add(symindex as usize); let relsec=sechdrs.add(relindex as usize); let dstsec=sechdrs.add((*relsec).sh_info as usize); let mut rel=(*relsec).sh_addr as *mut Elf32_Rel;
 let count=(*relsec).sh_size/core::mem::size_of::<Elf32_Rel>();
 for i in 0..count { let mut loc:usize; let mut sym:*mut Elf32_Sym; let symname:*const c_char; let mut offset:S32; let mut tmp:U32;
  offset=ELF32_R_SYM((*rel).r_info); if offset<0 || offset as usize > (*symsec).sh_size/core::mem::size_of::<Elf32_Sym>() { return -ENOEXEC; }
  sym=((*symsec).sh_addr as *mut Elf32_Sym).add(offset as usize); symname=strtab.add((*sym).st_name as usize);
  if (*rel).r_offset as usize > (*dstsec).sh_size-core::mem::size_of::<u32>() { return -ENOEXEC; } loc=(*dstsec).sh_addr+(*rel).r_offset as usize;
  match ELF32_R_TYPE((*rel).r_info) {
   R_ARM_NONE=>{}, R_ARM_ABS32|R_ARM_TARGET1=>{ *(loc as *mut u32)=(*(loc as *mut u32)).wrapping_add((*sym).st_value); },
   R_ARM_PC24|R_ARM_CALL|R_ARM_JUMP24=>{ if (*sym).st_value&3!=0{return -ENOEXEC;} offset=sign_extend32((__mem_to_opcode_arm(*(loc as *mut u32))&0xffffff)<<2,25); offset=offset.wrapping_add((*sym).st_value as i32).wrapping_sub(loc as i32); if offset<=-33554432||offset>=33554432{return -ENOEXEC;} let v=((offset>>2) as u32)&0xffffff; *(loc as *mut u32)=__opcode_to_mem_arm((__mem_to_opcode_arm(*(loc as *mut u32))&0xff000000)|v); },
   R_ARM_V4BX=>{let p=loc as *mut u32; *p=__opcode_to_mem_arm(__mem_to_opcode_arm(*p)&0xf000000f|0x01a0f000);},
   R_ARM_PREL31=>{offset=((*(loc as *mut i32))<<1)>>1; offset=offset.wrapping_add((*sym).st_value as i32).wrapping_sub(loc as i32); if offset>=0x40000000||offset< -0x40000000{return -ENOEXEC;} let p=loc as *mut u32; *p=(*p&0x80000000)|(offset as u32&0x7fffffff);},
   R_ARM_REL32=>{*(loc as *mut u32)=(*(loc as *mut u32)).wrapping_add((*sym).st_value).wrapping_sub(loc as u32);},
   R_ARM_MOVW_ABS_NC|R_ARM_MOVT_ABS|R_ARM_MOVW_PREL_NC|R_ARM_MOVT_PREL=>{ tmp=__mem_to_opcode_arm(*(loc as *mut u32)); offset=sign_extend32(((tmp&0xf0000)>>4)|(tmp&0xfff),15); offset=offset.wrapping_add((*sym).st_value as i32); if ELF32_R_TYPE((*rel).r_info)==R_ARM_MOVT_PREL||ELF32_R_TYPE((*rel).r_info)==R_ARM_MOVW_PREL_NC{offset=offset.wrapping_sub(loc as i32);} if ELF32_R_TYPE((*rel).r_info)==R_ARM_MOVT_ABS||ELF32_R_TYPE((*rel).r_info)==R_ARM_MOVT_PREL{offset >>=16;} tmp=(tmp&0xfff0f000)|(((offset as u32&0xf000)<<4)|(offset as u32&0xfff)); *(loc as *mut u32)=__opcode_to_mem_arm(tmp); },
   #[cfg(feature="CONFIG_ARM_HAS_GROUP_RELOCS")] R_ARM_ALU_PC_G0_NC|R_ARM_ALU_PC_G1_NC=>{ let mut group=if ELF32_R_TYPE((*rel).r_info)==R_ARM_ALU_PC_G0_NC{0}else{1}; tmp=__mem_to_opcode_arm(*(loc as *mut u32)); offset=ror32(tmp&0xff,(tmp&0xf00)>>7) as i32; if tmp&(1<<22)!=0{offset=-offset;} offset=offset.wrapping_add((*sym).st_value as i32).wrapping_sub(loc as i32); if offset<0{offset=-offset;tmp=(tmp&!(1<<23))|(1<<22);}else{tmp=(tmp&!(1<<22))|(1<<23);} let mut o=offset as u32; let shift=get_group_rem(&mut group,&mut o); if shift<24{o >>=24-shift;o|=(shift+8)<<7;} *(loc as *mut u32)=__opcode_to_mem_arm((tmp&!0xfff)|o); },
   #[cfg(feature="CONFIG_ARM_HAS_GROUP_RELOCS")] R_ARM_LDR_PC_G2=>{tmp=__mem_to_opcode_arm(*(loc as *mut u32));offset=(tmp&0xfff) as i32;if tmp&(1<<23)==0{offset=-offset;}offset=offset.wrapping_add((*sym).st_value as i32).wrapping_sub(loc as i32);if offset<0{offset=-offset;tmp&=!(1<<23)}else{tmp|=1<<23}let mut g=2;let mut o=offset as u32;get_group_rem(&mut g,&mut o);if o>0xfff{return -ENOEXEC;}*(loc as *mut u32)=__opcode_to_mem_arm((tmp&!0xfff)|o);},
   _=>return -ENOEXEC,
  } rel=rel.add(1);
 }
 0
}

unsafe fn find_mod_section(hdr:*const Elf32_Ehdr,sechdrs:*const Elf32_Shdr,name:*const c_char)->*const Elf32_Shdr { let secstrs=(hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset); for i in 0..(*hdr).e_shnum as usize {let s=sechdrs.add(i);if strcmp(name,secstrs.add((*s).sh_name as usize) as _)==0{return s;}} core::ptr::null() }

pub unsafe fn module_finalize(hdr:*const Elf32_Ehdr,sechdrs:*const Elf32_Shdr,modp:*mut Module)->c_int { let mut s=find_mod_section(hdr,sechdrs,b".pv_table\0".as_ptr() as _); if !s.is_null(){fixup_pv_table((*s).sh_addr as _,(*s).sh_size);} s=find_mod_section(hdr,sechdrs,b".alt.smp.init\0".as_ptr() as _); if !s.is_null()&&!is_smp(){fixup_smp((*s).sh_addr as _,(*s).sh_size);} 0 }
pub unsafe fn module_arch_cleanup(_modp:*mut Module) {}
pub unsafe fn module_arch_freeing_init(_modp:*mut Module) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
