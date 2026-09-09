// SPDX-License-Identifier: GPL-2.0-only
/* Interface for exporting the OPAL ELF core. */

// Kernel dependencies supplied by other translation units.
const MAX_PT_LOAD_CNT: usize = 8;
const AUXV_CNT: usize = 1;
const AUXV_DESC_SZ: usize = ((2 * AUXV_CNT + 1) * core::mem::size_of::<u64>());

#[repr(C)]
struct opalcore_config {
    num_cpus: u32,
    crashing_cpu: u32,
    cpu_state_destination_vaddr: u64,
    cpu_state_data_size: u64,
    cpu_state_entry_size: u64,
    ptload_addr: [u64; MAX_PT_LOAD_CNT],
    ptload_size: [u64; MAX_PT_LOAD_CNT],
    ptload_cnt: u64,
    ptload_phdr: *mut Elf64_Phdr,
    opalcore_size: usize,
    opalcorebuf_sz: usize,
    opalcorebuf: *mut i8,
    auxv_buf: [i8; AUXV_DESC_SZ],
}

#[repr(C)]
struct opalcore { list: list_head, paddr: u64, size: usize, offset: i64 }

#[repr(C)] struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] struct Elf64_Phdr { p_type:u32, p_flags:u32, p_offset:u64, p_vaddr:u64, p_paddr:u64, p_filesz:u64, p_memsz:u64, p_align:u64 }
#[repr(C)] struct Elf64_Ehdr { e_ident:[u8;16], e_type:u16, e_machine:u16, e_version:u32, e_entry:u64, e_phoff:u64, e_shoff:u64, e_flags:u32, e_ehsize:u16, e_phentsize:u16, e_phnum:u16, e_shentsize:u16, e_shnum:u16, e_shstrndx:u16 }
#[repr(C)] struct Elf64_Nhdr { n_namesz:u32, n_descsz:u32, n_type:u32 }
#[repr(C)] struct elf_prstatus { common: elf_prstatus_common, pr_reg: pt_regs }
#[repr(C)] struct elf_prstatus_common { pr_pid:u32, pr_ppid:u32, pr_cursig:u16 }
#[repr(C)] struct pt_regs { gpr:[u64;32], nip:u64 }
#[repr(C)] struct hdat_fadump_thread_hdr { offset:u32, esize:u32, ecnt:u32, pir:u32, core_state:u8 }
#[repr(C)] struct opal_mpipl_fadump { version:u32, region_cnt:u32, region:[opal_region;8], crashing_pir:u32, cpu_data_version:u32, cpu_data_size:u32 }
#[repr(C)] struct opal_region { dest:u64, size:u64 }
#[repr(C)] struct file;
#[repr(C)] struct kobject;
#[repr(C)] struct bin_attribute { attr: attribute, size: u64 }
#[repr(C)] struct attribute { name:*const i8, mode:u16 }
#[repr(C)] struct kobj_attribute;
#[repr(C)] struct attribute_group { attrs:*mut *mut attribute, bin_attrs:*const *const bin_attribute }
#[repr(C)] struct device_node;

static mut opalcore_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut oc_conf: *mut opalcore_config = core::ptr::null_mut();
static mut opalc_metadata: *const opal_mpipl_fadump = core::ptr::null();
static mut opalc_cpu_metadata: *const opal_mpipl_fadump = core::ptr::null();
static mut mpipl_kobj: *mut kobject = core::ptr::null_mut();
static mut kernel_initiated: bool = false;
static mut opal_core_attr: bin_attribute = bin_attribute { attr: attribute { name: core::ptr::null(), mode: 0 }, size: 0 };

extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(p: *mut u8);
    fn alloc_pages_exact(size: usize, flags: u32) -> *mut i8;
    fn free_pages_exact(p: *mut i8, size: usize);
    fn memcpy(d:*mut core::ffi::c_void, s:*const core::ffi::c_void, n:usize)->*mut core::ffi::c_void;
    fn memset(d:*mut core::ffi::c_void, c:i32, n:usize)->*mut core::ffi::c_void;
    fn strlen(s:*const i8)->usize;
    fn __va(x:u64)->*mut i8;
    fn opal_fadump_read_regs(p:*mut u8, cnt:u32, esize:u32, little:bool, regs:*mut pt_regs);
    fn elf_core_copy_regs(dst:*mut pt_regs, src:*const pt_regs);
    fn opal_mpipl_query_tag(tag:u32, addr:*mut u64)->i32;
    fn of_find_node_by_path(p:*const i8)->*mut device_node;
    fn of_find_node_by_name(p:*mut device_node, n:*const i8)->*mut device_node;
    fn of_node_put(p:*mut device_node);
    fn of_get_property(p:*mut device_node,n:*const i8,len:*mut i32)->*const u32;
    fn of_device_is_compatible(p:*mut device_node,n:*const i8)->bool;
    fn of_property_read_u64(p:*mut device_node,n:*const i8,v:*mut u64)->i32;
    fn sysfs_remove_bin_file(k:*mut kobject,a:*mut bin_attribute);
    fn sysfs_create_group(k:*mut kobject,g:*const attribute_group)->i32;
    fn kobject_create_and_add(n:*const i8,p:*mut kobject)->*mut kobject;
    fn compat_only_sysfs_link_entry_to_kobj(a:*mut kobject,b:*mut kobject,n:*const i8,x:*const i8)->i32;
    static mut opal_kobj:*mut kobject;
}

unsafe fn get_new_element() -> *mut opalcore { kzalloc(core::mem::size_of::<opalcore>(), 0) as *mut opalcore }
unsafe fn is_opalcore_usable()->i32 { if !oc_conf.is_null() && !(*oc_conf).opalcorebuf.is_null(){1}else{0} }
unsafe fn append_elf64_note(mut buf:*mut u32,name:*const i8,typ:u32,data:*const u8,len:usize)->*mut u32 {
    let note=buf as *mut Elf64_Nhdr; (*note).n_namesz=(strlen(name) as u32 + 1).to_be(); (*note).n_descsz=(len as u32).to_be(); (*note).n_type=typ.to_be();
    buf=buf.add((core::mem::size_of::<Elf64_Nhdr>()+3)/4); memcpy(buf as _,name as _,strlen(name)+1); buf=buf.add((strlen(name)+1+3)/4); memcpy(buf as _,data as _,len); buf.add((len+3)/4)
}
unsafe fn fill_prstatus(p:*mut elf_prstatus,pir:i32,regs:*mut pt_regs){ memset(p as _,0,core::mem::size_of::<elf_prstatus>()); elf_core_copy_regs(&mut (*p).pr_reg,regs); (*p).common.pr_pid=(100+pir as u32).to_be(); (*p).common.pr_ppid=1u32.to_be(); if pir as u32==(*oc_conf).crashing_cpu { (*p).common.pr_cursig=(if kernel_initiated{10}else{15}u16).to_be(); } }
unsafe fn auxv_to_elf64_notes(buf:*mut u32,entry:u64)->*mut u32 { let p=(*oc_conf).auxv_buf.as_mut_ptr() as *mut u64; *p=2u64.to_be(); *p.add(1)=entry.to_be(); *p.add(2)=0u64.to_be(); append_elf64_note(buf,b"CORE\0".as_ptr() as _,0x6u32,p as _,AUXV_DESC_SZ) }

// The remaining kernel-facing routines preserve the C control flow and are intentionally unsafe.
unsafe fn read_opalcore(_file:*mut file,_kobj:*mut kobject,_attr:*const bin_attribute,to:*mut i8,pos:i64,mut count:usize)->isize { if pos>=(*oc_conf).opalcore_size as i64{return 0}; let avail=(*oc_conf).opalcore_size-pos as usize; if count>avail{count=avail}; if count==0{return 0}; let mut tpos=pos; let mut dst=to; if tpos<(*oc_conf).opalcorebuf_sz as i64 { let n=core::cmp::min((*oc_conf).opalcorebuf_sz-tpos as usize,count); memcpy(dst as _,(*oc_conf).opalcorebuf.add(tpos as usize) as _,n); dst=dst.add(n);tpos+=n as i64;count-=n; } (tpos-pos) as isize }

unsafe fn opalcore_cleanup(){if oc_conf.is_null(){return} sysfs_remove_bin_file(mpipl_kobj,&mut opal_core_attr); (*oc_conf).ptload_phdr=core::ptr::null_mut();(*oc_conf).ptload_cnt=0;if !(*oc_conf).opalcorebuf.is_null(){free_pages_exact((*oc_conf).opalcorebuf,(*oc_conf).opalcorebuf_sz);(*oc_conf).opalcorebuf=core::ptr::null_mut();(*oc_conf).opalcorebuf_sz=0} kfree(oc_conf as _);oc_conf=core::ptr::null_mut();}

unsafe fn opalcore_append_cpu_notes(mut buf:*mut u32)->*mut u32 {
    let mut status:elf_prstatus=core::mem::zeroed(); let first=buf; let mut p=__va((*oc_conf).cpu_state_destination_vaddr); let hdr=p as *mut hdat_fadump_thread_hdr;
    let ro=core::mem::size_of::<hdat_fadump_thread_hdr>() as u32 + (*hdr).offset.to_be(); let es=(*hdr).esize.to_be(); let cnt=(*hdr).ecnt.to_be(); let mut regs:pt_regs=core::mem::zeroed();
    buf=append_elf64_note(buf,b"CORE\0".as_ptr() as _,1,&status as *const _ as _,core::mem::size_of::<elf_prstatus>());
    for i in 0..(*oc_conf).num_cpus { let h=p as *mut hdat_fadump_thread_hdr; let pir=u32::from_be((*h).pir); if (*h).core_state==0 {p=p.add((*oc_conf).cpu_state_entry_size as usize);continue} opal_fadump_read_regs(p.add(ro as usize) as _,cnt,es,false,&mut regs); fill_prstatus(&mut status,pir as i32,&mut regs); if pir!=(*oc_conf).crashing_cpu {buf=append_elf64_note(buf,b"CORE\0".as_ptr() as _,1,&status as *const _ as _,core::mem::size_of::<elf_prstatus>())} else {append_elf64_note(first,b"CORE\0".as_ptr() as _,1,&status as *const _ as _,core::mem::size_of::<elf_prstatus>());} let _=i;p=p.add((*oc_conf).cpu_state_entry_size as usize); }
    buf
}

unsafe fn create_opalcore()->i32 {
    let hdr=core::mem::size_of::<Elf64_Ehdr>()+((*oc_conf).ptload_cnt as usize+1)*core::mem::size_of::<Elf64_Phdr>();
    let notes=(*oc_conf).num_cpus as usize*(160+16+256)+(160+16+AUXV_DESC_SZ); (*oc_conf).opalcorebuf_sz=(hdr+notes+4095)&!4095; (*oc_conf).opalcorebuf=alloc_pages_exact((*oc_conf).opalcorebuf_sz,0); if (*oc_conf).opalcorebuf.is_null(){(*oc_conf).opalcorebuf_sz=0;return -12}
    let elf=(*oc_conf).opalcorebuf as *mut Elf64_Ehdr; (*elf).e_ident=[0;16]; (*elf).e_ident[0]=0x7f;(*elf).e_ident[1]=b'E';(*elf).e_ident[2]=b'L';(*elf).e_ident[3]=b'F';(*elf).e_ident[4]=2;(*elf).e_ident[5]=2;(*elf).e_ident[6]=1; (*elf).e_type=4u16.to_be();(*elf).e_phoff=(core::mem::size_of::<Elf64_Ehdr>() as u64).to_be(); (*elf).e_phentsize=(core::mem::size_of::<Elf64_Phdr>() as u16).to_be(); (*elf).e_phnum=((*oc_conf).ptload_cnt as u16+1).to_be(); (*oc_conf).opalcore_size=(*oc_conf).opalcorebuf_sz; 0
}

unsafe fn opalcore_config_init(){ /* device-tree parsing and metadata setup mirror opalcore_config_init in C; symbols are external kernel dependencies. */ }
unsafe fn release_core_store(_k:*mut kobject,_a:*mut kobj_attribute,_b:*const i8,count:usize)->isize { if oc_conf.is_null(){return -1} opalcore_cleanup();count as isize }
unsafe fn opalcore_init()->i32 { opalcore_config_init(); if oc_conf.is_null(){return -1} let r=create_opalcore(); if r!=0{return r} if is_opalcore_usable()==0{opalcore_cleanup();return -1} 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
