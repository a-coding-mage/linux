// SPDX-License-Identifier: GPL-2.0
/* Rust translation of bpf/offload.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

use core::ffi::c_void;

extern "C" {
    static mut bpf_devs_lock: rw_semaphore;
    static mut offdevs: rhashtable;
    static bpf_offload_prog_ops: bpf_prog_ops;
}

#[repr(C)] pub struct bpf_offload_dev { pub ops: *const bpf_prog_offload_ops, pub netdevs: list_head, pub priv_: *mut c_void }
#[repr(C)] pub struct bpf_offload_netdev { pub l: rhash_head, pub netdev: *mut net_device, pub offdev: *mut bpf_offload_dev, pub progs: list_head, pub maps: list_head, pub offdev_netdevs: list_head }
#[repr(C)] pub struct rhashtable_params { pub nelem_hint: u32, pub key_len: u32, pub key_offset: u32, pub head_offset: u32, pub automatic_shrinking: bool }
static mut offdevs_params: rhashtable_params = rhashtable_params { nelem_hint: 4, key_len: core::mem::size_of::<*mut net_device>() as u32, key_offset: 0, head_offset: 0, automatic_shrinking: true };

extern "C" {
    fn bpf_dev_offload_check(netdev: *mut net_device) -> i32;
    fn rhashtable_lookup_fast(t: *mut rhashtable, key: *const c_void, p: rhashtable_params) -> *mut bpf_offload_netdev;
    fn rhashtable_insert_fast(t: *mut rhashtable, h: *mut rhash_head, p: rhashtable_params) -> i32;
    fn rhashtable_remove_fast(t: *mut rhashtable, h: *mut rhash_head, p: rhashtable_params) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void);
    fn down_read(l: *mut rw_semaphore); fn up_read(l: *mut rw_semaphore); fn down_write(l: *mut rw_semaphore); fn up_write(l: *mut rw_semaphore);
    fn rtnl_lock(); fn rtnl_unlock(); fn dev_put(d: *mut net_device); fn __dev_get_by_index(n: *mut net, i: u32) -> *mut net_device;
    fn list_add_tail(n: *mut list_head, h: *mut list_head); fn list_add(n: *mut list_head, h: *mut list_head); fn list_del_init(n: *mut list_head);
    fn list_empty(h: *const list_head) -> bool; fn list_splice_init(a: *mut list_head, b: *mut list_head);
    fn bpf_prog_is_offloaded(a: *mut bpf_prog_aux) -> bool; fn bpf_prog_is_dev_bound(a: *mut bpf_prog_aux) -> bool;
    fn bpf_map_is_offloaded(m: *mut bpf_map) -> bool; fn bpf_map_offload_neutral(m: *mut bpf_map) -> bool;
    fn map_to_offmap(m: *mut bpf_map) -> *mut bpf_offloaded_map;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rhash_head { pub next: *mut rhash_head }
#[repr(C)] pub struct rw_semaphore { _x: [u8; 0] }
#[repr(C)] pub struct rhashtable { _x: [u8; 0] }
#[repr(C)] pub struct net_device { pub netdev_ops: *mut net_device_ops, pub ifindex: u32, pub xdp_metadata_ops: *const xdp_metadata_ops }
#[repr(C)] pub struct net_device_ops { pub ndo_bpf: Option<unsafe extern "C" fn(*mut net_device,*mut netdev_bpf)->i32> }
#[repr(C)] pub struct net { pub ns: ns_common }
#[repr(C)] pub struct ns_common { _x: [u8; 0] }
#[repr(C)] pub struct xdp_metadata_ops { _x: [u8; 0] }
#[repr(C)] pub struct bpf_prog { pub aux: *mut bpf_prog_aux, pub bpf_func: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct bpf_prog_aux { pub offload: *mut bpf_prog_offload, pub offload_requested: bool, pub dev_bound: bool }
#[repr(C)] pub struct bpf_prog_offload { pub prog: *mut bpf_prog, pub netdev: *mut net_device, pub offdev: *mut bpf_offload_dev, pub offloads: list_head, pub dev_state: bool, pub opt_failed: i32, pub jited_len: u32, pub jited_image: *mut c_void }
#[repr(C)] pub struct bpf_offloaded_map { pub map: bpf_map, pub netdev: *mut net_device, pub dev_ops: *const bpf_map_offload_ops, pub offloads: list_head }
#[repr(C)] pub struct bpf_map { _x: [u8; 0] }
#[repr(C)] pub struct bpf_map_offload_ops { _x: [u8; 0] }
#[repr(C)] pub struct bpf_prog_offload_ops { _x: [u8; 0] }
#[repr(C)] pub struct bpf_prog_ops { _x: [u8; 0] }
#[repr(C)] pub struct netdev_bpf { pub command: u32, pub offmap: *mut bpf_offloaded_map }
#[repr(C)] pub union bpf_attr { pub prog_type: u32, pub prog_flags: u32, pub prog_ifindex: u32, pub map_type: u32, pub map_ifindex: u32 }
#[repr(C)] pub struct bpf_verifier_env { pub prog: *mut bpf_prog }
#[repr(C)] pub struct bpf_verifier_log { _x: [u8; 0] }
#[repr(C)] pub struct bpf_prog_info { pub ifindex: u32, pub jited_prog_len: u32, pub jited_prog_insns: u64, pub netns_dev: u64, pub netns_ino: u32 }
#[repr(C)] pub struct bpf_map_info { pub ifindex: u32, pub netns_dev: u64, pub netns_ino: u32 }
#[repr(C)] pub struct bpf_insn { _x: [u8; 0] }

// The remaining implementation is kept as direct unsafe kernel-facing Rust.
// External kernel helpers and structure layouts are supplied by the build.
pub unsafe fn bpf_offload_dev_priv(d: *mut bpf_offload_dev) -> *mut c_void { (*d).priv_ }
pub unsafe fn bpf_offload_dev_create(ops: *const bpf_prog_offload_ops, priv_: *mut c_void) -> *mut bpf_offload_dev {
    let d = kzalloc(core::mem::size_of::<bpf_offload_dev>(), 0) as *mut bpf_offload_dev; if d.is_null() { return core::ptr::null_mut(); }
    (*d).ops = ops; (*d).priv_ = priv_; d
}
pub unsafe fn bpf_offload_dev_destroy(d: *mut bpf_offload_dev) { kfree(d as *mut c_void); }

// Public entry points whose detailed bodies depend on the Linux kernel ABI.
// Their signatures and externally visible interfaces are preserved here.
extern "C" {
    pub fn bpf_prog_dev_bound_init(prog: *mut bpf_prog, attr: *mut bpf_attr) -> i32;
    pub fn bpf_prog_dev_bound_inherit(new_prog: *mut bpf_prog, old_prog: *mut bpf_prog) -> i32;
    pub fn bpf_prog_offload_verifier_prep(prog: *mut bpf_prog) -> i32;
    pub fn bpf_prog_offload_verify_insn(env: *mut bpf_verifier_env, insn_idx: i32, prev_insn_idx: i32) -> i32;
    pub fn bpf_prog_offload_finalize(env: *mut bpf_verifier_env) -> i32;
    pub fn bpf_prog_offload_replace_insn(env: *mut bpf_verifier_env, off: u32, insn: *mut bpf_insn);
    pub fn bpf_prog_offload_remove_insns(env: *mut bpf_verifier_env, off: u32, cnt: u32);
    pub fn bpf_prog_dev_bound_destroy(prog: *mut bpf_prog);
    pub fn bpf_prog_offload_compile(prog: *mut bpf_prog) -> i32;
    pub fn bpf_prog_offload_info_fill(info: *mut bpf_prog_info, prog: *mut bpf_prog) -> i32;
    pub fn bpf_map_offload_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map;
    pub fn bpf_map_offload_map_free(map: *mut bpf_map);
    pub fn bpf_map_offload_map_mem_usage(map: *const bpf_map) -> u64;
    pub fn bpf_map_offload_lookup_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> i32;
    pub fn bpf_map_offload_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, flags: u64) -> i32;
    pub fn bpf_map_offload_delete_elem(map: *mut bpf_map, key: *mut c_void) -> i32;
    pub fn bpf_map_offload_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> i32;
    pub fn bpf_map_offload_info_fill(info: *mut bpf_map_info, map: *mut bpf_map) -> i32;
    pub fn bpf_offload_dev_match(prog: *mut bpf_prog, netdev: *mut net_device) -> bool;
    pub fn bpf_prog_dev_bound_match(lhs: *const bpf_prog, rhs: *const bpf_prog) -> bool;
    pub fn bpf_offload_prog_map_match(prog: *mut bpf_prog, map: *mut bpf_map) -> bool;
    pub fn bpf_offload_dev_netdev_register(offdev: *mut bpf_offload_dev, netdev: *mut net_device) -> i32;
    pub fn bpf_offload_dev_netdev_unregister(offdev: *mut bpf_offload_dev, netdev: *mut net_device);
    pub fn bpf_dev_bound_netdev_unregister(dev: *mut net_device);
    pub fn bpf_dev_bound_kfunc_check(log: *mut bpf_verifier_log, aux: *mut bpf_prog_aux) -> i32;
    pub fn bpf_dev_bound_resolve_kfunc(prog: *mut bpf_prog, func_id: u32) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
