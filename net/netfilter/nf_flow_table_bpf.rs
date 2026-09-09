// SPDX-License-Identifier: GPL-2.0-only
/* Unstable Flow Table Helpers for XDP hook
 *
 * These are called from the XDP programs.
 * Note that it is allowed to break compatibility for these functions since
 * the interface they are exposed through to BPF programs is explicitly
 * unstable.
 */

// External Linux kernel definitions supplied by the surrounding build.

#[repr(C)]
pub struct bpf_flowtable_opts {
    pub error: i32,
}

pub const NF_BPF_FLOWTABLE_OPTS_SZ: u32 = 4;

// The following types and functions are supplied by the kernel networking and
// BPF interfaces.
#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct flow_offload_tuple {
    pub iifidx: u32,
    pub l3proto: u16,
    pub l4proto: u8,
    pub src_port: u16,
    pub dst_port: u16,
    pub src_v4: in_addr,
    pub dst_v4: in_addr,
    pub src_v6: in6_addr,
    pub dst_v6: in6_addr,
}
#[repr(C)]
pub struct flow_offload_tuple_rhash {
    pub tuple: flow_offload_tuple,
    pub node: [u8; 0],
}
#[repr(C)]
pub struct flow_offload {
    pub tuplehash: [flow_offload_tuple_rhash; 2],
}
#[repr(C)]
pub struct nf_flowtable {
    _private: [u8; 0],
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
pub struct bpf_fib_lookup {
    pub family: u8,
    pub l4_protocol: u8,
    pub ifindex: u32,
    pub sport: u16,
    pub dport: u16,
    pub ipv4_src: u32,
    pub ipv4_dst: u32,
    pub ipv6_src: [u32; 4],
    pub ipv6_dst: [u32; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in_addr {
    pub s_addr: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr32: [u32; 4],
}

extern "C" {
    fn nf_flowtable_by_dev(dev: *mut net_device) -> *mut nf_flowtable;
    fn flow_offload_lookup(table: *mut nf_flowtable, tuple: *mut flow_offload_tuple)
        -> *mut flow_offload_tuple_rhash;
    fn flow_offload_refresh(table: *mut nf_flowtable, flow: *mut flow_offload, tcp: bool);
    fn htons(value: u16) -> u16;
    fn register_btf_kfunc_id_set(prog_type: u32, set: *const btf_kfunc_id_set) -> i32;
}

#[repr(C)]
pub struct btf_kfunc_id_set {
    pub owner: *mut core::ffi::c_void,
    pub set: *const core::ffi::c_void,
}

const ENOENT: i32 = 2;
const EINVAL: i32 = 22;
const EAFNOSUPPORT: i32 = 97;
const AF_INET: u8 = 2;
const AF_INET6: u8 = 10;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;
const BPF_PROG_TYPE_XDP: u32 = 6;

#[inline]
unsafe fn err_ptr(error: i32) -> *mut flow_offload_tuple_rhash {
    error as isize as *mut flow_offload_tuple_rhash
}

unsafe fn bpf_xdp_flow_tuple_lookup(
    dev: *mut net_device,
    tuple: *mut flow_offload_tuple,
    _proto: u16,
) -> *mut flow_offload_tuple_rhash {
    let nf_flow_table = nf_flowtable_by_dev(dev);
    if nf_flow_table.is_null() {
        return err_ptr(-ENOENT);
    }

    let tuplehash = flow_offload_lookup(nf_flow_table, tuple);
    if tuplehash.is_null() {
        return err_ptr(-ENOENT);
    }

    let nf_flow = tuplehash as *mut flow_offload;
    flow_offload_refresh(nf_flow_table, nf_flow, false);

    tuplehash
}

pub unsafe fn bpf_xdp_flow_lookup(
    ctx: *mut xdp_md,
    fib_tuple: *mut bpf_fib_lookup,
    opts: *mut bpf_flowtable_opts,
    opts_len: u32,
) -> *mut flow_offload_tuple_rhash {
    let xdp = ctx as *mut xdp_buff;
    let fib = &*fib_tuple;
    let mut tuple = flow_offload_tuple {
        iifidx: fib.ifindex,
        l3proto: fib.family as u16,
        l4proto: fib.l4_protocol,
        src_port: fib.sport,
        dst_port: fib.dport,
        src_v4: in_addr { s_addr: 0 },
        dst_v4: in_addr { s_addr: 0 },
        src_v6: in6_addr { s6_addr32: [0; 4] },
        dst_v6: in6_addr { s6_addr32: [0; 4] },
    };
    let proto: u16;

    if opts_len != NF_BPF_FLOWTABLE_OPTS_SZ {
        (*opts).error = -EINVAL;
        return core::ptr::null_mut();
    }

    match fib.family {
        AF_INET => {
            tuple.src_v4.s_addr = fib.ipv4_src;
            tuple.dst_v4.s_addr = fib.ipv4_dst;
            proto = htons(ETH_P_IP);
        }
        AF_INET6 => {
            tuple.src_v6 = in6_addr { s6_addr32: fib.ipv6_src };
            tuple.dst_v6 = in6_addr { s6_addr32: fib.ipv6_dst };
            proto = htons(ETH_P_IPV6);
        }
        _ => {
            (*opts).error = -EAFNOSUPPORT;
            return core::ptr::null_mut();
        }
    }

    let tuplehash = bpf_xdp_flow_tuple_lookup((*(*xdp).rxq).dev, &mut tuple, proto);
    if (tuplehash as isize) < 0 && (tuplehash as isize) >= -4095 {
        (*opts).error = tuplehash as isize as i32;
        return core::ptr::null_mut();
    }

    tuplehash
}

// BTF kfunc set declarations and registration, corresponding to the kernel
// BTF_KFUNCS_START/BTF_ID_FLAGS/BTF_KFUNCS_END definitions.
pub static mut nf_ft_kfunc_set: [u8; 0] = [];

pub static nf_flow_kfunc_set: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: core::ptr::null_mut(),
    set: core::ptr::null(),
};

pub unsafe fn nf_flow_register_bpf() -> i32 {
    register_btf_kfunc_id_set(BPF_PROG_TYPE_XDP, &nf_flow_kfunc_set)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
