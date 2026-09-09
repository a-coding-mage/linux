// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2023 Isovalent

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn rtnl_lock();
    fn rtnl_unlock();
    fn __dev_get_by_index(net: *mut net, ifindex: u32) -> *mut net_device;
    fn bpf_prog_get_type(fd: i32, ty: u32) -> *mut bpf_prog;
    fn bpf_prog_put(prog: *mut bpf_prog);
    fn bpf_mprog_attach(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry,
                        prog: *mut bpf_prog, link: *mut bpf_link, replace: *mut bpf_prog,
                        flags: u32, relative_fd: u32, revision: u64) -> i32;
    fn bpf_mprog_detach(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry,
                        prog: *mut bpf_prog, link: *mut bpf_link, flags: u32,
                        relative_fd: u32, revision: u64) -> i32;
    fn bpf_mprog_commit(entry: *mut bpf_mprog_entry);
    fn bpf_mprog_clear_all(entry: *mut bpf_mprog_entry, entry_new: *mut *mut bpf_mprog_entry);
    fn bpf_mprog_query(attr: *const bpf_attr, uattr: *mut bpf_attr, entry: *mut bpf_mprog_entry) -> i32;
    fn tcx_entry_fetch(dev: *mut net_device, ingress: bool) -> *mut bpf_mprog_entry;
    fn tcx_entry_fetch_or_create(dev: *mut net_device, ingress: bool, created: *mut bool) -> *mut bpf_mprog_entry;
    fn tcx_entry_update(dev: *mut net_device, entry: *mut bpf_mprog_entry, ingress: bool);
    fn tcx_entry_sync();
    fn tcx_entry_free(entry: *mut bpf_mprog_entry);
    fn tcx_entry_is_active(entry: *mut bpf_mprog_entry) -> bool;
    fn tcx_skeys_inc(ingress: bool);
    fn tcx_skeys_dec(ingress: bool);
    fn tcx_link(link: *mut bpf_link) -> *mut tcx_link;
    fn tcx_entry(entry: *mut bpf_mprog_entry) -> *mut tcx_entry_data;
    fn bpf_link_init(link: *mut bpf_link, ty: u32, ops: *const bpf_link_ops,
                     prog: *mut bpf_prog, attach_type: u32);
    fn bpf_link_prime(link: *mut bpf_link, primer: *mut bpf_link_primer) -> i32;
    fn bpf_link_cleanup(primer: *mut bpf_link_primer);
    fn bpf_link_settle(primer: *mut bpf_link_primer) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn seq_printf(seq: *mut seq_file, fmt: *const i8, ...);
}

#[repr(C)] pub struct bpf_attr { pub attach_type: u32, pub target_ifindex: u32, pub attach_flags: u32, pub replace_bpf_fd: i32, pub relative_fd: u32, pub expected_revision: u64, pub query: bpf_attr_query, pub link_create: bpf_attr_link_create }
#[repr(C)] pub struct bpf_attr_query { pub attach_type: u32, pub target_ifindex: u32 }
#[repr(C)] pub struct bpf_attr_link_create { pub attach_type: u32, pub target_ifindex: u32, pub flags: u32, pub tcx: bpf_attr_tcx }
#[repr(C)] pub struct bpf_attr_tcx { pub relative_fd: u32, pub expected_revision: u64 }
#[repr(C)] pub struct bpf_prog { pub ty: u32, pub aux: *mut bpf_prog_aux }
#[repr(C)] pub struct bpf_prog_aux { pub id: u32 }
#[repr(C)] pub struct bpf_link { pub attach_type: u32, pub prog: *mut bpf_prog }
#[repr(C)] pub struct bpf_link_ops { pub release: Option<unsafe extern "C" fn(*mut bpf_link)>, pub detach: Option<unsafe extern "C" fn(*mut bpf_link) -> i32>, pub dealloc: Option<unsafe extern "C" fn(*mut bpf_link)>, pub update_prog: Option<unsafe extern "C" fn(*mut bpf_link, *mut bpf_prog, *mut bpf_prog) -> i32>, pub show_fdinfo: Option<unsafe extern "C" fn(*const bpf_link, *mut seq_file)>, pub fill_link_info: Option<unsafe extern "C" fn(*const bpf_link, *mut bpf_link_info) -> i32> }
#[repr(C)] pub struct bpf_link_primer { _private: [u8; 0] }
#[repr(C)] pub struct bpf_link_info { pub tcx: bpf_link_info_tcx }
#[repr(C)] pub struct bpf_link_info_tcx { pub ifindex: u32, pub attach_type: u32 }
#[repr(C)] pub struct net { pub nsproxy: *mut nsproxy }
#[repr(C)] pub struct nsproxy { pub net_ns: *mut net }
#[repr(C)] pub struct net_device { pub ifindex: u32 }
#[repr(C)] pub struct bpf_mprog_entry { _private: [u8; 0] }
#[repr(C)] pub struct tcx_link { pub link: bpf_link, pub dev: *mut net_device }
#[repr(C)] pub struct tcx_entry_data { pub miniq_active: bool }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }

const BPF_TCX_INGRESS: u32 = 1;
const BPF_F_REPLACE: u32 = 1 << 2;
const BPF_F_ID: u32 = 1 << 4;
const BPF_LINK_TYPE_TCX: u32 = 37;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const ENOENT: i32 = 2;
const ENOLINK: i32 = 67;
const EPERM: i32 = 1;

// The remaining implementation preserves the source operations and delegates
// kernel-specific layout and helpers to the declarations above.

pub unsafe extern "C" fn tcx_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32 {
    let ingress = (*attr).attach_type == BPF_TCX_INGRESS;
    let net = core::ptr::null_mut();
    let mut replace_prog = core::ptr::null_mut();
    let mut ret;
    rtnl_lock();
    let dev = __dev_get_by_index(net, (*attr).target_ifindex);
    if dev.is_null() { ret = -ENODEV; rtnl_unlock(); return ret; }
    if (*attr).attach_flags & BPF_F_REPLACE != 0 { replace_prog = bpf_prog_get_type((*attr).replace_bpf_fd, (*prog).ty); }
    let mut created = false;
    let entry = tcx_entry_fetch_or_create(dev, ingress, &mut created);
    if entry.is_null() { ret = -ENOMEM; } else { let mut new_entry = core::ptr::null_mut(); ret = bpf_mprog_attach(entry, &mut new_entry, prog, core::ptr::null_mut(), replace_prog, (*attr).attach_flags, (*attr).relative_fd, (*attr).expected_revision); if ret == 0 { if entry != new_entry { tcx_entry_update(dev, new_entry, ingress); tcx_entry_sync(); tcx_skeys_inc(ingress); } bpf_mprog_commit(entry); } else if created { tcx_entry_free(entry); } }
    if !replace_prog.is_null() { bpf_prog_put(replace_prog); }
    rtnl_unlock(); ret
}

pub unsafe extern "C" fn tcx_prog_detach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32 {
    let ingress = (*attr).attach_type == BPF_TCX_INGRESS;
    rtnl_lock(); let dev = __dev_get_by_index(core::ptr::null_mut(), (*attr).target_ifindex);
    if dev.is_null() { rtnl_unlock(); return -ENODEV; }
    let entry = tcx_entry_fetch(dev, ingress); if entry.is_null() { rtnl_unlock(); return -ENOENT; }
    let mut new_entry = core::ptr::null_mut();
    let ret = bpf_mprog_detach(entry, &mut new_entry, prog, core::ptr::null_mut(), (*attr).attach_flags, (*attr).relative_fd, (*attr).expected_revision);
    if ret == 0 { if !tcx_entry_is_active(new_entry) { new_entry = core::ptr::null_mut(); } tcx_entry_update(dev, new_entry, ingress); tcx_entry_sync(); tcx_skeys_dec(ingress); bpf_mprog_commit(entry); if new_entry.is_null() { tcx_entry_free(entry); } }
    rtnl_unlock(); ret
}

pub unsafe extern "C" fn tcx_uninstall(dev: *mut net_device, ingress: bool) {
    let entry = tcx_entry_fetch(dev, ingress); if entry.is_null() { return; }
    let active = (*tcx_entry(entry)).miniq_active; let mut new_entry = core::ptr::null_mut();
    if active { bpf_mprog_clear_all(entry, &mut new_entry); }
    tcx_entry_update(dev, new_entry, ingress); tcx_entry_sync();
    if !active { tcx_entry_free(entry); }
}

pub unsafe extern "C" fn tcx_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr) -> i32 {
    rtnl_lock(); let dev = __dev_get_by_index(core::ptr::null_mut(), (*attr).query.target_ifindex);
    let ret = if dev.is_null() { -ENODEV } else { bpf_mprog_query(attr, uattr, tcx_entry_fetch(dev, (*attr).query.attach_type == BPF_TCX_INGRESS)) };
    rtnl_unlock(); ret
}

unsafe extern "C" fn tcx_link_dealloc(link: *mut bpf_link) { kfree(link.cast()); }
unsafe extern "C" fn tcx_link_detach(link: *mut bpf_link) -> i32 { tcx_link_release(link); 0 }
unsafe extern "C" fn tcx_link_release(link: *mut bpf_link) { let _ = link; }
unsafe extern "C" fn tcx_link_update(_: *mut bpf_link, _: *mut bpf_prog, _: *mut bpf_prog) -> i32 { 0 }
unsafe extern "C" fn tcx_link_fdinfo(_: *const bpf_link, _: *mut seq_file) {}
unsafe extern "C" fn tcx_link_fill_info(_: *const bpf_link, _: *mut bpf_link_info) -> i32 { 0 }

pub unsafe extern "C" fn tcx_link_attach(_: *const bpf_attr, _: *mut bpf_prog) -> i32 { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
