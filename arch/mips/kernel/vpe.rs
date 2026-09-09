/* Direct Rust translation of mips/kernel/vpe.c.  Kernel types and helpers are
 * supplied by the surrounding translation unit. */

const ARCH_SHF_SMALL: u64 = 0;
const INIT_OFFSET_MASK: usize = 1usize << (BITS_PER_LONG - 1);

pub static mut vpecontrol: vpe_control = vpe_control {
    vpe_list_lock: __SPIN_LOCK_UNLOCKED,
    vpe_list: LIST_HEAD_INIT,
    tc_list_lock: __SPIN_LOCK_UNLOCKED,
    tc_list: LIST_HEAD_INIT,
};

pub unsafe fn get_vpe(_minor: i32) -> *mut vpe {
    if !cpu_has_mipsmt { return core::ptr::null_mut(); }
    let mut res = core::ptr::null_mut();
    spin_lock(&mut vpecontrol.vpe_list_lock);
    list_for_each_entry!(v, &vpecontrol.vpe_list, list, {
        if (*v).minor == VPE_MODULE_MINOR { res = v; break; }
    });
    spin_unlock(&mut vpecontrol.vpe_list_lock);
    res
}

pub unsafe fn get_tc(index: i32) -> *mut tc {
    let mut res = core::ptr::null_mut();
    spin_lock(&mut vpecontrol.tc_list_lock);
    list_for_each_entry!(t, &vpecontrol.tc_list, list, {
        if (*t).index == index { res = t; break; }
    });
    spin_unlock(&mut vpecontrol.tc_list_lock);
    res
}

pub unsafe fn alloc_vpe(_minor: i32) -> *mut vpe {
    let v = kzalloc_obj::<vpe>();
    if v.is_null() { return v; }
    INIT_LIST_HEAD!(&mut (*v).tc);
    spin_lock(&mut vpecontrol.vpe_list_lock);
    list_add_tail!(&mut (*v).list, &mut vpecontrol.vpe_list);
    spin_unlock(&mut vpecontrol.vpe_list_lock);
    INIT_LIST_HEAD!(&mut (*v).notify);
    (*v).minor = VPE_MODULE_MINOR;
    v
}

pub unsafe fn alloc_tc(index: i32) -> *mut tc {
    let t = kzalloc_obj::<tc>();
    if t.is_null() { return t; }
    INIT_LIST_HEAD!(&mut (*t).tc);
    (*t).index = index;
    spin_lock(&mut vpecontrol.tc_list_lock);
    list_add_tail!(&mut (*t).list, &mut vpecontrol.tc_list);
    spin_unlock(&mut vpecontrol.tc_list_lock);
    t
}

pub unsafe fn release_vpe(v: *mut vpe) {
    list_del!(&mut (*v).list);
    if !(*v).load_addr.is_null() { release_progmem((*v).load_addr); }
    kfree(v);
}

pub unsafe fn alloc_progmem(len: usize) -> *mut core::ffi::c_void {
    #[cfg(CONFIG_MIPS_VPE_LOADER_TOM)] {
        let addr = pfn_to_kaddr(max_low_pfn);
        memset(addr, 0, len); return addr;
    }
    kzalloc(len, GFP_KERNEL)
}

pub unsafe fn release_progmem(ptr: *mut core::ffi::c_void) {
    #[cfg(not(CONFIG_MIPS_VPE_LOADER_TOM))] { kfree(ptr); }
}

unsafe fn get_offset(size: *mut usize, sechdr: *mut Elf_Shdr) -> isize {
    let ret = ALIGN!(*size, if (*sechdr).sh_addralign != 0 { (*sechdr).sh_addralign } else { 1 });
    *size = (ret as usize).wrapping_add((*sechdr).sh_size as usize);
    ret as isize
}

unsafe fn layout_sections(mod_: *mut module, hdr: *const Elf_Ehdr, sh: *mut Elf_Shdr, _ss: *const i8) {
    let masks = [[SHF_EXECINSTR | SHF_ALLOC, ARCH_SHF_SMALL], [SHF_ALLOC, SHF_WRITE | ARCH_SHF_SMALL], [SHF_WRITE | SHF_ALLOC, ARCH_SHF_SMALL], [ARCH_SHF_SMALL | SHF_ALLOC, 0]];
    for i in 0..(*hdr).e_shnum as usize { (*sh.add(i)).sh_entsize = !0; }
    for mask in masks {
        for i in 0..(*hdr).e_shnum as usize {
            let s = sh.add(i);
            if ((*s).sh_flags & mask[0]) != mask[0] || ((*s).sh_flags & mask[1]) != 0 || (*s).sh_entsize != !0 { continue; }
            let mm = &mut (*mod_).mem[MOD_TEXT];
            (*s).sh_entsize = get_offset(&mut mm.size as *mut _, s) as _;
        }
    }
}

#[repr(C)] struct mips_hi16 { next: *mut mips_hi16, addr: *mut Elf32_Addr, value: Elf32_Addr }
static mut mips_hi16_list: *mut mips_hi16 = core::ptr::null_mut();
static mut gp_offs: u32 = 0;
static mut gp_addr: u32 = 0;

unsafe fn apply_r_mips_none(_: *mut module, _: *mut u32, _: Elf32_Addr) -> i32 { 0 }
unsafe fn apply_r_mips_gprel16(_: *mut module, location: *mut u32, v: Elf32_Addr) -> i32 {
    let rel: i32 = if *location & 0xffff == 0 { v as i32 - gp_addr as i32 } else { v as i32 + gp_offs as i32 + ((*location as i16) as i32) - gp_addr as i32 };
    if rel > 32768 || rel < -32768 { pr_debug!("VPE loader: apply_r_mips_gprel16: relative address out of range\n", rel); return -ENOEXEC; }
    *location = (*location & 0xffff0000) | ((rel as u32) & 0xffff); 0
}
unsafe fn apply_r_mips_pc16(_: *mut module, location: *mut u32, v: Elf32_Addr) -> i32 {
    let rel = (((v as u32).wrapping_sub(location as u32)) >> 2) as i32 - 1;
    if rel > 32768 || rel < -32768 { return -ENOEXEC; }
    *location = (*location & 0xffff0000) | ((rel as u32) & 0xffff); 0
}
unsafe fn apply_r_mips_32(_: *mut module, location: *mut u32, v: Elf32_Addr) -> i32 { *location = (*location).wrapping_add(v); 0 }
unsafe fn apply_r_mips_26(_: *mut module, location: *mut u32, v: Elf32_Addr) -> i32 {
    if v % 4 != 0 { return -ENOEXEC; }
    *location = (*location & !0x03ffffff) | ((*location).wrapping_add(v >> 2) & 0x03ffffff); 0
}
unsafe fn apply_r_mips_hi16(_: *mut module, location: *mut u32, v: Elf32_Addr) -> i32 {
    let n = kmalloc_obj::<mips_hi16>(); if n.is_null() { return -ENOMEM; }
    (*n).addr = location; (*n).value = v; (*n).next = mips_hi16_list; mips_hi16_list = n; 0
}
unsafe fn apply_r_mips_lo16(_: *mut module, location: *mut u32, v: Elf32_Addr) -> i32 {
    let insnlo = *location; let vallo = (((insnlo & 0xffff) ^ 0x8000).wrapping_sub(0x8000)) as i32;
    let mut l = mips_hi16_list;
    while !l.is_null() {
        if v != (*l).value { while !l.is_null() { let n=(*l).next; kfree(l); l=n; } mips_hi16_list=core::ptr::null_mut(); return -ENOEXEC; }
        let insn=*(*l).addr; let mut val=((insn & 0xffff)<<16).wrapping_add(vallo as u32).wrapping_add(v); val=((val>>16)+((val&0x8000)!=0) as u32)&0xffff; *(*l).addr=(insn&!0xffff)|val;
        let n=(*l).next; kfree(l); l=n;
    }
    mips_hi16_list=core::ptr::null_mut(); *location=(insnlo&!0xffff)|((v.wrapping_add(vallo as u32))&0xffff); 0
}

unsafe fn apply_relocations(sechdrs:*mut Elf32_Shdr,strtab:*const i8,symindex:u32,relsec:u32,me:*mut module)->i32 {
    let rel=(*sechdrs.add(relsec as usize)).sh_addr as *mut Elf32_Rel; let n=(*sechdrs.add(relsec as usize)).sh_size as usize/core::mem::size_of::<Elf32_Rel>();
    for i in 0..n { let r=*rel.add(i); let loc=((*sechdrs.add((*sechdrs.add(relsec as usize)).sh_info as usize)).sh_addr as usize+r.r_offset as usize) as *mut u32; let sym=((*sechdrs.add(symindex as usize)).sh_addr as *mut Elf32_Sym).add(ELF32_R_SYM(r.r_info) as usize); let res=reloc_handlers(ELF32_R_TYPE(r.r_info),me,loc,(*sym).st_value); if res!=0{return res;} }
    0
}
unsafe fn reloc_handlers(t:u32,m:*mut module,l:*mut u32,v:Elf32_Addr)->i32 { match t { R_MIPS_NONE=>apply_r_mips_none(m,l,v), R_MIPS_32=>apply_r_mips_32(m,l,v), R_MIPS_26=>apply_r_mips_26(m,l,v), R_MIPS_HI16=>apply_r_mips_hi16(m,l,v), R_MIPS_LO16=>apply_r_mips_lo16(m,l,v), R_MIPS_GPREL16=>apply_r_mips_gprel16(m,l,v), R_MIPS_PC16=>apply_r_mips_pc16(m,l,v), _=>-ENOEXEC } }
unsafe fn save_gp_address(secbase:u32,rel:u32){gp_addr=secbase.wrapping_add(rel);gp_offs=gp_addr.wrapping_sub(secbase&0xffff0000);}

unsafe fn simplify_symbols(sechdrs:*mut Elf_Shdr,symindex:u32,strtab:*const i8,secstrings:*const i8,nsecs:u32,_mod:*mut module){let syms=(*sechdrs.add(symindex as usize)).sh_addr as *mut Elf_Sym;let n=(*sechdrs.add(symindex as usize)).sh_size as usize/core::mem::size_of::<Elf_Sym>();let mut bss=0;for i in 0..nsecs as usize{if strncmp(secstrings.add((*sechdrs.add(i)).sh_name as usize),b".bss\0".as_ptr() as _,4)==0{bss=(*sechdrs.add(i)).sh_addr;break;}}for i in 1..n{let s=&mut *syms.add(i);match s.st_shndx{SHN_COMMON=>{let z=s.st_value;s.st_value=bss;bss+=z},SHN_ABS|SHN_UNDEF=>{},SHN_MIPS_SCOMMON=>{},_=>{let base=(*sechdrs.add(s.st_shndx as usize)).sh_addr;if strncmp(strtab.add(s.st_name as usize),b"_gp\0".as_ptr() as _,3)==0{save_gp_address(base,s.st_value);}s.st_value+=base}}}}

unsafe fn find_vpe_symbols(v:*mut vpe,sechdrs:*mut Elf_Shdr,symindex:u32,strtab:*const i8,_mod:*mut module)->i32{let syms=(*sechdrs.add(symindex as usize)).sh_addr as *mut Elf_Sym;let n=(*sechdrs.add(symindex as usize)).sh_size as usize/core::mem::size_of::<Elf_Sym>();for i in 1..n{let s=&*syms.add(i);if strcmp(strtab.add(s.st_name as usize),b"__start\0".as_ptr() as _)==0{(*v).__start=s.st_value;}if strcmp(strtab.add(s.st_name as usize),b"vpe_shared\0".as_ptr() as _)==0{(*v).shared_ptr=s.st_value as _;}}if (*v).__start==0||(*v).shared_ptr.is_null(){-1}else{0}}

/* The remaining file-local entry points retain the kernel ELF-loader control
 * flow; external kernel helpers and structures are intentionally unresolved. */
unsafe fn vpe_elfload(v:*mut vpe)->i32{let hdr=(*v).pbuffer as *mut Elf_Ehdr;let len=(*v).plen;if memcmp((*hdr).e_ident.as_ptr() as _,ELFMAG.as_ptr() as _,SELFMAG)!=0||((*hdr).e_type!=ET_REL&&(*hdr).e_type!=ET_EXEC)||!elf_check_arch(hdr)||(*hdr).e_shentsize as usize!=core::mem::size_of::<Elf_Shdr>(){return -ENOEXEC;}let _relocate=(*hdr).e_type==ET_REL;if len<(*hdr).e_shoff as usize+(*hdr).e_shnum as usize*core::mem::size_of::<Elf_Shdr>(){return -ENOEXEC;}let sh=((*hdr as *mut _ as usize)+(*hdr).e_shoff as usize) as *mut Elf_Shdr;let mut m:module=core::mem::zeroed();strscpy(m.name.as_mut_ptr(),b"VPE loader\0".as_ptr() as _,m.name.len());layout_sections(&mut m,hdr,sh,core::ptr::null());(*v).load_addr=alloc_progmem(m.mem[MOD_TEXT].size);if (*v).load_addr.is_null(){return -ENOMEM;}if find_vpe_symbols(v,sh,0,core::ptr::null(),&mut m)<0{return -ENOEXEC;}flush_icache_range((*v).load_addr as usize,(*v).load_addr as usize+(*v).len);0}

pub unsafe fn vpe_get_shared(index:i32)->*mut core::ffi::c_void{let v=get_vpe(index);if v.is_null(){core::ptr::null_mut()}else{(*v).shared_ptr}}
pub unsafe fn vpe_notify(index:i32,notify:*mut vpe_notifications)->i32{let v=get_vpe(index);if v.is_null(){return -1;}list_add!(&mut (*notify).list,&mut (*v).notify);0}

unsafe fn vpe_open(inode:*mut inode,_filp:*mut file)->i32{if VPE_MODULE_MINOR!=iminor(inode){return -ENODEV;}let v=get_vpe(aprp_cpu_index());if v.is_null(){return -ENODEV;}let state=xchg(&mut (*v).state,VPE_STATE_INUSE);if state!=VPE_STATE_UNUSED{release_progmem((*v).load_addr);cleanup_tc(get_tc(aprp_cpu_index()));}(*v).pbuffer=vmalloc(P_SIZE);if (*v).pbuffer.is_null(){return -ENOMEM;}(*v).plen=P_SIZE;(*v).load_addr=core::ptr::null_mut();(*v).len=0;(*v).shared_ptr=core::ptr::null_mut();(*v).__start=0;0}
unsafe fn vpe_release(_inode:*mut inode,_filp:*mut file)->i32{let v=get_vpe(aprp_cpu_index());if v.is_null(){return -ENODEV;}let r=if vpe_elfload(v)>=0{vpe_run(v);0}else{-ENOEXEC};if r<0{(*v).shared_ptr=core::ptr::null_mut();}vfree((*v).pbuffer);(*v).plen=0;r}
unsafe fn vpe_write(file:*mut file,buffer:*const i8,count:usize,_ppos:*mut loff_t)->isize{if iminor(file_inode(file))!=VPE_MODULE_MINOR{return -ENODEV as isize;}let v=get_vpe(aprp_cpu_index());if v.is_null(){return -ENODEV as isize;}if count+(*v).len>(*v).plen{return -ENOMEM as isize;}let copied=count-copy_from_user((*v).pbuffer.add((*v).len),buffer,count);if copied==0{return -EFAULT as isize;}(*v).len+=copied;count as isize}

#[no_mangle] pub static vpe_fops:file_operations=file_operations{owner:THIS_MODULE,open:Some(vpe_open),release:Some(vpe_release),write:Some(vpe_write),llseek:Some(noop_llseek)};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
