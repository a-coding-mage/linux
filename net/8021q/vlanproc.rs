// SPDX-License-Identifier: GPL-2.0-or-later
/******************************************************************************
 * vlanproc.c VLAN Module. /proc filesystem interface.
 ******************************************************************************/

// Kernel includes and local headers are supplied by the surrounding repository.

/* Methods for preparing data for reading proc entries */
unsafe extern "C" {
    fn vlan_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
    fn vlan_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void;
    fn vlan_seq_next(seq: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void;
    fn vlan_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void);
    fn vlandev_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32;
}

static NAME_ROOT: &[u8] = b"vlan\0";
static NAME_CONF: &[u8] = b"config\0";

#[repr(C)]
struct seq_operations {
    start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut core::ffi::c_void>,
    next: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void, *mut loff_t) -> *mut core::ffi::c_void>,
    stop: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void)>,
    show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

static VLAN_SEQ_OPS: seq_operations = seq_operations {
    start: Some(vlan_seq_start),
    next: Some(vlan_seq_next),
    stop: Some(vlan_seq_stop),
    show: Some(vlan_seq_show),
};

static VLAN_NAME_TYPE_STR: [&[u8]; VLAN_NAME_TYPE_HIGHEST as usize] = [
    b"VLAN_NAME_TYPE_RAW_PLUS_VID\0",
    b"VLAN_NAME_TYPE_PLUS_VID_NO_PAD\0",
    b"VLAN_NAME_TYPE_RAW_PLUS_VID_NO_PAD\0",
    b"VLAN_NAME_TYPE_PLUS_VID\0",
];

pub unsafe fn vlan_proc_cleanup(net: *mut net) {
    let vn = net_generic(net, vlan_net_id) as *mut vlan_net;
    if !(*vn).proc_vlan_conf.is_null() {
        remove_proc_entry(NAME_CONF.as_ptr() as *const _, (*vn).proc_vlan_dir);
    }
    if !(*vn).proc_vlan_dir.is_null() {
        remove_proc_entry(NAME_ROOT.as_ptr() as *const _, (*net).proc_net);
    }
}

pub unsafe fn vlan_proc_init(net: *mut net) -> i32 {
    let vn = net_generic(net, vlan_net_id) as *mut vlan_net;
    (*vn).proc_vlan_dir = proc_net_mkdir(net, NAME_ROOT.as_ptr() as *const _, (*net).proc_net);
    if (*vn).proc_vlan_dir.is_null() { return vlan_proc_init_err(net); }
    (*vn).proc_vlan_conf = proc_create_net(
        NAME_CONF.as_ptr() as *const _, (S_IFREG | 0o600) as _, (*vn).proc_vlan_dir,
        &VLAN_SEQ_OPS, core::mem::size_of::<seq_net_private>(),
    );
    if (*vn).proc_vlan_conf.is_null() { return vlan_proc_init_err(net); }
    0
}

unsafe fn vlan_proc_init_err(net: *mut net) -> i32 {
    pr_err!("can't create entry in proc filesystem!\n");
    vlan_proc_cleanup(net);
    -ENOBUFS
}

pub unsafe fn vlan_proc_add_dev(vlandev: *mut net_device) -> i32 {
    let vlan = vlan_dev_priv(vlandev);
    let vn = net_generic(dev_net(vlandev), vlan_net_id) as *mut vlan_net;
    if strcmp((*vlandev).name.as_ptr(), NAME_CONF.as_ptr()) == 0 { return -EINVAL; }
    (*vlan).dent = proc_create_single_data(
        (*vlandev).name.as_ptr(), (S_IFREG | 0o600) as _, (*vn).proc_vlan_dir,
        Some(vlandev_seq_show), vlandev as *mut _ as *mut core::ffi::c_void,
    );
    if (*vlan).dent.is_null() { return -ENOBUFS; }
    0
}

pub unsafe fn vlan_proc_rem_dev(vlandev: *mut net_device) {
    proc_remove((*vlan_dev_priv(vlandev)).dent);
    (*vlan_dev_priv(vlandev)).dent = core::ptr::null_mut();
}

unsafe fn vlan_seq_from_index(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let mut ifindex = *pos as u64;
    let mut dev: *mut net_device = core::ptr::null_mut();
    for_each_netdev_dump!(seq_file_net(seq), dev, ifindex, {
        if !is_vlan_dev(dev) { continue; }
        *pos = (*dev).ifindex as loff_t;
        return dev as *mut _;
    });
    core::ptr::null_mut()
}

unsafe fn vlan_seq_start_impl(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    rcu_read_lock();
    if *pos == 0 { return SEQ_START_TOKEN; }
    vlan_seq_from_index(seq, pos)
}

unsafe fn vlan_seq_next_impl(seq: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    vlan_seq_from_index(seq, pos)
}

unsafe fn vlan_seq_stop_impl(_seq: *mut seq_file, _v: *mut core::ffi::c_void) { rcu_read_unlock(); }

unsafe fn vlandev_seq_show_impl(seq: *mut seq_file, _offset: *mut core::ffi::c_void) -> i32 {
    let vlandev = (*seq).private as *mut net_device;
    let vlan = vlan_dev_priv(vlandev);
    if !is_vlan_dev(vlandev) { return 0; }
    let mut temp: rtnl_link_stats64 = core::mem::zeroed();
    let stats = dev_get_stats(vlandev, &mut temp);
    seq_printf!(seq, "%s  VID: %d\t REORDER_HDR: %i  dev->priv_flags: %x\n", (*vlandev).name, (*vlan).vlan_id, ((*vlan).flags & 1) as i32, (*vlandev).priv_flags as u32);
    seq_printf!(seq, "%30s %12llu\n", "total frames received", (*stats).rx_packets);
    seq_printf!(seq, "%30s %12llu\n", "total bytes received", (*stats).rx_bytes);
    seq_printf!(seq, "%30s %12llu\n", "Broadcast/Multicast Rcvd", (*stats).multicast);
    seq_puts!(seq, "\n");
    seq_printf!(seq, "%30s %12llu\n", "total frames transmitted", (*stats).tx_packets);
    seq_printf!(seq, "%30s %12llu\n", "total bytes transmitted", (*stats).tx_bytes);
    seq_printf!(seq, "Device: %s", (*vlan).real_dev);
    seq_printf!(seq, "\nINGRESS priority mappings: 0:%u  1:%u  2:%u  3:%u  4:%u  5:%u  6:%u 7:%u\n", (*vlan).ingress_priority_map[0], (*vlan).ingress_priority_map[1], (*vlan).ingress_priority_map[2], (*vlan).ingress_priority_map[3], (*vlan).ingress_priority_map[4], (*vlan).ingress_priority_map[5], (*vlan).ingress_priority_map[6], (*vlan).ingress_priority_map[7]);
    seq_printf!(seq, " EGRESS priority mappings: ");
    rcu_read_lock();
    for i in 0..16 { let mut mp = rcu_dereference((*vlan).egress_priority_map[i]); while !mp.is_null() { let vlan_qos = READ_ONCE((*mp).vlan_qos); seq_printf!(seq, "%u:%d ", (*mp).priority, ((vlan_qos >> 13) & 7)); mp = rcu_dereference((*mp).next); } }
    rcu_read_unlock();
    seq_puts!(seq, "\n");
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
