/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* C header dependency: <linux/types.h> */

#[allow(non_camel_case_types)]
pub type __u32 = u32;
#[allow(non_camel_case_types)]
pub type __u64 = u64;
#[allow(non_camel_case_types)]
pub type __s32 = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fastrpc_map_flags {
    FASTRPC_MAP_STATIC = 0,
    FASTRPC_MAP_RESERVED,
    FASTRPC_MAP_FD = 2,
    FASTRPC_MAP_FD_DELAYED,
    FASTRPC_MAP_FD_NOMAP = 16,
    FASTRPC_MAP_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fastrpc_proc_attr {
    FASTRPC_MODE_DEBUG = 1 << 0,
    FASTRPC_MODE_PTRACE = 1 << 1,
    FASTRPC_MODE_CRC = 1 << 2,
    FASTRPC_MODE_UNSIGNED_MODULE = 1 << 3,
    FASTRPC_MODE_ADAPTIVE_QOS = 1 << 4,
    FASTRPC_MODE_SYSTEM_PROCESS = 1 << 5,
    FASTRPC_MODE_PRIVILEGED = 1 << 6,
}

pub const FASTRPC_ATTR_SECUREMAP: __u32 = 1;
pub const FASTRPC_POLL_MODE: __u32 = 1;
pub const FASTRPC_POLL_MODE_DISABLE: __u32 = 0;
pub const FASTRPC_POLL_MODE_ENABLE: __u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_invoke_args { pub ptr: __u64, pub length: __u64, pub fd: __s32, pub attr: __u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_invoke { pub handle: __u32, pub sc: __u32, pub args: __u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_init_create { pub filelen: __u32, pub filefd: __s32, pub attrs: __u32, pub siglen: __u32, pub file: __u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_init_create_static { pub namelen: __u32, pub memlen: __u32, pub name: __u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_alloc_dma_buf { pub fd: __s32, pub flags: __u32, pub size: __u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_req_mmap { pub fd: __s32, pub flags: __u32, pub vaddrin: __u64, pub size: __u64, pub vaddrout: __u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_mem_map { pub version: __s32, pub fd: __s32, pub offset: __s32, pub flags: __u32, pub vaddrin: __u64, pub length: __u64, pub vaddrout: __u64, pub attrs: __s32, pub reserved: [__s32; 4] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_req_munmap { pub vaddrout: __u64, pub size: __u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_mem_unmap { pub vesion: __s32, pub fd: __s32, pub vaddr: __u64, pub length: __u64, pub reserved: [__s32; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_ioctl_set_option { pub request_id: __u32, pub value: __u32, pub reserved: [__s32; 6] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastrpc_ioctl_capability { pub unused: __u32, pub attribute_id: __u32, pub capability: __u32, pub reserved: [__u32; 4] }

/* _IO/_IOWR values are supplied by the target Linux ioctl ABI. */
pub const FASTRPC_IOCTL_INIT_ATTACH: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_INIT_ATTACH_SNS: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_ALLOC_DMA_BUFF: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_FREE_DMA_BUFF: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_INVOKE: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_INIT_CREATE: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_MMAP: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_MUNMAP: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_INIT_CREATE_STATIC: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_MEM_MAP: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_MEM_UNMAP: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_SET_OPTION: libc::c_ulong = 0;
pub const FASTRPC_IOCTL_GET_DSP_INFO: libc::c_ulong = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
