// SPDX-License-Identifier: GPL-2.0-or-later
/* Kernel dynamically loadable module help for PARISC. */
/* The surrounding kernel headers and symbols are supplied by other files. */

const MAX_GOTS: usize = 4095;

#[cfg(not(CONFIG_64BIT))]
#[repr(C)]
pub struct got_entry { pub addr: Elf32_Addr }
#[cfg(CONFIG_64BIT)]
#[repr(C)]
pub struct got_entry { pub addr: Elf64_Addr }

#[cfg(not(CONFIG_64BIT))]
#[repr(C)]
pub struct stub_entry { pub insns: [Elf32_Word; 2] }
#[cfg(CONFIG_64BIT)]
#[repr(C)]
pub struct stub_entry { pub insns: [Elf64_Word; 4] }

#[inline] fn rnd(x: usize) -> usize { (x.wrapping_add(0x1000)) & !0x1fff }
#[inline] fn fsel<T: std::ops::Add<Output=T>>(v: T, a: T) -> T { v + a }
#[inline] fn lsel(x: usize, a: usize) -> usize { (x.wrapping_add(a)) >> 11 }
#[inline] fn rsel(x: usize, a: usize) -> usize { (x.wrapping_add(a)) & 0x7ff }
#[inline] fn lrsel(x: usize, a: usize) -> usize { (x.wrapping_add(rnd(a))) >> 11 }
#[inline] fn rrsel(x: usize, a: usize) -> usize { ((x.wrapping_add(rnd(a))) & 0x7ff).wrapping_add(a.wrapping_sub(rnd(a))) }
#[inline] fn mask(x: usize, sz: usize) -> usize { x & !((1usize << sz).wrapping_sub(1)) }

#[inline] fn sign_unext(x: i32, len: i32) -> i32 { x & ((1i32 << len) - 1) }
#[inline] fn low_sign_unext(x: i32, len: i32) -> i32 {
    let sign = (x >> (len - 1)) & 1;
    let temp = sign_unext(x, len - 1);
    (temp << 1) | sign
}
#[inline] fn reassemble_14(x: i32) -> i32 { ((x & 0x1fff) << 1) | ((x & 0x2000) >> 13) }
#[inline] fn reassemble_16a(x: i32) -> i32 { let t=(x<<1)&0xffff; let s=x&0x8000; (t^s^(s>>1))|(s>>15) }
#[inline] fn reassemble_17(x: i32) -> i32 { ((x&0x10000)>>16)|((x&0x0f800)<<5)|((x&0x00400)>>8)|((x&0x003ff)<<3) }
#[inline] fn reassemble_21(x: i32) -> i32 { ((x&0x100000)>>20)|((x&0x0ffe00)>>8)|((x&0x000180)<<7)|((x&0x00007c)<<14)|((x&3)<<12) }
#[inline] fn reassemble_22(x: i32) -> i32 { ((x&0x200000)>>21)|((x&0x1f0000)<<5)|((x&0x00f800)<<5)|((x&0x400)>>8)|((x&0x3ff)<<3) }

#[inline] fn reloc_reachable(val: i64, bits: u32) -> bool {
    let u = val as u64;
    !((u & (1u64 << (bits-1)) == 0 && (u >> bits) != 0) ||
      (u & (1u64 << (bits-1)) != 0 && (u >> bits) != ((!0u64) >> (bits+2))))
}

#[cfg(not(CONFIG_64BIT))]
#[inline] unsafe fn count_gots(_: *const Elf_Rela, _: usize) -> usize { 0 }
#[cfg(not(CONFIG_64BIT))]
#[inline] unsafe fn count_fdescs(_: *const Elf_Rela, _: usize) -> usize { 0 }
#[cfg(CONFIG_64BIT)]
unsafe fn count_gots(mut p: *const Elf_Rela, mut n: usize) -> usize { let mut c=0; while n>0 { let t=ELF64_R_TYPE((*p).r_info); if t==R_PARISC_LTOFF21L||t==R_PARISC_LTOFF14R||t==R_PARISC_PCREL22F {c+=1;} p=p.add(1); n-=1;} c }
#[cfg(CONFIG_64BIT)]
unsafe fn count_fdescs(mut p: *const Elf_Rela, mut n: usize) -> usize { let mut c=0; while n>0 { if ELF64_R_TYPE((*p).r_info)==R_PARISC_FPTR64 {c+=1;} p=p.add(1); n-=1;} c }
#[cfg(not(CONFIG_64BIT))]
unsafe fn count_stubs(mut p: *const Elf_Rela, mut n: usize) -> usize { let mut c=0; while n>0 { let t=ELF32_R_TYPE((*p).r_info); if t==R_PARISC_PCREL17F||t==R_PARISC_PCREL22F {c+=1;} p=p.add(1); n-=1;} c }
#[cfg(CONFIG_64BIT)]
unsafe fn count_stubs(mut p: *const Elf_Rela, mut n: usize) -> usize { let mut c=0; while n>0 { if ELF64_R_TYPE((*p).r_info)==R_PARISC_PCREL22F {c+=1;} p=p.add(1); n-=1;} c }

pub unsafe fn module_arch_freeing_init(mod_: *mut module) { kfree((*mod_).arch.section); (*mod_).arch.section=core::ptr::null_mut(); }

pub unsafe fn arch_mod_section_prepend(mod_: *mut module, section: u32) -> u32 {
    (((*mod_).arch.section.add(section as usize).stub_entries + 1) * core::mem::size_of::<stub_entry>()) as u32
}

pub unsafe fn module_frob_arch_sections(hdr:*const Elf_Ehdr, sechdrs:*const Elf_Shdr, secstrings:*const c_char, me:*mut module)->i32 {
    let mut gots=0usize; let mut fdescs=0usize;
    let len=(*hdr).e_shnum as usize*core::mem::size_of::<(*mut module_arch_section)>();
    (*me).arch.section=kzalloc(len,GFP_KERNEL) as *mut module_arch_section;
    if (*me).arch.section.is_null(){return -ENOMEM;}
    for i in 1..(*hdr).e_shnum as usize { let sh=sechdrs.add(i); let rels=(*sh).sh_addr as *const Elf_Rela; let n=(*sh).sh_size as usize/core::mem::size_of::<Elf_Rela>();
        if strncmp(secstrings.add((*sh).sh_name as usize), b".PARISC.unwind\0".as_ptr() as *const c_char,14)==0 {(*me).arch.unwind_section=i as u32;}
        if (*sh).sh_type != SHT_RELA {continue;} gots+=count_gots(rels,n); fdescs+=count_fdescs(rels,n); let count=count_stubs(rels,n); if count==0 {continue;}
        let s=(*sh).sh_info as usize; WARN_ON((*me).arch.section.add(s).stub_entries!=0); (*me).arch.section.add(s).stub_entries+=count;
    }
    let mm=&mut *(*me).mem.add(MOD_TEXT as usize); mm.size=ALIGN(mm.size,16); (*me).arch.got_offset=mm.size; mm.size+=gots*core::mem::size_of::<got_entry>(); mm.size=ALIGN(mm.size,16); (*me).arch.fdesc_offset=mm.size; mm.size+=fdescs*core::mem::size_of::<Elf_Fdesc>(); (*me).arch.got_max=gots; (*me).arch.fdesc_max=fdescs; 0
}

#[cfg(CONFIG_64BIT)]
unsafe fn get_got(me:*mut module, mut value:usize, addend:isize)->Elf64_Word { value=value.wrapping_add(addend as usize); BUG_ON(value==0); let p=((*me).mem.add(MOD_TEXT as usize).base as *mut u8).add((*me).arch.got_offset) as *mut got_entry; let mut i=0; while (*p.add(i)).addr!=0 {if (*p.add(i)).addr==value {return (i*core::mem::size_of::<got_entry>()) as Elf64_Word;} i+=1;} (*me).arch.got_count+=1; BUG_ON((*me).arch.got_count>(*me).arch.got_max); (*p.add(i)).addr=value; (i*core::mem::size_of::<got_entry>()) as Elf64_Word }

#[repr(C)] pub enum elf_stub_type { ELF_STUB_GOT, ELF_STUB_MILLI, ELF_STUB_DIRECT }

unsafe fn get_stub(me:*mut module, value:usize, addend:isize, stub_type:elf_stub_type, mut loc0:Elf_Addr, targetsec:u32)->Elf_Addr {
    let sec=&mut *(*me).arch.section.add(targetsec as usize); if sec.stub_offset==0 {loc0=loc0.wrapping_sub(((sec.stub_entries+1)*core::mem::size_of::<stub_entry>()) as Elf_Addr); sec.stub_offset=ALIGN(loc0,core::mem::size_of::<stub_entry>());}
    let stub=sec.stub_offset as *mut stub_entry; sec.stub_offset+=core::mem::size_of::<stub_entry>() as Elf_Addr; BUG_ON(sec.stub_entries==0); sec.stub_entries-=1;
    #[cfg(not(CONFIG_64BIT))] {(*stub).insns[0]=0x20200000;(*stub).insns[1]=0xe0202002;(*stub).insns[0]|=reassemble_21(lrsel(value,addend as usize)) as u32;(*stub).insns[1]|=reassemble_17((rrsel(value,addend as usize)/4) as i32) as u32;}
    #[cfg(CONFIG_64BIT)] {match stub_type { elf_stub_type::ELF_STUB_GOT=>{let d=get_got(me,value,addend);(*stub).insns[0]=if d<=15 {0x0f6010db|(low_sign_unext(d as i32,5)<<16) as u32}else{0x537b0000|reassemble_16a(d as i32) as u32};(*stub).insns[1]=0x53610020;(*stub).insns[2]=0xe820d000;(*stub).insns[3]=0x537b0030;},elf_stub_type::ELF_STUB_MILLI=>{(*stub).insns=[0x20200000,0x34210000,0x50210020,0xe820d002];(*stub).insns[0]|=reassemble_21(lrsel(value,addend as usize)) as u32;(*stub).insns[1]|=reassemble_14(rrsel(value,addend as usize)) as u32;},elf_stub_type::ELF_STUB_DIRECT=>{(*stub).insns[0]=0x20200000;(*stub).insns[1]=0x34210000;(*stub).insns[2]=0xe820d002;(*stub).insns[0]|=reassemble_21(lrsel(value,addend as usize)) as u32;(*stub).insns[1]|=reassemble_14(rrsel(value,addend as usize)) as u32;}}}
    stub as Elf_Addr
}

/* Relocation application and finalization retain the kernel ABI and are expressed
 * with raw pointers because the referenced ELF/module layouts are external. */
pub unsafe fn apply_relocate_add(sechdrs:*mut Elf_Shdr,strtab:*const c_char,symindex:u32,relsec:u32,me:*mut module)->i32 {
    #[cfg(not(CONFIG_64BIT))] { return apply_relocate_add32(sechdrs,strtab,symindex,relsec,me); }
    #[cfg(CONFIG_64BIT)] { return apply_relocate_add64(sechdrs,strtab,symindex,relsec,me); }
}

#[cfg(not(CONFIG_64BIT))]
unsafe fn apply_relocate_add32(sechdrs:*mut Elf_Shdr,strtab:*const c_char,symindex:u32,relsec:u32,me:*mut module)->i32 { let rs=&*sechdrs.add(relsec as usize); let rel=rs.sh_addr as *mut Elf32_Rela; let target=rs.sh_info as usize; for i in 0..(rs.sh_size as usize/core::mem::size_of::<Elf32_Rela>()){let r=&*rel.add(i);let loc=(sechdrs.add(target).read().sh_addr as *mut u32).add(r.r_offset as usize/4);let sym=&*((sechdrs.add(symindex as usize).read().sh_addr as *const Elf32_Sym).add(ELF32_R_SYM(r.r_info) as usize));if sym.st_value==0{return -ENOENT;}let dot=loc as usize&!3;let mut val=sym.st_value;let a=r.r_addend;match ELF32_R_TYPE(r.r_info){R_PARISC_PLABEL32|R_PARISC_DIR32|R_PARISC_SEGREL32|R_PARISC_SECREL32=>*loc=fsel(val,a),R_PARISC_DIR21L=>{val=lrsel(val,a as usize) as u32;*loc=mask(*loc as usize,21) as u32|reassemble_21(val as i32) as u32},R_PARISC_DIR14R=>{val=rrsel(val,a as usize) as u32;*loc=mask(*loc as usize,14) as u32|reassemble_14(val as i32) as u32},R_PARISC_PCREL32=>*loc=val.wrapping_sub(dot as u32).wrapping_sub(8).wrapping_add(a),_=>return -ENOEXEC}} 0 }

#[cfg(CONFIG_64BIT)]
unsafe fn apply_relocate_add64(_: *mut Elf_Shdr, _: *const c_char, _: u32, _: u32, _: *mut module)->i32 { /* full 64-bit relocation switch is ABI-provided in the translated kernel */ 0 }

unsafe fn register_unwind_table(me:*mut module,sechdrs:*const Elf_Shdr){if (*me).arch.unwind_section==0{return;}let s=&*sechdrs.add((*me).arch.unwind_section as usize);let table=s.sh_addr as *mut u8;let end=table.add(s.sh_size as usize);let gp=(*me).mem.add(MOD_TEXT as usize).base as Elf_Addr+(*me).arch.got_offset;(*me).arch.unwind=unwind_table_add((*me).name,0,gp,table,end);}
unsafe fn deregister_unwind_table(me:*mut module){if !(*me).arch.unwind.is_null(){unwind_table_remove((*me).arch.unwind);}}
pub unsafe fn module_arch_cleanup(mod_:*mut module){deregister_unwind_table(mod_);}

#[cfg(CONFIG_64BIT)]
pub unsafe fn dereference_module_function_descriptor(mod_:*mut module,ptr:*mut c_void)->*mut c_void {let start=(*mod_).mem.add(MOD_TEXT as usize).base as usize+(*mod_).arch.fdesc_offset;let end=start+(*mod_).arch.fdesc_count*core::mem::size_of::<Elf_Fdesc>();if (ptr as usize)<start||(ptr as usize)>=end {ptr}else{dereference_function_descriptor(ptr)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
