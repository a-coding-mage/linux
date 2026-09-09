/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2020 Intel Corporation
 */

/*
 * Extended manifest is a place to store metadata about firmware, known during
 * compilation time - for example firmware version or used compiler.
 * Given information are read on host side before firmware startup.
 * This part of output binary is not signed.
 */

/* In ASCII `XMan` */
pub const SOF_EXT_MAN_MAGIC_NUMBER: u32 = 0x6e61_4d58;

/* Build u32 number in format MMmmmppp */
#[macro_export]
macro_rules! SOF_EXT_MAN_BUILD_VERSION {
    ($major:expr, $minor:expr, $path:expr) => {
        ((($major as u32) << 24) | (($minor as u32) << 12) | ($path as u32))
    };
}

/* check extended manifest version consistency */
#[macro_export]
macro_rules! SOF_EXT_MAN_VERSION_INCOMPATIBLE {
    ($host_ver:expr, $cli_ver:expr) => {
        (($host_ver as u32) & 0xff00_0000) != (($cli_ver as u32) & 0xff00_0000)
    };
}

/* used extended manifest header version */
pub const SOF_EXT_MAN_VERSION: u32 = SOF_EXT_MAN_BUILD_VERSION!(1, 0, 0);

/* extended manifest header, deleting any field breaks backward compatibility */
#[repr(C, packed)]
pub struct sof_ext_man_header {
    pub magic: u32,       /*< identification number, EXT_MAN_MAGIC_NUMBER */
    pub full_size: u32,   /*< [bytes] full size of ext_man, (header + content + padding) */
    pub header_size: u32, /*< [bytes] makes header extensionable */
    pub header_version: u32, /*< value of EXT_MAN_VERSION */
    /* just after this header should be list of ext_man_elem_* elements */
}

/* Now define extended manifest elements */

/* Extended manifest elements types */
#[repr(u32)]
pub enum sof_ext_man_elem_type {
    SOF_EXT_MAN_ELEM_FW_VERSION = 0,
    SOF_EXT_MAN_ELEM_WINDOW = 1,
    SOF_EXT_MAN_ELEM_CC_VERSION = 2,
    SOF_EXT_MAN_ELEM_PROBE_INFO = 3,
    SOF_EXT_MAN_ELEM_DBG_ABI = 4,
    SOF_EXT_MAN_ELEM_CONFIG_DATA = 5, /*< ABI3.17 */
    SOF_EXT_MAN_ELEM_PLATFORM_CONFIG_DATA = 6,
}

/* extended manifest element header */
#[repr(C, packed)]
pub struct sof_ext_man_elem_header {
    pub type_: u32, /*< SOF_EXT_MAN_ELEM_ */
    pub size: u32,  /*< in bytes, including header size */
    /* just after this header should be type dependent content */
}

/* FW version */
#[repr(C, packed)]
pub struct sof_ext_man_fw_version {
    pub hdr: sof_ext_man_elem_header,
    /* use sof_ipc struct because of code re-use */
    pub version: sof_ipc_fw_version,
    pub flags: u32,
}

/* extended data memory windows for IPC, trace and debug */
#[repr(C, packed)]
pub struct sof_ext_man_window {
    pub hdr: sof_ext_man_elem_header,
    /* use sof_ipc struct because of code re-use */
    pub ipc_window: sof_ipc_window,
}

/* Used C compiler description */
#[repr(C, packed)]
pub struct sof_ext_man_cc_version {
    pub hdr: sof_ext_man_elem_header,
    /* use sof_ipc struct because of code re-use */
    pub cc_version: sof_ipc_cc_version,
}

#[repr(C, packed)]
pub struct ext_man_dbg_abi {
    pub hdr: sof_ext_man_elem_header,
    /* use sof_ipc struct because of code re-use */
    pub dbg_abi: sof_ipc_user_abi_version,
}

/* EXT_MAN_ELEM_CONFIG_DATA elements identificators, ABI3.17 */
#[repr(u32)]
pub enum config_elem_type {
    SOF_EXT_MAN_CONFIG_EMPTY = 0,
    SOF_EXT_MAN_CONFIG_IPC_MSG_SIZE = 1,
    SOF_EXT_MAN_CONFIG_MEMORY_USAGE_SCAN = 2, /*< ABI 3.18 */
}

#[repr(C, packed)]
pub struct sof_config_elem {
    pub token: u32,
    pub value: u32,
}

/* firmware configuration information */
#[repr(C, packed)]
pub struct sof_ext_man_config_data {
    pub hdr: sof_ext_man_elem_header,
    pub elems: [sof_config_elem; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
