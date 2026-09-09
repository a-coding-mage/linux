// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of memfd_luo.c. Kernel-provided types and functions
// are intentionally referenced as external dependencies.

use core::ffi::c_void;

#[repr(C)]
struct file { f_pos: u64 }
#[repr(C)] struct inode { i_mapping: *mut address_space, i_nlink: u32 }
#[repr(C)] struct address_space;
#[repr(C)] struct folio { index: u64 }
#[repr(C)] struct kho_vmalloc;
#[repr(C)] struct liveupdate_file_handler;
#[repr(C)] struct liveupdate_file_ops;
#[repr(C)] struct memfd_luo_folio_ser { pfn: u64, flags: u32, index: u64 }
#[repr(C)] struct memfd_luo_ser { pos: u64, size: u64, seals: i32, folios: kho_vmalloc, nr_folios: u64 }
#[repr(C)] struct liveupdate_file_op_args {
    file: *mut file, serialized_data: u64, private_data: *mut c_void, retrieve_status: i32,
}

const MEMFD_LUO_FOLIO_DIRTY: u32 = 1;
const MEMFD_LUO_FOLIO_UPTODATE: u32 = 2;
const MFD_ALLOW_SEALING: u32 = 0x0002;
const PAGE_SIZE: u64 = 4096;
const UINT_MAX: u64 = 0xffff_ffff;
const EINVAL: i32 = 22; const ENOMEM: i32 = 12; const EOPNOTSUPP: i32 = 95;
const EFBIG: i32 = 27; const EIO: i32 = 5; const MAX_LFS_FILESIZE: u64 = u64::MAX;

extern "C" {
    static MEMFD_LUO_ALL_SEALS: i32;
    static MEMFD_LUO_FH_COMPATIBLE: u32;
    fn file_inode(f: *mut file) -> *mut inode;
    fn i_size_read(i: *mut inode) -> i64; fn i_size_write(i: *mut inode, n: u64);
    fn kvmalloc_objs<T>(p: *mut *mut T, n: u32) -> *mut *mut T;
    fn kvfree(p: *mut c_void); fn vcalloc(n: u64, size: usize) -> *mut c_void; fn vfree(p: *mut c_void);
    fn memfd_pin_folios(f: *mut file, start: u64, end: i64, out: *mut *mut folio, max: u32, off: *mut u64) -> i64;
    fn unpin_folios(p: *mut *mut folio, n: u64); fn unpin_folio(p: *mut folio);
    fn kho_preserve_folio(p: *mut folio) -> i32; fn kho_unpreserve_folio(p: *mut folio);
    fn kho_preserve_vmalloc(p: *mut memfd_luo_folio_ser, v: *mut kho_vmalloc) -> i32;
    fn kho_unpreserve_vmalloc(v: *mut kho_vmalloc); fn kho_alloc_preserve(n: usize) -> *mut memfd_luo_ser;
    fn kho_unpreserve_free(p: *mut memfd_luo_ser); fn kho_restore_free(p: *mut memfd_luo_ser);
    fn kho_restore_vmalloc(v: *mut kho_vmalloc) -> *mut memfd_luo_folio_ser;
    fn kho_restore_folio(p: u64) -> *mut folio;
    fn virt_to_phys(p: *mut memfd_luo_ser) -> u64; fn phys_to_virt(p: u64) -> *mut memfd_luo_ser;
    fn memfd_get_seals(f: *mut file) -> i32; fn memfd_alloc_file(n: *const u8, flags: u32) -> *mut file;
    fn memfd_add_seals(f: *mut file, seals: i32) -> i32; fn fput(f: *mut file);
    fn vfs_setpos(f: *mut file, pos: u64, max: u64) -> u64;
    fn inode_lock(i: *mut inode); fn inode_unlock(i: *mut inode); fn shmem_freeze(i: *mut inode, freeze: bool);
    fn shmem_file(f: *mut file) -> bool; fn mapping_gfp_mask(m: *mut address_space) -> u32;
    fn __folio_set_locked(p: *mut folio); fn __folio_set_swapbacked(p: *mut folio);
    fn mem_cgroup_charge(p: *mut folio, memcg: *mut c_void, mask: u32) -> i32;
    fn shmem_add_to_page_cache(p: *mut folio, m: *mut address_space, idx: u64, x: *mut c_void, mask: u32) -> i32;
    fn filemap_remove_folio(p: *mut folio); fn folio_add_lru(p: *mut folio); fn folio_put(p: *mut folio);
    fn folio_unlock(p: *mut folio); fn folio_lock(p: *mut folio); fn folio_mark_dirty(p: *mut folio);
    fn folio_mark_uptodate(p: *mut folio); fn folio_test_uptodate(p: *mut folio) -> bool;
    fn folio_zero_range(p: *mut folio, a: u64, n: u64); fn flush_dcache_folio(p: *mut folio);
    fn folio_size(p: *mut folio) -> u64; fn folio_pfn(p: *mut folio) -> u64; fn folio_nr_pages(p: *mut folio) -> i64;
    fn shmem_inode_acct_blocks(i: *mut inode, n: i64) -> i32; fn shmem_recalc_inode(i: *mut inode, a: i64, b: i64);
    fn liveupdate_register_file_handler(h: *mut liveupdate_file_handler) -> i32;
}

unsafe fn memfd_luo_preserve_folios(file: *mut file, vm: *mut kho_vmalloc, out: *mut *mut memfd_luo_folio_ser, nr: *mut u64) -> i32 {
    let inode = file_inode(file); let size = i_size_read(inode);
    if size == 0 { *nr = 0; *out = core::ptr::null_mut(); return 0; }
    let max = (((size as u64) + PAGE_SIZE - 1) / PAGE_SIZE) as u32;
    let mut offset = 0u64; let folios = kvmalloc_objs(core::ptr::null_mut(), max);
    if folios.is_null() { return -ENOMEM; }
    let pinned = memfd_pin_folios(file, 0, size - 1, folios, max, &mut offset);
    if pinned < 0 { kvfree(folios as *mut c_void); return pinned as i32; }
    let n = pinned as u64; let ser = vcalloc(n, core::mem::size_of::<memfd_luo_folio_ser>()) as *mut memfd_luo_folio_ser;
    if ser.is_null() { unpin_folios(folios, n); kvfree(folios as *mut c_void); return -ENOMEM; }
    for i in 0..n { let folio = *folios.add(i as usize); if kho_preserve_folio(folio) != 0 { vfree(ser as *mut c_void); unpin_folios(folios,n); kvfree(folios as *mut c_void); return -EINVAL; } folio_lock(folio); folio_mark_dirty(folio); if !folio_test_uptodate(folio) { folio_zero_range(folio,0,folio_size(folio)); flush_dcache_folio(folio); folio_mark_uptodate(folio); } folio_unlock(folio); (*ser.add(i as usize)).pfn=folio_pfn(folio); (*ser.add(i as usize)).flags=MEMFD_LUO_FOLIO_DIRTY|MEMFD_LUO_FOLIO_UPTODATE; (*ser.add(i as usize)).index=(*folio).index; }
    if kho_preserve_vmalloc(ser,vm) != 0 { for i in 0..n { kho_unpreserve_folio(*folios.add(i as usize)); } vfree(ser as *mut c_void); unpin_folios(folios,n); kvfree(folios as *mut c_void); return -EINVAL; }
    kvfree(folios as *mut c_void); *nr=n; *out=ser; 0
}

unsafe fn memfd_luo_unpreserve_folios(vm: *mut kho_vmalloc, ser: *mut memfd_luo_folio_ser, n: u64) { if n==0{return;} kho_unpreserve_vmalloc(vm); for i in 0..n { let s=&*ser.add(i as usize); if s.pfn!=0 { let f=kho_restore_folio(s.pfn*PAGE_SIZE); if !f.is_null(){kho_unpreserve_folio(f);unpin_folio(f);} } } vfree(ser as *mut c_void); }

unsafe fn memfd_luo_preserve(a:*mut liveupdate_file_op_args)->i32 { let i=file_inode((*a).file); inode_lock(i); shmem_freeze(i,true); let s=kho_alloc_preserve(core::mem::size_of::<memfd_luo_ser>()); if s.is_null(){shmem_freeze(i,false);inode_unlock(i);return -ENOMEM;} let seals=memfd_get_seals((*a).file); if seals<0 {kho_unpreserve_free(s);shmem_freeze(i,false);inode_unlock(i);return seals;} if (seals & !MEMFD_LUO_ALL_SEALS)!=0 {kho_unpreserve_free(s);shmem_freeze(i,false);inode_unlock(i);return -EOPNOTSUPP;} (*s).pos=(*(*a).file).f_pos; (*s).size=i_size_read(i) as u64; (*s).seals=seals; let mut fs=core::ptr::null_mut(); let mut n=0; let e=memfd_luo_preserve_folios((*a).file,&mut (*s).folios,&mut fs,&mut n); if e!=0 {kho_unpreserve_free(s);shmem_freeze(i,false);inode_unlock(i);return e;} (*s).nr_folios=n; inode_unlock(i);(*a).private_data=fs as *mut c_void;(*a).serialized_data=virt_to_phys(s);0 }
unsafe fn memfd_luo_freeze(a:*mut liveupdate_file_op_args)->i32 { let s=phys_to_virt((*a).serialized_data); if s.is_null(){return -EINVAL;} (*s).pos=(*(*a).file).f_pos;0 }
unsafe fn memfd_luo_unpreserve(a:*mut liveupdate_file_op_args) { let s=phys_to_virt((*a).serialized_data); if s.is_null(){return;} let i=file_inode((*a).file);inode_lock(i);shmem_freeze(i,false);memfd_luo_unpreserve_folios(&mut (*s).folios,(*a).private_data as *mut _,(*s).nr_folios);kho_unpreserve_free(s);inode_unlock(i); }
unsafe fn memfd_luo_retrieve(a:*mut liveupdate_file_op_args)->i32 { let s=phys_to_virt((*a).serialized_data);if s.is_null(){return -EINVAL;}if ((*s).seals & !MEMFD_LUO_ALL_SEALS)!=0{return -EOPNOTSUPP;}let f=memfd_alloc_file(b"\0".as_ptr(),MFD_ALLOW_SEALING);if f.is_null(){return -ENOMEM;}let mut e=memfd_add_seals(f,(*s).seals);if e!=0{fput(f);return e;}vfs_setpos(f,(*s).pos,MAX_LFS_FILESIZE);i_size_write(file_inode(f),(*s).size);if (*s).nr_folios!=0{let fs=kho_restore_vmalloc(&mut (*s).folios);if fs.is_null(){fput(f);return -EINVAL;}for j in 0..(*s).nr_folios{let q=&*fs.add(j as usize);if q.pfn!=0{let x=kho_restore_folio(q.pfn*PAGE_SIZE);if !x.is_null(){let m=(*file_inode(f)).i_mapping;__folio_set_locked(x);__folio_set_swapbacked(x);e=mem_cgroup_charge(x,core::ptr::null_mut(),mapping_gfp_mask(m));if e==0{e=shmem_add_to_page_cache(x,m,q.index,core::ptr::null_mut(),mapping_gfp_mask(m));}if e==0{if q.flags&MEMFD_LUO_FOLIO_UPTODATE!=0{folio_mark_uptodate(x);}if q.flags&MEMFD_LUO_FOLIO_DIRTY!=0{folio_mark_dirty(x);}folio_add_lru(x);}folio_unlock(x);folio_put(x);if e!=0{break;}}}}vfree(fs as *mut c_void);if e!=0{fput(f);return e;}}(*a).file=f;kho_restore_free(s);0 }
unsafe fn memfd_luo_finish(a:*mut liveupdate_file_op_args){if (*a).retrieve_status!=0{return;}let s=phys_to_virt((*a).serialized_data);if !s.is_null(){kho_restore_free(s);}}
unsafe fn memfd_luo_can_preserve(_: *mut liveupdate_file_handler,f:*mut file)->bool{let i=file_inode(f);shmem_file(f)&&(*i).i_nlink==0}
unsafe fn memfd_luo_get_id(f:*mut file)->usize{file_inode(f) as usize}
#[no_mangle] pub unsafe extern "C" fn memfd_luo_init()->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
