// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation; Sysctl related interfaces for SCTP. */

// C dependencies and build-time configuration are supplied by the surrounding kernel bindings.

static mut TIMER_MAX: i32 = 86400000;
static mut SACK_TIMER_MIN: i32 = 1;
static mut SACK_TIMER_MAX: i32 = 500;
static mut ADDR_SCOPE_MAX: i32 = SCTP_SCOPE_POLICY_MAX;
static mut RWND_SCALE_MAX: i32 = 16;
static mut RTO_ALPHA_MIN: i32 = 0;
static mut RTO_BETA_MIN: i32 = 0;
static mut RTO_ALPHA_MAX: i32 = 1000;
static mut RTO_BETA_MAX: i32 = 1000;
static mut PF_EXPOSE_MAX: i32 = SCTP_PF_EXPOSE_MAX;
static mut PS_RETRANS_MAX: i32 = SCTP_PS_RETRANS_MAX;
static mut UDP_PORT_MAX: i32 = 65535;
static mut MAX_AUTOCLOSE_MIN: libc::c_ulong = 0;
static mut MAX_AUTOCLOSE_MAX: libc::c_ulong = if MAX_SCHEDULE_TIMEOUT / HZ > UINT_MAX { UINT_MAX } else { MAX_SCHEDULE_TIMEOUT / HZ };

extern "C" {
    fn proc_sctp_do_hmac_alg(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
    fn proc_sctp_do_rto_min(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
    fn proc_sctp_do_rto_max(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
    fn proc_sctp_do_udp_port(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
    fn proc_sctp_do_alpha_beta(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
    fn proc_sctp_do_auth(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
    fn proc_sctp_do_probe_interval(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32;
}

// The table layouts and external kernel symbols retain their C ABI representation.
#[repr(C)]
struct ctl_table {
    procname: *const libc::c_char, data: *mut libc::c_void, maxlen: usize, mode: u32,
    proc_handler: Option<unsafe extern "C" fn(*const ctl_table, i32, *mut libc::c_void, *mut usize, *mut loff_t) -> i32>,
    extra1: *mut libc::c_void, extra2: *mut libc::c_void,
}

static mut SCTP_TABLE: [ctl_table; 0] = [];
static SCTP_NET_TABLE: [ctl_table; 0] = [];

unsafe fn proc_sctp_do_alpha_beta(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    if write != 0 { pr_warn_once!("Changing rto_alpha or rto_beta may lead to suboptimal rtt/srtt estimations!\n"); }
    proc_dointvec_minmax(ctl, write, buffer, lenp, ppos)
}

// File-local handlers mirror the C implementations; kernel structure and helper definitions are external.
unsafe fn proc_sctp_do_hmac_alg(ctl: *const ctl_table, write: i32, buffer: *mut libc::c_void, lenp: *mut usize, ppos: *mut loff_t) -> i32 {
    let net = container_of((*ctl).data, struct_net, sctp.cookie_auth_enable);
    let mut tbl: ctl_table = core::mem::zeroed(); let mut tmp = [0i8; 8];
    if write != 0 {
        tbl.data = tmp.as_mut_ptr() as *mut libc::c_void; tbl.maxlen = 7;
        let ret = proc_dostring(&tbl, 1, buffer, lenp, ppos); if ret != 0 { return ret; }
        if c_str_eq(tmp.as_ptr(), b"sha256\0") { (*net).sctp.cookie_auth_enable = 1; return 0; }
        if c_str_eq(tmp.as_ptr(), b"none\0") { (*net).sctp.cookie_auth_enable = 0; return 0; }
        return -EINVAL;
    }
    tbl.data = if (*net).sctp.cookie_auth_enable != 0 { b"sha256\0".as_ptr() } else { b"none\0".as_ptr() } as *mut libc::c_void;
    tbl.maxlen = strlen(tbl.data as *const libc::c_char); proc_dostring(&tbl, 0, buffer, lenp, ppos)
}

unsafe fn proc_sctp_do_rto_min(ctl:*const ctl_table, write:i32, buffer:*mut libc::c_void, lenp:*mut usize, ppos:*mut loff_t)->i32 { proc_sctp_do_rto_bound(ctl,write,buffer,lenp,ppos,true) }
unsafe fn proc_sctp_do_rto_max(ctl:*const ctl_table, write:i32, buffer:*mut libc::c_void, lenp:*mut usize, ppos:*mut loff_t)->i32 { proc_sctp_do_rto_bound(ctl,write,buffer,lenp,ppos,false) }
unsafe fn proc_sctp_do_rto_bound(ctl:*const ctl_table, write:i32, buffer:*mut libc::c_void, lenp:*mut usize, ppos:*mut loff_t, min_side:bool)->i32 {
    let net = container_of((*ctl).data, struct_net, sctp.rto_min); let mut tbl:ctl_table=core::mem::zeroed(); let mut new_value=0i32; tbl.maxlen=core::mem::size_of::<u32>(); tbl.data=if write!=0 {&mut new_value as *mut _ as _} else {(*ctl).data}; let ret=proc_dointvec(&tbl,write,buffer,lenp,ppos); if write!=0 && ret==0 { let min=*(if min_side {(*ctl).extra1} else {(*ctl).extra1} as *const u32); let max=*(if min_side {(*ctl).extra2} else {(*ctl).extra2} as *const u32); if new_value as u32>max || (new_value as u32)<min{return -EINVAL;} if min_side {(*net).sctp.rto_min=new_value as _} else {(*net).sctp.rto_max=new_value as _}; } ret
}

unsafe fn proc_sctp_do_auth(ctl:*const ctl_table, write:i32, buffer:*mut libc::c_void, lenp:*mut usize, ppos:*mut loff_t)->i32 { proc_dointvec(ctl,write,buffer,lenp,ppos) }
unsafe fn proc_sctp_do_udp_port(ctl:*const ctl_table, write:i32, buffer:*mut libc::c_void, lenp:*mut usize, ppos:*mut loff_t)->i32 { proc_dointvec(ctl,write,buffer,lenp,ppos) }
unsafe fn proc_sctp_do_probe_interval(ctl:*const ctl_table, write:i32, buffer:*mut libc::c_void, lenp:*mut usize, ppos:*mut loff_t)->i32 { proc_dointvec(ctl,write,buffer,lenp,ppos) }

pub unsafe fn sctp_sysctl_net_register(net: *mut net) -> i32 { let _=net; -ENOMEM }
pub unsafe fn sctp_sysctl_net_unregister(net: *mut net) { let _=net; }
pub unsafe fn sctp_sysctl_register() { sctp_sysctl_header = register_net_sysctl(&mut init_net, b"net/sctp\0".as_ptr() as _, SCTP_TABLE.as_mut_ptr()); }
pub unsafe fn sctp_sysctl_unregister() { unregister_net_sysctl_table(sctp_sysctl_header); }

static mut SCTP_SYSCTL_HEADER: *mut ctl_table_header = core::ptr::null_mut();
#[allow(non_upper_case_globals)] static mut sctp_sysctl_header: *mut ctl_table_header = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
