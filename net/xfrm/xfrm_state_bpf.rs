// SPDX-License-Identifier: GPL-2.0-only
/* Unstable XFRM state BPF helpers.
 *
 * Note that it is allowed to break compatibility for these functions since the
 * interface they are exposed through to BPF programs is explicitly unstable.
 */

// Kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct bpf_xfrm_state_opts {
    pub error: i32,
    pub netns_id: i32,
    pub mark: u32,
    pub daddr: xfrm_address_t,
    pub spi: __be32,
    pub proto: u8,
    pub family: u16,
}

pub const BPF_XFRM_STATE_OPTS_SZ: usize = core::mem::size_of::<bpf_xfrm_state_opts>();

extern "C" {
    pub fn dev_net(dev: *mut net_device) -> *mut net;
    pub fn get_net_ns_by_id(net: *mut net, id: i32) -> *mut net;
    pub fn put_net(net: *mut net);
    pub fn xfrm_state_lookup(
        net: *mut net,
        mark: u32,
        daddr: *const xfrm_address_t,
        spi: __be32,
        proto: u8,
        family: u16,
    ) -> *mut xfrm_state;
    pub fn xfrm_state_put(x: *mut xfrm_state);
    pub fn register_btf_kfunc_id_set(
        prog_type: u32,
        set: *const btf_kfunc_id_set,
    ) -> i32;

    pub static mut xfrm_state_kfunc_set: btf_kfunc_set;
    pub static mut THIS_MODULE: *mut module;
}

// The following types and constants are provided by the kernel headers.
#[repr(C)]
pub struct xfrm_address_t {
    pub a: [u32; 4],
}
#[repr(C)]
pub struct xdp_md {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xdp_buff {
    pub rxq: *mut xdp_rxq_info,
}
#[repr(C)]
pub struct xdp_rxq_info {
    pub dev: *mut net_device,
}
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct net { _private: [u8; 0] }
#[repr(C)]
pub struct xfrm_state { _private: [u8; 0] }
#[repr(C)]
pub struct module { _private: [u8; 0] }
#[repr(C)]
pub struct btf_kfunc_id_set { _private: [u8; 0] }
pub type __be32 = u32;

pub const BPF_F_CURRENT_NETNS: i32 = -1;
pub const BPF_PROG_TYPE_XDP: u32 = 0;
pub const EINVAL: i32 = 22;
pub const ENONET: i32 = 64;
pub const ENOENT: i32 = 2;

pub unsafe extern "C" fn bpf_xdp_get_xfrm_state(
    ctx: *mut xdp_md,
    opts: *mut bpf_xfrm_state_opts,
    opts__sz: u32,
) -> *mut xfrm_state {
    let xdp = ctx as *mut xdp_buff;
    let mut net = dev_net((*(*xdp).rxq).dev);
    let x: *mut xfrm_state;

    if (opts__sz as usize) < core::mem::size_of::<i32>() {
        return core::ptr::null_mut();
    }

    if (opts__sz as usize) != BPF_XFRM_STATE_OPTS_SZ {
        (*opts).error = -EINVAL;
        return core::ptr::null_mut();
    }

    if (*opts).netns_id < BPF_F_CURRENT_NETNS {
        (*opts).error = -EINVAL;
        return core::ptr::null_mut();
    }

    if (*opts).netns_id >= 0 {
        net = get_net_ns_by_id(net, (*opts).netns_id);
        if net.is_null() {
            (*opts).error = -ENONET;
            return core::ptr::null_mut();
        }
    }

    x = xfrm_state_lookup(
        net,
        (*opts).mark,
        &(*opts).daddr,
        (*opts).spi,
        (*opts).proto,
        (*opts).family,
    );

    if (*opts).netns_id >= 0 {
        put_net(net);
    }
    if x.is_null() {
        (*opts).error = -ENOENT;
    }

    x
}

pub unsafe extern "C" fn bpf_xdp_xfrm_state_release(x: *mut xfrm_state) {
    xfrm_state_put(x);
}

// BTF_KFUNCS_START(xfrm_state_kfunc_set)
// BTF_ID_FLAGS(func, bpf_xdp_get_xfrm_state, KF_RET_NULL | KF_ACQUIRE)
// BTF_ID_FLAGS(func, bpf_xdp_xfrm_state_release, KF_RELEASE)
// BTF_KFUNCS_END(xfrm_state_kfunc_set)

// static const struct btf_kfunc_id_set xfrm_state_xdp_kfunc_set = {
//     .owner = THIS_MODULE,
//     .set   = &xfrm_state_kfunc_set,
// };

pub unsafe extern "C" fn register_xfrm_state_bpf() -> i32 {
    register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP, core::ptr::addr_of!(xfrm_state_kfunc_set))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
