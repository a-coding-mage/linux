/* SPDX-License-Identifier: GPL-2.0 */
/* Shared Memory Communications Direct over ISM devices (SMC-D)
 *
 * SMC-D ISM device structure definitions.
 *
 * Copyright IBM Corp. 2018
 */

// Dependencies supplied by the surrounding kernel translation.

pub const SMC_EMULATED_ISM_CHID_MASK: u32 = 0xFF00;
pub const SMC_ISM_IDENT_MASK: u32 = 0x00FFFF;

#[repr(C)]
pub struct smcd_dev_list {
    pub list: list_head,
    /* Protects list of devices */
    pub mutex: mutex,
}

extern "C" {
    pub static mut smcd_dev_list: smcd_dev_list;
}

#[repr(C)]
pub struct smc_ism_vlanid {
    pub list: list_head,
    /* Vlan id */
    pub vlanid: u16,
    /* Reference count */
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct smc_ism_seid {
    pub seid_string: [u8; 24],
    pub serial_number: [u8; 4],
    pub type_: [u8; 4],
}

#[repr(C)]
pub struct smcd_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn smc_ism_cantalk(peer_gid: *mut smcd_gid, vlan_id: u16, dev: *mut smcd_dev) -> i32;
    pub fn smc_ism_set_conn(conn: *mut smc_connection);
    pub fn smc_ism_unset_conn(conn: *mut smc_connection);
    pub fn smc_ism_get_vlan(dev: *mut smcd_dev, vlan_id: u16) -> i32;
    pub fn smc_ism_put_vlan(dev: *mut smcd_dev, vlan_id: u16) -> i32;
    pub fn smc_ism_register_dmb(lgr: *mut smc_link_group, buf_size: i32, dmb_desc: *mut smc_buf_desc) -> i32;
    pub fn smc_ism_unregister_dmb(dev: *mut smcd_dev, dmb_desc: *mut smc_buf_desc);
    pub fn smc_ism_support_dmb_nocopy(smcd: *mut smcd_dev) -> bool;
    pub fn smc_ism_attach_dmb(dev: *mut smcd_dev, token: u64, dmb_desc: *mut smc_buf_desc) -> i32;
    pub fn smc_ism_detach_dmb(dev: *mut smcd_dev, token: u64) -> i32;
    pub fn smc_ism_signal_shutdown(lgr: *mut smc_link_group) -> i32;
    pub fn smc_ism_get_system_eid(eid: *mut *mut u8);
    pub fn smc_ism_get_chid(dev: *mut smcd_dev) -> u16;
    pub fn smc_ism_is_v2_capable() -> bool;
    pub fn smc_ism_set_v2_capable();
    pub fn smc_ism_init() -> i32;
    pub fn smc_ism_exit();
    pub fn smcd_nl_get_device(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32;
}

#[inline]
pub unsafe fn smc_ism_write(
    smcd: *mut smcd_dev, dmb_tok: u64, idx: u32, sf: bool, offset: u32,
    data: *mut core::ffi::c_void, len: usize,
) -> i32 {
    let rc = (*(*smcd).dibs).ops.move_data((*smcd).dibs, dmb_tok, idx, sf, offset, data, len);
    if rc < 0 { rc } else { 0 }
}

#[inline]
pub fn __smc_ism_is_emulated(chid: u16) -> bool {
    /* CHIDs in range of 0xFF00 to 0xFFFF are reserved
     * for Emulated-ISM device.
     *
     * loopback-ism:  0xFFFF
     * virtio-ism:    0xFF00 ~ 0xFFFE
     */
    (chid & 0xFF00) == 0xFF00
}

#[inline]
pub unsafe fn smc_ism_is_emulated(smcd: *mut smcd_dev) -> bool {
    let chid = (*(*smcd).dibs).ops.get_fabric_id((*smcd).dibs);
    __smc_ism_is_emulated(chid)
}

#[inline]
pub unsafe fn smc_ism_is_loopback(dibs: *mut dibs_dev) -> bool {
    (*dibs).ops.get_fabric_id(dibs) == DIBS_LOOPBACK_FABRIC
}

#[inline]
pub unsafe fn copy_to_smcdgid(sgid: *mut smcd_gid, dibs_gid: *mut uuid_t) {
    let mut temp: u64;
    core::ptr::copy_nonoverlapping(dibs_gid as *const u8, &mut temp as *mut u64 as *mut u8, core::mem::size_of_val(&(*sgid).gid));
    (*sgid).gid = ntohll(temp);
    core::ptr::copy_nonoverlapping((dibs_gid as *const u8).add(core::mem::size_of_val(&(*sgid).gid)), &mut temp as *mut u64 as *mut u8, core::mem::size_of_val(&(*sgid).gid_ext));
    (*sgid).gid_ext = ntohll(temp);
}

#[inline]
pub unsafe fn copy_to_dibsgid(dibs_gid: *mut uuid_t, sgid: *mut smcd_gid) {
    let mut temp = htonll((*sgid).gid);
    core::ptr::copy_nonoverlapping(&temp as *const u64 as *const u8, dibs_gid as *mut u8, core::mem::size_of_val(&(*sgid).gid));
    temp = htonll((*sgid).gid_ext);
    core::ptr::copy_nonoverlapping(&temp as *const u64 as *const u8, (dibs_gid as *mut u8).add(core::mem::size_of_val(&(*sgid).gid)), core::mem::size_of_val(&(*sgid).gid_ext));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
