/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000, 2001, 2002 Broadcom Corporation
 */

/* C header guard: CFE_API_INT_H */

/*
 * Constants.
 */
pub const CFE_CMD_FW_GETINFO: u64 = 0;
pub const CFE_CMD_FW_RESTART: u64 = 1;
pub const CFE_CMD_FW_BOOT: u64 = 2;
pub const CFE_CMD_FW_CPUCTL: u64 = 3;
pub const CFE_CMD_FW_GETTIME: u64 = 4;
pub const CFE_CMD_FW_MEMENUM: u64 = 5;
pub const CFE_CMD_FW_FLUSHCACHE: u64 = 6;

pub const CFE_CMD_DEV_GETHANDLE: u64 = 9;
pub const CFE_CMD_DEV_ENUM: u64 = 10;
pub const CFE_CMD_DEV_OPEN: u64 = 11;
pub const CFE_CMD_DEV_INPSTAT: u64 = 12;
pub const CFE_CMD_DEV_READ: u64 = 13;
pub const CFE_CMD_DEV_WRITE: u64 = 14;
pub const CFE_CMD_DEV_IOCTL: u64 = 15;
pub const CFE_CMD_DEV_CLOSE: u64 = 16;
pub const CFE_CMD_DEV_GETINFO: u64 = 17;

pub const CFE_CMD_ENV_ENUM: u64 = 20;
pub const CFE_CMD_ENV_GET: u64 = 22;
pub const CFE_CMD_ENV_SET: u64 = 23;
pub const CFE_CMD_ENV_DEL: u64 = 24;

pub const CFE_CMD_MAX: u64 = 32;

pub const CFE_CMD_VENDOR_USE: u64 = 0x8000; /* codes above this are for customer use */

/*
 * Structures.
 */

/* eeek, signed "pointers" */
pub type cfe_xptr_t = i64;

#[repr(C)]
pub struct xiocb_buffer {
    pub buf_offset: u64,   /* offset on device (bytes) */
    pub buf_ptr: cfe_xptr_t, /* pointer to a buffer */
    pub buf_length: u64,   /* length of this buffer */
    pub buf_retlen: u64,   /* returned length (for read ops) */
    pub buf_ioctlcmd: u64, /* IOCTL command (used only for IOCTLs) */
}

#[repr(C)]
pub struct xiocb_inpstat {
    pub inp_status: u64, /* 1 means input available */
}

#[repr(C)]
pub struct xiocb_envbuf {
    pub enum_idx: i64,     /* 0-based enumeration index */
    pub name_ptr: cfe_xptr_t, /* name string buffer */
    pub name_length: i64,  /* size of name buffer */
    pub val_ptr: cfe_xptr_t, /* value string buffer */
    pub val_length: i64,   /* size of value string buffer */
}

#[repr(C)]
pub struct xiocb_cpuctl {
    pub cpu_number: u64, /* cpu number to control */
    pub cpu_command: u64, /* command to issue to CPU */
    pub start_addr: u64, /* CPU start address */
    pub gp_val: u64,     /* starting GP value */
    pub sp_val: u64,     /* starting SP value */
    pub a1_val: u64,     /* starting A1 value */
}

#[repr(C)]
pub struct xiocb_time {
    pub ticks: i64, /* current time in ticks */
}

#[repr(C)]
pub struct xiocb_exitstat {
    pub status: i64,
}

#[repr(C)]
pub struct xiocb_meminfo {
    pub mi_idx: i64,  /* 0-based enumeration index */
    pub mi_type: i64, /* type of memory block */
    pub mi_addr: u64, /* physical start address */
    pub mi_size: u64, /* block size */
}

#[repr(C)]
pub struct xiocb_fwinfo {
    pub fwi_version: i64,     /* major, minor, eco version */
    pub fwi_totalmem: i64,   /* total installed mem */
    pub fwi_flags: i64,      /* various flags */
    pub fwi_boardid: i64,    /* board ID */
    pub fwi_bootarea_va: i64, /* VA of boot area */
    pub fwi_bootarea_pa: i64, /* PA of boot area */
    pub fwi_bootarea_size: i64, /* size of boot area */
    pub fwi_reserved1: i64,
    pub fwi_reserved2: i64,
    pub fwi_reserved3: i64,
}

#[repr(C)]
pub union cfe_xiocb_plist {
    /* buffer parameters */
    pub xiocb_buffer: xiocb_buffer,
    /* input status parameters */
    pub xiocb_inpstat: xiocb_inpstat,
    /* environment function parameters */
    pub xiocb_envbuf: xiocb_envbuf,
    /* CPU control parameters */
    pub xiocb_cpuctl: xiocb_cpuctl,
    /* timer parameters */
    pub xiocb_time: xiocb_time,
    /* memory arena info parameters */
    pub xiocb_meminfo: xiocb_meminfo,
    /* firmware information */
    pub xiocb_fwinfo: xiocb_fwinfo,
    /* Exit Status */
    pub xiocb_exitstat: xiocb_exitstat,
}

#[repr(C)]
pub struct cfe_xiocb {
    pub xiocb_fcode: u64, /* IOCB function code */
    pub xiocb_status: i64, /* return status */
    pub xiocb_handle: i64, /* file/device handle */
    pub xiocb_flags: u64, /* flags for this IOCB */
    pub xiocb_psize: u64, /* size of parameter list */
    pub plist: cfe_xiocb_plist,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
