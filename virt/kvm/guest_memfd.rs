// SPDX-License-Identifier: GPL-2.0
// Translated from kvm/guest_memfd.c.  C include dependencies are expected to
// be supplied by the surrounding kernel Rust bindings.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u64 = u64;
type loff_t = i64;
type pgoff_t = c_ulong;
type gfn_t = u64;
type gpa_t = u64;
type uoff_t = u64;
type kvm_pfn_t = u64;
type vm_fault_t = c_uint;
type c_uint = u32;

const PAGE_SHIFT: c_int = 12;
const PAGE_SIZE: c_ulong = 1 << PAGE_SHIFT;
const FALLOC_FL_KEEP_SIZE: c_int = 0x01;
const FALLOC_FL_PUNCH_HOLE: c_int = 0x02;
const O_RDWR: c_int = 0o2;
const O_LARGEFILE: c_int = 0o00100000;
const S_IFREG: c_uint = 0o100000;
const GFP_KERNEL: c_uint = 0;
const GFP_HIGHUSER: c_uint = 0;
const SLAB_ACCOUNT: c_uint = 0;
const XA_PRESENT: c_ulong = 8;
const FGP_LOCK: c_uint = 0x00000001;
const FGP_CREAT: c_uint = 0x00000002;
const VM_SHARED: c_ulong = 0x00000008;
const VM_MAYSHARE: c_ulong = 0x00000080;
const VM_FAULT_LOCKED: vm_fault_t = 0x00002000;
const VM_FAULT_SIGBUS: vm_fault_t = 0x00000002;
const VM_FAULT_RETRY: vm_fault_t = 0x00000400;
const MF_DELAYED: c_int = 1;
const MNT_NOEXEC: c_int = 0x04;
const GUEST_MEMFD_FLAG_INIT_SHARED: u64 = 1 << 0;
const GUEST_MEMFD_FLAG_MMAP: u64 = 1 << 1;
const GUEST_MEMFD_MAGIC: c_ulong = 0x474d454d;
const KVM_MEMORY_ATTRIBUTE_PRIVATE: u64 = 1 << 3;
const FOLL_WRITE: c_uint = 0x01;

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const EINTR: c_int = 4;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EBADF: c_int = 9;
const EEXIST: c_int = 17;
const EFAULT: c_int = 14;
const EIO: c_int = 5;
const EHWPOISON: c_int = 133;
const ENODEV: c_int = 19;
const EAGAIN: c_int = 11;

#[repr(C)]
pub struct vfsmount {
    pub mnt_sb: *mut super_block,
    pub mnt_flags: c_int,
}
#[repr(C)]
pub struct super_block;
#[repr(C)]
pub struct module;
#[repr(C)]
pub struct mempolicy;
#[repr(C)]
pub struct shared_policy;
#[repr(C)]
pub struct xarray;
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct address_space {
    pub a_ops: *const address_space_operations,
    pub host: *mut inode,
}
#[repr(C)]
pub struct inode {
    pub i_mapping: *mut address_space,
    pub i_op: *const inode_operations,
    pub i_mode: c_uint,
    pub i_size: loff_t,
    pub i_ino: c_ulong,
}
#[repr(C)]
pub struct folio {
    pub index: pgoff_t,
}
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct file {
    pub f_op: *const file_operations,
    pub f_flags: c_int,
    pub private_data: *mut c_void,
    pub f_mapping: *mut address_space,
}
#[repr(C)]
pub struct kvm {
    pub slots_lock: mutex,
}
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct kvm_memory_slot_gmem {
    pub file: *mut file,
    pub pgoff: pgoff_t,
}
#[repr(C)]
pub struct kvm_memory_slot {
    pub base_gfn: gfn_t,
    pub npages: c_ulong,
    pub flags: c_ulong,
    pub gmem: kvm_memory_slot_gmem,
}
#[repr(C)]
pub struct kvm_gfn_range {
    pub start: gfn_t,
    pub end: gfn_t,
    pub slot: *mut kvm_memory_slot,
    pub may_block: bool_,
    pub attr_filter: kvm_gfn_range_filter,
}
#[repr(C)]
pub struct vm_area_struct {
    pub vm_file: *mut file,
    pub vm_flags: c_ulong,
    pub vm_ops: *const vm_operations_struct,
}
#[repr(C)]
pub struct vm_fault {
    pub vma: *mut vm_area_struct,
    pub pgoff: pgoff_t,
    pub page: *mut page,
}
#[repr(C)]
pub struct mnt_idmap;
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct iattr;
#[repr(C)]
pub struct fs_context {
    pub fs_private: *mut c_void,
}
#[repr(C)]
pub struct pseudo_fs_context {
    pub ops: *const super_operations,
}
#[repr(C)]
pub struct kmem_cache;
#[repr(C)]
pub struct kmem_cache_args {
    pub align: c_uint,
    pub ctor: Option<unsafe extern "C" fn(*mut c_void)>,
}
#[repr(C)]
pub struct kvm_create_guest_memfd {
    pub size: loff_t,
    pub flags: u64,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum kvm_gfn_range_filter {
    KVM_FILTER_PRIVATE = 0,
    KVM_FILTER_SHARED = 1,
}
use kvm_gfn_range_filter::{KVM_FILTER_PRIVATE, KVM_FILTER_SHARED};

#[repr(C)]
pub enum migrate_mode {
    MIGRATE_ASYNC = 0,
}

type kvm_gmem_populate_cb = Option<
    unsafe extern "C" fn(*mut kvm, gfn_t, kvm_pfn_t, *mut page, *mut c_void) -> c_int,
>;

#[repr(C)]
pub struct vm_operations_struct {
    pub fault: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t>,
    pub get_policy:
        Option<unsafe extern "C" fn(*mut vm_area_struct, c_ulong, *mut pgoff_t) -> *mut mempolicy>,
    pub set_policy: Option<unsafe extern "C" fn(*mut vm_area_struct, *mut mempolicy) -> c_int>,
}
#[repr(C)]
pub struct file_operations {
    pub owner: *mut module,
    pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub fallocate: Option<unsafe extern "C" fn(*mut file, c_int, loff_t, loff_t) -> c_long>,
}
#[repr(C)]
pub struct address_space_operations {
    pub dirty_folio: Option<unsafe extern "C" fn(*mut address_space, *mut folio) -> bool_>,
    pub migrate_folio:
        Option<unsafe extern "C" fn(*mut address_space, *mut folio, *mut folio, migrate_mode) -> c_int>,
    pub error_remove_folio: Option<unsafe extern "C" fn(*mut address_space, *mut folio) -> c_int>,
    pub free_folio: Option<unsafe extern "C" fn(*mut folio)>,
}
#[repr(C)]
pub struct inode_operations {
    pub setattr: Option<unsafe extern "C" fn(*mut mnt_idmap, *mut dentry, *mut iattr) -> c_int>,
}
#[repr(C)]
pub struct super_operations {
    pub statfs: Option<unsafe extern "C" fn() -> c_int>,
    pub alloc_inode: Option<unsafe extern "C" fn(*mut super_block) -> *mut inode>,
    pub destroy_inode: Option<unsafe extern "C" fn(*mut inode)>,
    pub free_inode: Option<unsafe extern "C" fn(*mut inode)>,
}
#[repr(C)]
pub struct file_system_type {
    pub name: *const c_char,
    pub init_fs_context: Option<unsafe extern "C" fn(*mut fs_context) -> c_int>,
    pub kill_sb: Option<unsafe extern "C" fn(*mut super_block)>,
}

/*
 * A guest_memfd instance can be associated multiple VMs, each with its own
 * "view" of the underlying physical memory.
 *
 * The gmem's inode is effectively the raw underlying physical storage, and is
 * used to track properties of the physical memory, while each gmem file is
 * effectively a single VM's view of that storage, and is used to track assets
 * specific to its associated VM, e.g. memslots=>gmem bindings.
 */
#[repr(C)]
pub struct gmem_file {
    pub kvm: *mut kvm,
    pub bindings: xarray,
    pub entry: list_head,
}

#[repr(C)]
pub struct gmem_inode {
    pub policy: shared_policy,
    pub vfs_inode: inode,
    pub gmem_file_list: list_head,
    pub flags: u64,
}

unsafe extern "C" {
    static mut current: *mut c_void;
    fn container_of_gmem_inode(inode: *mut inode) -> *mut gmem_inode;
    fn folio_pfn(folio: *mut folio) -> kvm_pfn_t;
    fn folio_nr_pages(folio: *mut folio) -> pgoff_t;
    fn folio_order(folio: *mut folio) -> c_int;
    fn folio_next_index(folio: *mut folio) -> pgoff_t;
    fn folio_unlock(folio: *mut folio);
    fn folio_put(folio: *mut folio);
    fn folio_test_large(folio: *mut folio) -> bool_;
    fn folio_test_uptodate(folio: *mut folio) -> bool_;
    fn folio_mark_uptodate(folio: *mut folio);
    fn folio_test_hwpoison(folio: *mut folio) -> bool_;
    fn folio_page(folio: *mut folio, n: c_int) -> *mut page;
    fn folio_file_page(folio: *mut folio, index: pgoff_t) -> *mut page;
    fn filemap_lock_folio(mapping: *mut address_space, index: pgoff_t) -> *mut folio;
    fn __filemap_get_folio_mpol(
        mapping: *mut address_space,
        index: pgoff_t,
        flags: c_uint,
        gfp: c_uint,
        policy: *mut mempolicy,
    ) -> *mut folio;
    fn mapping_gfp_mask(mapping: *mut address_space) -> c_uint;
    fn mpol_shared_policy_lookup(policy: *mut shared_policy, index: pgoff_t) -> *mut mempolicy;
    fn mpol_cond_put(policy: *mut mempolicy);
    fn mpol_set_shared_policy(
        policy: *mut shared_policy,
        vma: *mut vm_area_struct,
        mpol: *mut mempolicy,
    ) -> c_int;
    fn mpol_shared_policy_init(policy: *mut shared_policy, mpol: *mut mempolicy);
    fn mpol_free_shared_policy(policy: *mut shared_policy);
    fn linear_page_index(vma: *mut vm_area_struct, addr: c_ulong) -> pgoff_t;
    fn filemap_invalidate_lock(mapping: *mut address_space);
    fn filemap_invalidate_unlock(mapping: *mut address_space);
    fn filemap_invalidate_lock_shared(mapping: *mut address_space);
    fn filemap_invalidate_unlock_shared(mapping: *mut address_space);
    fn truncate_inode_pages_range(mapping: *mut address_space, start: loff_t, end: loff_t);
    fn signal_pending(task: *mut c_void) -> bool_;
    fn cond_resched();
    fn file_inode(file: *mut file) -> *mut inode;
    fn i_size_read(inode: *mut inode) -> loff_t;
    fn file_modified(file: *mut file);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn xa_init(xa: *mut xarray);
    fn xa_destroy(xa: *mut xarray);
    fn xa_empty(xa: *mut xarray) -> bool_;
    fn xa_find(xa: *mut xarray, index: *mut c_ulong, max: c_ulong, filter: c_ulong) -> *mut c_void;
    fn xa_load(xa: *mut xarray, index: c_ulong) -> *mut c_void;
    fn xa_store_range(
        xa: *mut xarray,
        first: c_ulong,
        last: c_ulong,
        entry: *mut c_void,
        gfp: c_uint,
    ) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn kvm_mmu_invalidate_start(kvm: *mut kvm);
    fn kvm_mmu_invalidate_end(kvm: *mut kvm);
    fn kvm_mmu_unmap_gfn_range(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool_;
    fn kvm_flush_remote_tlbs(kvm: *mut kvm);
    fn kvm_arch_gmem_invalidate_range(kvm: *mut kvm, range: *mut kvm_gfn_range);
    fn kvm_arch_gmem_reclaim(pfn: kvm_pfn_t, nr_pages: pgoff_t);
    fn kvm_arch_gmem_make_private(kvm: *mut kvm, gfn: gfn_t, pfn: kvm_pfn_t, nr_pages: kvm_pfn_t) -> c_int;
    fn get_unused_fd_flags(flags: c_uint) -> c_int;
    fn put_unused_fd(fd: c_int);
    fn fd_install(fd: c_int, file: *mut file);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn fops_get(fops: *mut file_operations) -> *mut file_operations;
    fn fops_put(fops: *mut file_operations);
    fn anon_inode_make_secure_inode(sb: *mut super_block, name: *const c_char, ctx: *mut c_void) -> *mut inode;
    fn alloc_file_pseudo(
        inode: *mut inode,
        mnt: *mut vfsmount,
        name: *const c_char,
        flags: c_int,
        fops: *mut file_operations,
    ) -> *mut file;
    fn iput(inode: *mut inode);
    fn mapping_set_gfp_mask(mapping: *mut address_space, mask: c_uint);
    fn mapping_set_inaccessible(mapping: *mut address_space);
    fn mapping_unevictable(mapping: *mut address_space) -> bool_;
    fn kvm_get_kvm(kvm: *mut kvm);
    fn kvm_put_kvm(kvm: *mut kvm);
    fn fget(fd: c_uint) -> *mut file;
    fn fput(file: *mut file);
    fn get_file_active(file: *mut *mut file) -> *mut file;
    fn clear_highpage(page: *mut page);
    fn vmf_error(err: c_int) -> vm_fault_t;
    fn generic_file_open(inode: *mut inode, file: *mut file) -> c_int;
    fn noop_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool_;
    fn simple_statfs() -> c_int;
    fn kill_anon_super(sb: *mut super_block);
    fn init_pseudo(fc: *mut fs_context, magic: c_ulong) -> bool_;
    fn kern_mount(fs: *mut file_system_type) -> *mut vfsmount;
    fn kern_unmount(mnt: *mut vfsmount);
    fn rcu_barrier();
    fn kmem_cache_create(
        name: *const c_char,
        size: usize,
        args: *mut kmem_cache_args,
        flags: c_uint,
    ) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn alloc_inode_sb(sb: *mut super_block, cache: *mut kmem_cache, flags: c_uint) -> *mut gmem_inode;
    fn inode_init_once(inode: *mut inode);
    fn kvm_gmem_get_supported_flags(kvm: *mut kvm) -> u64;
    fn gfn_to_memslot(kvm: *mut kvm, gfn: gfn_t) -> *mut kvm_memory_slot;
    fn kvm_slot_has_gmem(slot: *mut kvm_memory_slot) -> bool_;
    fn kvm_range_has_memory_attributes(
        kvm: *mut kvm,
        start: gfn_t,
        end: gfn_t,
        attrs: u64,
        mask: u64,
    ) -> bool_;
    fn get_user_pages_fast(uaddr: c_ulong, nr_pages: c_int, flags: c_uint, pages: *mut *mut page) -> c_int;
    fn put_page(page: *mut page);
}

static mut kvm_gmem_mnt: *mut vfsmount = ptr::null_mut();
static mut kvm_gmem_inode_cachep: *mut kmem_cache = ptr::null_mut();

#[inline(always)]
unsafe fn GMEM_I(inode: *mut inode) -> *mut gmem_inode {
    container_of_gmem_inode(inode)
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn ERR_PTR<T>(err: c_int) -> *mut T {
    err as isize as *mut T
}

fn PAGE_ALIGNED<T: Into<i128>>(x: T) -> bool {
    (x.into() & ((PAGE_SIZE as i128) - 1)) == 0
}

unsafe fn WARN_ON_ONCE(condition: bool_) -> bool_ {
    condition
}

unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

unsafe fn WRITE_ONCE<T>(p: *mut T, v: T) {
    core::ptr::write_volatile(p, v);
}

/**
 * folio_file_pfn - like folio_file_page, but return a pfn.
 * @folio: The folio which contains this index.
 * @index: The index we want to look up.
 *
 * Return: The pfn for this index.
 */
#[inline]
unsafe fn folio_file_pfn(folio: *mut folio, index: pgoff_t) -> kvm_pfn_t {
    folio_pfn(folio).wrapping_add(index & (folio_nr_pages(folio) - 1))
}

unsafe fn kvm_gmem_get_index(slot: *mut kvm_memory_slot, gfn: gfn_t) -> pgoff_t {
    gfn.wrapping_sub((*slot).base_gfn).wrapping_add((*slot).gmem.pgoff as u64) as pgoff_t
}

unsafe fn kvm_gmem_is_private_mem(inode: *mut inode, _index: pgoff_t) -> bool_ {
    ((*GMEM_I(inode)).flags & GUEST_MEMFD_FLAG_INIT_SHARED) == 0
}

unsafe fn kvm_gmem_is_shared_mem(inode: *mut inode, index: pgoff_t) -> bool_ {
    !kvm_gmem_is_private_mem(inode, index)
}

/*
 * Returns a locked folio on success.  The caller is responsible for
 * setting the up-to-date flag before the memory is mapped into the guest.
 * There is no backing storage for the memory, so the folio will remain
 * up-to-date until it's removed.
 *
 * Ignore accessed, referenced, and dirty flags.  The memory is
 * unevictable and there is no storage to write back to.
 */
unsafe fn kvm_gmem_get_folio(inode: *mut inode, index: pgoff_t) -> *mut folio {
    /* TODO: Support huge pages. */
    let mut policy: *mut mempolicy;
    let mut folio: *mut folio;

    /*
     * Fast-path: See if folio is already present in mapping to avoid
     * policy_lookup.
     */
    folio = filemap_lock_folio((*inode).i_mapping, index);
    if !IS_ERR(folio) {
        return folio;
    }

    policy = mpol_shared_policy_lookup(&mut (*GMEM_I(inode)).policy, index);
    folio = __filemap_get_folio_mpol(
        (*inode).i_mapping,
        index,
        FGP_LOCK | FGP_CREAT,
        mapping_gfp_mask((*inode).i_mapping),
        policy,
    );
    mpol_cond_put(policy);

    /*
     * External interfaces like kvm_gmem_get_pfn() support dealing
     * with hugepages to a degree, but internally, guest_memfd currently
     * assumes that all folios are order-0 and handling would need
     * to be updated for anything otherwise (e.g. page-clearing
     * operations).
     */
    WARN_ON_ONCE(!IS_ERR(folio) && folio_order(folio) != 0);

    folio
}

unsafe fn kvm_gmem_get_invalidate_filter(inode: *mut inode) -> kvm_gfn_range_filter {
    if ((*GMEM_I(inode)).flags & GUEST_MEMFD_FLAG_INIT_SHARED) != 0 {
        return KVM_FILTER_SHARED;
    }
    KVM_FILTER_PRIVATE
}

unsafe fn __kvm_gmem_invalidate_start(
    f: *mut gmem_file,
    start: pgoff_t,
    end: pgoff_t,
    attr_filter: kvm_gfn_range_filter,
) {
    let mut flush = false;
    let mut found_memslot = false;
    let mut slot: *mut kvm_memory_slot;
    let kvm = (*f).kvm;
    let mut index: c_ulong = start;

    while {
        slot = xa_find(&mut (*f).bindings, &mut index, end.wrapping_sub(1), XA_PRESENT)
            as *mut kvm_memory_slot;
        !slot.is_null()
    } {
        let pgoff = (*slot).gmem.pgoff;
        let mut gfn_range = kvm_gfn_range {
            start: (*slot).base_gfn.wrapping_add(core::cmp::max(pgoff, start) as gfn_t).wrapping_sub(pgoff as gfn_t),
            end: (*slot).base_gfn.wrapping_add(core::cmp::min(pgoff + (*slot).npages, end) as gfn_t).wrapping_sub(pgoff as gfn_t),
            slot,
            may_block: true,
            attr_filter,
        };

        if !found_memslot {
            found_memslot = true;
            KVM_MMU_LOCK(kvm);
            kvm_mmu_invalidate_start(kvm);
        }

        flush |= kvm_mmu_unmap_gfn_range(kvm, &mut gfn_range);

        // #ifdef CONFIG_HAVE_KVM_ARCH_GMEM_INVALIDATE
        kvm_arch_gmem_invalidate_range(kvm, &mut gfn_range);
        // #endif

        index = index.wrapping_add(1);
    }

    if flush {
        kvm_flush_remote_tlbs(kvm);
    }
    if found_memslot {
        KVM_MMU_UNLOCK(kvm);
    }
}

unsafe fn KVM_MMU_LOCK(_kvm: *mut kvm) {}
unsafe fn KVM_MMU_UNLOCK(_kvm: *mut kvm) {}

unsafe fn kvm_gmem_invalidate_start(inode: *mut inode, start: pgoff_t, end: pgoff_t) {
    let attr_filter = kvm_gmem_get_invalidate_filter(inode);
    let mut pos = (*GMEM_I(inode)).gmem_file_list.next;
    let head = &mut (*GMEM_I(inode)).gmem_file_list as *mut list_head;

    while pos != head {
        let f = pos as *mut gmem_file;
        __kvm_gmem_invalidate_start(f, start, end, attr_filter);
        pos = (*pos).next;
    }
}

unsafe fn __kvm_gmem_invalidate_end(f: *mut gmem_file, mut start: pgoff_t, end: pgoff_t) {
    let kvm = (*f).kvm;
    if !xa_find(&mut (*f).bindings, &mut start, end - 1, XA_PRESENT).is_null() {
        KVM_MMU_LOCK(kvm);
        kvm_mmu_invalidate_end(kvm);
        KVM_MMU_UNLOCK(kvm);
    }
}

unsafe fn kvm_gmem_invalidate_end(inode: *mut inode, start: pgoff_t, end: pgoff_t) {
    let mut pos = (*GMEM_I(inode)).gmem_file_list.next;
    let head = &mut (*GMEM_I(inode)).gmem_file_list as *mut list_head;
    while pos != head {
        __kvm_gmem_invalidate_end(pos as *mut gmem_file, start, end);
        pos = (*pos).next;
    }
}

unsafe fn kvm_gmem_punch_hole(inode: *mut inode, offset: loff_t, len: loff_t) -> c_long {
    let start = (offset >> PAGE_SHIFT) as pgoff_t;
    let end = ((offset + len) >> PAGE_SHIFT) as pgoff_t;

    /*
     * Bindings must be stable across invalidation to ensure the start+end
     * are balanced.
     */
    filemap_invalidate_lock((*inode).i_mapping);
    kvm_gmem_invalidate_start(inode, start, end);
    truncate_inode_pages_range((*inode).i_mapping, offset, offset + len - 1);
    kvm_gmem_invalidate_end(inode, start, end);
    filemap_invalidate_unlock((*inode).i_mapping);
    0
}

unsafe fn kvm_gmem_allocate(inode: *mut inode, offset: loff_t, len: loff_t) -> c_long {
    let mapping = (*inode).i_mapping;
    let start: pgoff_t;
    let mut index: pgoff_t;
    let end: pgoff_t;
    let mut r: c_int;

    /* Dedicated guest is immutable by default. */
    if offset + len > i_size_read(inode) {
        return -EINVAL as c_long;
    }

    filemap_invalidate_lock_shared(mapping);
    start = (offset >> PAGE_SHIFT) as pgoff_t;
    end = ((offset + len) >> PAGE_SHIFT) as pgoff_t;

    r = 0;
    index = start;
    while index < end {
        let folio = kvm_gmem_get_folio(inode, index);
        if signal_pending(current) {
            r = -EINTR;
            break;
        }
        if IS_ERR(folio) {
            r = PTR_ERR(folio);
            break;
        }
        index = folio_next_index(folio);
        folio_unlock(folio);
        folio_put(folio);
        /* 64-bit only, wrapping the index should be impossible. */
        if WARN_ON_ONCE(index == 0) {
            break;
        }
        cond_resched();
    }

    filemap_invalidate_unlock_shared(mapping);
    r as c_long
}

unsafe extern "C" fn kvm_gmem_fallocate(
    file: *mut file,
    mode: c_int,
    offset: loff_t,
    len: loff_t,
) -> c_long {
    let ret: c_int;

    if (mode & FALLOC_FL_KEEP_SIZE) == 0 {
        return -EOPNOTSUPP as c_long;
    }
    if (mode & !(FALLOC_FL_KEEP_SIZE | FALLOC_FL_PUNCH_HOLE)) != 0 {
        return -EOPNOTSUPP as c_long;
    }
    if !PAGE_ALIGNED(offset) || !PAGE_ALIGNED(len) {
        return -EINVAL as c_long;
    }

    if (mode & FALLOC_FL_PUNCH_HOLE) != 0 {
        ret = kvm_gmem_punch_hole(file_inode(file), offset, len) as c_int;
    } else {
        ret = kvm_gmem_allocate(file_inode(file), offset, len) as c_int;
    }

    if ret == 0 {
        file_modified(file);
    }
    ret as c_long
}

unsafe extern "C" fn kvm_gmem_release(inode: *mut inode, file: *mut file) -> c_int {
    let f = (*file).private_data as *mut gmem_file;
    let kvm = (*f).kvm;
    let mut index: c_ulong = 0;

    mutex_lock(&mut (*kvm).slots_lock);
    filemap_invalidate_lock((*inode).i_mapping);

    loop {
        let slot = xa_find(&mut (*f).bindings, &mut index, !0, XA_PRESENT) as *mut kvm_memory_slot;
        if slot.is_null() {
            break;
        }
        WRITE_ONCE(&mut (*slot).gmem.file, ptr::null_mut());
        index = index.wrapping_add(1);
    }

    /*
     * All in-flight operations are gone and new bindings can be created.
     * Zap all SPTEs pointed at by this file.  Do not free the backing
     * memory, as its lifetime is associated with the inode, not the file.
     */
    __kvm_gmem_invalidate_start(f, 0, !0, kvm_gmem_get_invalidate_filter(inode));
    __kvm_gmem_invalidate_end(f, 0, !0);

    list_del(&mut (*f).entry);
    filemap_invalidate_unlock((*inode).i_mapping);
    mutex_unlock(&mut (*kvm).slots_lock);
    xa_destroy(&mut (*f).bindings);
    kfree(f as *mut c_void);
    kvm_put_kvm(kvm);
    0
}

#[inline]
unsafe fn kvm_gmem_get_file(slot: *mut kvm_memory_slot) -> *mut file {
    /*
     * Do not return slot->gmem.file if it has already been closed;
     * there might be some time between the last fput() and when
     * kvm_gmem_release() clears slot->gmem.file.
     */
    get_file_active(&mut (*slot).gmem.file)
}

unsafe fn kvm_gmem_supports_mmap(inode: *mut inode) -> bool_ {
    ((*GMEM_I(inode)).flags & GUEST_MEMFD_FLAG_MMAP) != 0
}

unsafe extern "C" fn kvm_gmem_fault_user_mapping(vmf: *mut vm_fault) -> vm_fault_t {
    let inode = file_inode((*(*vmf).vma).vm_file);
    let folio: *mut folio;
    let mut ret: vm_fault_t = VM_FAULT_LOCKED;

    if (((*vmf).pgoff as loff_t) << PAGE_SHIFT) >= i_size_read(inode) {
        return VM_FAULT_SIGBUS;
    }
    if !kvm_gmem_is_shared_mem(inode, (*vmf).pgoff) {
        return VM_FAULT_SIGBUS;
    }

    folio = kvm_gmem_get_folio(inode, (*vmf).pgoff);
    if IS_ERR(folio) {
        if PTR_ERR(folio) == -EAGAIN {
            return VM_FAULT_RETRY;
        }
        return vmf_error(PTR_ERR(folio));
    }

    if WARN_ON_ONCE(folio_test_large(folio)) {
        ret = VM_FAULT_SIGBUS;
    } else {
        if !folio_test_uptodate(folio) {
            clear_highpage(folio_page(folio, 0));
            folio_mark_uptodate(folio);
        }
        (*vmf).page = folio_file_page(folio, (*vmf).pgoff);
    }

    if ret != VM_FAULT_LOCKED {
        folio_unlock(folio);
        folio_put(folio);
    }
    ret
}

// #ifdef CONFIG_NUMA
unsafe extern "C" fn kvm_gmem_set_policy(vma: *mut vm_area_struct, mpol: *mut mempolicy) -> c_int {
    let inode = file_inode((*vma).vm_file);
    mpol_set_shared_policy(&mut (*GMEM_I(inode)).policy, vma, mpol)
}

unsafe extern "C" fn kvm_gmem_get_policy(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ilx: *mut pgoff_t,
) -> *mut mempolicy {
    let pgoff = linear_page_index(vma, addr);
    let inode = file_inode((*vma).vm_file);
    *ilx = (*inode).i_ino;
    /*
     * Return the memory policy for this index, or NULL if none is set.
     *
     * Returning NULL, e.g. instead of the current task's memory policy, is
     * important for the .get_policy kernel ABI: it indicates that no
     * explicit policy has been set via mbind() for this memory. The caller
     * can then replace NULL with the default memory policy instead of the
     * current task's memory policy.
     */
    mpol_shared_policy_lookup(&mut (*GMEM_I(inode)).policy, pgoff)
}
// #endif /* CONFIG_NUMA */

static kvm_gmem_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(kvm_gmem_fault_user_mapping),
    // #ifdef CONFIG_NUMA
    get_policy: Some(kvm_gmem_get_policy),
    set_policy: Some(kvm_gmem_set_policy),
    // #endif
};

unsafe extern "C" fn kvm_gmem_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    if !kvm_gmem_supports_mmap(file_inode(file)) {
        return -ENODEV;
    }
    if ((*vma).vm_flags & (VM_SHARED | VM_MAYSHARE)) != (VM_SHARED | VM_MAYSHARE) {
        return -EINVAL;
    }
    (*vma).vm_ops = &kvm_gmem_vm_ops;
    0
}

static mut kvm_gmem_fops: file_operations = file_operations {
    owner: ptr::null_mut(),
    mmap: Some(kvm_gmem_mmap),
    open: Some(generic_file_open),
    release: Some(kvm_gmem_release),
    fallocate: Some(kvm_gmem_fallocate),
};

unsafe extern "C" fn kvm_gmem_migrate_folio(
    _mapping: *mut address_space,
    _dst: *mut folio,
    _src: *mut folio,
    _mode: migrate_mode,
) -> c_int {
    WARN_ON_ONCE(true);
    -EINVAL
}

unsafe extern "C" fn kvm_gmem_error_folio(mapping: *mut address_space, folio: *mut folio) -> c_int {
    let start: pgoff_t;
    let end: pgoff_t;
    filemap_invalidate_lock_shared(mapping);
    start = (*folio).index;
    end = start + folio_nr_pages(folio);
    kvm_gmem_invalidate_start((*mapping).host, start, end);
    /*
     * Do not truncate the range, what action is taken in response to the
     * error is userspace's decision (assuming the architecture supports
     * gracefully handling memory errors).  If/when the guest attempts to
     * access a poisoned page, kvm_gmem_get_pfn() will return -EHWPOISON,
     * at which point KVM can either terminate the VM or propagate the
     * error to userspace.
     */
    kvm_gmem_invalidate_end((*mapping).host, start, end);
    filemap_invalidate_unlock_shared(mapping);
    MF_DELAYED
}

// #ifdef CONFIG_HAVE_KVM_ARCH_GMEM_RECLAIM
unsafe extern "C" fn kvm_gmem_free_folio(folio: *mut folio) {
    kvm_arch_gmem_reclaim(folio_file_pfn(folio, 0), folio_nr_pages(folio));
}
// #endif

static kvm_gmem_aops: address_space_operations = address_space_operations {
    dirty_folio: Some(noop_dirty_folio),
    migrate_folio: Some(kvm_gmem_migrate_folio),
    error_remove_folio: Some(kvm_gmem_error_folio),
    // #ifdef CONFIG_HAVE_KVM_ARCH_GMEM_RECLAIM
    free_folio: Some(kvm_gmem_free_folio),
    // #endif
};

unsafe extern "C" fn kvm_gmem_setattr(
    _idmap: *mut mnt_idmap,
    _dentry: *mut dentry,
    _attr: *mut iattr,
) -> c_int {
    -EINVAL
}

static kvm_gmem_iops: inode_operations = inode_operations {
    setattr: Some(kvm_gmem_setattr),
};

#[no_mangle]
pub unsafe extern "C" fn kvm_arch_supports_gmem_init_shared(_kvm: *mut kvm) -> bool_ {
    true
}

unsafe fn __kvm_gmem_create(kvm: *mut kvm, size: loff_t, flags: u64) -> c_int {
    static NAME: &[u8] = b"[kvm-gmem]\0";
    let mut f: *mut gmem_file;
    let mut inode: *mut inode;
    let mut file: *mut file;
    let fd: c_int;
    let mut err: c_int;

    fd = get_unused_fd_flags(0);
    if fd < 0 {
        return fd;
    }

    f = kzalloc(core::mem::size_of::<gmem_file>(), GFP_KERNEL) as *mut gmem_file;
    if f.is_null() {
        err = -ENOMEM;
        put_unused_fd(fd);
        return err;
    }

    /* __fput() will take care of fops_put(). */
    if fops_get(&raw mut kvm_gmem_fops).is_null() {
        err = -ENOENT;
        kfree(f as *mut c_void);
        put_unused_fd(fd);
        return err;
    }

    inode = anon_inode_make_secure_inode((*kvm_gmem_mnt).mnt_sb, NAME.as_ptr() as *const c_char, ptr::null_mut());
    if IS_ERR(inode) {
        err = PTR_ERR(inode);
        fops_put(&raw mut kvm_gmem_fops);
        kfree(f as *mut c_void);
        put_unused_fd(fd);
        return err;
    }

    (*inode).i_op = &kvm_gmem_iops;
    (*(*inode).i_mapping).a_ops = &kvm_gmem_aops;
    (*inode).i_mode |= S_IFREG;
    (*inode).i_size = size;
    mapping_set_gfp_mask((*inode).i_mapping, GFP_HIGHUSER);
    mapping_set_inaccessible((*inode).i_mapping);
    /* Unmovable mappings are supposed to be marked unevictable as well. */
    WARN_ON_ONCE(!mapping_unevictable((*inode).i_mapping));
    (*GMEM_I(inode)).flags = flags;

    file = alloc_file_pseudo(inode, kvm_gmem_mnt, NAME.as_ptr() as *const c_char, O_RDWR, &raw mut kvm_gmem_fops);
    if IS_ERR(file) {
        err = PTR_ERR(file);
        iput(inode);
        fops_put(&raw mut kvm_gmem_fops);
        kfree(f as *mut c_void);
        put_unused_fd(fd);
        return err;
    }

    (*file).f_flags |= O_LARGEFILE;
    (*file).private_data = f as *mut c_void;
    kvm_get_kvm(kvm);
    (*f).kvm = kvm;
    xa_init(&mut (*f).bindings);
    list_add(&mut (*f).entry, &mut (*GMEM_I(inode)).gmem_file_list);
    fd_install(fd, file);
    fd
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_create(kvm: *mut kvm, args: *mut kvm_create_guest_memfd) -> c_int {
    let size = (*args).size;
    let flags = (*args).flags;
    if (flags & !kvm_gmem_get_supported_flags(kvm)) != 0 {
        return -EINVAL;
    }
    if size <= 0 || !PAGE_ALIGNED(size) {
        return -EINVAL;
    }
    __kvm_gmem_create(kvm, size, flags)
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_bind(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    fd: c_uint,
    offset: uoff_t,
) -> c_int {
    let size = (*slot).npages << PAGE_SHIFT;
    let start: c_ulong;
    let end: c_ulong;
    let f: *mut gmem_file;
    let inode: *mut inode;
    let file: *mut file;
    let mut r: c_int = -EINVAL;

    file = fget(fd);
    if file.is_null() {
        return -EBADF;
    }
    if (*file).f_op != &raw const kvm_gmem_fops {
        goto_err(file, r)
    } else {
        f = (*file).private_data as *mut gmem_file;
        if (*f).kvm != kvm {
            goto_err(file, r)
        } else {
            inode = file_inode(file);
            if !PAGE_ALIGNED(offset as i128) || offset + size as u64 > i_size_read(inode) as u64 {
                goto_err(file, r)
            } else {
                filemap_invalidate_lock((*inode).i_mapping);
                start = (offset >> PAGE_SHIFT) as c_ulong;
                end = start + (*slot).npages;
                if !xa_empty(&mut (*f).bindings)
                    && !xa_find(&mut (*f).bindings, &mut (start.clone()), end - 1, XA_PRESENT).is_null()
                {
                    r = -EEXIST;
                    filemap_invalidate_unlock((*inode).i_mapping);
                    goto_err(file, r)
                } else {
                    /*
                     * memslots of flag KVM_MEM_GUEST_MEMFD are immutable to change, so
                     * kvm_gmem_bind() must occur on a new memslot.  Because the memslot
                     * is not visible yet, kvm_gmem_get_pfn() is guaranteed to see the file.
                     */
                    WRITE_ONCE(&mut (*slot).gmem.file, file);
                    (*slot).gmem.pgoff = start;
                    if kvm_gmem_supports_mmap(inode) {
                        (*slot).flags |= 0; // KVM_MEMSLOT_GMEM_ONLY, supplied externally.
                    }
                    xa_store_range(&mut (*f).bindings, start, end - 1, slot as *mut c_void, GFP_KERNEL);
                    filemap_invalidate_unlock((*inode).i_mapping);
                    /*
                     * Drop the reference to the file, even on success.  The file pins KVM,
                     * not the other way 'round.  Active bindings are invalidated if the
                     * file is closed before memslots are destroyed.
                     */
                    r = 0;
                    goto_err(file, r)
                }
            }
        }
    }
}

unsafe fn goto_err(file: *mut file, r: c_int) -> c_int {
    fput(file);
    r
}

unsafe fn __kvm_gmem_unbind(slot: *mut kvm_memory_slot, f: *mut gmem_file) {
    let start = (*slot).gmem.pgoff;
    let end = start + (*slot).npages;
    xa_store_range(&mut (*f).bindings, start, end - 1, ptr::null_mut(), GFP_KERNEL);
    /*
     * synchronize_srcu(&kvm->srcu) ensured that kvm_gmem_get_pfn()
     * cannot see this memslot.
     */
    WRITE_ONCE(&mut (*slot).gmem.file, ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_unbind(slot: *mut kvm_memory_slot) {
    /*
     * Nothing to do if the underlying file was _already_ closed, as
     * kvm_gmem_release() invalidates and nullifies all bindings.
     */
    if (*slot).gmem.file.is_null() {
        return;
    }
    let file = kvm_gmem_get_file(slot);
    /*
     * However, if the file is _being_ closed, then the bindings need to be
     * removed as kvm_gmem_release() might not run until after the memslot
     * is freed.  Note, modifying the bindings is safe even though the file
     * is dying as kvm_gmem_release() nullifies slot->gmem.file under
     * slots_lock, and only puts its reference to KVM after destroying all
     * bindings.  I.e. reaching this point means kvm_gmem_release() hasn't
     * yet destroyed the bindings or freed the gmem_file, and can't do so
     * until the caller drops slots_lock.
     */
    if file.is_null() {
        __kvm_gmem_unbind(slot, (*(*slot).gmem.file).private_data as *mut gmem_file);
        return;
    }
    filemap_invalidate_lock((*file).f_mapping);
    __kvm_gmem_unbind(slot, (*file).private_data as *mut gmem_file);
    filemap_invalidate_unlock((*file).f_mapping);
    fput(file);
}

/* Returns a locked folio on success.  */
unsafe fn __kvm_gmem_get_pfn(
    file: *mut file,
    slot: *mut kvm_memory_slot,
    index: pgoff_t,
    pfn: *mut kvm_pfn_t,
    max_order: *mut c_int,
) -> *mut folio {
    let slot_file = READ_ONCE(&(*slot).gmem.file);
    let f = (*file).private_data as *mut gmem_file;
    let folio: *mut folio;

    if file != slot_file {
        WARN_ON_ONCE(!slot_file.is_null());
        return ERR_PTR(-EFAULT);
    }
    if xa_load(&mut (*f).bindings, index) != slot as *mut c_void {
        WARN_ON_ONCE(!xa_load(&mut (*f).bindings, index).is_null());
        return ERR_PTR(-EIO);
    }

    folio = kvm_gmem_get_folio(file_inode(file), index);
    if IS_ERR(folio) {
        return folio;
    }
    if folio_test_hwpoison(folio) {
        folio_unlock(folio);
        folio_put(folio);
        return ERR_PTR(-EHWPOISON);
    }
    *pfn = folio_file_pfn(folio, index);
    if !max_order.is_null() {
        *max_order = 0;
    }
    folio
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_get_pfn(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    gfn: gfn_t,
    pfn: *mut kvm_pfn_t,
    page: *mut *mut page,
    mut max_order: *mut c_int,
) -> c_int {
    let index = kvm_gmem_get_index(slot, gfn);
    let folio: *mut folio;
    let mut r: c_int = 0;
    let mut __order: c_int = 0;

    if max_order.is_null() {
        max_order = &mut __order;
    }
    let file = kvm_gmem_get_file(slot);
    if file.is_null() {
        return -EFAULT;
    }
    folio = __kvm_gmem_get_pfn(file, slot, index, pfn, max_order);
    if IS_ERR(folio) {
        fput(file);
        return PTR_ERR(folio);
    }
    if !folio_test_uptodate(folio) {
        clear_highpage(folio_page(folio, 0));
        folio_mark_uptodate(folio);
    }
    // #ifdef CONFIG_HAVE_KVM_ARCH_GMEM_CONVERT
    if kvm_gmem_is_private_mem(file_inode(file), index) {
        r = kvm_arch_gmem_make_private(kvm, gfn, *pfn, (1 as kvm_pfn_t) << *max_order);
    }
    // #endif
    folio_unlock(folio);
    if r == 0 {
        *page = folio_file_page(folio, index);
    } else {
        folio_put(folio);
    }
    fput(file);
    r
}
// EXPORT_SYMBOL_FOR_KVM_INTERNAL(kvm_gmem_get_pfn);

// #ifdef CONFIG_HAVE_KVM_ARCH_GMEM_POPULATE
unsafe fn __kvm_gmem_populate(
    kvm: *mut kvm,
    slot: *mut kvm_memory_slot,
    file: *mut file,
    gfn: gfn_t,
    src_page: *mut page,
    post_populate: kvm_gmem_populate_cb,
    opaque: *mut c_void,
) -> c_long {
    let index = kvm_gmem_get_index(slot, gfn);
    let folio: *mut folio;
    let mut pfn: kvm_pfn_t = 0;
    let mut ret: c_int;

    filemap_invalidate_lock((*file).f_mapping);
    folio = __kvm_gmem_get_pfn(file, slot, index, &mut pfn, ptr::null_mut());
    if IS_ERR(folio) {
        ret = PTR_ERR(folio);
        filemap_invalidate_unlock((*file).f_mapping);
        return ret as c_long;
    }
    folio_unlock(folio);
    if !kvm_range_has_memory_attributes(
        kvm,
        gfn,
        gfn + 1,
        KVM_MEMORY_ATTRIBUTE_PRIVATE,
        KVM_MEMORY_ATTRIBUTE_PRIVATE,
    ) {
        ret = -EINVAL;
    } else {
        ret = post_populate.unwrap()(kvm, gfn, pfn, src_page, opaque);
        if ret == 0 {
            folio_mark_uptodate(folio);
        }
    }
    folio_put(folio);
    filemap_invalidate_unlock((*file).f_mapping);
    ret as c_long
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_populate(
    kvm: *mut kvm,
    start_gfn: gfn_t,
    src: *mut c_void,
    mut npages: c_long,
    may_writeback_src: bool_,
    post_populate: kvm_gmem_populate_cb,
    opaque: *mut c_void,
) -> c_long {
    let slot: *mut kvm_memory_slot;
    let mut ret: c_int = 0;
    let mut i: c_long = 0;

    if WARN_ON_ONCE(npages <= 0) {
        return -EINVAL as c_long;
    }
    if WARN_ON_ONCE(!PAGE_ALIGNED(src as usize as i128)) {
        return -EINVAL as c_long;
    }
    slot = gfn_to_memslot(kvm, start_gfn);
    if !kvm_slot_has_gmem(slot) {
        return -EINVAL as c_long;
    }
    let file = kvm_gmem_get_file(slot);
    if file.is_null() {
        return -EFAULT as c_long;
    }
    npages = core::cmp::min(((*slot).npages - (start_gfn - (*slot).base_gfn) as c_ulong) as c_long, npages);
    while i < npages {
        let mut src_page: *mut page = ptr::null_mut();
        if signal_pending(current) {
            ret = -EINTR;
            break;
        }
        if !src.is_null() {
            let uaddr = src as c_ulong + i as c_ulong * PAGE_SIZE;
            let flags = if may_writeback_src { FOLL_WRITE } else { 0 };
            ret = get_user_pages_fast(uaddr, 1, flags, &mut src_page);
            if ret < 0 {
                break;
            }
            if ret != 1 {
                ret = -ENOMEM;
                break;
            }
        }
        ret = __kvm_gmem_populate(kvm, slot, file, start_gfn + i as gfn_t, src_page, post_populate, opaque) as c_int;
        if !src_page.is_null() {
            put_page(src_page);
        }
        if ret != 0 {
            break;
        }
        i += 1;
    }
    fput(file);
    if ret != 0 && i == 0 { ret as c_long } else { i }
}
// EXPORT_SYMBOL_FOR_KVM_INTERNAL(kvm_gmem_populate);
// #endif

unsafe extern "C" fn kvm_gmem_init_inode_once(__gi: *mut c_void) {
    let gi = __gi as *mut gmem_inode;
    /*
     * Note!  Don't initialize the inode with anything specific to the
     * guest_memfd instance, or that might be specific to how the inode is
     * used (from the VFS-layer's perspective).  This hook is called only
     * during the initial slab allocation, i.e. only fields/state that are
     * idempotent across _all_ use of the inode _object_ can be initialized
     * at this time!
     */
    inode_init_once(&mut (*gi).vfs_inode);
}

unsafe extern "C" fn kvm_gmem_alloc_inode(sb: *mut super_block) -> *mut inode {
    let gi = alloc_inode_sb(sb, kvm_gmem_inode_cachep, GFP_KERNEL);
    if gi.is_null() {
        return ptr::null_mut();
    }
    mpol_shared_policy_init(&mut (*gi).policy, ptr::null_mut());
    (*gi).flags = 0;
    INIT_LIST_HEAD(&mut (*gi).gmem_file_list);
    &mut (*gi).vfs_inode
}

unsafe extern "C" fn kvm_gmem_destroy_inode(inode: *mut inode) {
    mpol_free_shared_policy(&mut (*GMEM_I(inode)).policy);
}

unsafe extern "C" fn kvm_gmem_free_inode(inode: *mut inode) {
    kmem_cache_free(kvm_gmem_inode_cachep, GMEM_I(inode) as *mut c_void);
}

static kvm_gmem_super_operations: super_operations = super_operations {
    statfs: Some(simple_statfs),
    alloc_inode: Some(kvm_gmem_alloc_inode),
    destroy_inode: Some(kvm_gmem_destroy_inode),
    free_inode: Some(kvm_gmem_free_inode),
};

unsafe extern "C" fn kvm_gmem_init_fs_context(fc: *mut fs_context) -> c_int {
    let ctx: *mut pseudo_fs_context;
    if !init_pseudo(fc, GUEST_MEMFD_MAGIC) {
        return -ENOMEM;
    }
    ctx = (*fc).fs_private as *mut pseudo_fs_context;
    (*ctx).ops = &kvm_gmem_super_operations;
    0
}

static mut kvm_gmem_fs: file_system_type = file_system_type {
    name: b"guest_memfd\0".as_ptr() as *const c_char,
    init_fs_context: Some(kvm_gmem_init_fs_context),
    kill_sb: Some(kill_anon_super),
};

unsafe fn kvm_gmem_init_mount() -> c_int {
    kvm_gmem_mnt = kern_mount(&raw mut kvm_gmem_fs);
    if IS_ERR(kvm_gmem_mnt) {
        return PTR_ERR(kvm_gmem_mnt);
    }
    (*kvm_gmem_mnt).mnt_flags |= MNT_NOEXEC;
    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_init(module: *mut module) -> c_int {
    let mut args = kmem_cache_args {
        align: 0,
        ctor: Some(kvm_gmem_init_inode_once),
    };
    let ret: c_int;

    kvm_gmem_fops.owner = module;
    kvm_gmem_inode_cachep = kmem_cache_create(
        b"kvm_gmem_inode_cache\0".as_ptr() as *const c_char,
        core::mem::size_of::<gmem_inode>(),
        &mut args,
        SLAB_ACCOUNT,
    );
    if kvm_gmem_inode_cachep.is_null() {
        return -ENOMEM;
    }
    ret = kvm_gmem_init_mount();
    if ret != 0 {
        kmem_cache_destroy(kvm_gmem_inode_cachep);
        return ret;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kvm_gmem_exit() {
    kern_unmount(kvm_gmem_mnt);
    kvm_gmem_mnt = ptr::null_mut();
    rcu_barrier();
    kmem_cache_destroy(kvm_gmem_inode_cachep);
}
