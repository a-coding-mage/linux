// SPDX-License-Identifier: GPL-2.0
/* memfd_create system call and file sealing support. */
/* Kernel headers and symbols referenced below are supplied by other units. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type PgoffT = c_ulong;
type GfpT = c_uint;
type VmaFlagsT = c_ulong;

#[repr(C)] pub struct Folio { _private: [u8; 0] }
#[repr(C)] pub struct XaState { _private: [u8; 0] }
#[repr(C)] pub struct File { pub f_mode: c_uint, pub f_flags: c_uint, pub f_mapping: *mut AddressSpace }
#[repr(C)] pub struct AddressSpace { pub i_pages: c_void }
#[repr(C)] pub struct Inode { pub i_mode: c_uint }
#[repr(C)] pub struct Hstate { _private: [u8; 0] }
#[repr(C)] pub struct PidNamespace { _private: [u8; 0] }

// Values and macros from the kernel headers remain external dependencies.
extern "C" {
    fn folio_ref_count(folio: *mut Folio) -> c_int;
    fn folio_expected_ref_count(folio: *mut Folio) -> c_int;
    fn lru_add_drain();
    fn xas_lock_irq(xas: *mut XaState);
    fn xas_unlock_irq(xas: *mut XaState);
    fn xas_pause(xas: *mut XaState);
    fn cond_resched();
    fn xas_set_mark(xas: *mut XaState, mark: c_uint);
    fn xas_clear_mark(xas: *mut XaState, mark: c_uint);
    fn xas_marked(xas: *mut XaState, mark: c_uint) -> bool;
    fn xa_is_value(entry: *mut Folio) -> bool;
    fn file_inode(file: *mut File) -> *mut Inode;
    fn is_file_hugepages(file: *mut File) -> bool;
    fn shmem_read_folio(mapping: *mut AddressSpace, index: PgoffT) -> *mut Folio;
    fn folio_put(folio: *mut Folio);
    fn folio_unlock(folio: *mut Folio);
    fn inode_lock(inode: *mut Inode);
    fn inode_unlock(inode: *mut Inode);
    fn mapping_deny_writable(mapping: *mut AddressSpace) -> c_int;
    fn mapping_allow_writable(mapping: *mut AddressSpace);
    fn lru_add_drain_all();
    fn schedule_timeout_killable(timeout: c_long) -> c_long;
    fn shmem_file(file: *mut File) -> bool;
    fn memfd_file_seals_ptr(file: *mut File) -> *mut c_uint;
}

const MEMFD_TAG_PINNED: c_uint = PAGECACHE_TAG_TOWRITE;
const LAST_SCAN: c_int = 4;

#[inline]
unsafe fn memfd_folio_has_extra_refs(folio: *mut Folio) -> bool {
    folio_ref_count(folio) != folio_expected_ref_count(folio)
}

unsafe fn memfd_tag_pins(xas: *mut XaState) {
    let mut latency = 0;
    lru_add_drain();
    xas_lock_irq(xas);
    // xas_for_each(xas, folio, ULONG_MAX)
    for folio in core::iter::empty::<*mut Folio>() {
        if !xa_is_value(folio) && memfd_folio_has_extra_refs(folio) { xas_set_mark(xas, MEMFD_TAG_PINNED); }
        latency += 1;
        if latency < XA_CHECK_SCHED { continue; }
        latency = 0; xas_pause(xas); xas_unlock_irq(xas); cond_resched(); xas_lock_irq(xas);
    }
    xas_unlock_irq(xas);
}

pub unsafe fn memfd_alloc_folio(memfd: *mut File, mut idx: PgoffT) -> *mut Folio {
    // CONFIG_HUGETLB_PAGE conditional code is retained as the direct kernel path.
    #[cfg(feature = "CONFIG_HUGETLB_PAGE")]
    {
        if is_file_hugepages(memfd) {
            let inode = file_inode(memfd);
            let h = hstate_file(memfd);
            let mut err: c_int = -ENOMEM;
            let gfp_mask = htlb_alloc_mask(h) & !(__GFP_HIGHMEM | __GFP_MOVABLE);
            idx >>= huge_page_order(h);
            let nr_resv = hugetlb_reserve_pages(inode, idx, idx + 1, core::ptr::null_mut(), EMPTY_VMA_FLAGS);
            if nr_resv < 0 { return ERR_PTR(nr_resv); }
            let folio = alloc_hugetlb_folio_reserve(h, numa_node_id(), core::ptr::null_mut(), gfp_mask);
            if !folio.is_null() {
                folio_zero_user(folio, 0); __folio_mark_uptodate(folio);
                let hash = hugetlb_fault_mutex_hash((*memfd).f_mapping, idx);
                mutex_lock(&mut hugetlb_fault_mutex_table[hash as usize]);
                err = hugetlb_add_to_page_cache(folio, (*memfd).f_mapping, idx);
                mutex_unlock(&mut hugetlb_fault_mutex_table[hash as usize]);
                if err != 0 { folio_put(folio); } else {
                    hugetlb_set_folio_subpool(folio, subpool_inode(inode)); folio_unlock(folio); return folio;
                }
            }
            if nr_resv > 0 { hugetlb_unreserve_pages(inode, idx, idx + 1, 0); }
            return ERR_PTR(err);
        }
    }
    shmem_read_folio((*memfd).f_mapping, idx)
}

unsafe fn memfd_wait_for_pins(mapping: *mut AddressSpace) -> c_int {
    let mut xas = XA_STATE(mapping, 0);
    memfd_tag_pins(&mut xas);
    let mut error = 0;
    for scan in 0..=LAST_SCAN {
        let mut latency = 0;
        if !xas_marked(&mut xas, MEMFD_TAG_PINNED) { break; }
        if scan == 0 { lru_add_drain_all(); }
        else if schedule_timeout_killable((HZ << scan) / 200) != 0 { /* scan = LAST_SCAN */ }
        xas_set(&mut xas, 0); xas_lock_irq(&mut xas);
        // xas_for_each_marked(xas, folio, ULONG_MAX, MEMFD_TAG_PINNED)
        for folio in core::iter::empty::<*mut Folio>() {
            let mut clear = true;
            if !xa_is_value(folio) && memfd_folio_has_extra_refs(folio) {
                if scan == LAST_SCAN { error = -EBUSY; } else { clear = false; }
            }
            if clear { xas_clear_mark(&mut xas, MEMFD_TAG_PINNED); }
            latency += 1;
            if latency >= XA_CHECK_SCHED { latency = 0; xas_pause(&mut xas); xas_unlock_irq(&mut xas); cond_resched(); xas_lock_irq(&mut xas); }
        }
        xas_unlock_irq(&mut xas);
    }
    error
}

const F_ALL_SEALS: c_uint = F_SEAL_SEAL | F_SEAL_EXEC | F_SEAL_SHRINK | F_SEAL_GROW | F_SEAL_WRITE | F_SEAL_FUTURE_WRITE;

pub unsafe fn memfd_add_seals(file: *mut File, mut seals: c_uint) -> c_int {
    let inode = file_inode(file);
    let mut error;
    if (*file).f_mode & FMODE_WRITE == 0 { return -EPERM; }
    if seals & !F_ALL_SEALS != 0 { return -EINVAL; }
    inode_lock(inode);
    let ptr = memfd_file_seals_ptr(file);
    if ptr.is_null() { error = -EINVAL; } else if *ptr & F_SEAL_SEAL != 0 { error = -EPERM; }
    else {
        if seals & F_SEAL_EXEC != 0 && (*inode).i_mode & 0o111 != 0 { seals |= F_SEAL_SHRINK | F_SEAL_GROW | F_SEAL_WRITE | F_SEAL_FUTURE_WRITE; }
        if seals & F_SEAL_WRITE != 0 && *ptr & F_SEAL_WRITE == 0 {
            error = mapping_deny_writable((*file).f_mapping);
            if error == 0 { error = memfd_wait_for_pins((*file).f_mapping); if error != 0 { mapping_allow_writable((*file).f_mapping); } }
            if error != 0 { inode_unlock(inode); return error; }
        }
        *ptr |= seals; error = 0;
    }
    inode_unlock(inode); error
}

pub unsafe fn memfd_get_seals(file: *mut File) -> c_int { let p = memfd_file_seals_ptr(file); if p.is_null() { -EINVAL } else { *p as c_int } }

pub unsafe fn memfd_fcntl(file: *mut File, cmd: c_uint, arg: c_uint) -> c_long {
    match cmd { F_ADD_SEALS => memfd_add_seals(file, arg) as c_long, F_GET_SEALS => memfd_get_seals(file) as c_long, _ => -EINVAL as c_long }
}

const MFD_NAME_PREFIX: &[u8] = b"memfd:\0";
const MFD_NAME_PREFIX_LEN: usize = MFD_NAME_PREFIX.len() - 1;
const MFD_NAME_MAX_LEN: usize = NAME_MAX - MFD_NAME_PREFIX_LEN;
const MFD_ALL_FLAGS: c_uint = MFD_CLOEXEC | MFD_ALLOW_SEALING | MFD_HUGETLB | MFD_NOEXEC_SEAL | MFD_EXEC;

unsafe fn check_sysctl_memfd_noexec(_flags: *mut c_uint) -> c_int { 0 }
#[inline] unsafe fn is_write_sealed(seals: c_uint) -> bool { seals & (F_SEAL_WRITE | F_SEAL_FUTURE_WRITE) != 0 }
unsafe fn check_write_seal(v: *mut VmaFlagsT) -> c_int {
    if !vma_flags_test(v, VMA_SHARED_BIT) { return 0; }
    if vma_flags_test(v, VMA_WRITE_BIT) { return -EPERM; }
    vma_flags_clear(v, VMA_MAYWRITE_BIT); 0
}
pub unsafe fn memfd_check_seals_mmap(file: *mut File, v: *mut VmaFlagsT) -> c_int {
    let p = memfd_file_seals_ptr(file); let seals = if p.is_null() { 0 } else { *p };
    if is_write_sealed(seals) { check_write_seal(v) } else { 0 }
}

unsafe fn sanitize_flags(flags: *mut c_uint) -> c_int {
    let f = *flags;
    if f & MFD_HUGETLB == 0 { if f & !MFD_ALL_FLAGS != 0 { return -EINVAL; } }
    else if f & !(MFD_ALL_FLAGS | (MFD_HUGE_MASK << MFD_HUGE_SHIFT)) != 0 { return -EINVAL; }
    if f & MFD_EXEC != 0 && f & MFD_NOEXEC_SEAL != 0 { return -EINVAL; }
    check_sysctl_memfd_noexec(flags)
}

unsafe fn alloc_name(uname: *const c_char) -> *mut c_char {
    let name = kmalloc(NAME_MAX + 1, GFP_KERNEL); if name.is_null() { return ERR_PTR(-ENOMEM); }
    core::ptr::copy_nonoverlapping(MFD_NAME_PREFIX.as_ptr() as *const c_char, name, MFD_NAME_PREFIX_LEN);
    let len = strncpy_from_user(name.add(MFD_NAME_PREFIX_LEN), uname, (MFD_NAME_MAX_LEN + 1) as c_long);
    if len < 0 { kfree(name); ERR_PTR(-EFAULT) } else if len > MFD_NAME_MAX_LEN as c_long { kfree(name); ERR_PTR(-EINVAL) } else { name }
}

pub unsafe fn memfd_alloc_file(name: *const c_char, flags: c_uint) -> *mut File {
    let file = if flags & MFD_HUGETLB != 0 { hugetlb_file_setup(name, 0, mk_vma_flags(VMA_NORESERVE_BIT), HUGETLB_ANONHUGE_INODE, (flags >> MFD_HUGE_SHIFT) & MFD_HUGE_MASK) } else { shmem_file_setup(name, 0, mk_vma_flags(VMA_NORESERVE_BIT)) };
    if IS_ERR(file) { return file; }
    let inode = file_inode(file); let err = security_inode_init_security_anon(inode, &QSTR(MEMFD_ANON_NAME), core::ptr::null_mut());
    if err != 0 { fput(file); return ERR_PTR(err); }
    (*file).f_mode |= FMODE_LSEEK | FMODE_PREAD | FMODE_PWRITE; (*file).f_flags |= O_LARGEFILE;
    if flags & MFD_NOEXEC_SEAL != 0 { (*inode).i_mode &= !0o111; let p = memfd_file_seals_ptr(file); if !p.is_null() { *p &= !F_SEAL_SEAL; *p |= F_SEAL_EXEC; } }
    else if flags & MFD_ALLOW_SEALING != 0 { let p = memfd_file_seals_ptr(file); if !p.is_null() { *p &= !F_SEAL_SEAL; } }
    file
}

// SYSCALL_DEFINE2(memfd_create, uname, flags)
pub unsafe fn memfd_create(uname: *const c_char, mut flags: c_uint) -> c_long {
    let error = sanitize_flags(&mut flags); if error < 0 { return error as c_long; }
    let name = alloc_name(uname); if IS_ERR(name) { return PTR_ERR(name); }
    let fd_flags = if flags & MFD_CLOEXEC != 0 { O_CLOEXEC } else { 0 };
    FD_ADD(fd_flags, memfd_alloc_file(name, flags))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
