/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/netfilter.h and linux/netfilter/nf_log.h

/* Log tcp sequence, tcp options, ip options and uid owning local socket */
pub const NF_LOG_DEFAULT_MASK: u32 = 0x0f;

/* This flag indicates that copy_len field in nf_loginfo is set */
pub const NF_LOG_F_COPY_LEN: u32 = 0x1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nf_log_type {
    NF_LOG_TYPE_LOG = 0,
    NF_LOG_TYPE_ULOG,
    NF_LOG_TYPE_MAX,
}

#[repr(C)]
pub union nf_loginfo_u {
    pub ulog: nf_loginfo_ulog,
    pub log: nf_loginfo_log,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_loginfo_ulog {
    /* copy_len will be used iff you set
     * NF_LOG_F_COPY_LEN in flags
     */
    pub copy_len: u32,
    pub group: u16,
    pub qthreshold: u16,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_loginfo_log {
    pub level: u8,
    pub logflags: u8,
}

#[repr(C)]
pub struct nf_loginfo {
    pub type_: u8,
    pub u: nf_loginfo_u,
}

pub type nf_logfn = unsafe extern "C" fn(
    net: *mut net,
    pf: u8,
    hooknum: core::ffi::c_uint,
    skb: *const sk_buff,
    in_: *const net_device,
    out: *const net_device,
    li: *const nf_loginfo,
    prefix: *const core::ffi::c_char,
);

#[repr(C)]
pub struct nf_logger {
    pub name: *mut core::ffi::c_char,
    pub type_: nf_log_type,
    pub logfn: Option<nf_logfn>,
    pub me: *mut module,
}

/* sysctl_nf_log_all_netns - allow LOG target in all network namespaces */
extern "C" {
    pub static mut sysctl_nf_log_all_netns: core::ffi::c_int;

    /* Function to register/unregister log function. */
    pub fn nf_log_register(pf: u8, logger: *mut nf_logger) -> core::ffi::c_int;
    pub fn nf_log_unregister(logger: *mut nf_logger);

    /* Check if any logger is registered for a given protocol family. */
    pub fn nf_log_is_registered(pf: u8) -> bool;

    pub fn nf_log_set(
        net: *mut net,
        pf: u8,
        logger: *const nf_logger,
    ) -> core::ffi::c_int;
    pub fn nf_log_unset(net: *mut net, logger: *const nf_logger);

    pub fn nf_log_bind_pf(
        net: *mut net,
        pf: u8,
        logger: *const nf_logger,
    ) -> core::ffi::c_int;
    pub fn nf_log_unbind_pf(net: *mut net, pf: u8);

    pub fn nf_logger_find_get(pf: core::ffi::c_int, type_: nf_log_type) -> core::ffi::c_int;
    pub fn nf_logger_put(pf: core::ffi::c_int, type_: nf_log_type);

    /* Calls the registered backend logging function */
    pub fn nf_log_packet(
        net: *mut net,
        pf: u8,
        hooknum: core::ffi::c_uint,
        skb: *const sk_buff,
        in_: *const net_device,
        out: *const net_device,
        li: *const nf_loginfo,
        fmt: *const core::ffi::c_char,
        ...,
    );

    pub fn nf_log_trace(
        net: *mut net,
        pf: u8,
        hooknum: core::ffi::c_uint,
        skb: *const sk_buff,
        in_: *const net_device,
        out: *const net_device,
        li: *const nf_loginfo,
        fmt: *const core::ffi::c_char,
        ...,
    );

    pub fn nf_log_buf_open() -> *mut nf_log_buf;
    pub fn nf_log_buf_add(
        m: *mut nf_log_buf,
        f: *const core::ffi::c_char,
        ...,
    ) -> core::ffi::c_int;
    pub fn nf_log_buf_close(m: *mut nf_log_buf);
}

// MODULE_ALIAS_NF_LOGGER(family, type) expands to:
// MODULE_ALIAS("nf-logger-" __stringify(family) "-" __stringify(type))

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
