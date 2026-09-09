// SPDX-License-Identifier: GPL-2.0
/* Shared Memory Communications Direct over ISM devices (SMC-D)
 *
 * Functions for ISM device.
 *
 * Copyright IBM Corp. 2018
 */

// Dependencies supplied by the surrounding kernel/project translation.

#[repr(C)]
pub union SmcdSwEventInfo {
    pub info: u64,
    pub fields: SmcdSwEventFields,
}
#[repr(C)]
pub struct SmcdSwEventFields {
    pub uid: [u8; SMC_LGR_ID_SIZE as usize],
    pub vlan_id: u16,
    pub code: u16,
}

pub static mut smcd_dev_list: smcd_dev_list = smcd_dev_list {
    list: LIST_HEAD_INIT(smcd_dev_list.list),
    mutex: __MUTEX_INITIALIZER(smcd_dev_list.mutex),
};

static mut smc_ism_v2_capable: bool = false;
static mut smc_ism_v2_system_eid: [u8; SMC_MAX_EID_LEN as usize] = [0; SMC_MAX_EID_LEN as usize];

extern "C" {
    fn smcd_register_dev(dibs: *mut dibs_dev);
    fn smcd_unregister_dev(dibs: *mut dibs_dev);
    fn smcd_handle_event(dibs: *mut dibs_dev, event: *const dibs_event);
    fn smcd_handle_irq(dibs: *mut dibs_dev, dmbno: c_uint, dmbemask: u16);
}

static mut smc_client_ops: dibs_client_ops = dibs_client_ops {
    add_dev: Some(smcd_register_dev), del_dev: Some(smcd_unregister_dev),
    handle_event: Some(smcd_handle_event), handle_irq: Some(smcd_handle_irq),
};
static mut smc_dibs_client: dibs_client = dibs_client {
    name: b"SMC-D\0".as_ptr() as *const c_char,
    ops: unsafe { &mut smc_client_ops },
};

unsafe fn smc_ism_create_system_eid() {
    let seid = &mut *(&mut smc_ism_v2_system_eid as *mut _ as *mut smc_ism_seid);
    #[cfg(target_arch = "s390x")]
    {
        let mut id: cpuid = core::mem::zeroed();
        let mut tmp = [0i8; 5];
        memcpy(seid.seid_string.as_mut_ptr() as *mut c_void, b"IBM-SYSZ-ISMSEID00000000".as_ptr() as *const c_void, 24);
        get_cpu_id(&mut id);
        let ident_tail = (id.ident & SMC_ISM_IDENT_MASK) as u16;
        snprintf(tmp.as_mut_ptr(), 5, b"%04X\0".as_ptr() as *const c_char, ident_tail);
        memcpy(seid.serial_number.as_mut_ptr() as *mut c_void, tmp.as_ptr() as *const c_void, 4);
        snprintf(tmp.as_mut_ptr(), 5, b"%04X\0".as_ptr() as *const c_char, id.machine);
        memcpy(seid.type_.as_mut_ptr() as *mut c_void, tmp.as_ptr() as *const c_void, 4);
    }
    #[cfg(not(target_arch = "s390x"))]
    { memset(seid as *mut _ as *mut c_void, 0, SMC_MAX_EID_LEN as usize); }
}

pub unsafe fn smc_ism_cantalk(peer_gid: *mut smcd_gid, vlan_id: u16, smcd: *mut smcd_dev) -> c_int {
    let dibs = (*smcd).dibs;
    let mut ism_rgid: uuid_t = core::mem::zeroed();
    copy_to_dibsgid(&mut ism_rgid, peer_gid);
    ((*(*dibs).ops).query_remote_gid.unwrap())(dibs, &mut ism_rgid, if vlan_id != 0 { 1 } else { 0 }, vlan_id)
}

pub unsafe fn smc_ism_get_system_eid(eid: *mut *mut u8) {
    *eid = if !smc_ism_v2_capable { core::ptr::null_mut() } else { smc_ism_v2_system_eid.as_mut_ptr() };
}
pub unsafe fn smc_ism_get_chid(smcd: *mut smcd_dev) -> u16 { ((*(*(*smcd).dibs).ops).get_fabric_id.unwrap())((*smcd).dibs) }
pub unsafe fn smc_ism_is_v2_capable() -> bool { smc_ism_v2_capable }
pub unsafe fn smc_ism_set_v2_capable() { smc_ism_v2_capable = true; }

/* Set a connection using this DMBE. */
pub unsafe fn smc_ism_set_conn(conn: *mut smc_connection) {
    let mut flags = 0ul;
    spin_lock_irqsave(&mut (*(*(*conn).lgr).smcd).lock, &mut flags);
    (*(*(*conn).lgr).smcd).conn[(*(*conn).rmb_desc).sba_idx as usize] = conn;
    spin_unlock_irqrestore(&mut (*(*(*conn).lgr).smcd).lock, flags);
}
/* Unset a connection using this DMBE. */
pub unsafe fn smc_ism_unset_conn(conn: *mut smc_connection) {
    if (*conn).rmb_desc.is_null() { return; }
    let mut flags = 0ul;
    spin_lock_irqsave(&mut (*(*(*conn).lgr).smcd).lock, &mut flags);
    (*(*(*conn).lgr).smcd).conn[(*(*conn).rmb_desc).sba_idx as usize] = core::ptr::null_mut();
    spin_unlock_irqrestore(&mut (*(*(*conn).lgr).smcd).lock, flags);
}

pub unsafe fn smc_ism_get_vlan(smcd: *mut smcd_dev, vlanid: u16) -> c_int {
    let mut new_vlan: *mut smc_ism_vlanid;
    let mut flags = 0ul;
    let mut rc = 0;
    if vlanid == 0 { return -EINVAL; }
    if (*(*smcd).dibs).ops.add_vlan_id.is_none() { return -EOPNOTSUPP; }
    new_vlan = kzalloc_obj();
    if new_vlan.is_null() { return -ENOMEM; }
    (*new_vlan).vlanid = vlanid; refcount_set(&mut (*new_vlan).refcnt, 1);
    spin_lock_irqsave(&mut (*smcd).lock, &mut flags);
    let mut vlan = (*smcd).vlan.next as *mut smc_ism_vlanid;
    while vlan != smcd as *mut _ {
        if (*vlan).vlanid == vlanid { refcount_inc(&mut (*vlan).refcnt); kfree(new_vlan as *mut c_void); goto_out!(); }
        vlan = (*vlan).list.next as *mut smc_ism_vlanid;
    }
    if ((*(*smcd).dibs).ops.add_vlan_id.unwrap())((*smcd).dibs, vlanid) != 0 { kfree(new_vlan as *mut c_void); rc = -EIO; goto_out!(); }
    list_add_tail(&mut (*new_vlan).list, &mut (*smcd).vlan);
    goto_out!();
    macro_rules! goto_out { () => { spin_unlock_irqrestore(&mut (*smcd).lock, flags); return rc; }; }
}

pub unsafe fn smc_ism_put_vlan(smcd: *mut smcd_dev, vlanid: u16) -> c_int {
    let mut flags = 0ul; let mut found = false; let mut rc = 0;
    if vlanid == 0 { return -EINVAL; }
    if (*(*smcd).dibs).ops.del_vlan_id.is_none() { return -EOPNOTSUPP; }
    spin_lock_irqsave(&mut (*smcd).lock, &mut flags);
    let mut vlan = (*smcd).vlan.next as *mut smc_ism_vlanid;
    while vlan != smcd as *mut _ { if (*vlan).vlanid == vlanid { if !refcount_dec_and_test(&mut (*vlan).refcnt) { spin_unlock_irqrestore(&mut (*smcd).lock, flags); return 0; } found = true; break; } vlan = (*vlan).list.next as *mut smc_ism_vlanid; }
    if !found { rc = -ENOENT; spin_unlock_irqrestore(&mut (*smcd).lock, flags); return rc; }
    if ((*(*smcd).dibs).ops.del_vlan_id.unwrap())((*smcd).dibs, vlanid) != 0 { rc = -EIO; }
    list_del(&mut (*vlan).list); kfree(vlan as *mut c_void);
    spin_unlock_irqrestore(&mut (*smcd).lock, flags); rc
}

pub unsafe fn smc_ism_unregister_dmb(smcd: *mut smcd_dev, dmb_desc: *mut smc_buf_desc) {
    if (*dmb_desc).dma_addr == 0 { return; }
    let mut dmb: dibs_dmb = core::mem::zeroed(); dmb.dmb_tok=(*dmb_desc).token; dmb.idx=(*dmb_desc).sba_idx; dmb.cpu_addr=(*dmb_desc).cpu_addr; dmb.dma_addr=(*dmb_desc).dma_addr; dmb.dmb_len=(*dmb_desc).len;
    ((*(*smcd).dibs).ops.unregister_dmb.unwrap())((*smcd).dibs, &mut dmb);
}

pub unsafe fn smc_ism_register_dmb(lgr: *mut smc_link_group, dmb_len: c_int, dmb_desc: *mut smc_buf_desc) -> c_int {
    let mut dmb: dibs_dmb = core::mem::zeroed(); dmb.dmb_len=dmb_len; dmb.idx=(*dmb_desc).sba_idx; dmb.vlan_id=(*lgr).vlan_id; copy_to_dibsgid(&mut dmb.rgid, &mut (*lgr).peer_gid);
    let dibs=(*(*lgr).smcd).dibs; let rc=((*(*dibs).ops).register_dmb.unwrap())(dibs,&mut dmb,&mut smc_dibs_client);
    if rc==0 { (*dmb_desc).sba_idx=dmb.idx; (*dmb_desc).token=dmb.dmb_tok; (*dmb_desc).cpu_addr=dmb.cpu_addr; (*dmb_desc).dma_addr=dmb.dma_addr; (*dmb_desc).len=dmb.dmb_len; } rc
}

pub unsafe fn smc_ism_support_dmb_nocopy(smcd: *mut smcd_dev) -> bool {
    (*(*smcd).dibs).ops.support_mmapped_rdmb.map_or(false, |f| f((*smcd).dibs))
}
pub unsafe fn smc_ism_attach_dmb(dev: *mut smcd_dev, token: u64, dmb_desc: *mut smc_buf_desc) -> c_int {
    let f=match (*(*dev).dibs).ops.attach_dmb { Some(f)=>f, None=>return -EINVAL }; let mut dmb: dibs_dmb=core::mem::zeroed(); dmb.dmb_tok=token; let rc=f((*dev).dibs,&mut dmb); if rc==0 { (*dmb_desc).sba_idx=dmb.idx; (*dmb_desc).token=dmb.dmb_tok; (*dmb_desc).cpu_addr=dmb.cpu_addr; (*dmb_desc).dma_addr=dmb.dma_addr; (*dmb_desc).len=dmb.dmb_len; (*dmb_desc).is_attached=true; } rc
}
pub unsafe fn smc_ism_detach_dmb(dev: *mut smcd_dev, token: u64) -> c_int { match (*(*dev).dibs).ops.detach_dmb { Some(f)=>f((*dev).dibs,token), None=>-EINVAL } }

/* Netlink device dump, event workers, device registration and interrupt handling retain
 * the kernel callbacks and data structures supplied by the surrounding translation. */
pub unsafe fn smcd_nl_get_device(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int { smc_nl_prep_smcd_dev(&mut smcd_dev_list, skb, cb); (*skb).len as c_int }

pub unsafe fn smc_ism_signal_shutdown(lgr: *mut smc_link_group) -> c_int {
    if (*lgr).peer_shutdown || (*(*(*lgr).smcd).dibs).ops.signal_event.is_none() { return 0; }
    let mut ev: SmcdSwEventInfo=core::mem::zeroed(); memcpy(ev.fields.uid.as_mut_ptr() as *mut c_void, (*lgr).id.as_ptr() as *const c_void, SMC_LGR_ID_SIZE as usize); ev.fields.vlan_id=(*lgr).vlan_id; ev.fields.code=ISM_EVENT_REQUEST;
    let mut rgid: uuid_t=core::mem::zeroed(); copy_to_dibsgid(&mut rgid,&mut (*lgr).peer_gid);
    ((*(*(*lgr).smcd).dibs).ops.signal_event.unwrap())((*(*lgr).smcd).dibs,&mut rgid,ISM_EVENT_REQUEST_IR,ISM_EVENT_CODE_SHUTDOWN,ev.info)
}
pub unsafe fn smc_ism_init() -> c_int { smc_ism_v2_capable=false; smc_ism_create_system_eid(); dibs_register_client(&mut smc_dibs_client) }
pub unsafe fn smc_ism_exit() { dibs_unregister_client(&mut smc_dibs_client); }

const ISM_EVENT_REQUEST: u16 = 0x0001;
const ISM_EVENT_RESPONSE: u16 = 0x0002;
const ISM_EVENT_REQUEST_IR: u32 = 0x00000001;
const ISM_EVENT_CODE_SHUTDOWN: u8 = 0x80;
const ISM_EVENT_CODE_TESTLINK: u8 = 0x83;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
