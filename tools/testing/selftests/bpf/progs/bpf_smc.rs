// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_tracing_net.h"

#[used]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub const SMC_HS_CTRL_NAME_MAX: usize = 16;

pub const BPF_SMC_LISTEN: i32 = 10;

#[repr(C)]
pub struct smc_sock___local {
    pub sk: sock,
    pub listen_smc: *mut smc_sock,
    pub use_fallback: bool,
}

#[repr(C)]
pub struct smc_hs_ctrl___local {
    pub name: [::core::ffi::c_char; SMC_HS_CTRL_NAME_MAX],
    pub syn_option: Option<unsafe extern "C" fn(*mut tcp_sock) -> i32>,
    pub synack_option:
        Option<unsafe extern "C" fn(*const tcp_sock, *mut inet_request_sock) -> i32>,
}

#[repr(C)]
pub struct netns_smc___local {
    pub hs_ctrl: *mut smc_hs_ctrl___local,
}

#[repr(C)]
pub struct net___local {
    pub smc: netns_smc___local,
}

pub static mut smc_cnt: i32 = 0;
pub static mut fallback_cnt: i32 = 0;

// SEC("fentry/smc_release")
#[no_mangle]
pub unsafe extern "C" fn bpf_smc_release(sock: *mut socket) -> i32 {
    /* only count from one side (client) */
    if (*(*sock).sk).__sk_common.skc_state == BPF_SMC_LISTEN as _ {
        return 0;
    }
    smc_cnt += 1;
    0
}

// SEC("fentry/smc_switch_to_fallback")
#[no_mangle]
pub unsafe extern "C" fn bpf_smc_switch_to_fallback(smc: *mut smc_sock___local) -> i32 {
    /* only count from one side (client) */
    if !smc.is_null() && (*smc).listen_smc.is_null() {
        fallback_cnt += 1;
    }
    0
}

/* go with default value if no strat was found */
pub static mut default_ip_strat_value: bool = true;

#[repr(C)]
pub struct smc_policy_ip_key {
    pub sip: __u32,
    pub dip: __u32,
}

#[repr(C)]
pub struct smc_policy_ip_value {
    pub mode: __u8,
}

#[repr(C)]
pub struct smc_policy_ip_map {
    // Original C map metadata:
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __uint(key_size, sizeof(struct smc_policy_ip_key));
    // __uint(value_size, sizeof(struct smc_policy_ip_value));
    // __uint(max_entries, 128);
    // __uint(map_flags, BPF_F_NO_PREALLOC);
    pub _private: [u8; 0],
}

// SEC(".maps")
#[no_mangle]
#[link_section = ".maps"]
pub static mut smc_policy_ip: smc_policy_ip_map = smc_policy_ip_map { _private: [] };

unsafe fn smc_check(src: __u32, dst: __u32) -> bool {
    let mut key: smc_policy_ip_key = smc_policy_ip_key { sip: src, dip: dst };
    let value: *mut smc_policy_ip_value = bpf_map_lookup_elem(
        &raw mut smc_policy_ip as *mut _ as *mut ::core::ffi::c_void,
        &mut key as *mut _ as *mut ::core::ffi::c_void,
    ) as *mut smc_policy_ip_value;

    if !value.is_null() {
        (*value).mode != 0
    } else {
        default_ip_strat_value
    }
}

// SEC("fmod_ret/update_socket_protocol")
#[no_mangle]
pub unsafe extern "C" fn smc_run(family: i32, type_: i32, protocol: i32) -> i32 {
    let task: *mut task_struct;

    if family != AF_INET && family != AF_INET6 {
        return protocol;
    }

    if (type_ & 0xf) != SOCK_STREAM {
        return protocol;
    }

    if protocol != 0 && protocol != IPPROTO_TCP {
        return protocol;
    }

    task = bpf_get_current_task_btf() as *mut task_struct;
    /* Prevent from affecting other tests */
    if task.is_null() {
        return protocol;
    } else {
        let net: *mut net___local = (*(*task).nsproxy).net_ns as *mut net___local;

        // Original C condition also used:
        // !bpf_core_field_exists(struct net___local, smc)
        if !bpf_core_field_exists_net___local_smc() || (*net).smc.hs_ctrl.is_null() {
            return protocol;
        }
    }

    IPPROTO_SMC
}

// SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_smc_set_tcp_option_cond(
    _tp: *const tcp_sock,
    ireq: *mut inet_request_sock,
) -> i32 {
    smc_check(
        (*ireq).req.__req_common.skc_daddr,
        (*ireq).req.__req_common.skc_rcv_saddr,
    ) as i32
}

// SEC("struct_ops")
#[no_mangle]
pub unsafe extern "C" fn bpf_smc_set_tcp_option(tp: *mut tcp_sock) -> i32 {
    smc_check(
        (*tp).inet_conn.icsk_inet.sk.__sk_common.skc_rcv_saddr,
        (*tp).inet_conn.icsk_inet.sk.__sk_common.skc_daddr,
    ) as i32
}

// SEC(".struct_ops")
#[no_mangle]
#[link_section = ".struct_ops"]
pub static mut linkcheck: smc_hs_ctrl___local = smc_hs_ctrl___local {
    name: [
        b'l' as ::core::ffi::c_char,
        b'i' as ::core::ffi::c_char,
        b'n' as ::core::ffi::c_char,
        b'k' as ::core::ffi::c_char,
        b'c' as ::core::ffi::c_char,
        b'h' as ::core::ffi::c_char,
        b'e' as ::core::ffi::c_char,
        b'c' as ::core::ffi::c_char,
        b'k' as ::core::ffi::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    syn_option: Some(bpf_smc_set_tcp_option),
    synack_option: Some(bpf_smc_set_tcp_option_cond),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
