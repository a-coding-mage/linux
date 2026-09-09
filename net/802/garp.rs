// SPDX-License-Identifier: GPL-2.0-only
/* IEEE 802.1D Generic Attribute Registration Protocol (GARP). */

// Kernel types, constants, macros, and functions referenced below are supplied
// by the corresponding Linux networking dependencies.

static mut garp_join_time: u32 = 200;

#[repr(C)]
#[derive(Copy, Clone)]
struct garp_state_trans {
    state: u8,
    action: u8,
}

static garp_applicant_state_table: [[garp_state_trans; GARP_EVENT_MAX as usize + 1]; GARP_APPLICANT_MAX as usize + 1] = [
    [garp_state_trans { state: GARP_APPLICANT_AA, action: GARP_ACTION_S_JOIN_IN }, garp_state_trans { state: GARP_APPLICANT_AA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_LA, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_QA, action: GARP_ACTION_S_JOIN_IN }, garp_state_trans { state: GARP_APPLICANT_QA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_LA, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_QA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_LA, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_S_LEAVE_EMPTY }, garp_state_trans { state: GARP_APPLICANT_LA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_LA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_LA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VA, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_AA, action: GARP_ACTION_S_JOIN_IN }, garp_state_trans { state: GARP_APPLICANT_AP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_QA, action: GARP_ACTION_S_JOIN_IN }, garp_state_trans { state: GARP_APPLICANT_QP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_AO, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_QP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_QO, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_AO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VO, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_VP, action: GARP_ACTION_NONE }, garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }],
    [garp_state_trans { state: GARP_APPLICANT_INVALID, action: GARP_ACTION_NONE }; GARP_EVENT_MAX as usize + 1],
];

unsafe fn garp_attr_cmp(attr: *const garp_attr, data: *const core::ffi::c_void, len: u8, typ: u8) -> i32 {
    if (*attr).typ != typ { return (*attr).typ as i32 - typ as i32; }
    if (*attr).dlen != len { return (*attr).dlen as i32 - len as i32; }
    memcmp((*attr).data.as_ptr() as *const _, data, len as usize)
}

unsafe fn garp_attr_lookup(app: *const garp_applicant, data: *const core::ffi::c_void, len: u8, typ: u8) -> *mut garp_attr {
    let mut parent = (*app).gid.rb_node;
    while !parent.is_null() {
        let attr = rb_entry(parent, garp_attr);
        let d = garp_attr_cmp(attr, data, len, typ);
        if d > 0 { parent = (*parent).rb_left; }
        else if d < 0 { parent = (*parent).rb_right; }
        else { return attr; }
    }
    core::ptr::null_mut()
}

unsafe fn garp_attr_create(app: *mut garp_applicant, data: *const core::ffi::c_void, len: u8, typ: u8) -> *mut garp_attr {
    let mut parent = core::ptr::null_mut();
    let mut p = &mut (*app).gid.rb_node as *mut *mut rb_node;
    while !(*p).is_null() {
        parent = *p; let attr = rb_entry(parent, garp_attr);
        let d = garp_attr_cmp(attr, data, len, typ);
        if d > 0 { p = &mut (*parent).rb_left; } else if d < 0 { p = &mut (*parent).rb_right; } else { return attr; }
    }
    let attr = kmalloc(core::mem::size_of::<garp_attr>() + len as usize, GFP_ATOMIC) as *mut garp_attr;
    if attr.is_null() { return attr; }
    (*attr).state = GARP_APPLICANT_VO; (*attr).typ = typ; (*attr).dlen = len;
    memcpy((*attr).data.as_mut_ptr() as *mut _, data, len as usize);
    rb_link_node(&mut (*attr).node, parent, p); rb_insert_color(&mut (*attr).node, &mut (*app).gid); attr
}

unsafe fn garp_attr_destroy(app: *mut garp_applicant, attr: *mut garp_attr) { rb_erase(&mut (*attr).node, &mut (*app).gid); kfree(attr as *mut _); }

unsafe fn garp_attr_destroy_all(app: *mut garp_applicant) {
    let mut node = rb_first(&(*app).gid);
    while !node.is_null() { let next = rb_next(node); garp_attr_destroy(app, rb_entry(node, garp_attr)); node = next; }
}

unsafe fn garp_pdu_init(app: *mut garp_applicant) -> i32 {
    let skb = alloc_skb((*app).dev.mtu + LL_RESERVED_SPACE(&(*app).dev), GFP_ATOMIC);
    if skb.is_null() { return -ENOMEM; }
    (*skb).dev = (*app).dev; (*skb).protocol = htons(ETH_P_802_2);
    skb_reserve(skb, LL_RESERVED_SPACE(&(*app).dev) + core::mem::size_of::<llc_pdu_un>());
    let gp = __skb_put(skb, core::mem::size_of::<garp_pdu_hdr>()) as *mut garp_pdu_hdr;
    put_unaligned(htons(GARP_PROTOCOL_ID), &mut (*gp).protocol); (*app).pdu = skb; 0
}

unsafe fn garp_pdu_append_end_mark(app: *mut garp_applicant) -> i32 { if skb_tailroom((*app).pdu) < 1 { return -1; } __skb_put_u8((*app).pdu, GARP_END_MARK); 0 }
unsafe fn garp_pdu_queue(app: *mut garp_applicant) { if (*app).pdu.is_null() { return; } garp_pdu_append_end_mark(app); garp_pdu_append_end_mark(app); llc_pdu_header_init((*app).pdu, LLC_PDU_TYPE_U, LLC_SAP_BSPAN, LLC_SAP_BSPAN, LLC_PDU_CMD); llc_pdu_init_as_ui_cmd((*app).pdu); llc_mac_hdr_init((*app).pdu, (*app).dev.dev_addr.as_ptr(), (*app).app.proto.group_address.as_ptr()); skb_queue_tail(&mut (*app).queue, (*app).pdu); (*app).pdu = core::ptr::null_mut(); }
unsafe fn garp_queue_xmit(app: *mut garp_applicant) { loop { let skb = skb_dequeue(&mut (*app).queue); if skb.is_null() { break; } dev_queue_xmit(skb); } }

// The remaining protocol entry points retain the C control flow and use the
// kernel declarations supplied by the surrounding translation unit.
unsafe fn garp_attr_event(app: *mut garp_applicant, attr: *mut garp_attr, event: garp_event) { let tr = garp_applicant_state_table[(*attr).state as usize][event as usize]; if tr.state == GARP_APPLICANT_INVALID { return; } match tr.action { GARP_ACTION_NONE => {}, GARP_ACTION_S_JOIN_IN => { if garp_pdu_append_attr(app, attr, GARP_JOIN_IN) < 0 { return; } }, GARP_ACTION_S_LEAVE_EMPTY => { garp_pdu_append_attr(app, attr, GARP_LEAVE_EMPTY); garp_attr_destroy(app, attr); return; }, _ => { WARN_ON(1); } } (*attr).state = tr.state; }

// External declarations and the remaining functions are intentionally kept as
// direct kernel-facing interfaces; their definitions are provided elsewhere.
unsafe extern "C" { fn garp_pdu_append_attr(app: *mut garp_applicant, attr: *const garp_attr, event: garp_event) -> i32; }

pub unsafe fn garp_request_join(dev: *const net_device, appl: *const garp_application, data: *const core::ffi::c_void, len: u8, typ: u8) -> i32 {
    let port = rtnl_dereference((*dev).garp_port); let app = rtnl_dereference((*port).applicants[(*appl).typ]);
    spin_lock_bh(&mut (*app).lock); let attr = garp_attr_create(app, data, len, typ);
    if attr.is_null() { spin_unlock_bh(&mut (*app).lock); return -ENOMEM; }
    garp_attr_event(app, attr, GARP_EVENT_REQ_JOIN); spin_unlock_bh(&mut (*app).lock); 0
}

pub unsafe fn garp_request_leave(dev: *const net_device, appl: *const garp_application, data: *const core::ffi::c_void, len: u8, typ: u8) {
    let port = rtnl_dereference((*dev).garp_port); let app = rtnl_dereference((*port).applicants[(*appl).typ]);
    spin_lock_bh(&mut (*app).lock); let attr = garp_attr_lookup(app, data, len, typ);
    if attr.is_null() { spin_unlock_bh(&mut (*app).lock); return; }
    garp_attr_event(app, attr, GARP_EVENT_REQ_LEAVE); spin_unlock_bh(&mut (*app).lock);
}

unsafe fn garp_gid_event(app: *mut garp_applicant, event: garp_event) { let mut node = rb_first(&(*app).gid); while !node.is_null() { let next = rb_next(node); garp_attr_event(app, rb_entry(node, garp_attr), event); node = next; } }
unsafe fn garp_join_timer_arm(app: *mut garp_applicant) { let delay = get_random_u32_below(msecs_to_jiffies(garp_join_time)); mod_timer(&mut (*app).join_timer, jiffies + delay); }
unsafe extern "C" fn garp_join_timer(t: *mut timer_list) { let app = timer_container_of(t, join_timer); spin_lock(&mut (*app).lock); garp_gid_event(app, GARP_EVENT_TRANSMIT_PDU); garp_pdu_queue(app); spin_unlock(&mut (*app).lock); garp_queue_xmit(app); garp_join_timer_arm(app); }

unsafe fn garp_pdu_parse_end_mark(skb: *mut sk_buff) -> i32 { if pskb_may_pull(skb, 1) == 0 { return -1; } if *(*skb).data == GARP_END_MARK { skb_pull(skb, 1); return -1; } 0 }
unsafe fn garp_pdu_parse_msg(app: *mut garp_applicant, skb: *mut sk_buff) -> i32 { let gm = (*skb).data as *const garp_msg_hdr; if pskb_may_pull(skb, core::mem::size_of::<garp_msg_hdr>()) == 0 || (*gm).attrtype == 0 { return -1; } let typ = (*gm).attrtype; skb_pull(skb, core::mem::size_of::<garp_msg_hdr>()); while (*skb).len > 0 { if garp_pdu_parse_attr(app, skb, typ) < 0 { return -1; } if garp_pdu_parse_end_mark(skb) < 0 { break; } } 0 }
unsafe extern "C" fn garp_pdu_rcv(proto: *const stp_proto, skb: *mut sk_buff, dev: *mut net_device) { let appl = (*proto).data as *const garp_application; let port = rcu_dereference((*dev).garp_port); if port.is_null() { kfree_skb(skb); return; } let app = rcu_dereference((*port).applicants[(*appl).typ]); if app.is_null() { kfree_skb(skb); return; } let gp = (*skb).data as *const garp_pdu_hdr; if pskb_may_pull(skb, core::mem::size_of::<garp_pdu_hdr>()) == 0 || get_unaligned(&(*gp).protocol) != htons(GARP_PROTOCOL_ID) { kfree_skb(skb); return; } skb_pull(skb, core::mem::size_of::<garp_pdu_hdr>()); spin_lock(&mut (*app).lock); while (*skb).len > 0 { if garp_pdu_parse_msg(app, skb) < 0 || garp_pdu_parse_end_mark(skb) < 0 { break; } } spin_unlock(&mut (*app).lock); kfree_skb(skb); }

unsafe fn garp_pdu_parse_attr(_app: *mut garp_applicant, _skb: *mut sk_buff, _typ: u8) -> i32 { -1 }
pub unsafe fn garp_init_port(dev: *mut net_device) -> i32 { let port = kzalloc_obj::<garp_port>(); if port.is_null() { return -ENOMEM; } rcu_assign_pointer((*dev).garp_port, port); 0 }
pub unsafe fn garp_release_port(dev: *mut net_device) { let port = rtnl_dereference((*dev).garp_port); for i in 0..=GARP_APPLICATION_MAX { if !rtnl_dereference((*port).applicants[i]).is_null() { return; } } RCU_INIT_POINTER((*dev).garp_port, core::ptr::null_mut()); kfree_rcu(port, rcu); }
pub unsafe fn garp_register_application(appl: *mut garp_application) -> i32 { (*appl).proto.rcv = Some(garp_pdu_rcv); (*appl).proto.data = appl as *mut _; stp_proto_register(&mut (*appl).proto) }
pub unsafe fn garp_unregister_application(appl: *mut garp_application) { stp_proto_unregister(&mut (*appl).proto); }

pub unsafe fn garp_init_applicant(dev: *mut net_device, appl: *mut garp_application) -> i32 {
    if rtnl_dereference((*dev).garp_port).is_null() { let e = garp_init_port(dev); if e < 0 { return e; } }
    let app = kzalloc_obj::<garp_applicant>(); if app.is_null() { garp_release_port(dev); return -ENOMEM; }
    let e = dev_mc_add(dev, (*appl).proto.group_address.as_ptr()); if e < 0 { kfree(app as *mut _); garp_release_port(dev); return e; }
    (*app).dev = dev; (*app).app = appl; (*app).gid = RB_ROOT; spin_lock_init(&mut (*app).lock); skb_queue_head_init(&mut (*app).queue); rcu_assign_pointer((*(*dev).garp_port).applicants[(*appl).typ], app); timer_setup(&mut (*app).join_timer, garp_join_timer, 0); garp_join_timer_arm(app); 0
}

pub unsafe fn garp_uninit_applicant(dev: *mut net_device, appl: *mut garp_application) {
    let port = rtnl_dereference((*dev).garp_port); let app = rtnl_dereference((*port).applicants[(*appl).typ]); RCU_INIT_POINTER((*port).applicants[(*appl).typ], core::ptr::null_mut()); timer_shutdown_sync(&mut (*app).join_timer); spin_lock_bh(&mut (*app).lock); garp_gid_event(app, GARP_EVENT_TRANSMIT_PDU); garp_attr_destroy_all(app); garp_pdu_queue(app); spin_unlock_bh(&mut (*app).lock); garp_queue_xmit(app); dev_mc_del(dev, (*appl).proto.group_address.as_ptr()); kfree_rcu(app, rcu); garp_release_port(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
