/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translation of the C header guard: _UAPI_LINUX_SEG6_GENL_H

pub const SEG6_GENL_NAME: &str = "SEG6";
pub const SEG6_GENL_VERSION: i32 = 0x1;

#[repr(i32)]
pub enum Seg6Attr {
    SEG6_ATTR_UNSPEC = 0,
    SEG6_ATTR_DST,
    SEG6_ATTR_DSTLEN,
    SEG6_ATTR_HMACKEYID,
    SEG6_ATTR_SECRET,
    SEG6_ATTR_SECRETLEN,
    SEG6_ATTR_ALGID,
    SEG6_ATTR_HMACINFO,
    __SEG6_ATTR_MAX,
}

pub const SEG6_ATTR_MAX: i32 = Seg6Attr::__SEG6_ATTR_MAX as i32 - 1;

#[repr(i32)]
pub enum Seg6Cmd {
    SEG6_CMD_UNSPEC = 0,
    SEG6_CMD_SETHMAC,
    SEG6_CMD_DUMPHMAC,
    SEG6_CMD_SET_TUNSRC,
    SEG6_CMD_GET_TUNSRC,
    __SEG6_CMD_MAX,
}

pub const SEG6_CMD_MAX: i32 = Seg6Cmd::__SEG6_CMD_MAX as i32 - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
