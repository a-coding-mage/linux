// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

// External Linux BPF types, constants, globals, and helper functions are
// supplied by the surrounding translation unit.

unsafe fn bpf_mprog_link(
    tuple: *mut bpf_tuple,
    id_or_fd: u32,
    flags: u32,
    ty: bpf_prog_type,
) -> i32 {
    let mut link: *mut bpf_link = ERR_PTR(-EINVAL);
    let id = flags & BPF_F_ID != 0;

    if id {
        link = bpf_link_by_id(id_or_fd);
    } else if id_or_fd != 0 {
        link = bpf_link_get_from_fd(id_or_fd);
    }
    if IS_ERR(link) {
        return PTR_ERR(link);
    }
    if ty != BPF_PROG_TYPE_UNSPEC && (*link).prog.read().type_ != ty {
        bpf_link_put(link);
        return -EINVAL;
    }

    (*tuple).link = link;
    (*tuple).prog = (*link).prog;
    0
}

unsafe fn bpf_mprog_prog(
    tuple: *mut bpf_tuple,
    id_or_fd: u32,
    flags: u32,
    ty: bpf_prog_type,
) -> i32 {
    let mut prog: *mut bpf_prog = ERR_PTR(-EINVAL);
    let id = flags & BPF_F_ID != 0;

    if id {
        prog = bpf_prog_by_id(id_or_fd);
    } else if id_or_fd != 0 {
        prog = bpf_prog_get(id_or_fd);
    }
    if IS_ERR(prog) {
        return PTR_ERR(prog);
    }
    if ty != BPF_PROG_TYPE_UNSPEC && (*prog).type_ != ty {
        bpf_prog_put(prog);
        return -EINVAL;
    }

    (*tuple).link = core::ptr::null_mut();
    (*tuple).prog = prog;
    0
}

unsafe fn bpf_mprog_tuple_relative(
    tuple: *mut bpf_tuple,
    id_or_fd: u32,
    flags: u32,
    ty: bpf_prog_type,
) -> i32 {
    let link = flags & BPF_F_LINK != 0;
    let id = flags & BPF_F_ID != 0;

    core::ptr::write_bytes(tuple, 0, 1);
    if link {
        return bpf_mprog_link(tuple, id_or_fd, flags, ty);
    }
    /* If no relevant flag is set and no id_or_fd was passed, then
     * tuple link/prog is just NULLed. This is the case when before/
     * after selects first/last position without passing fd.
     */
    if !id && id_or_fd == 0 {
        return 0;
    }
    bpf_mprog_prog(tuple, id_or_fd, flags, ty)
}

unsafe fn bpf_mprog_tuple_put(tuple: *mut bpf_tuple) {
    if !(*tuple).link.is_null() {
        bpf_link_put((*tuple).link);
    } else if !(*tuple).prog.is_null() {
        bpf_prog_put((*tuple).prog);
    }
}

/* The bpf_mprog_{replace,delete}() operate on exact idx position with the
 * one exception that for deletion we support delete from front/back. In
 * case of front idx is -1, in case of back idx is bpf_mprog_total(entry).
 * Adjustment to first and last entry is trivial. The bpf_mprog_insert()
 * we have to deal with the following cases:
 *
 * idx + before:
 *
 * Insert P4 before P3: idx for old array is 1, idx for new array is 2,
 * hence we adjust target idx for the new array, so that memmove copies
 * P1 and P2 to the new entry, and we insert P4 into idx 2. Inserting
 * before P1 would have old idx -1 and new idx 0.
 *
 * +--+--+--+     +--+--+--+--+     +--+--+--+--+
 * |P1|P2|P3| ==> |P1|P2|  |P3| ==> |P1|P2|P4|P3|
 * +--+--+--+     +--+--+--+--+     +--+--+--+--+
 *
 * idx + after:
 *
 * Insert P4 after P2: idx for old array is 2, idx for new array is 2.
 * Again, memmove copies P1 and P2 to the new entry, and we insert P4
 * into idx 2. Inserting after P3 would have both old/new idx at 4 aka
 * bpf_mprog_total(entry).
 *
 * +--+--+--+     +--+--+--+--+     +--+--+--+--+
 * |P1|P2|P3| ==> |P1|P2|  |P3| ==> |P1|P2|P4|P3|
 * +--+--+--+     +--+--+--+--+     +--+--+--+--+
 */
unsafe fn bpf_mprog_replace(
    entry: *mut bpf_mprog_entry,
    entry_new: *mut *mut bpf_mprog_entry,
    ntuple: *mut bpf_tuple,
    idx: i32,
) -> i32 {
    let mut fp: *mut bpf_mprog_fp = core::ptr::null_mut();
    let mut cp: *mut bpf_mprog_cp = core::ptr::null_mut();

    bpf_mprog_read(entry, idx, &mut fp, &mut cp);
    let oprog = READ_ONCE((*fp).prog);
    bpf_mprog_write(fp, cp, ntuple);
    if (*ntuple).link.is_null() {
        WARN_ON_ONCE(!(*cp).link.is_null());
        bpf_prog_put(oprog);
    }
    *entry_new = entry;
    0
}

unsafe fn bpf_mprog_insert(
    entry: *mut bpf_mprog_entry,
    entry_new: *mut *mut bpf_mprog_entry,
    ntuple: *mut bpf_tuple,
    mut idx: i32,
    flags: u32,
) -> i32 {
    let total = bpf_mprog_total(entry);
    let peer = bpf_mprog_peer(entry);
    let mut fp: *mut bpf_mprog_fp = core::ptr::null_mut();
    let mut cp: *mut bpf_mprog_cp = core::ptr::null_mut();

    bpf_mprog_entry_copy(peer, entry);
    if idx != total {
        if flags & BPF_F_BEFORE != 0 {
            idx += 1;
        }
        bpf_mprog_entry_grow(peer, idx);
    }
    bpf_mprog_read(peer, idx, &mut fp, &mut cp);
    bpf_mprog_write(fp, cp, ntuple);
    bpf_mprog_inc(peer);
    *entry_new = peer;
    0
}

unsafe fn bpf_mprog_delete(
    entry: *mut bpf_mprog_entry,
    entry_new: *mut *mut bpf_mprog_entry,
    dtuple: *mut bpf_tuple,
    mut idx: i32,
) -> i32 {
    let total = bpf_mprog_total(entry);
    let peer = bpf_mprog_peer(entry);

    bpf_mprog_entry_copy(peer, entry);
    if idx == -1 {
        idx = 0;
    } else if idx == total {
        idx = total - 1;
    }
    bpf_mprog_entry_shrink(peer, idx);
    bpf_mprog_dec(peer);
    bpf_mprog_mark_for_release(peer, dtuple);
    *entry_new = peer;
    0
}

/* In bpf_mprog_pos_*() we evaluate the target position for the BPF
 * program/link that needs to be replaced, inserted or deleted for
 * each "rule" independently. If all rules agree on that position
 * or existing element, then enact replacement, addition or deletion.
 * If this is not the case, then the request cannot be satisfied and
 * we bail out with an error.
 */
unsafe fn bpf_mprog_pos_exact(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) -> i32 {
    let mut i = 0;
    while i < bpf_mprog_total(entry) {
        let mut fp = core::ptr::null_mut();
        let mut cp = core::ptr::null_mut();
        bpf_mprog_read(entry, i, &mut fp, &mut cp);
        if (*tuple).prog == READ_ONCE((*fp).prog) {
            return if (*tuple).link == (*cp).link { i } else { -EBUSY };
        }
        i += 1;
    }
    -ENOENT
}

unsafe fn bpf_mprog_pos_before(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) -> i32 {
    let mut i = 0;
    while i < bpf_mprog_total(entry) {
        let mut fp = core::ptr::null_mut();
        let mut cp = core::ptr::null_mut();
        bpf_mprog_read(entry, i, &mut fp, &mut cp);
        if (*tuple).prog == READ_ONCE((*fp).prog)
            && ((*tuple).link.is_null() || (*tuple).link == (*cp).link)
        {
            return i - 1;
        }
        i += 1;
    }
    if !(*tuple).prog.is_null() { -ENOENT } else { -1 }
}

unsafe fn bpf_mprog_pos_after(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple) -> i32 {
    let mut i = 0;
    while i < bpf_mprog_total(entry) {
        let mut fp = core::ptr::null_mut();
        let mut cp = core::ptr::null_mut();
        bpf_mprog_read(entry, i, &mut fp, &mut cp);
        if (*tuple).prog == READ_ONCE((*fp).prog)
            && ((*tuple).link.is_null() || (*tuple).link == (*cp).link)
        {
            return i + 1;
        }
        i += 1;
    }
    if !(*tuple).prog.is_null() { -ENOENT } else { bpf_mprog_total(entry) }
}

unsafe fn bpf_mprog_attach(
    entry: *mut bpf_mprog_entry,
    entry_new: *mut *mut bpf_mprog_entry,
    prog_new: *mut bpf_prog,
    link: *mut bpf_link,
    prog_old: *mut bpf_prog,
    flags: u32,
    id_or_fd: u32,
    revision: u64,
) -> i32 {
    let mut rtuple = core::mem::zeroed::<bpf_tuple>();
    let mut ntuple = bpf_tuple { prog: prog_new, link };
    let otuple = bpf_tuple { prog: prog_old, link };
    let mut idx = -ERANGE;

    if revision != 0 && revision != bpf_mprog_revision(entry) { return -ESTALE; }
    if bpf_mprog_exists(entry, prog_new) { return -EEXIST; }
    let mut ret = bpf_mprog_tuple_relative(&mut rtuple, id_or_fd, flags & !BPF_F_REPLACE, (*prog_new).type_);
    if ret != 0 { return ret; }
    if flags & BPF_F_REPLACE != 0 {
        let tidx = bpf_mprog_pos_exact(entry, &otuple);
        if tidx < 0 { ret = tidx; bpf_mprog_tuple_put(&mut rtuple); return ret; }
        idx = tidx;
    } else if bpf_mprog_total(entry) == bpf_mprog_max() {
        ret = -ERANGE; bpf_mprog_tuple_put(&mut rtuple); return ret;
    }
    if flags & BPF_F_BEFORE != 0 {
        let tidx = bpf_mprog_pos_before(entry, &rtuple);
        if tidx < -1 || (idx >= -1 && tidx != idx) { ret = if tidx < -1 { tidx } else { -ERANGE }; bpf_mprog_tuple_put(&mut rtuple); return ret; }
        idx = tidx;
    }
    if flags & BPF_F_AFTER != 0 {
        let tidx = bpf_mprog_pos_after(entry, &rtuple);
        if tidx < -1 || (idx >= -1 && tidx != idx) { ret = if tidx < 0 { tidx } else { -ERANGE }; bpf_mprog_tuple_put(&mut rtuple); return ret; }
        idx = tidx;
    }
    if idx < -1 {
        if !rtuple.prog.is_null() || flags != 0 { ret = -EINVAL; bpf_mprog_tuple_put(&mut rtuple); return ret; }
        idx = bpf_mprog_total(entry);
    }
    if idx >= bpf_mprog_max() { ret = -ERANGE; bpf_mprog_tuple_put(&mut rtuple); return ret; }
    ret = if flags & BPF_F_REPLACE != 0 { bpf_mprog_replace(entry, entry_new, &mut ntuple, idx) } else { bpf_mprog_insert(entry, entry_new, &mut ntuple, idx, flags) };
    bpf_mprog_tuple_put(&mut rtuple);
    ret
}

unsafe fn bpf_mprog_fetch(entry: *mut bpf_mprog_entry, tuple: *mut bpf_tuple, mut idx: i32) -> i32 {
    let total = bpf_mprog_total(entry);
    if idx == -1 { idx = 0; } else if idx == total { idx = total - 1; }
    let mut fp = core::ptr::null_mut(); let mut cp = core::ptr::null_mut();
    bpf_mprog_read(entry, idx, &mut fp, &mut cp);
    let prog = READ_ONCE((*fp).prog); let link = (*cp).link;
    if !link.is_null() && (*tuple).link.is_null() { return -EBUSY; }
    WARN_ON_ONCE(!(*tuple).prog.is_null() && (*tuple).prog != prog);
    WARN_ON_ONCE(!(*tuple).link.is_null() && (*tuple).link != link);
    (*tuple).prog = prog; (*tuple).link = link; 0
}

unsafe fn bpf_mprog_detach(
    entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry,
    prog: *mut bpf_prog, link: *mut bpf_link, flags: u32, id_or_fd: u32, revision: u64,
) -> i32 {
    if flags & BPF_F_REPLACE != 0 { return -EINVAL; }
    if revision != 0 && revision != bpf_mprog_revision(entry) { return -ESTALE; }
    if bpf_mprog_total(entry) == 0 { return -ENOENT; }
    let mut rtuple = core::mem::zeroed::<bpf_tuple>();
    let mut dtuple = bpf_tuple { prog, link };
    let mut idx = -ERANGE;
    let mut ret = bpf_mprog_tuple_relative(&mut rtuple, id_or_fd, flags, if !prog.is_null() { (*prog).type_ } else { BPF_PROG_TYPE_UNSPEC });
    if ret != 0 { return ret; }
    if !dtuple.prog.is_null() { let tidx = bpf_mprog_pos_exact(entry, &dtuple); if tidx < 0 { ret = tidx; bpf_mprog_tuple_put(&mut rtuple); return ret; } idx = tidx; }
    if flags & BPF_F_BEFORE != 0 { let tidx = bpf_mprog_pos_before(entry, &rtuple); if tidx < -1 || (idx >= -1 && tidx != idx) { ret = if tidx < -1 { tidx } else { -ERANGE }; bpf_mprog_tuple_put(&mut rtuple); return ret; } idx = tidx; }
    if flags & BPF_F_AFTER != 0 { let tidx = bpf_mprog_pos_after(entry, &rtuple); if tidx < -1 || (idx >= -1 && tidx != idx) { ret = if tidx < 0 { tidx } else { -ERANGE }; bpf_mprog_tuple_put(&mut rtuple); return ret; } idx = tidx; }
    if idx < -1 { if !rtuple.prog.is_null() || flags != 0 { ret = -EINVAL; bpf_mprog_tuple_put(&mut rtuple); return ret; } idx = bpf_mprog_total(entry); }
    if idx >= bpf_mprog_max() { ret = -ERANGE; bpf_mprog_tuple_put(&mut rtuple); return ret; }
    ret = bpf_mprog_fetch(entry, &mut dtuple, idx);
    if ret == 0 { ret = bpf_mprog_delete(entry, entry_new, &mut dtuple, idx); }
    bpf_mprog_tuple_put(&mut rtuple); ret
}

unsafe fn bpf_mprog_query(attr: *const bpf_attr, uattr: *mut bpf_attr, entry: *mut bpf_mprog_entry) -> i32 {
    let flags: u32 = 0; let mut count = 0; let mut revision = 1u64;
    if (*attr).query.query_flags != 0 || (*attr).query.attach_flags != 0 { return -EINVAL; }
    if !entry.is_null() { revision = bpf_mprog_revision(entry); count = bpf_mprog_total(entry); }
    if copy_to_user(&mut (*uattr).query.attach_flags, &flags, core::mem::size_of_val(&flags)) != 0 { return -EFAULT; }
    if copy_to_user(&mut (*uattr).query.revision, &revision, core::mem::size_of_val(&revision)) != 0 { return -EFAULT; }
    if copy_to_user(&mut (*uattr).query.count, &count, core::mem::size_of_val(&count)) != 0 { return -EFAULT; }
    let uprog_id = u64_to_user_ptr((*attr).query.prog_ids); let uprog_flags = u64_to_user_ptr((*attr).query.prog_attach_flags);
    let ulink_id = u64_to_user_ptr((*attr).query.link_ids); let ulink_flags = u64_to_user_ptr((*attr).query.link_attach_flags);
    if (*attr).query.count == 0 || uprog_id.is_null() || count == 0 { return 0; }
    let mut ret = 0; if (*attr).query.count < count { count = (*attr).query.count; ret = -ENOSPC; }
    let mut i = 0; while i < bpf_mprog_max() {
        let mut fp = core::ptr::null_mut(); let mut cp = core::ptr::null_mut(); bpf_mprog_read(entry, i, &mut fp, &mut cp);
        let prog = READ_ONCE((*fp).prog); if prog.is_null() { break; }
        let id = (*prog).aux.id; if copy_to_user(uprog_id.add(i as usize), &id, core::mem::size_of_val(&id)) != 0 { return -EFAULT; }
        if !uprog_flags.is_null() && copy_to_user(uprog_flags.add(i as usize), &flags, core::mem::size_of_val(&flags)) != 0 { return -EFAULT; }
        let id = if !(*cp).link.is_null() { (*cp).link.id } else { 0 }; if !ulink_id.is_null() && copy_to_user(ulink_id.add(i as usize), &id, core::mem::size_of_val(&id)) != 0 { return -EFAULT; }
        if !ulink_flags.is_null() && copy_to_user(ulink_flags.add(i as usize), &flags, core::mem::size_of_val(&flags)) != 0 { return -EFAULT; }
        if i + 1 == count { break; } i += 1;
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
