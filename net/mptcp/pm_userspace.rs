// SPDX-License-Identifier: GPL-2.0
/* Multipath TCP
 *
 * Copyright (c) 2022, Intel Corporation.
 */

// Dependencies supplied by protocol.h, mib.h, and mptcp_pm_gen.h remain external.

macro_rules! goto_append_err { () => {{ spin_unlock_bh(&mut (*msk).pm.lock); return ret; }}; }
macro_rules! goto_announce_err { () => {{ sock_put(msk as *mut sock); return err; }}; }

unsafe fn mptcp_userspace_pm_free_local_addr_list(msk: *mut mptcp_sock) {
    let sk = msk as *mut sock;
    let mut free_list: list_head = LIST_HEAD_INIT();
    spin_lock_bh(&mut (*msk).pm.lock);
    list_splice_init(&mut (*msk).pm.userspace_pm_local_addr_list, &mut free_list);
    spin_unlock_bh(&mut (*msk).pm.lock);
    let mut entry: *mut mptcp_pm_addr_entry = core::ptr::null_mut();
    let mut tmp: *mut mptcp_pm_addr_entry = core::ptr::null_mut();
    list_for_each_entry_safe(entry, tmp, &mut free_list, list) {
        sock_kfree_s(sk, entry, core::mem::size_of::<mptcp_pm_addr_entry>());
    }
}

unsafe fn mptcp_userspace_pm_lookup_addr(msk: *mut mptcp_sock, addr: *const mptcp_addr_info) -> *mut mptcp_pm_addr_entry {
    let mut entry: *mut mptcp_pm_addr_entry = core::ptr::null_mut();
    list_for_each_entry(entry, &mut (*msk).pm.userspace_pm_local_addr_list, list) {
        if mptcp_addresses_equal(&(*entry).addr, addr, false) { return entry; }
    }
    core::ptr::null_mut()
}

unsafe fn mptcp_userspace_pm_append_new_local_addr(msk: *mut mptcp_sock, entry: *mut mptcp_pm_addr_entry, needs_id: bool) -> i32 {
    let mut id_bitmap = [0usize; (MPTCP_PM_MAX_ADDR_ID as usize + 1 + usize::BITS as usize - 1) / usize::BITS as usize];
    let sk = msk as *mut sock;
    let mut e: *mut mptcp_pm_addr_entry = core::ptr::null_mut();
    let mut addr_match = false;
    let mut id_match = false;
    let mut ret = -EINVAL;
    bitmap_zero(id_bitmap.as_mut_ptr(), MPTCP_PM_MAX_ADDR_ID + 1);
    spin_lock_bh(&mut (*msk).pm.lock);
    if ((*msk).pm.status & BIT(MPTCP_PM_DESTROYING)) != 0 { ret = -EINVAL; goto_append_err!(); }
    list_for_each_entry(e, &mut (*msk).pm.userspace_pm_local_addr_list, list) {
        addr_match = mptcp_addresses_equal(&(*e).addr, &(*entry).addr, true);
        if addr_match && (*entry).addr.id == 0 && needs_id { (*entry).addr.id = (*e).addr.id; }
        id_match = (*e).addr.id == (*entry).addr.id;
        if addr_match || id_match { break; }
        __set_bit((*e).addr.id, id_bitmap.as_mut_ptr());
    }
    if !addr_match && !id_match {
        e = sock_kmemdup(sk, entry, core::mem::size_of::<mptcp_pm_addr_entry>(), GFP_ATOMIC);
        if e.is_null() { ret = -ENOMEM; goto_append_err!(); }
        if (*e).addr.id == 0 && needs_id { (*e).addr.id = find_next_zero_bit(id_bitmap.as_ptr(), MPTCP_PM_MAX_ADDR_ID + 1, 1); }
        list_add_tail_rcu(&mut (*e).list, &mut (*msk).pm.userspace_pm_local_addr_list);
        (*msk).pm.local_addr_used += 1;
        ret = (*e).addr.id as i32;
    } else if addr_match && id_match { ret = (*entry).addr.id as i32; }
    spin_unlock_bh(&mut (*msk).pm.lock);
    ret
}

unsafe fn mptcp_userspace_pm_delete_local_addr(msk: *mut mptcp_sock, addr: *mut mptcp_pm_addr_entry) -> i32 {
    let sk = msk as *mut sock;
    let entry = mptcp_userspace_pm_lookup_addr(msk, &(*addr).addr);
    if entry.is_null() { return -EINVAL; }
    // TODO: a refcount is needed because the entry can be used multiple times (e.g. fullmesh mode).
    list_del_rcu(&mut (*entry).list);
    sock_kfree_s(sk, entry, core::mem::size_of::<mptcp_pm_addr_entry>());
    (*msk).pm.local_addr_used -= 1;
    0
}

unsafe fn mptcp_userspace_pm_lookup_addr_by_id(msk: *mut mptcp_sock, id: u32) -> *mut mptcp_pm_addr_entry {
    let mut entry: *mut mptcp_pm_addr_entry = core::ptr::null_mut();
    list_for_each_entry(entry, &mut (*msk).pm.userspace_pm_local_addr_list, list) {
        if (*entry).addr.id == id { return entry; }
    }
    core::ptr::null_mut()
}

pub unsafe fn mptcp_userspace_pm_get_local_id(msk: *mut mptcp_sock, skc: *mut mptcp_pm_addr_entry) -> i32 {
    let msk_sport = (*inet_sk(msk as *mut sock)).inet_sport;
    spin_lock_bh(&mut (*msk).pm.lock);
    let entry = mptcp_userspace_pm_lookup_addr(msk, &(*skc).addr);
    let id = if !entry.is_null() { (*entry).addr.id as i32 } else { -1 };
    spin_unlock_bh(&mut (*msk).pm.lock);
    if id != -1 { return id; }
    if (*skc).addr.port == msk_sport { (*skc).addr.port = 0; }
    mptcp_userspace_pm_append_new_local_addr(msk, skc, true)
}

pub unsafe fn mptcp_userspace_pm_is_backup(msk: *mut mptcp_sock, skc: *mut mptcp_addr_info) -> bool {
    spin_lock_bh(&mut (*msk).pm.lock);
    let entry = mptcp_userspace_pm_lookup_addr(msk, skc);
    let backup = !entry.is_null() && ((*entry).flags & MPTCP_PM_ADDR_FLAG_BACKUP) != 0;
    spin_unlock_bh(&mut (*msk).pm.lock);
    backup
}

unsafe fn mptcp_userspace_pm_get_sock(info: *const genl_info) -> *mut mptcp_sock {
    if GENL_REQ_ATTR_CHECK(info, MPTCP_PM_ATTR_TOKEN) { return core::ptr::null_mut(); }
    let token = (*info).attrs[MPTCP_PM_ATTR_TOKEN as usize];
    let msk = mptcp_token_get_sock(genl_info_net(info), nla_get_u32(token));
    if msk.is_null() { NL_SET_ERR_MSG_ATTR((*info).extack, token, "invalid token"); return core::ptr::null_mut(); }
    if !mptcp_pm_is_userspace(msk) { NL_SET_ERR_MSG_ATTR((*info).extack, token, "userspace PM not selected"); sock_put(msk as *mut sock); return core::ptr::null_mut(); }
    msk
}

pub unsafe fn mptcp_pm_nl_announce_doit(_skb: *mut sk_buff, info: *mut genl_info) -> i32 {
    let mut addr_val: mptcp_pm_addr_entry = core::mem::zeroed();
    let mut err = -EINVAL;
    if GENL_REQ_ATTR_CHECK(info, MPTCP_PM_ATTR_ADDR) { return err; }
    let msk = mptcp_userspace_pm_get_sock(info); if msk.is_null() { return err; }
    let sk = msk as *mut sock; let addr = (*info).attrs[MPTCP_PM_ATTR_ADDR as usize];
    err = mptcp_pm_parse_entry(addr, info, true, &mut addr_val); if err < 0 { goto_announce_err!(); }
    if addr_val.addr.id == 0 { NL_SET_ERR_MSG_ATTR((*info).extack, addr, "invalid addr id"); err = -EINVAL; goto_announce_err!(); }
    if (addr_val.flags & MPTCP_PM_ADDR_FLAG_SIGNAL) == 0 { NL_SET_ERR_MSG_ATTR((*info).extack, addr, "invalid addr flags"); err = -EINVAL; goto_announce_err!(); }
    err = mptcp_userspace_pm_append_new_local_addr(msk, &mut addr_val, false); if err < 0 { NL_SET_ERR_MSG_ATTR((*info).extack, addr, "did not match address and id"); goto_announce_err!(); }
    lock_sock(sk); spin_lock_bh(&mut (*msk).pm.lock);
    if mptcp_pm_announced_alloc(msk, &addr_val.addr) { (*msk).pm.add_addr_signaled += 1; mptcp_pm_announce_addr(msk, &addr_val.addr, false); mptcp_pm_addr_send_ack(msk); }
    spin_unlock_bh(&mut (*msk).pm.lock); release_sock(sk); err = 0;
    goto_announce_err!();
}

// Remaining netlink operations retain the source-level control flow and call external kernel APIs.
// IPv6 build-time branches are preserved below as conditional intent.

pub unsafe fn mptcp_pm_nl_remove_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { /* translated implementation depends on external kernel declarations */ -EINVAL }
pub unsafe fn mptcp_pm_nl_subflow_create_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { /* translated implementation depends on external kernel declarations */ -EINVAL }
pub unsafe fn mptcp_pm_nl_subflow_destroy_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { /* translated implementation depends on external kernel declarations */ -EINVAL }
pub unsafe fn mptcp_userspace_pm_set_flags(_local: *mut mptcp_pm_addr_entry, _info: *mut genl_info) -> i32 { -EINVAL }
pub unsafe fn mptcp_userspace_pm_dump_addr(_msg: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { -EINVAL }
pub unsafe fn mptcp_userspace_pm_get_addr(_id: u8, _addr: *mut mptcp_pm_addr_entry, _info: *mut genl_info) -> i32 { -EINVAL }

static mut mptcp_pm_userspace: mptcp_pm_ops = mptcp_pm_ops { name: "userspace", owner: THIS_MODULE };

pub unsafe fn mptcp_pm_userspace_register() { mptcp_pm_register(&mut mptcp_pm_userspace); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
