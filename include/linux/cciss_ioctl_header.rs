/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations from <uapi/linux/cciss_ioctl.h> are supplied externally. */

/* 32 bit compatible ioctl structs */
#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct IOCTL32_Command_struct {
    pub LUN_info: LUNAddr_struct,
    pub Request: RequestBlock_struct,
    pub error_info: ErrorInfo_struct,
    pub buf_size: WORD, /* size in bytes of the buf */
    pub buf: __u32, /* 32 bit pointer to data buffer */
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
pub struct BIG_IOCTL32_Command_struct {
    pub LUN_info: LUNAddr_struct,
    pub Request: RequestBlock_struct,
    pub error_info: ErrorInfo_struct,
    pub malloc_size: DWORD, /* < MAX_KMALLOC_SIZE in cciss.c */
    pub buf_size: DWORD, /* size in bytes of the buf */
    /* < malloc_size * MAXSGENTRIES */
    pub buf: __u32, /* 32 bit pointer to data buffer */
}

/* _IOWR and CCISS_IOC_MAGIC are supplied by the uapi header. */
#[cfg(CONFIG_COMPAT)]
macro_rules! CCISS_PASSTHRU32 {
    () => { _IOWR!(CCISS_IOC_MAGIC, 11, IOCTL32_Command_struct) };
}

#[cfg(CONFIG_COMPAT)]
macro_rules! CCISS_BIG_PASSTHRU32 {
    () => { _IOWR!(CCISS_IOC_MAGIC, 18, BIG_IOCTL32_Command_struct) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
