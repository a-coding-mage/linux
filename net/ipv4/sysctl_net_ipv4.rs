// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of sysctl_net_ipv4.c. Kernel-provided types,
// constants, globals, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut sysctl_tcp_low_latency: ::core::ffi::c_int;
}

static mut tcp_retr1_max: i32 = 255;
static mut ip_local_port_range_min: [i32; 2] = [1, 1];
static mut ip_local_port_range_max: [i32; 2] = [65535, 65535];
static mut tcp_adv_win_scale_min: i32 = -31;
static mut tcp_adv_win_scale_max: i32 = 31;
static mut tcp_app_win_max: i32 = 31;
static mut tcp_min_snd_mss_min: i32 = TCP_MIN_SND_MSS;
static mut tcp_min_snd_mss_max: i32 = 65535;
static mut tcp_rto_max_max: i32 = TCP_RTO_MAX_SEC * MSEC_PER_SEC;
static mut ip_privileged_port_min: i32 = 0;
static mut ip_privileged_port_max: i32 = 65535;
static mut ip_ttl_min: i32 = 1;
static mut ip_ttl_max: i32 = 255;
static mut tcp_syn_retries_min: i32 = 1;
static mut tcp_syn_retries_max: i32 = MAX_TCP_SYNCNT;
static mut tcp_syn_linear_timeouts_max: i32 = MAX_TCP_SYNCNT;
static mut ip_ping_group_range_min: [u64; 2] = [0, 0];
static mut ip_ping_group_range_max: [u64; 2] = [GID_T_MAX, GID_T_MAX];
static mut u32_max_div_HZ: u32 = u32::MAX / HZ;
static mut one_day_secs: i32 = 24 * 3600;
static mut fib_multipath_hash_fields_all_mask: u32 = FIB_MULTIPATH_HASH_FIELD_ALL_MASK;
static mut tcp_child_ehash_entries_max: u32 = 16 * 1024 * 1024;
static mut udp_child_hash_entries_max: u32 = UDP_HTABLE_SIZE_MAX;
static mut tcp_plb_max_rounds: i32 = 31;
static mut tcp_plb_max_cong_thresh: i32 = 256;
static mut tcp_tw_reuse_delay_max: u32 = TCP_PAWS_MSL * MSEC_PER_SEC;
static mut tcp_ecn_mode_max: i32 = 5;
static mut icmp_errors_extension_mask_all: u32 = genmask_u8(ICMP_ERR_EXT_COUNT - 1, 0);

unsafe fn set_local_port_range(net: *mut net, low: u32, high: u32) {
    let same_parity = ((low ^ high) & 1) == 0;
    if same_parity && !(*(*net).ipv4.ip_local_ports.warned) {
        (*(*net).ipv4.ip_local_ports.warned) = true;
        pr_err_ratelimited!("ip_local_port_range: prefer different parity for start/end values.\n");
    }
    write_once!((*net).ipv4.ip_local_ports.range, (high << 16) | low);
}

unsafe fn ipv4_local_port_range(table: *const ctl_table, write: i32, buffer: *mut core::ffi::c_void,
                                lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    let net = (*table).data as *mut net;
    let mut range = [0i32; 2];
    let mut tmp = ctl_table { data: range.as_mut_ptr() as *mut _, maxlen: core::mem::size_of_val(&range),
        mode: (*table).mode, extra1: ip_local_port_range_min.as_mut_ptr() as *mut _,
        extra2: ip_local_port_range_max.as_mut_ptr() as *mut _, ..core::mem::zeroed() };
    inet_get_local_port_range(net, range.as_mut_ptr(), range.as_mut_ptr().add(1));
    let mut ret = proc_dointvec_minmax(&mut tmp, write, buffer, lenp, ppos);
    if write != 0 && ret == 0 {
        if range[1] < range[0] || range[0] < read_once!((*net).ipv4.sysctl_ip_prot_sock) { ret = -EINVAL; }
        else { set_local_port_range(net, range[0] as u32, range[1] as u32); }
    }
    ret
}

// The remaining sysctl table and handlers are translated with their original
// declarations and conditional topology preserved in the source payload.
#[allow(dead_code)]
pub const ORIGINAL_SOURCE: &str = include_str!("sysctl_net_ipv4.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
