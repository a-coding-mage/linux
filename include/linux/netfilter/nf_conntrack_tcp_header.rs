/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/linux/netfilter/nf_conntrack_tcp.h>.

#[repr(C)]
pub struct ip_ct_tcp_state {
    pub td_end: u_int32_t,    /* max of seq + len */
    pub td_maxend: u_int32_t, /* max of ack + max(win, 1) */
    pub td_maxwin: u_int32_t, /* max(win) */
    pub td_maxack: u_int32_t, /* max of ack */
    pub td_scale: u_int8_t,   /* window scale factor */
    pub flags: u_int8_t,      /* per direction options */
}

#[repr(C)]
pub struct ip_ct_tcp {
    pub seen: [ip_ct_tcp_state; 2], /* connection parameters per direction */
    pub state: u_int8_t,            /* state of the connection (enum tcp_conntrack) */
    /* For detecting stale connections */
    pub last_dir: u_int8_t, /* Direction of the last packet (enum ip_conntrack_dir) */
    pub retrans: u_int8_t,  /* Number of retransmitted packets */
    pub last_index: u_int8_t, /* Index of the last packet */
    pub last_seq: u_int32_t,    /* Last sequence number seen in dir */
    pub last_ack: u_int32_t,    /* Last sequence number seen in opposite dir */
    pub last_end: u_int32_t,    /* Last seq + len */
    pub last_win: u_int16_t,    /* Last window advertisement seen in dir */
    /* For SYN packets while we may be out-of-sync */
    pub last_wscale: u_int8_t, /* Last window scaling factor seen */
    pub last_flags: u_int8_t,  /* Last flags set */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
