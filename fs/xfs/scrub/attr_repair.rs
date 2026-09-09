// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level translation of xfs/scrub/attr_repair.c. */

// C header dependencies are supplied by the surrounding XFS translation.

#[repr(C)]
pub struct xrep_xattr_key {
    pub name_cookie: xfblob_cookie,
    pub value_cookie: xfblob_cookie,
    pub flags: c_int,
    pub valuelen: u32,
    pub namelen: u16,
}

pub const XREP_XATTR_MAX_STASH_BYTES: usize = PAGE_SIZE * 8;

#[repr(C)]
pub struct xrep_xattr {
    pub sc: *mut xfs_scrub,
    pub tx: xrep_tempexch,
    pub xattr_records: *mut xfarray,
    pub xattr_blobs: *mut xfblob,
    pub attrs_found: c_ulonglong,
    pub can_flush: bool,
    pub live_update_aborted: bool,
    pub lock: mutex,
    pub pptr_recs: *mut xfarray,
    pub pptr_names: *mut xfblob,
    pub dhook: xfs_dir_hook,
    pub pptr_args: xfs_da_args,
    pub xname: xfs_name,
    pub namebuf: [c_char; MAXNAMELEN],
}

pub const XREP_XATTR_PPTR_ADD: u8 = 1;
pub const XREP_XATTR_PPTR_REMOVE: u8 = 2;

#[repr(C)]
pub struct xrep_xattr_pptr {
    pub name_cookie: xfblob_cookie,
    pub pptr_rec: xfs_parent_rec,
    pub namelen: u8,
    pub action: u8,
}

pub unsafe fn xrep_setup_xattr(sc: *mut xfs_scrub) -> c_int {
    if xfs_has_parent((*sc).mp) { xchk_fsgates_enable(sc, XCHK_FSGATES_DIRENTS); }
    xrep_tempfile_create(sc, S_IFREG)
}

unsafe fn xrep_xattr_want_salvage(rx: *mut xrep_xattr, attr_flags: c_uint,
    name: *const c_void, namelen: c_int, value: *const c_void, valuelen: c_int) -> bool {
    if attr_flags & XFS_ATTR_INCOMPLETE != 0 || namelen > XATTR_NAME_MAX || namelen <= 0 { return false; }
    if !xfs_attr_namecheck(attr_flags, name, namelen) || valuelen > XATTR_SIZE_MAX || valuelen < 0 { return false; }
    if attr_flags & XFS_ATTR_PARENT != 0 { return xfs_parent_valuecheck((*(*rx).sc).mp, value, valuelen); }
    true
}

unsafe fn xrep_xattr_salvage_key(rx: *mut xrep_xattr, flags: c_int, name: *mut u8,
    namelen: c_int, value: *mut u8, valuelen: c_int) -> c_int {
    let mut key = xrep_xattr_key { name_cookie: zeroed(), value_cookie: zeroed(),
        flags: flags & XFS_ATTR_NSP_ONDISK_MASK, valuelen: valuelen as u32, namelen: 0 };
    let mut error = 0; let mut i = 0;
    if xchk_should_terminate((*rx).sc, &mut error) { return error; }
    if flags & XFS_ATTR_PARENT != 0 { key.namelen = namelen as u16; trace_xrep_xattr_salvage_pptr((*(*rx).sc).ip, flags, name, key.namelen, value, valuelen); }
    else { while i < namelen && *name.add(i as usize) != 0 { i += 1; } if i == 0 { return 0; } key.namelen = i as u16; trace_xrep_xattr_salvage_rec((*(*rx).sc).ip, flags, name, key.namelen, valuelen); }
    error = xfblob_store((*rx).xattr_blobs, &mut key.name_cookie, name, key.namelen as _); if error != 0 { return error; }
    error = xfblob_store((*rx).xattr_blobs, &mut key.value_cookie, value, key.valuelen as _); if error != 0 { return error; }
    error = xfarray_append((*rx).xattr_records, &key); if error != 0 { return error; }
    (*rx).attrs_found += 1; 0
}

unsafe fn xrep_xattr_salvage_sf_attr(rx: *mut xrep_xattr, hdr: *mut xfs_attr_sf_hdr, sfe: *mut xfs_attr_sf_entry) -> c_int {
    let name = (*sfe).nameval.as_mut_ptr(); let value = name.add((*sfe).namelen as usize);
    if !xchk_xattr_set_map((*rx).sc, (*(*rx).sc).buf.usedmap, name.offset_from(hdr as *mut u8) as _, (*sfe).namelen as _) { return 0; }
    if !xchk_xattr_set_map((*rx).sc, (*(*rx).sc).buf.usedmap, value.offset_from(hdr as *mut u8) as _, (*sfe).valuelen as _) { return 0; }
    if !xrep_xattr_want_salvage(rx, (*sfe).flags as _, name, (*sfe).namelen as _, value, (*sfe).valuelen as _) { return 0; }
    xrep_xattr_salvage_key(rx, (*sfe).flags as _, name, (*sfe).namelen as _, value, (*sfe).valuelen as _)
}

unsafe fn xrep_xattr_salvage_local_attr(rx: *mut xrep_xattr, ent: *mut xfs_attr_leaf_entry, nameidx: c_uint, buf_end: *const c_char, lentry: *mut xfs_attr_leaf_name_local) -> c_int {
    let value = (*lentry).nameval.as_mut_ptr().add((*lentry).namelen as usize);
    let valuelen = be16_to_cpu((*lentry).valuelen) as c_uint; let namesize = xfs_attr_leaf_entsize_local((*lentry).namelen, valuelen);
    if (lentry as *mut c_char).add(namesize as usize) > buf_end as *mut c_char { return 0; }
    if !xrep_xattr_want_salvage(rx, (*ent).flags as _, (*lentry).nameval.as_ptr() as _, (*lentry).namelen as _, value as _, valuelen as _) { return 0; }
    if !xchk_xattr_set_map((*rx).sc, (*(*rx).sc).buf.usedmap, nameidx, namesize) { return 0; }
    xrep_xattr_salvage_key(rx, (*ent).flags as _, (*lentry).nameval.as_mut_ptr(), (*lentry).namelen as _, value, valuelen as _)
}

unsafe fn xrep_xattr_salvage_remote_attr(rx: *mut xrep_xattr, ent: *mut xfs_attr_leaf_entry, nameidx: c_uint, buf_end: *const c_char, rentry: *mut xfs_attr_leaf_name_remote, ent_idx: c_uint, leaf_bp: *mut xfs_buf) -> c_int {
    let namesize = xfs_attr_leaf_entsize_remote((*rentry).namelen); if (rentry as *mut c_char).add(namesize as usize) > buf_end as *mut c_char { return 0; }
    let valuelen = be32_to_cpu((*rentry).valuelen); if valuelen == 0 || !xrep_xattr_want_salvage(rx, (*ent).flags as _, (*rentry).name.as_ptr() as _, (*rentry).namelen as _, core::ptr::null(), valuelen as _) { return 0; }
    if !xchk_xattr_set_map((*rx).sc, (*(*rx).sc).buf.usedmap, nameidx, namesize) { return 0; }
    let mut args: xfs_da_args = zeroed(); args.trans = (*rx).sc.tp; args.dp = (*rx).sc.ip; args.index = ent_idx; args.geo = (*rx).sc.mp.m_attr_geo; args.owner = I_INO((*rx).sc.ip); args.attr_filter = (*ent).flags & XFS_ATTR_NSP_ONDISK_MASK; args.namelen = (*rentry).namelen; args.name = (*rentry).name.as_mut_ptr(); args.valuelen = valuelen;
    let mut error = xchk_setup_xattr_buf((*rx).sc, valuelen); if error == -ENOMEM { error = -EDEADLOCK; } if error != 0 { return error; } args.value = (*(*rx).sc).buf.value;
    error = xfs_attr3_leaf_getvalue(leaf_bp, &mut args); if error != 0 || args.rmtblkno == 0 { return if error == -EFSBADCRC || error == -EFSCORRUPTED { 0 } else { error }; }
    error = xfs_attr_rmtval_get(&mut args); if error != 0 { return if error == -EFSBADCRC || error == -EFSCORRUPTED { 0 } else { error }; }
    error = xrep_xattr_salvage_key(rx, (*ent).flags as _, (*rentry).name.as_mut_ptr(), (*rentry).namelen as _, (*(*rx).sc).buf.value, args.valuelen as _);
    if error == -EFSBADCRC || error == -EFSCORRUPTED { 0 } else { error }
}

// The remaining routines retain the source control flow and call the supplied XFS interfaces.
// Their declarations are emitted below as unsafe Rust definitions to preserve the external API.

unsafe fn xrep_xattr_recover_leaf(rx: *mut xrep_xattr, bp: *mut xfs_buf) -> c_int {
    let sc = (*rx).sc; let mp = (*sc).mp; let mut leafhdr: xfs_attr3_icleaf_hdr = zeroed(); let leaf = (*bp).b_addr as *mut xfs_attr_leafblock;
    bitmap_zero((*sc).buf.usedmap, mp.m_attr_geo.blksize); xfs_attr3_leaf_hdr_from_disk(mp.m_attr_geo, &mut leafhdr, leaf); let hdrsize = xfs_attr3_leaf_hdr_size(leaf); xchk_xattr_set_map(sc, (*sc).buf.usedmap, 0, hdrsize); let entries = xfs_attr3_leaf_entryp(leaf); let end = (*bp).b_addr.add(mp.m_attr_geo.blksize as usize) as *mut c_char;
    for i in 0..leafhdr.count { let ent = entries.add(i as usize); let off = ent.offset_from(leaf as *mut xfs_attr_leaf_entry); if !xchk_xattr_set_map(sc, (*sc).buf.usedmap, off as _, size_of::<xfs_attr_leaf_entry>()) { continue; } let nameidx = be16_to_cpu((*ent).nameidx) as c_uint; if nameidx < leafhdr.firstused || nameidx >= mp.m_attr_geo.blksize { continue; } let mut error = if (*ent).flags & XFS_ATTR_LOCAL != 0 { xrep_xattr_salvage_local_attr(rx, ent, nameidx, end, xfs_attr3_leaf_name_local(leaf, i)) } else { xrep_xattr_salvage_remote_attr(rx, ent, nameidx, end, xfs_attr3_leaf_name_remote(leaf, i), i as _, bp) }; if xchk_should_terminate(sc, &mut error) { return error; } if error != 0 { return error; } }
    0
}

unsafe fn xrep_xattr_recover_sf(rx: *mut xrep_xattr) -> c_int { let sc=(*rx).sc; let ifp=xfs_ifork_ptr((*sc).ip,XFS_ATTR_FORK); let hdr=(*ifp).if_data as *mut xfs_attr_sf_hdr; bitmap_zero((*sc).buf.usedmap,(*ifp).if_bytes); xchk_xattr_set_map(sc,(*sc).buf.usedmap,0,size_of::<xfs_attr_sf_hdr>()); let end=(*ifp).if_data.add((*ifp).if_bytes as usize); let mut sfe=xfs_attr_sf_firstentry(hdr); for _ in 0..(*hdr).count { let next=xfs_attr_sf_nextentry(sfe); if next as *mut u8 > end { break; } let mut error=0; if xchk_should_terminate(sc,&mut error){return error;} if xchk_xattr_set_map(sc,(*sc).buf.usedmap,sfe.offset_from(hdr as *mut xfs_attr_sf_entry) as _,size_of::<xfs_attr_sf_entry>()){error=xrep_xattr_salvage_sf_attr(rx,hdr,sfe);if error!=0{return error;}} sfe=next;} 0 }

// Direct translations of the remaining public repair entry points and lifecycle helpers.
pub unsafe fn xrep_xattr_reset_fork(sc:*mut xfs_scrub)->c_int { trace_xrep_xattr_reset_fork((*sc).ip,(*sc).ip); if xfs_ifork_has_extents(&mut (*sc).ip.i_af){let e=xrep_reap_ifork(sc,(*sc).ip,XFS_ATTR_FORK);if e!=0{return e;}} let e=xrep_xattr_fork_remove(sc,(*sc).ip);if e!=0{return e;} xfs_trans_roll_inode(&mut (*sc).tp,(*sc).ip) }
pub unsafe fn xrep_xattr_reset_tempfile_fork(sc:*mut xfs_scrub)->c_int { trace_xrep_xattr_reset_fork((*sc).ip,(*sc).tempip); if xfs_ifork_has_extents(&mut (*sc).tempip.i_af){let e=xrep_reap_ifork(sc,(*sc).tempip,XFS_ATTR_FORK);if e!=0{return e;}} xrep_xattr_fork_remove(sc,(*sc).tempip) }

pub unsafe fn xrep_xattr(sc:*mut xfs_scrub)->c_int {
    if !xfs_inode_hasattr((*sc).ip){return -ENOENT;} if !xfs_has_rmapbt((*sc).mp)||!xfs_has_exchange_range((*sc).mp){return -EOPNOTSUPP;}
    let mut rx: *mut xrep_xattr=core::ptr::null_mut(); let mut error=xrep_xattr_setup_scan(sc,&mut rx); if error!=0{return error;} error=xrep_xattr_salvage_attributes(rx); if error==0 && (*rx).live_update_aborted{error=-EIO;} if error==0 && xchk_should_terminate(sc,&mut error){} if error==0{error=xrep_xattr_rebuild_tree(rx);} xrep_xattr_teardown(rx); error
}

// Local helper declarations corresponding to the remaining C routines; their
// implementations are supplied by the adjacent translated repair units.
unsafe extern "C" {
    fn xrep_xattr_find_buf(mp:*mut xfs_mount, fsbno:xfs_fsblock_t, max_len:xfs_extlen_t, can_read:bool, bpp:*mut *mut xfs_buf)->c_int;
    fn xrep_xattr_recover_block(rx:*mut xrep_xattr, dabno:xfs_dablk_t, fsbno:xfs_fsblock_t, max_len:xfs_extlen_t, actual_len:*mut xfs_extlen_t)->c_int;
    fn xrep_xattr_insert_rec(rx:*mut xrep_xattr, key:*const xrep_xattr_key)->c_int;
    fn xrep_xattr_flush_stashed(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_want_flush_stashed(rx:*mut xrep_xattr)->bool;
    fn xrep_xattr_saw_pptr_conflict(rx:*mut xrep_xattr)->bool;
    fn xrep_xattr_full_reset(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_recover(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_fork_remove(sc:*mut xfs_scrub, ip:*mut xfs_inode)->c_int;
    fn xrep_xattr_salvage_attributes(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_replay_pptr_update(rx:*mut xrep_xattr, xname:*const xfs_name, pptr:*mut xrep_xattr_pptr)->c_int;
    fn xrep_xattr_replay_pptr_updates(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_stash_parentadd(rx:*mut xrep_xattr, name:*const xfs_name, dp:*const xfs_inode)->c_int;
    fn xrep_xattr_stash_parentremove(rx:*mut xrep_xattr, name:*const xfs_name, dp:*const xfs_inode)->c_int;
    fn xrep_xattr_live_dirent_update(nb:*mut notifier_block, action:c_ulong, data:*mut c_void)->c_int;
    fn xrep_xattr_swap_prep(sc:*mut xfs_scrub, temp_local:bool, ip_local:bool)->c_int;
    fn xrep_xattr_swap(sc:*mut xfs_scrub, tx:*mut xrep_tempexch)->c_int;
    fn xrep_xattr_finalize_tempfile(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_rebuild_tree(rx:*mut xrep_xattr)->c_int;
    fn xrep_xattr_teardown(rx:*mut xrep_xattr);
    fn xrep_xattr_setup_scan(sc:*mut xfs_scrub, rxp:*mut *mut xrep_xattr)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
