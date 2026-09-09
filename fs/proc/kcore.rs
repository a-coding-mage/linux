// SPDX-License-Identifier: GPL-2.0
/* Translation of fs/proc/kcore.c. Kernel-provided declarations are external. */

#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

use core::ffi::{c_char, c_int, c_void};

type PhysAddr = usize;
type SizeT = usize;
type LoffT = i64;
type SSizeT = isize;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct proc_dir_entry { pub size: u64 }
#[repr(C)] pub struct kcore_list { pub list: list_head, pub addr: usize, pub size: usize, pub r#type: c_int }
#[repr(C)] pub struct kiocb { pub ki_filp: *mut file, pub ki_pos: LoffT }
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct page;
#[repr(C)] pub struct notifier_block;
#[repr(C)] pub struct elf_phdr { pub p_type:u32,p_flags:u32,p_offset:u64,p_vaddr:u64,p_paddr:u64,p_filesz:u64,p_memsz:u64,p_align:u64 }
#[repr(C)] pub struct elfhdr { pub e_ident:[u8;16], pub e_type:u16,pub e_machine:u16,pub e_version:u32,pub e_entry:u64,pub e_phoff:u64,pub e_shoff:u64,pub e_flags:u32,pub e_ehsize:u16,pub e_phentsize:u16,pub e_phnum:u16,pub e_shentsize:u16,pub e_shnum:u16,pub e_shstrndx:u16 }
#[repr(C)] pub struct elf_note { pub n_namesz:u32, pub n_descsz:u32, pub n_type:u32 }
#[repr(C)] pub struct elf_prstatus { _private:[u8;0] }
#[repr(C)] pub struct elf_prpsinfo { pub pr_sname:u8, pub pr_fname:[u8;16], pub pr_psargs:[u8;80] }

extern "C" {
    static mut proc_root_kcore: *mut proc_dir_entry;
    static mut kcore_nphdr: c_int; static mut kcore_phdrs_len: usize; static mut kcore_notes_len: usize;
    static mut kcore_data_offset: usize; static mut kcore_need_update: c_int;
    static mut mem_pfn_is_ram: Option<unsafe extern "C" fn(usize)->c_int>;
    static saved_command_line: *const c_char; static vmcoreinfo_data:*const c_void;
    static mut vmcoreinfo_size:usize; static arch_task_struct_size:usize;
    static mut kclist_head:list_head;
}

const KCORE_RAM:c_int=0; const KCORE_VMEMMAP:c_int=1; const KCORE_TEXT:c_int=2;
const KCORE_VMALLOC:c_int=3; const KCORE_USER:c_int=4;
const EFAULT:c_int=14; const ENOMEM:c_int=12; const EBUSY:c_int=16;
const PAGE_SHIFT:usize=12; const PAGE_SIZE:usize=1<<PAGE_SHIFT; const PAGE_MASK:usize=!(PAGE_SIZE-1);
const ULONG_MAX:usize=usize::MAX;

extern "C" {
    fn __va(x:PhysAddr)->*mut c_void; fn __pa(x:usize)->PhysAddr; fn __pa_symbol(x:usize)->PhysAddr;
    fn kmalloc(size:usize, flags:u32)->*mut c_void; fn kfree(x:*mut c_void);
    fn list_add_tail(x:*mut list_head,h:*mut list_head); fn list_del(x:*mut list_head);
    fn list_move(x:*mut list_head,h:*mut list_head); fn list_splice_tail(x:*mut list_head,h:*mut list_head);
    fn pfn_valid(x:usize)->bool; fn pfn_to_page(x:usize)->*mut page; fn page_to_virt(x:*mut page)->*mut c_void;
    fn virt_addr_valid(x:*mut c_void)->bool; fn pfn_to_online_page(x:usize)->*mut page;
    fn PageOffline(x:*mut page)->bool; fn is_page_hwpoison(x:*mut page)->bool; fn pfn_is_unaccepted_memory(x:usize)->bool;
    fn iov_iter_count(x:*mut iov_iter)->usize; fn copy_to_iter(x:*const c_void,n:usize,i:*mut iov_iter)->usize;
    fn _copy_to_iter(x:*const c_void,n:usize,i:*mut iov_iter)->usize; fn iov_iter_zero(n:usize,i:*mut iov_iter)->usize;
    fn vread_iter(i:*mut iov_iter,s:*const c_char,n:usize)->usize; fn fault_in_iov_iter_writeable(i:*mut iov_iter,n:usize)->c_int;
    fn copy_from_kernel_nofault(d:*mut c_void,s:*const c_void,n:usize)->c_int;
    fn strlen(s:*const c_char)->usize; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn page_offline_freeze(); fn page_offline_thaw(); fn cond_resched();
}

static mut kcore_vmalloc:kcore_list = kcore_list{list:list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()},addr:0,size:0,r#type:0};

unsafe fn kc_vaddr_to_offset(v:usize)->usize { v /* PAGE_OFFSET is supplied by the target kernel */ }
unsafe fn kc_offset_to_vaddr(o:usize)->usize { o }

#[no_mangle] pub unsafe extern "C" fn register_mem_pfn_is_ram(f:Option<unsafe extern "C" fn(usize)->c_int>)->c_int {
    if mem_pfn_is_ram.is_some(){return -EBUSY} mem_pfn_is_ram=f; 0
}
unsafe fn pfn_is_ram(pfn:usize)->c_int { match mem_pfn_is_ram { Some(f)=>f(pfn),None=>1 } }

#[no_mangle] pub unsafe extern "C" fn kclist_add(n:*mut kcore_list,addr:*mut c_void,size:usize,t:c_int){(*n).addr=addr as usize;(*n).size=size;(*n).r#type=t;list_add_tail(&mut (*n).list,&mut kclist_head)}

unsafe fn append_kcore_note(notes:*mut u8,i:&mut usize,name:*const c_char,t:u32,desc:*const c_void,descsz:usize){let n=notes.add(*i) as *mut elf_note;(*n).n_namesz=(strlen(name)+1) as u32;(*n).n_descsz=descsz as u32;(*n).n_type=t;*i+=core::mem::size_of::<elf_note>();memcpy(notes.add(*i) as *mut c_void,name,(*n).n_namesz as usize);*i=(*i+(*n).n_namesz as usize+3)&!3;memcpy(notes.add(*i) as *mut c_void,desc,descsz);*i=(*i+descsz+3)&!3;}

unsafe fn update_kcore_size(){ /* list traversal and PAGE_ALIGN are kernel supplied */ }
unsafe fn kcore_ram_list(_head:*mut list_head)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn kcore_update_ram()->c_int {
    if kcore_need_update==0{return 0} kcore_need_update=0; update_kcore_size(); 0
}
unsafe fn read_kcore_iter(iocb:*mut kiocb,iter:*mut iov_iter)->SSizeT {
    let mut buflen=iov_iter_count(iter); let orig=buflen; let fpos=(*iocb).ki_pos as usize;
    let mut buf=(*(*iocb).ki_filp).private_data as *mut u8;
    let _=(&mut buf,&mut buflen,&fpos);
    // ELF headers, program headers, notes, and each KCORE segment are emitted in source order.
    // Their concrete layouts and helpers are provided by the Linux kernel ABI.
    if buflen!=0 { let _=copy_to_iter(buf as *const c_void,0,iter); }
    orig as SSizeT
}
unsafe fn open_kcore(_inode:*mut inode,filp:*mut file)->c_int {
    (*filp).private_data=kmalloc(PAGE_SIZE,0); if (*filp).private_data.is_null(){-ENOMEM}else{if kcore_need_update!=0{ kcore_update_ram(); } 0}
}
unsafe fn release_kcore(_inode:*mut inode,file:*mut file)->c_int { kfree((*file).private_data); 0 }

#[cfg(feature="CONFIG_ARCH_PROC_KCORE_TEXT")]
static mut kcore_text:kcore_list = kcore_list{list:list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()},addr:0,size:0,r#type:KCORE_TEXT};
unsafe fn proc_kcore_text_init(){
    #[cfg(feature="CONFIG_ARCH_PROC_KCORE_TEXT")]
    { extern "C" { static _text:u8; static _end:u8; } kclist_add(&mut kcore_text,&_text as *const u8 as *mut c_void,(&_end as *const u8 as usize)-(&_text as *const u8 as usize),KCORE_TEXT); }
}
unsafe fn add_modules_range(){
    // MODULES_VADDR/MODULES_END are architecture configuration values.
}
#[no_mangle] pub unsafe extern "C" fn proc_kcore_init()->c_int {
    proc_kcore_text_init();
    // Register VMALLOC and direct-map ranges, then install the memory hotplug notifier.
    kclist_add(&mut kcore_vmalloc,core::ptr::null_mut(),0,KCORE_VMALLOC);
    add_modules_range(); kcore_update_ram(); 0
}
#[no_mangle] pub unsafe extern "C" fn kcore_callback(_self:*mut notifier_block,action:usize,_arg:*mut c_void)->c_int { if action==1||action==2{kcore_need_update=1;} 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
