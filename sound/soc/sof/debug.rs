// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// Generic debug routines used to export DSP MMIO and memories to userspace
// for firmware debugging.
//

// C dependencies:
// linux/debugfs.h, linux/io.h, linux/pm_runtime.h,
// sound/sof/ext_manifest.h, sound/sof/debug.h, sof-priv.h, ops.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type mode_t = c_uint;
type u8 = u8;
type u32 = u32;
type bool_ = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const PAGE_SIZE: size_t = 4096;
const SOF_IPC_MSG_MAX_SIZE: size_t = 4096;
const SOF_IPC_GLB_DEBUG: u32 = 0;
const SOF_IPC_DEBUG_MEM_USAGE: u32 = 0;
const SOF_DBG_DUMP_OPTIONAL: u32 = 0;
const SOF_DBG_PRINT_ALL_DUMPS: u32 = 0;
const SOF_DBG_RETAIN_CTX: u32 = 0;
const SOF_DBG_DUMP_REGS: u32 = 0;
const SOF_DBG_DUMP_MBOX: u32 = 0;
const SOF_IPC_TYPE_3: c_int = 3;

static KERN_DEBUG: *const c_char = b"\0".as_ptr() as *const c_char;
static KERN_ERR: *const c_char = b"\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
}

#[repr(C)]
pub enum sof_debugfs_access_type {
    SOF_DEBUGFS_ACCESS_ALWAYS = 0,
    SOF_DEBUGFS_ACCESS_D0_ONLY = 1,
}

#[repr(C)]
pub enum snd_sof_fw_blk_type {
    _SND_SOF_FW_BLK_TYPE = 0,
}

#[repr(C)]
pub enum sof_fw_state {
    SOF_FW_BOOT_NOT_STARTED = 0,
    SOF_DSPLESS_MODE = 1,
    SOF_FW_BOOT_PREPARE = 2,
    SOF_FW_BOOT_IN_PROGRESS = 3,
    SOF_FW_BOOT_FAILED = 4,
    SOF_FW_BOOT_READY_FAILED = 5,
    SOF_FW_BOOT_READY_OK = 6,
    SOF_FW_BOOT_COMPLETE = 7,
    SOF_FW_CRASHED = 8,
}

#[repr(C)]
pub enum snd_sof_dfsentry_type {
    SOF_DFSENTRY_TYPE_IOMEM = 0,
    SOF_DFSENTRY_TYPE_BUF = 1,
}

#[repr(C)]
pub struct snd_sof_dfsentry {
    pub list: list_head,
    pub sdev: *mut snd_sof_dev,
    pub type_: snd_sof_dfsentry_type,
    pub io_mem: *mut u8,
    pub buf: *mut c_void,
    pub cache_buf: *mut u8,
    pub size: size_t,
    pub buf_data_size: size_t,
    pub access_type: sof_debugfs_access_type,
}

#[repr(C)]
pub struct snd_sof_pdata {
    pub fw_filename_prefix: *mut c_char,
    pub fw_lib_prefix: *mut c_char,
    pub tplg_filename_prefix: *mut c_char,
    pub fw_filename: *mut c_char,
    pub tplg_filename: *mut c_char,
    pub ipc_type: c_int,
}

#[repr(C)]
pub struct snd_sof_debugfs_map {
    pub bar: c_int,
    pub offset: u32,
    pub size: size_t,
    pub name: *const c_char,
    pub access_type: sof_debugfs_access_type,
}

#[repr(C)]
pub struct snd_sof_dsp_ops {
    pub debug_map_count: c_int,
    pub debug_map: *const snd_sof_debugfs_map,
    pub dbg_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev, u32)>,
    pub ipc_dump: Option<unsafe extern "C" fn(*mut snd_sof_dev)>,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub debugfs_root: *mut dentry,
    pub dfsentry_list: list_head,
    pub bar: [*mut u8; 8],
    pub pdata: *mut snd_sof_pdata,
    pub ipc: *mut c_void,
    pub fw_state: sof_fw_state,
    pub dbg_dump_printed: bool_,
    pub ipc_dump_printed: bool_,
    pub d3_prevented: bool_,
}

#[repr(C)]
pub struct sof_ipc_cmd_hdr {
    pub size: u32,
    pub cmd: u32,
}

#[repr(C)]
pub struct sof_ipc_reply_hdr_hdr {
    pub size: u32,
}

#[repr(C)]
pub struct sof_ipc_reply_hdr {
    pub hdr: sof_ipc_reply_hdr_hdr,
    pub error: c_int,
}

#[repr(C)]
pub struct sof_ipc_dbg_mem_usage_elem {
    pub zone: u32,
    pub id: u32,
    pub used: u32,
    pub free: u32,
}

#[repr(C)]
pub struct sof_ipc_dbg_mem_usage {
    pub rhdr: sof_ipc_reply_hdr,
    pub num_elems: u32,
    pub elems: [sof_ipc_dbg_mem_usage_elem; 0],
}

#[repr(C)]
struct soc_fw_state_info {
    state: sof_fw_state,
    name: *const c_char,
}

unsafe extern "C" {
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kmalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, count: size_t);
    fn memcpy(dst: *mut c_void, src: *const c_void, count: size_t) -> *mut c_void;
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> size_t;
    fn pm_runtime_active(dev: *mut device) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device);
    fn pm_runtime_get_if_in_use(dev: *mut device) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_printk(level: *const c_char, dev: *mut device, fmt: *const c_char, ...);
    fn debugfs_create_file(
        name: *const c_char,
        mode: mode_t,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_str(
        name: *const c_char,
        mode: mode_t,
        parent: *mut dentry,
        value: *mut *mut c_char,
    ) -> *mut dentry;
    fn debugfs_create_u32(
        name: *const c_char,
        mode: mode_t,
        parent: *mut dentry,
        value: *mut u32,
    ) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn snd_sof_dsp_get_bar_index(sdev: *mut snd_sof_dev, blk_type: snd_sof_fw_blk_type) -> c_int;
    fn sof_ops(sdev: *mut snd_sof_dev) -> *const snd_sof_dsp_ops;
    fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
    fn sof_ipc_tx_message(
        ipc: *mut c_void,
        msg: *mut sof_ipc_cmd_hdr,
        msg_bytes: u32,
        reply: *mut sof_ipc_dbg_mem_usage,
        reply_bytes: size_t,
    ) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn sof_debug_check_flag(flag: u32) -> bool_;
    fn sof_fw_trace_fw_crashed(sdev: *mut snd_sof_dev);
}

#[inline]
fn ALIGN_DOWN(x: loff_t, a: loff_t) -> loff_t {
    x & !(a - 1)
}

#[inline]
fn ALIGN(x: size_t, a: size_t) -> size_t {
    (x + a - 1) & !(a - 1)
}

#[inline]
unsafe fn struct_size_dbg_mem_usage(num_elems: u32) -> size_t {
    size_of::<sof_ipc_dbg_mem_usage>() + size_of::<sof_ipc_dbg_mem_usage_elem>() * num_elems as size_t
}

unsafe extern "C" fn sof_dfsentry_read(
    file: *mut file,
    buffer: *mut c_char,
    mut count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let dfse = (*file).private_data as *mut snd_sof_dfsentry;
    let sdev = (*dfse).sdev;
    let mut pos = *ppos;
    let size_ret: size_t;
    let mut skip: c_int = 0;
    let mut size: c_int;
    let buf: *mut u8;

    size = (*dfse).size as c_int;

    /* validate position & count */
    if pos < 0 {
        return -(EINVAL as ssize_t);
    }
    if pos >= size as loff_t || count == 0 {
        return 0;
    }
    /* find the minimum. min() is not used since it adds sparse warnings */
    if count > (size as loff_t - pos) as size_t {
        count = (size as loff_t - pos) as size_t;
    }

    /* align io read start to u32 multiple */
    pos = ALIGN_DOWN(pos, 4);

    /* intermediate buffer size must be u32 multiple */
    size = ALIGN(count, 4) as c_int;

    /* if start position is unaligned, read extra u32 */
    if pos != *ppos {
        skip = (*ppos - pos) as c_int;
        if pos + size as loff_t + 4 < (*dfse).size as loff_t {
            size += 4;
        }
    }

    buf = kzalloc(size as size_t, GFP_KERNEL) as *mut u8;
    if buf.is_null() {
        return -(ENOMEM as ssize_t);
    }

    if (*dfse).type_ as c_int == snd_sof_dfsentry_type::SOF_DFSENTRY_TYPE_IOMEM as c_int {
        // Original code conditionally uses a debugfs cache when
        // CONFIG_SND_SOC_SOF_DEBUG_ENABLE_DEBUGFS_CACHE is enabled.
        if pm_runtime_active((*sdev).dev) == 0
            && (*dfse).access_type as c_int
                == sof_debugfs_access_type::SOF_DEBUGFS_ACCESS_D0_ONLY as c_int
        {
            dev_err(
                (*sdev).dev,
                b"error: debugfs entry cannot be read in DSP D3\n\0".as_ptr() as *const c_char,
            );
            kfree(buf as *mut c_void);
            return -(EINVAL as ssize_t);
        }

        memcpy_fromio(
            buf as *mut c_void,
            (*dfse).io_mem.offset(pos as isize) as *const c_void,
            size as size_t,
        );
    } else {
        memcpy(
            buf as *mut c_void,
            ((*dfse).buf as *mut u8).offset(pos as isize) as *const c_void,
            size as size_t,
        );
    }

    /* copy to userspace */
    size_ret = copy_to_user(buffer, buf.offset(skip as isize) as *const c_void, count);

    kfree(buf as *mut c_void);

    /* update count & position if copy succeeded */
    if size_ret != 0 {
        return -(EFAULT as ssize_t);
    }

    *ppos = pos + count as loff_t;

    count as ssize_t
}

static sof_dfs_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(sof_dfsentry_read),
    llseek: Some(default_llseek),
};

/* create FS entry for debug files that can expose DSP memories, registers */
unsafe extern "C" fn snd_sof_debugfs_io_item(
    sdev: *mut snd_sof_dev,
    base: *mut c_void,
    size: size_t,
    name: *const c_char,
    access_type: sof_debugfs_access_type,
) -> c_int {
    let dfse: *mut snd_sof_dfsentry;

    if sdev.is_null() {
        return -EINVAL;
    }

    dfse = devm_kzalloc((*sdev).dev, size_of::<snd_sof_dfsentry>(), GFP_KERNEL) as *mut snd_sof_dfsentry;
    if dfse.is_null() {
        return -ENOMEM;
    }

    (*dfse).type_ = snd_sof_dfsentry_type::SOF_DFSENTRY_TYPE_IOMEM;
    (*dfse).io_mem = base as *mut u8;
    (*dfse).size = size;
    (*dfse).sdev = sdev;
    (*dfse).access_type = access_type;

    // Original code allocates dfse->cache_buf here when
    // CONFIG_SND_SOC_SOF_DEBUG_ENABLE_DEBUGFS_CACHE is enabled and access is D0-only.

    debugfs_create_file(name, 0o444, (*sdev).debugfs_root, dfse as *mut c_void, &sof_dfs_fops);

    /* add to dfsentry list */
    list_add(&mut (*dfse).list, &mut (*sdev).dfsentry_list);

    0
}

pub unsafe extern "C" fn snd_sof_debugfs_add_region_item_iomem(
    sdev: *mut snd_sof_dev,
    blk_type: snd_sof_fw_blk_type,
    offset: u32,
    size: size_t,
    name: *const c_char,
    access_type: sof_debugfs_access_type,
) -> c_int {
    let bar = snd_sof_dsp_get_bar_index(sdev, blk_type);

    if bar < 0 {
        return bar;
    }

    snd_sof_debugfs_io_item(
        sdev,
        (*sdev).bar[bar as usize].offset(offset as isize) as *mut c_void,
        size,
        name,
        access_type,
    )
}

/* create FS entry for debug files to expose kernel memory */
pub unsafe extern "C" fn snd_sof_debugfs_buf_item(
    sdev: *mut snd_sof_dev,
    base: *mut c_void,
    size: size_t,
    name: *const c_char,
    mode: mode_t,
) -> c_int {
    let dfse: *mut snd_sof_dfsentry;

    if sdev.is_null() {
        return -EINVAL;
    }

    dfse = devm_kzalloc((*sdev).dev, size_of::<snd_sof_dfsentry>(), GFP_KERNEL) as *mut snd_sof_dfsentry;
    if dfse.is_null() {
        return -ENOMEM;
    }

    (*dfse).type_ = snd_sof_dfsentry_type::SOF_DFSENTRY_TYPE_BUF;
    (*dfse).buf = base;
    (*dfse).size = size;
    (*dfse).sdev = sdev;

    debugfs_create_file(name, mode, (*sdev).debugfs_root, dfse as *mut c_void, &sof_dfs_fops);
    /* add to dfsentry list */
    list_add(&mut (*dfse).list, &mut (*sdev).dfsentry_list);

    0
}

unsafe extern "C" fn memory_info_update(
    sdev: *mut snd_sof_dev,
    buf: *mut c_char,
    buff_size: size_t,
) -> c_int {
    let mut msg = sof_ipc_cmd_hdr {
        size: size_of::<sof_ipc_cmd_hdr>() as u32,
        cmd: SOF_IPC_GLB_DEBUG | SOF_IPC_DEBUG_MEM_USAGE,
    };
    let reply: *mut sof_ipc_dbg_mem_usage;
    let mut len: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    reply = kmalloc(SOF_IPC_MSG_MAX_SIZE, GFP_KERNEL) as *mut sof_ipc_dbg_mem_usage;
    if reply.is_null() {
        return -ENOMEM;
    }

    ret = pm_runtime_resume_and_get((*sdev).dev);
    if ret < 0 && ret != -EACCES {
        dev_err((*sdev).dev, b"error: enabling device failed: %d\n\0".as_ptr() as *const c_char, ret);
        goto_error(reply, ret)
    } else {
        /* Make sure the DSP/firmware is booted up */
        ret = snd_sof_boot_dsp_firmware(sdev);
        if ret == 0 {
            ret = sof_ipc_tx_message(
                (*sdev).ipc,
                &mut msg,
                msg.size,
                reply,
                SOF_IPC_MSG_MAX_SIZE,
            );
        }

        pm_runtime_put_autosuspend((*sdev).dev);
        if ret < 0 || (*reply).rhdr.error < 0 {
            ret = if ret < (*reply).rhdr.error { ret } else { (*reply).rhdr.error };
            dev_err(
                (*sdev).dev,
                b"error: reading memory info failed, %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            goto_error(reply, ret)
        } else if struct_size_dbg_mem_usage((*reply).num_elems) != (*reply).rhdr.hdr.size as size_t {
            dev_err(
                (*sdev).dev,
                b"error: invalid memory info ipc struct size, %d\n\0".as_ptr() as *const c_char,
                (*reply).rhdr.hdr.size,
            );
            ret = -EINVAL;
            goto_error(reply, ret)
        } else {
            i = 0;
            len = 0;
            while i < (*reply).num_elems as c_int {
                let elem = (*reply).elems.as_ptr().offset(i as isize);
                ret = scnprintf(
                    buf.offset(len as isize),
                    buff_size - len as size_t,
                    b"zone %d.%d used %#8x free %#8x\n\0".as_ptr() as *const c_char,
                    (*elem).zone,
                    (*elem).id,
                    (*elem).used,
                    (*elem).free,
                );
                if ret < 0 {
                    return goto_error(reply, ret);
                }
                len += ret;
                i += 1;
            }

            ret = len;
            kfree(reply as *mut c_void);
            ret
        }
    }
}

unsafe fn goto_error(reply: *mut sof_ipc_dbg_mem_usage, ret: c_int) -> c_int {
    kfree(reply as *mut c_void);
    ret
}

unsafe extern "C" fn memory_info_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let dfse = (*file).private_data as *mut snd_sof_dfsentry;
    let sdev = (*dfse).sdev;
    let data_length: c_int;

    /* read memory info from FW only once for each file read */
    if *ppos == 0 {
        (*dfse).buf_data_size = 0;
        data_length = memory_info_update(sdev, (*dfse).buf as *mut c_char, (*dfse).size);
        if data_length < 0 {
            return data_length as ssize_t;
        }
        (*dfse).buf_data_size = data_length as size_t;
    }

    simple_read_from_buffer(to, count, ppos, (*dfse).buf, (*dfse).buf_data_size)
}

unsafe extern "C" fn memory_info_open(inode: *mut inode, file: *mut file) -> c_int {
    let dfse = (*inode).i_private as *mut snd_sof_dfsentry;
    let sdev = (*dfse).sdev;

    (*file).private_data = dfse as *mut c_void;

    /* allocate buffer memory only in first open run, to save memory when unused */
    if (*dfse).buf.is_null() {
        (*dfse).buf = devm_kmalloc((*sdev).dev, PAGE_SIZE, GFP_KERNEL);
        if (*dfse).buf.is_null() {
            return -ENOMEM;
        }
        (*dfse).size = PAGE_SIZE;
    }

    0
}

static memory_info_fops: file_operations = file_operations {
    open: Some(memory_info_open),
    read: Some(memory_info_read),
    llseek: Some(default_llseek),
};

pub unsafe extern "C" fn snd_sof_dbg_memory_info_init(sdev: *mut snd_sof_dev) -> c_int {
    let dfse: *mut snd_sof_dfsentry;

    dfse = devm_kzalloc((*sdev).dev, size_of::<snd_sof_dfsentry>(), GFP_KERNEL) as *mut snd_sof_dfsentry;
    if dfse.is_null() {
        return -ENOMEM;
    }

    /* don't allocate buffer before first usage, to save memory when unused */
    (*dfse).type_ = snd_sof_dfsentry_type::SOF_DFSENTRY_TYPE_BUF;
    (*dfse).sdev = sdev;

    debugfs_create_file(
        b"memory_info\0".as_ptr() as *const c_char,
        0o444,
        (*sdev).debugfs_root,
        dfse as *mut c_void,
        &memory_info_fops,
    );

    /* add to dfsentry list */
    list_add(&mut (*dfse).list, &mut (*sdev).dfsentry_list);
    0
}

pub unsafe extern "C" fn snd_sof_dbg_init(sdev: *mut snd_sof_dev) -> c_int {
    let ops = sof_ops(sdev);
    let plat_data = (*sdev).pdata;
    let mut map: *const snd_sof_debugfs_map;
    let fw_profile: *mut dentry;
    let mut i: c_int;
    let mut err: c_int;

    /* use "sof" as top level debugFS dir */
    (*sdev).debugfs_root = debugfs_create_dir(b"sof\0".as_ptr() as *const c_char, ptr::null_mut());

    /* expose firmware/topology prefix/names for test purposes */
    fw_profile = debugfs_create_dir(b"fw_profile\0".as_ptr() as *const c_char, (*sdev).debugfs_root);

    debugfs_create_str(
        b"fw_path\0".as_ptr() as *const c_char,
        0o444,
        fw_profile,
        &mut (*plat_data).fw_filename_prefix,
    );
    /* library path is not valid for IPC3 */
    if (*plat_data).ipc_type != SOF_IPC_TYPE_3 {
        /*
         * fw_lib_prefix can be NULL if the vendor/platform does not
         * support loadable libraries
         */
        if !(*plat_data).fw_lib_prefix.is_null() {
            debugfs_create_str(
                b"fw_lib_path\0".as_ptr() as *const c_char,
                0o444,
                fw_profile,
                &mut (*plat_data).fw_lib_prefix,
            );
        } else {
            static mut fw_lib_path: *mut c_char = ptr::null_mut();

            fw_lib_path = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                b"Not supported\0".as_ptr() as *const c_char,
            );
            if fw_lib_path.is_null() {
                return -ENOMEM;
            }

            debugfs_create_str(
                b"fw_lib_path\0".as_ptr() as *const c_char,
                0o444,
                fw_profile,
                &mut fw_lib_path,
            );
        }
    }
    debugfs_create_str(
        b"tplg_path\0".as_ptr() as *const c_char,
        0o444,
        fw_profile,
        &mut (*plat_data).tplg_filename_prefix,
    );
    debugfs_create_str(
        b"fw_name\0".as_ptr() as *const c_char,
        0o444,
        fw_profile,
        &mut (*plat_data).fw_filename,
    );
    debugfs_create_str(
        b"tplg_name\0".as_ptr() as *const c_char,
        0o444,
        fw_profile,
        &mut (*plat_data).tplg_filename,
    );
    debugfs_create_u32(
        b"ipc_type\0".as_ptr() as *const c_char,
        0o444,
        fw_profile,
        &mut (*plat_data).ipc_type as *mut c_int as *mut u32,
    );

    /* init dfsentry list */
    INIT_LIST_HEAD(&mut (*sdev).dfsentry_list);

    /* create debugFS files for platform specific MMIO/DSP memories */
    i = 0;
    while i < (*ops).debug_map_count {
        map = (*ops).debug_map.offset(i as isize);

        err = snd_sof_debugfs_io_item(
            sdev,
            (*sdev).bar[(*map).bar as usize].offset((*map).offset as isize) as *mut c_void,
            (*map).size,
            (*map).name,
            (*map).access_type,
        );
        /* errors are only due to memory allocation, not debugfs */
        if err < 0 {
            return err;
        }
        i += 1;
    }

    snd_sof_debugfs_buf_item(
        sdev,
        &mut (*sdev).fw_state as *mut sof_fw_state as *mut c_void,
        size_of::<sof_fw_state>(),
        b"fw_state\0".as_ptr() as *const c_char,
        0o444,
    )
}

pub unsafe extern "C" fn snd_sof_free_debug(sdev: *mut snd_sof_dev) {
    debugfs_remove_recursive((*sdev).debugfs_root);
}

static fw_state_dbg: [soc_fw_state_info; 9] = [
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_NOT_STARTED,
        name: b"SOF_FW_BOOT_NOT_STARTED\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_DSPLESS_MODE,
        name: b"SOF_DSPLESS_MODE\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_PREPARE,
        name: b"SOF_FW_BOOT_PREPARE\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_IN_PROGRESS,
        name: b"SOF_FW_BOOT_IN_PROGRESS\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_FAILED,
        name: b"SOF_FW_BOOT_FAILED\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_READY_FAILED,
        name: b"SOF_FW_BOOT_READY_FAILED\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_READY_OK,
        name: b"SOF_FW_BOOT_READY_OK\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_BOOT_COMPLETE,
        name: b"SOF_FW_BOOT_COMPLETE\0".as_ptr() as *const c_char,
    },
    soc_fw_state_info {
        state: sof_fw_state::SOF_FW_CRASHED,
        name: b"SOF_FW_CRASHED\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn snd_sof_dbg_print_fw_state(sdev: *mut snd_sof_dev, level: *const c_char) {
    let mut i: c_int;

    i = 0;
    while (i as usize) < fw_state_dbg.len() {
        if (*sdev).fw_state as c_int == fw_state_dbg[i as usize].state as c_int {
            dev_printk(
                level,
                (*sdev).dev,
                b"fw_state: %s (%d)\n\0".as_ptr() as *const c_char,
                fw_state_dbg[i as usize].name,
                i,
            );
            return;
        }
        i += 1;
    }

    dev_printk(
        level,
        (*sdev).dev,
        b"fw_state: UNKNOWN (%d)\n\0".as_ptr() as *const c_char,
        (*sdev).fw_state as c_int,
    );
}

pub unsafe extern "C" fn snd_sof_dsp_dbg_dump(
    sdev: *mut snd_sof_dev,
    msg: *const c_char,
    flags: u32,
) {
    let level = if flags & SOF_DBG_DUMP_OPTIONAL != 0 {
        KERN_DEBUG
    } else {
        KERN_ERR
    };
    let print_all = sof_debug_check_flag(SOF_DBG_PRINT_ALL_DUMPS);
    let ops = sof_ops(sdev);

    if flags & SOF_DBG_DUMP_OPTIONAL != 0 && !print_all {
        return;
    }

    if (*ops).dbg_dump.is_some() && !(*sdev).dbg_dump_printed {
        dev_printk(
            level,
            (*sdev).dev,
            b"------------[ DSP dump start ]------------\n\0".as_ptr() as *const c_char,
        );
        if !msg.is_null() {
            dev_printk(level, (*sdev).dev, b"%s\n\0".as_ptr() as *const c_char, msg);
        }
        snd_sof_dbg_print_fw_state(sdev, level);
        ((*ops).dbg_dump.unwrap())(sdev, flags);
        dev_printk(
            level,
            (*sdev).dev,
            b"------------[ DSP dump end ]------------\n\0".as_ptr() as *const c_char,
        );
        if !print_all {
            (*sdev).dbg_dump_printed = true;
        }
    } else if !msg.is_null() {
        dev_printk(level, (*sdev).dev, b"%s\n\0".as_ptr() as *const c_char, msg);
    }
}

unsafe extern "C" fn snd_sof_ipc_dump(sdev: *mut snd_sof_dev) {
    let ops = sof_ops(sdev);

    if (*ops).ipc_dump.is_some() && !(*sdev).ipc_dump_printed {
        dev_err(
            (*sdev).dev,
            b"------------[ IPC dump start ]------------\n\0".as_ptr() as *const c_char,
        );
        ((*ops).ipc_dump.unwrap())(sdev);
        dev_err(
            (*sdev).dev,
            b"------------[ IPC dump end ]------------\n\0".as_ptr() as *const c_char,
        );
        if !sof_debug_check_flag(SOF_DBG_PRINT_ALL_DUMPS) {
            (*sdev).ipc_dump_printed = true;
        }
    }
}

pub unsafe extern "C" fn snd_sof_handle_fw_exception(sdev: *mut snd_sof_dev, msg: *const c_char) {
    // Original code also tests IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_RETAIN_DSP_CONTEXT).
    if sof_debug_check_flag(SOF_DBG_RETAIN_CTX) && !(*sdev).d3_prevented {
        /* should we prevent DSP entering D3 ? */
        if !(*sdev).ipc_dump_printed {
            dev_info(
                (*sdev).dev,
                b"Attempting to prevent DSP from entering D3 state to preserve context\n\0".as_ptr()
                    as *const c_char,
            );
        }

        if pm_runtime_get_if_in_use((*sdev).dev) == 1 {
            (*sdev).d3_prevented = true;
        }
    }

    /* dump vital information to the logs */
    snd_sof_ipc_dump(sdev);
    snd_sof_dsp_dbg_dump(sdev, msg, SOF_DBG_DUMP_REGS | SOF_DBG_DUMP_MBOX);
    sof_fw_trace_fw_crashed(sdev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
