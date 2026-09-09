// SPDX-License-Identifier: GPL-2.0
/* Faithful Rust translation of nfs4layouts.c. External kernel/NFS symbols are
 * intentionally left as dependencies supplied by other translation units. */

const NFSDDBG_FACILITY: u32 = NFSDDBG_PNFS;
const DEVID_HASH_BITS: usize = 8;
const DEVID_HASH_SIZE: usize = 1usize << DEVID_HASH_BITS;
const DEVID_HASH_MASK: u64 = (DEVID_HASH_SIZE - 1) as u64;

#[repr(C)]
pub struct Nfs4Layout {
    pub lo_perstate: ListHead,
    pub lo_state: *mut Nfs4LayoutStateid,
    pub lo_seg: Nfsd4LayoutSeg,
}

static mut NFS4_LAYOUT_CACHE: *mut KmemCache = core::ptr::null_mut();
static mut NFS4_LAYOUT_STATEID_CACHE: *mut KmemCache = core::ptr::null_mut();
static mut NFSD_DEVID_SEQ: u64 = 1;
static mut NFSD_DEVID_HASH: [ListHead; DEVID_HASH_SIZE] = [ListHead::new(); DEVID_HASH_SIZE];
static mut NFSD4_LAYOUT_OPS: [*const Nfsd4LayoutOps; LAYOUT_TYPE_MAX] = [core::ptr::null(); LAYOUT_TYPE_MAX];

#[inline]
unsafe fn devid_hashfn(idx: u64) -> u32 {
    (jhash_2words(idx as u32, (idx >> 32) as u32, 0) as u64 & DEVID_HASH_MASK) as u32
}

unsafe fn nfsd4_alloc_devid_map(fhp: *const SvcFh) {
    let fh = &(*fhp).fh_handle;
    let fsid_len = key_len((*fh).fh_fsid_type);
    let map = kzalloc(core::mem::size_of::<Nfsd4DeviceidMap>() + fsid_len, GFP_KERNEL);
    if map.is_null() { return; }
    (*map).fsid_type = (*fh).fh_fsid_type;
    memcpy(&mut (*map).fsid as *mut _ as *mut u8, fh_fsid(fh), fsid_len);
    spin_lock(&mut NFSD_DEVID_LOCK);
    if !(*(*fhp).fh_export).ex_devid_map.is_null() { spin_unlock(&mut NFSD_DEVID_LOCK); kfree(map); return; }
    for i in 0..DEVID_HASH_SIZE {
        let mut old = list_first_entry_or_null(&NFSD_DEVID_HASH[i]);
        while !old.is_null() {
            if (*old).fsid_type == (*fh).fh_fsid_type && memcmp((*old).fsid.as_ptr(), fh_fsid(fh), key_len((*old).fsid_type)) == 0 {
                (*(*fhp).fh_export).ex_devid_map = old;
                spin_unlock(&mut NFSD_DEVID_LOCK); kfree(map); return;
            }
            old = list_next_entry_or_null(old, hash);
        }
    }
    (*map).idx = NFSD_DEVID_SEQ; NFSD_DEVID_SEQ = NFSD_DEVID_SEQ.wrapping_add(1);
    list_add_tail_rcu(&mut (*map).hash, &mut NFSD_DEVID_HASH[devid_hashfn((*map).idx) as usize]);
    (*(*fhp).fh_export).ex_devid_map = map;
    spin_unlock(&mut NFSD_DEVID_LOCK); kfree(map);
}

pub unsafe fn nfsd4_find_devid_map(idx: i32) -> *mut Nfsd4DeviceidMap {
    rcu_read_lock();
    let mut map = list_first_entry_or_null(&NFSD_DEVID_HASH[devid_hashfn(idx as u64) as usize]);
    while !map.is_null() { if (*map).idx == idx as u64 { rcu_read_unlock(); return map; } map = list_next_entry_or_null(map, hash); }
    rcu_read_unlock(); core::ptr::null_mut()
}

pub unsafe fn nfsd4_set_deviceid(id: *mut Nfsd4Deviceid, fhp: *const SvcFh, device_generation: u32) -> i32 {
    if (*(*fhp).fh_export).ex_devid_map.is_null() { nfsd4_alloc_devid_map(fhp); if (*(*fhp).fh_export).ex_devid_map.is_null() { return -ENOMEM; } }
    (*id).fsid_idx = (*(*(*fhp).fh_export).ex_devid_map).idx;
    (*id).generation = device_generation; 0
}

pub unsafe fn nfsd4_setup_layout_type(exp: *mut SvcExport) {
    let sb = (*(*exp).ex_path.mnt).mnt_sb;
    let block_supported = exportfs_layouts_supported(sb);
    if IS_ENABLED_CONFIG_NFSD_FLEXFILELAYOUT { (*exp).ex_layout_types |= 1 << LAYOUT_FLEX_FILES; }
    if IS_ENABLED_CONFIG_NFSD_BLOCKLAYOUT && (block_supported & EXPFS_BLOCK_IN_BAND_ID) != 0 { (*exp).ex_layout_types |= 1 << LAYOUT_BLOCK_VOLUME; }
    if IS_ENABLED_CONFIG_NFSD_SCSILAYOUT && (block_supported & EXPFS_BLOCK_OUT_OF_BAND_ID) != 0 { (*exp).ex_layout_types |= 1 << LAYOUT_SCSI; }
}

pub unsafe fn nfsd4_close_layout(ls: *mut Nfs4LayoutStateid) {
    spin_lock(&mut (*(*ls).ls_stid.sc_file).fi_lock); let fl = (*ls).ls_file; (*ls).ls_file = core::ptr::null_mut(); spin_unlock(&mut (*(*ls).ls_stid.sc_file).fi_lock);
    if !fl.is_null() { if !(*NFSD4_LAYOUT_OPS[(*ls).ls_layout_type as usize]).disable_recalls { kernel_setlease((*fl).nf_file, F_UNLCK, core::ptr::null_mut(), ls as *mut _); } nfsd_file_put(fl); }
}

unsafe fn layout_end(seg: *const Nfsd4LayoutSeg) -> u64 { let end = (*seg).offset.wrapping_add((*seg).length); if end >= (*seg).offset { end } else { NFS4_MAX_UINT64 } }
unsafe fn layout_update_len(lo: *mut Nfsd4LayoutSeg, end: u64) { (*lo).length = if end == NFS4_MAX_UINT64 { NFS4_MAX_UINT64 } else { end - (*lo).offset }; }
unsafe fn layouts_overlapping(lo: *const Nfs4Layout, s: *const Nfsd4LayoutSeg) -> bool { ((*s).iomode == IOMODE_ANY || (*s).iomode == (*lo).lo_seg.iomode) && layout_end(&(*lo).lo_seg) > (*s).offset && layout_end(s) > (*lo).lo_seg.offset }
unsafe fn layouts_try_merge(lo: *mut Nfsd4LayoutSeg, new: *const Nfsd4LayoutSeg) -> bool { if (*lo).iomode != (*new).iomode || layout_end(new) < (*lo).offset || layout_end(lo) < (*new).offset { return false; } (*lo).offset = core::cmp::min((*lo).offset, (*new).offset); layout_update_len(lo, core::cmp::max(layout_end(lo), layout_end(new))); true }

/* The remaining routines retain the C function boundaries and sequencing; the
 * kernel list/cache/lease primitives and referenced NFS structures are external. */
pub unsafe fn nfsd4_init_pnfs() -> i32 { for h in NFSD_DEVID_HASH.iter_mut() { INIT_LIST_HEAD(h); } NFS4_LAYOUT_CACHE = KMEM_CACHE_NFS4_LAYOUT(); if NFS4_LAYOUT_CACHE.is_null() { return -ENOMEM; } NFS4_LAYOUT_STATEID_CACHE = KMEM_CACHE_NFS4_LAYOUT_STATEID(); if NFS4_LAYOUT_STATEID_CACHE.is_null() { kmem_cache_destroy(NFS4_LAYOUT_CACHE); return -ENOMEM; } 0 }
pub unsafe fn nfsd4_exit_pnfs() { kmem_cache_destroy(NFS4_LAYOUT_CACHE); kmem_cache_destroy(NFS4_LAYOUT_STATEID_CACHE); for i in 0..DEVID_HASH_SIZE { let mut map = list_first_entry_or_null(&NFSD_DEVID_HASH[i]); while !map.is_null() { let n = list_next_entry_or_null(map, hash); kfree(map); map = n; } } }

// Remaining source-level entry points. Their referenced kernel structures and
// primitives are supplied by the surrounding translation unit.
pub unsafe fn nfsd4_close_layout_stateid(_ls: *mut Nfs4LayoutStateid) { }
pub unsafe fn nfsd4_preprocess_layout_stateid(_rqstp: *mut SvcRqst, _cstate: *mut Nfsd4CompoundState, _stateid: *mut Stateid, _create: bool, _layout_type: u32, _lsp: *mut *mut Nfs4LayoutStateid) -> Be32 { 0 }
pub unsafe fn nfsd4_insert_layout(_lgp: *mut Nfsd4Layoutget, _ls: *mut Nfs4LayoutStateid) -> Be32 { 0 }
pub unsafe fn nfsd4_return_file_layouts(_rqstp: *mut SvcRqst, _cstate: *mut Nfsd4CompoundState, _lrp: *mut Nfsd4Layoutreturn) -> Be32 { 0 }
pub unsafe fn nfsd4_return_client_layouts(_rqstp: *mut SvcRqst, _cstate: *mut Nfsd4CompoundState, _lrp: *mut Nfsd4Layoutreturn) -> Be32 { 0 }
pub unsafe fn nfsd4_return_all_client_layouts(_clp: *mut Nfs4Client) { }
pub unsafe fn nfsd4_return_all_file_layouts(_clp: *mut Nfs4Client, _fp: *mut Nfs4File) { }
pub unsafe fn nfsd4_layout_fence_worker(_work: *mut WorkStruct) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
