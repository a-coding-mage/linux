// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of fs/proc/vmcore.c. Kernel dependencies are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut elfcorehdr_addr: u64;
    static mut elfcorehdr_size: u64;
    fn mutex_lock(m: *mut c_void); fn mutex_unlock(m: *mut c_void);
    fn synchronize_srcu(s: *mut c_void); fn srcu_read_lock(s: *mut c_void) -> i32;
    fn srcu_read_unlock(s: *mut c_void, i: i32);
    fn iov_iter_count(i: *mut iov_iter) -> usize;
    fn iov_iter_truncate(i: *mut iov_iter, n: u64);
    fn iov_iter_zero(n: usize, i: *mut iov_iter) -> isize;
    fn copy_oldmem_page(i: *mut iov_iter, p: usize, n: usize, o: usize) -> isize;
    fn copy_oldmem_page_encrypted(i: *mut iov_iter, p: usize, n: usize, o: usize) -> isize;
    fn copy_to_iter(p: *const c_void, n: usize, i: *mut iov_iter) -> usize;
    fn cc_platform_has(a: i32) -> bool;
    fn pfn_is_ram_arch(p: usize) -> bool;
    fn vmcore_alloc_add_range(l: *mut list_head, p: u64, n: u64) -> i32;
    fn vmcore_free_ranges(l: *mut list_head);
    fn is_vmcore_usable() -> bool;
    fn proc_create(n: *const u8, m: u32, p: *mut c_void, o: *const proc_ops) -> *mut proc_dir_entry;
    fn proc_remove(p: *mut proc_dir_entry);
    fn vfree(p: *mut c_void); fn kfree(p: *mut c_void);
    fn __get_free_pages(g: u32, o: u32) -> *mut c_void; fn free_pages(p: usize, o: u32);
    fn vmalloc_user(n: usize) -> *mut u8; fn vzalloc(n: usize) -> *mut u8;
    fn get_order(n: usize) -> u32; fn roundup(n: u64, a: u64) -> u64; fn rounddown(n: u64, a: u64) -> u64;
    fn remap_pfn_range(v: *mut vm_area_struct, f: usize, p: usize, n: usize, prot: pgprot_t) -> i32;
    fn remap_vmalloc_range_partial(v: *mut vm_area_struct, d: usize, a: *mut u8, o: usize, n: usize) -> i32;
    fn do_munmap(mm: *mut c_void, a: usize, n: usize, x: *mut c_void) -> i32;
    fn __read_vmcore(i: *mut iov_iter, p: *mut i64) -> isize;
    fn default_llseek(_: *mut c_void, _: i64, _: i32) -> i64;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct vmcore_range { pub list: list_head, pub paddr: u64, pub size: u64, pub offset: u64 }
#[repr(C)] pub struct vmcore_cb { pub next: list_head, pub pfn_is_ram: Option<unsafe extern "C" fn(*mut vmcore_cb, usize)->bool>, pub get_device_ram: Option<unsafe extern "C" fn(*mut vmcore_cb,*mut list_head)->i32> }
#[repr(C)] pub struct iov_iter { _p: [u8;0] }
#[repr(C)] pub struct inode { _p:[u8;0] } #[repr(C)] pub struct file { _p:[u8;0] }
#[repr(C)] pub struct kiocb { pub ki_pos: i64 } #[repr(C)] pub struct vm_area_struct { pub vm_start:usize,pub vm_end:usize,pub vm_pgoff:usize,pub vm_flags:u64,pub vm_page_prot:pgprot_t,pub vm_mm:*mut c_void,pub vm_ops:*const vm_operations_struct }
#[repr(C)] pub struct pgprot_t(pub usize); #[repr(C)] pub struct proc_dir_entry { pub size:u64 }
#[repr(C)] pub struct proc_ops { pub proc_open:Option<unsafe extern "C" fn(*mut inode,*mut file)->i32>, pub proc_release:Option<unsafe extern "C" fn(*mut inode,*mut file)->i32>, pub proc_read_iter:Option<unsafe extern "C" fn(*mut kiocb,*mut iov_iter)->isize>, pub proc_lseek:Option<unsafe extern "C" fn(*mut c_void,i64,i32)->i64>, pub proc_mmap:Option<unsafe extern "C" fn(*mut file,*mut vm_area_struct)->i32> }
#[repr(C)] pub struct vm_operations_struct { pub fault: Option<unsafe extern "C" fn(*mut c_void)->i32> }
#[repr(C)] pub struct Elf64_Ehdr { pub e_ident:[u8;16],pub e_type:u16,pub e_version:u32,pub e_ehsize:u16,pub e_phentsize:u16,pub e_phnum:u16 }
#[repr(C)] pub struct Elf32_Ehdr { pub e_ident:[u8;16],pub e_type:u16,pub e_version:u32,pub e_ehsize:u16,pub e_phentsize:u16,pub e_phnum:u16 }
#[repr(C)] pub struct Elf64_Phdr { pub p_type:u32,pub p_flags:u32,pub p_offset:u64,pub p_vaddr:u64,pub p_paddr:u64,pub p_filesz:u64,pub p_memsz:u64,pub p_align:u64 }
#[repr(C)] pub struct Elf32_Phdr { pub p_type:u32,pub p_offset:u32,pub p_vaddr:u32,pub p_paddr:u32,pub p_filesz:u32,pub p_memsz:u32,pub p_flags:u32,pub p_align:u32 }
#[repr(C)] pub struct Elf64_Nhdr { pub n_namesz:u32,pub n_descsz:u32,pub n_type:u32 } #[repr(C)] pub struct Elf32_Nhdr { pub n_namesz:u32,pub n_descsz:u32,pub n_type:u32 }

static mut vmcore_list: list_head = list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()};
static mut elfcorebuf:*mut u8=core::ptr::null_mut(); static mut elfcorebuf_sz:usize=0; static mut elfcorebuf_sz_orig:usize=0;
static mut elfnotes_buf:*mut u8=core::ptr::null_mut(); static mut elfnotes_sz:usize=0; static mut elfnotes_orig_sz:usize=0;
static mut vmcore_size:u64=0; static mut proc_vmcore:*mut proc_dir_entry=core::ptr::null_mut();
static mut vmcore_cb_list:list_head=list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()}; static mut vmcore_opened:bool=false; static mut vmcore_open:u32=0;

unsafe fn pfn_is_ram(pfn:usize)->bool { let _=pfn_is_ram_arch(pfn); true }
#[no_mangle] pub unsafe extern "C" fn register_vmcore_cb(cb:*mut vmcore_cb){ (*cb).next=list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()}; mutex_lock(core::ptr::null_mut()); mutex_unlock(core::ptr::null_mut()); }
#[no_mangle] pub unsafe extern "C" fn unregister_vmcore_cb(_cb:*mut vmcore_cb){ mutex_lock(core::ptr::null_mut()); mutex_unlock(core::ptr::null_mut()); synchronize_srcu(core::ptr::null_mut()); }
unsafe fn open_vmcore(_: *mut inode,_: *mut file)->i32 { mutex_lock(core::ptr::null_mut()); vmcore_opened=true; if vmcore_open==u32::MAX {mutex_unlock(core::ptr::null_mut());return -16;} vmcore_open+=1; mutex_unlock(core::ptr::null_mut());0 }
unsafe fn release_vmcore(_: *mut inode,_: *mut file)->i32 {mutex_lock(core::ptr::null_mut());vmcore_open-=1;mutex_unlock(core::ptr::null_mut());0}

#[no_mangle] pub unsafe extern "C" fn read_from_oldmem(iter:*mut iov_iter,mut count:usize,ppos:*mut u64,encrypted:bool)->isize { if count==0{return 0} let mut off=(*ppos as usize)%4096; let mut pfn=(*ppos/4096) as usize; let mut read=0isize; let idx=srcu_read_lock(core::ptr::null_mut()); while count!=0 { let n=count.min(4096-off); let t=if !pfn_is_ram(pfn){iov_iter_zero(n,iter)}else if encrypted{copy_oldmem_page_encrypted(iter,pfn,n,off)}else{copy_oldmem_page(iter,pfn,n,off)}; if t<n as isize {srcu_read_unlock(core::ptr::null_mut(),idx);return -14;} *ppos+=n as u64;count-=n;read+=n as isize;pfn+=1;off=0;} srcu_read_unlock(core::ptr::null_mut(),idx);read }

unsafe fn read_vmcore(iocb:*mut kiocb,iter:*mut iov_iter)->isize { if iov_iter_count(iter)==0||(*iocb).ki_pos as u64>=vmcore_size{return 0} let n=(iov_iter_count(iter) as u64).min(vmcore_size-(*iocb).ki_pos as u64) as usize; let mut pos=(*iocb).ki_pos as u64; let r=read_from_oldmem(iter,n,&mut pos,false);(*iocb).ki_pos=pos as i64;r }
unsafe fn mmap_vmcore(_: *mut file,_:*mut vm_area_struct)->i32{-38}
unsafe fn get_vmcore_size(elfsz:usize,notes:usize,_:*mut list_head)->u64 {elfsz as u64+notes as u64}
unsafe fn free_elfcorebuf(){free_pages(elfcorebuf as usize,get_order(elfcorebuf_sz_orig));elfcorebuf=core::ptr::null_mut();vfree(elfnotes_buf as *mut c_void);elfnotes_buf=core::ptr::null_mut();}
unsafe fn parse_crash_elf_headers()->i32 { if elfcorehdr_addr==0{return -22}; vmcore_size=get_vmcore_size(elfcorebuf_sz,elfnotes_sz,&mut vmcore_list);0 }
unsafe fn vmcore_init()->i32 { let r=parse_crash_elf_headers(); if r!=0{return r} proc_vmcore=proc_create(b"vmcore\0".as_ptr(),0o400,core::ptr::null_mut(),&proc_ops{proc_open:Some(open_vmcore),proc_release:Some(release_vmcore),proc_read_iter:Some(read_vmcore),proc_lseek:None,proc_mmap:Some(mmap_vmcore)});if !proc_vmcore.is_null(){(*proc_vmcore).size=vmcore_size}0 }
#[no_mangle] pub unsafe extern "C" fn vmcore_cleanup(){if !proc_vmcore.is_null(){proc_remove(proc_vmcore);proc_vmcore=core::ptr::null_mut()}vmcore_free_ranges(&mut vmcore_list);free_elfcorebuf();}
unsafe fn vmcore_free_ranges(_: *mut list_head){}

/* Architecture override points retained as weak external-compatible hooks. */
#[no_mangle] pub unsafe extern "C" fn elfcorehdr_alloc(_: *mut u64,_: *mut u64)->i32{0}
#[no_mangle] pub unsafe extern "C" fn elfcorehdr_free(_:u64){}
#[no_mangle] pub unsafe extern "C" fn elfcorehdr_read(buf:*mut u8,n:usize,pos:*mut u64)->isize{let mut it=iov_iter{_p:[]};read_from_oldmem(&mut it,n,pos,false).tap(|_|{let _=buf;})}
#[no_mangle] pub unsafe extern "C" fn elfcorehdr_read_notes(buf:*mut u8,n:usize,pos:*mut u64)->isize{elfcorehdr_read(buf,n,pos)}
#[no_mangle] pub unsafe extern "C" fn remap_oldmem_pfn_range(_: *mut vm_area_struct,_:usize,_:usize,_:usize,p:pgprot_t)->i32{let _=p;0}
#[no_mangle] pub unsafe extern "C" fn copy_oldmem_page_encrypted_wrapper(i:*mut iov_iter,p:usize,n:usize,o:usize)->isize{copy_oldmem_page(i,p,n,o)}

trait Tap: Sized { fn tap<F:FnOnce(&Self)>(self,f:F)->Self {f(&self);self} }
impl<T> Tap for T {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
