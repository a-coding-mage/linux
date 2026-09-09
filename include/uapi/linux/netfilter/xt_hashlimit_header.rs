/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency equivalents supplied by the surrounding Linux UAPI bindings:
// linux/types.h, linux/limits.h, and linux/if.h.

/* timings are in milliseconds. */
pub const XT_HASHLIMIT_SCALE: u32 = 10000;
pub const XT_HASHLIMIT_SCALE_v2: u64 = 1000000u64;
/* 1/10,000 sec period => max of 10,000/sec.  Min rate is then 429490
 * seconds, or one packet every 59 hours.
 */

/* packet length accounting is done in 16-byte steps */
pub const XT_HASHLIMIT_BYTE_SHIFT: u32 = 4;

/* details of this structure hidden by the implementation */
#[repr(C)]
pub struct xt_hashlimit_htable {
    _private: [u8; 0],
}

pub const XT_HASHLIMIT_HASH_DIP: u32 = 1 << 0;
pub const XT_HASHLIMIT_HASH_DPT: u32 = 1 << 1;
pub const XT_HASHLIMIT_HASH_SIP: u32 = 1 << 2;
pub const XT_HASHLIMIT_HASH_SPT: u32 = 1 << 3;
pub const XT_HASHLIMIT_INVERT: u32 = 1 << 4;
pub const XT_HASHLIMIT_BYTES: u32 = 1 << 5;
pub const XT_HASHLIMIT_RATE_MATCH: u32 = 1 << 6;

#[repr(C)]
pub struct hashlimit_cfg {
    pub mode: u32,      /* bitmask of XT_HASHLIMIT_HASH_* */
    pub avg: u32,       /* Average secs between packets * scale */
    pub burst: u32,     /* Period multiplier for upper limit. */

    /* user specified */
    pub size: u32,      /* how many buckets */
    pub max: u32,       /* max number of entries */
    pub gc_interval: u32, /* gc interval */
    pub expire: u32,    /* when do entries expire? */
}

#[repr(C)]
pub struct xt_hashlimit_info {
    pub name: [::core::ffi::c_char; IFNAMSIZ], /* name */
    pub cfg: hashlimit_cfg,

    /* Used internally by the kernel */
    pub hinfo: *mut xt_hashlimit_htable,
    pub u: xt_hashlimit_info__u,
}

#[repr(C)]
pub union xt_hashlimit_info__u {
    pub ptr: *mut ::core::ffi::c_void,
    pub master: *mut xt_hashlimit_info,
}

#[repr(C)]
pub struct hashlimit_cfg1 {
    pub mode: u32,      /* bitmask of XT_HASHLIMIT_HASH_* */
    pub avg: u32,       /* Average secs between packets * scale */
    pub burst: u32,     /* Period multiplier for upper limit. */

    /* user specified */
    pub size: u32,      /* how many buckets */
    pub max: u32,       /* max number of entries */
    pub gc_interval: u32, /* gc interval */
    pub expire: u32,    /* when do entries expire? */

    pub srcmask: u8,
    pub dstmask: u8,
}

#[repr(C)]
pub struct hashlimit_cfg2 {
    pub avg: u64,       /* Average secs between packets * scale */
    pub burst: u64,     /* Period multiplier for upper limit. */
    pub mode: u32,      /* bitmask of XT_HASHLIMIT_HASH_* */

    /* user specified */
    pub size: u32,      /* how many buckets */
    pub max: u32,       /* max number of entries */
    pub gc_interval: u32, /* gc interval */
    pub expire: u32,    /* when do entries expire? */

    pub srcmask: u8,
    pub dstmask: u8,
}

#[repr(C)]
pub struct hashlimit_cfg3 {
    pub avg: u64,       /* Average secs between packets * scale */
    pub burst: u64,     /* Period multiplier for upper limit. */
    pub mode: u32,      /* bitmask of XT_HASHLIMIT_HASH_* */

    /* user specified */
    pub size: u32,      /* how many buckets */
    pub max: u32,       /* max number of entries */
    pub gc_interval: u32, /* gc interval */
    pub expire: u32,    /* when do entries expire? */

    pub interval: u32,
    pub srcmask: u8,
    pub dstmask: u8,
}

#[repr(C)]
pub struct xt_hashlimit_mtinfo1 {
    pub name: [::core::ffi::c_char; IFNAMSIZ],
    pub cfg: hashlimit_cfg1,

    /* Used internally by the kernel */
    pub hinfo: *mut xt_hashlimit_htable, // __attribute__((aligned(8)))
}

#[repr(C)]
pub struct xt_hashlimit_mtinfo2 {
    pub name: [::core::ffi::c_char; NAME_MAX],
    pub cfg: hashlimit_cfg2,

    /* Used internally by the kernel */
    pub hinfo: *mut xt_hashlimit_htable, // __attribute__((aligned(8)))
}

#[repr(C)]
pub struct xt_hashlimit_mtinfo3 {
    pub name: [::core::ffi::c_char; NAME_MAX],
    pub cfg: hashlimit_cfg3,

    /* Used internally by the kernel */
    pub hinfo: *mut xt_hashlimit_htable, // __attribute__((aligned(8)))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
