// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation

// C dependencies: linux/debugfs.h, linux/sched/signal.h, linux/sched/clock.h,
// sound/sof/ipc4/header.h, sof-priv.h, ipc4-priv.h

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = u32;
type u64 = u64;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type bool_ = bool;

const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const GFP_KERNEL: c_uint = 0;
const TASK_INTERRUPTIBLE: c_int = 1;
const MAX_SCHEDULE_TIMEOUT: i64 = i64::MAX;
const NSEC_PER_USEC: u64 = 1000;

const FW_EPOCH_DELTA: i64 = 11644473600;

const MAX_ALLOWED_LIBRARIES: usize = 16;

const SOF_IPC4_INVALID_SLOT_OFFSET: u32 = 0xffffffff;

/* for debug and critical types */
const SOF_MTRACE_SLOT_CORE_MASK: u32 = 0xff;
const SOF_MTRACE_SLOT_TYPE_MASK: u32 = 0xffffff00;

const DEFAULT_AGING_TIMER_PERIOD_MS: u32 = 0x100;
const DEFAULT_FIFO_FULL_TIMER_PERIOD_MS: u32 = 0x1000;

/* ipc4 log level and source definitions for logs_priorities_mask */
const SOF_MTRACE_LOG_LEVEL_CRITICAL: u32 = 1 << 0;
const SOF_MTRACE_LOG_LEVEL_ERROR: u32 = 1 << 1;
const SOF_MTRACE_LOG_LEVEL_WARNING: u32 = 1 << 2;
const SOF_MTRACE_LOG_LEVEL_INFO: u32 = 1 << 3;
const SOF_MTRACE_LOG_LEVEL_VERBOSE: u32 = 1 << 4;
const SOF_MTRACE_LOG_SOURCE_INFRA: u32 = 1 << 5; /* log source 0 */
const SOF_MTRACE_LOG_SOURCE_HAL: u32 = 1 << 6;
const SOF_MTRACE_LOG_SOURCE_MODULE: u32 = 1 << 7;
const SOF_MTRACE_LOG_SOURCE_AUDIO: u32 = 1 << 8;
const SOF_MTRACE_LOG_SOURCE_SCHEDULER: u32 = 1 << 9;
const SOF_MTRACE_LOG_SOURCE_ULP_INFRA: u32 = 1 << 10;
const SOF_MTRACE_LOG_SOURCE_ULP_MODULE: u32 = 1 << 11;
const SOF_MTRACE_LOG_SOURCE_VISION: u32 = 1 << 12; /* log source 7 */
const DEFAULT_LOGS_PRIORITIES_MASK: u32 = SOF_MTRACE_LOG_LEVEL_CRITICAL
    | SOF_MTRACE_LOG_LEVEL_ERROR
    | SOF_MTRACE_LOG_LEVEL_WARNING
    | SOF_MTRACE_LOG_LEVEL_INFO
    | SOF_MTRACE_LOG_SOURCE_INFRA
    | SOF_MTRACE_LOG_SOURCE_HAL
    | SOF_MTRACE_LOG_SOURCE_MODULE
    | SOF_MTRACE_LOG_SOURCE_AUDIO;

extern "C" {
    static current: *mut task_struct;
    static THIS_MODULE: *mut module;

    fn debugfs_file_get(dentry: *mut dentry) -> c_int;
    fn debugfs_file_put(dentry: *mut dentry);
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn init_waitqueue_entry(wait: *mut wait_queue_entry_t, task: *mut task_struct);
    fn set_current_state(state: c_int);
    fn add_wait_queue(head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn signal_pending(task: *mut task_struct) -> c_int;
    fn schedule_timeout(timeout: i64) -> i64;
    fn remove_wait_queue(head: *mut wait_queue_head_t, wait: *mut wait_queue_entry_t);
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> c_int;
    fn sof_debug_check_flag(flag: c_int) -> bool_;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn sof_mailbox_read(sdev: *mut snd_sof_dev, offset: u32, dest: *mut c_void, bytes: size_t);
    fn sof_mailbox_write(sdev: *mut snd_sof_dev, offset: u32, src: *const c_void, bytes: size_t);
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn simple_read_from_buffer(
        to: *mut c_char,
        count: size_t,
        ppos: *mut loff_t,
        from: *const c_void,
        available: size_t,
    ) -> ssize_t;
    fn memdup_user_nul(src: *const c_char, len: size_t) -> *mut c_char;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_u32(name: *const c_char, mode: c_uint, parent: *mut dentry, value: *mut u32);
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn local_clock() -> u64;
    fn div64_u64(dividend: u64, divisor: u64) -> u64;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn init_waitqueue_head(head: *mut wait_queue_head_t);
    fn mutex_init(mutex: *mut mutex);
    fn wake_up(head: *mut wait_queue_head_t);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
}

#[repr(C)]
pub struct inode {
    pub i_private: *mut c_void,
}

#[repr(C)]
pub struct path {
    pub dentry: *mut dentry,
}

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
    pub f_path: path,
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_entry_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pm_message_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct debug_box {
    pub offset: u32,
}

#[repr(C)]
pub struct snd_sof_dev {
    pub fw_trace_data: *mut c_void,
    pub debugfs_root: *mut dentry,
    pub ipc: *mut sof_ipc,
    pub private: *mut c_void,
    pub fw_trace_is_supported: bool_,
    pub dev: *mut device,
    pub num_cores: c_int,
    pub debug_box: debug_box,
}

#[repr(C)]
pub struct sof_ipc {
    pub ops: *const sof_ipc_ops,
}

#[repr(C)]
pub struct sof_ipc_ops {
    pub set_get_data:
        Option<unsafe extern "C" fn(*mut snd_sof_dev, *mut sof_ipc4_msg, size_t, bool_) -> c_int>,
}

#[repr(C)]
pub struct sof_ipc4_msg {
    pub primary: u32,
    pub extension: u32,
    pub data_size: size_t,
    pub data_ptr: *mut c_void,
}

#[repr(C)]
pub struct sof_ipc4_fw_data {
    pub mtrace_log_bytes: u32,
    pub mtrace_type: u32,
}

#[repr(C, packed)]
pub struct sof_log_state_info {
    pub aging_timer_period: u32,
    pub fifo_full_timer_period: u32,
    pub enable: u32,
    pub logs_priorities_mask: [u32; MAX_ALLOWED_LIBRARIES],
}

#[repr(C)]
pub enum sof_mtrace_state {
    SOF_MTRACE_DISABLED,
    SOF_MTRACE_INITIALIZING,
    SOF_MTRACE_ENABLED,
}

#[repr(C)]
pub struct sof_mtrace_core_data {
    pub sdev: *mut snd_sof_dev,

    pub id: c_int,
    pub slot_offset: u32,
    pub log_buffer: *mut c_void,
    pub buffer_lock: mutex, /* for log_buffer alloc/free */
    pub host_read_ptr: u32,
    pub dsp_write_ptr: u32,
    /* pos update IPC arrived before the slot offset is known, queried */
    pub delayed_pos_update: bool_,
    pub trace_sleep: wait_queue_head_t,
}

#[repr(C)]
pub struct sof_mtrace_priv {
    pub sdev: *mut snd_sof_dev,
    pub mtrace_state: sof_mtrace_state,
    pub state_info: sof_log_state_info,

    pub cores: [sof_mtrace_core_data; 0],
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub owner: *mut module,
}

extern "C" {
    static SOF_IPC4_DEBUG_SLOT_SIZE: u32;
    static SOF_IPC4_DEBUG_DESCRIPTOR_SIZE: u32;
    static SOF_IPC4_MAX_DEBUG_SLOTS: c_int;
    static SOF_IPC4_DEBUG_SLOT_DEBUG_LOG: u32;
    static SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS: c_int;
    static SOF_IPC4_MODULE_MSG: u32;
    static SOF_IPC4_MSG_REQUEST: u32;
    static SOF_IPC4_MOD_INIT_BASEFW_MOD_ID: u32;
    static SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID: u32;
    static SOF_IPC4_FW_PARAM_SYSTEM_TIME: u32;
    static SOF_IPC4_FW_PARAM_ENABLE_LOGS: u32;
    static SOF_IPC4_MTRACE_INTEL_CAVS_2: u32;

    fn SOF_IPC4_MSG_TARGET(x: u32) -> u32;
    fn SOF_IPC4_MSG_DIR(x: u32) -> u32;
    fn SOF_IPC4_MOD_ID(x: u32) -> u32;
    fn SOF_IPC4_MOD_INSTANCE(x: u32) -> u32;
    fn SOF_IPC4_MOD_EXT_MSG_PARAM_ID(x: u32) -> u32;
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool {
    ptr.is_null() || IS_ERR(ptr)
}

unsafe fn PTR_ERR(ptr: *const c_void) -> c_int {
    ptr as isize as c_int
}

unsafe fn cores_ptr(priv_: *mut sof_mtrace_priv, index: c_int) -> *mut sof_mtrace_core_data {
    (*priv_).cores.as_mut_ptr().add(index as usize)
}

unsafe extern "C" fn sof_ipc4_mtrace_dfs_open(inode: *mut inode, file: *mut file) -> c_int {
    let core_data = (*inode).i_private as *mut sof_mtrace_core_data;
    let mut ret: c_int;

    mutex_lock(ptr::addr_of_mut!((*core_data).buffer_lock));

    if !(*core_data).log_buffer.is_null() {
        mutex_unlock(ptr::addr_of_mut!((*core_data).buffer_lock));
        return -EBUSY;
    }

    ret = debugfs_file_get((*file).f_path.dentry);
    if ret != 0 {
        mutex_unlock(ptr::addr_of_mut!((*core_data).buffer_lock));
        return ret;
    }

    (*core_data).log_buffer = kmalloc(SOF_IPC4_DEBUG_SLOT_SIZE as size_t, GFP_KERNEL);
    if (*core_data).log_buffer.is_null() {
        debugfs_file_put((*file).f_path.dentry);
        mutex_unlock(ptr::addr_of_mut!((*core_data).buffer_lock));
        return -ENOMEM;
    }

    ret = simple_open(inode, file);
    if ret != 0 {
        kfree((*core_data).log_buffer);
        debugfs_file_put((*file).f_path.dentry);
    }

    mutex_unlock(ptr::addr_of_mut!((*core_data).buffer_lock));
    ret
}

unsafe fn sof_wait_mtrace_avail(core_data: *mut sof_mtrace_core_data) -> bool_ {
    let mut wait = core::mem::MaybeUninit::<wait_queue_entry_t>::uninit();

    /* data immediately available */
    if (*core_data).host_read_ptr != (*core_data).dsp_write_ptr {
        return true;
    }

    /* wait for available trace data from FW */
    init_waitqueue_entry(wait.as_mut_ptr(), current);
    set_current_state(TASK_INTERRUPTIBLE);
    add_wait_queue(ptr::addr_of_mut!((*core_data).trace_sleep), wait.as_mut_ptr());

    if signal_pending(current) == 0 {
        /* set timeout to max value, no error code */
        schedule_timeout(MAX_SCHEDULE_TIMEOUT);
    }
    remove_wait_queue(ptr::addr_of_mut!((*core_data).trace_sleep), wait.as_mut_ptr());

    if (*core_data).host_read_ptr != (*core_data).dsp_write_ptr {
        return true;
    }

    false
}

unsafe extern "C" fn sof_ipc4_mtrace_dfs_read(
    file: *mut file,
    buffer: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let core_data = (*file).private_data as *mut sof_mtrace_core_data;
    let mut log_buffer_offset: u32;
    let log_buffer_size: u32;
    let mut read_ptr: u32;
    let write_ptr: u32;
    let sdev = (*core_data).sdev;
    let priv_ = (*sdev).fw_trace_data as *mut sof_mtrace_priv;
    let log_buffer = (*core_data).log_buffer;
    let lpos = *ppos;
    let mut avail: u32;
    let mut ret: c_int;

    /* check pos and count */
    if lpos < 0 {
        return -EINVAL as ssize_t;
    }
    if count == 0 || count < size_of::<u32>() {
        return 0;
    }

    /* get available count based on current host offset */
    if !sof_wait_mtrace_avail(core_data) {
        /* No data available */
        avail = 0;
        if copy_to_user(buffer, ptr::addr_of!(avail) as *const c_void, size_of::<u32>()) != 0 {
            return -EFAULT as ssize_t;
        }

        return 0;
    }

    if (*core_data).slot_offset == SOF_IPC4_INVALID_SLOT_OFFSET {
        return 0;
    }

    /* The log data buffer starts after the two pointer in the slot */
    log_buffer_offset = (*core_data).slot_offset + (size_of::<u32>() as u32 * 2);
    /* The log data size excludes the pointers */
    log_buffer_size = SOF_IPC4_DEBUG_SLOT_SIZE - (size_of::<u32>() as u32 * 2);

    read_ptr = (*core_data).host_read_ptr;
    write_ptr = (*core_data).dsp_write_ptr;

    if read_ptr < write_ptr {
        avail = write_ptr - read_ptr;
    } else {
        avail = log_buffer_size - read_ptr + write_ptr;
    }

    if avail == 0 {
        return 0;
    }

    if avail > log_buffer_size {
        avail = log_buffer_size;
    }

    /* Need space for the initial u32 of the avail */
    if avail as size_t > count - size_of::<u32>() {
        avail = (count - size_of::<u32>()) as u32;
    }

    if sof_debug_check_flag(SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS) {
        dev_dbg(
            (*sdev).dev,
            b"core%d, host read: %#x, dsp write: %#x, avail: %#x\n\0".as_ptr() as *const c_char,
            (*core_data).id,
            read_ptr,
            write_ptr,
            avail,
        );
    }

    if read_ptr < write_ptr {
        /* Read data between read pointer and write pointer */
        sof_mailbox_read(sdev, log_buffer_offset + read_ptr, log_buffer, avail as size_t);
    } else {
        /* read from read pointer to end of the slot */
        sof_mailbox_read(
            sdev,
            log_buffer_offset + read_ptr,
            log_buffer,
            (avail - write_ptr) as size_t,
        );
        /* read from slot start to write pointer */
        if write_ptr != 0 {
            sof_mailbox_read(
                sdev,
                log_buffer_offset,
                (log_buffer as *mut u8).add((avail - write_ptr) as usize) as *mut c_void,
                write_ptr as size_t,
            );
        }
    }

    /* first write the number of bytes we have gathered */
    ret = copy_to_user(buffer, ptr::addr_of!(avail) as *const c_void, size_of::<u32>());
    if ret != 0 {
        return -EFAULT as ssize_t;
    }

    /* Followed by the data itself */
    ret = copy_to_user(buffer.add(size_of::<u32>()), log_buffer, avail as size_t);
    if ret != 0 {
        return -EFAULT as ssize_t;
    }

    /* Update the host_read_ptr in the slot for this core */
    read_ptr += avail;
    if read_ptr >= log_buffer_size {
        read_ptr -= log_buffer_size;
    }
    sof_mailbox_write(
        sdev,
        (*core_data).slot_offset,
        ptr::addr_of!(read_ptr) as *const c_void,
        size_of::<u32>(),
    );

    /* Only update the host_read_ptr if mtrace is enabled */
    if (*priv_).mtrace_state as c_int != sof_mtrace_state::SOF_MTRACE_DISABLED as c_int {
        (*core_data).host_read_ptr = read_ptr;
    }

    /*
     * Ask for a new buffer from user space for the next chunk, not
     * streaming due to the heading number of bytes value.
     */
    *ppos += count as loff_t;

    count as ssize_t
}

unsafe extern "C" fn sof_ipc4_mtrace_dfs_release(inode: *mut inode, file: *mut file) -> c_int {
    let core_data = (*inode).i_private as *mut sof_mtrace_core_data;

    debugfs_file_put((*file).f_path.dentry);

    mutex_lock(ptr::addr_of_mut!((*core_data).buffer_lock));
    kfree((*core_data).log_buffer);
    (*core_data).log_buffer = ptr::null_mut();
    mutex_unlock(ptr::addr_of_mut!((*core_data).buffer_lock));

    0
}

static sof_dfs_mtrace_fops: file_operations = file_operations {
    open: Some(sof_ipc4_mtrace_dfs_open),
    read: Some(sof_ipc4_mtrace_dfs_read),
    write: None,
    llseek: Some(default_llseek),
    release: Some(sof_ipc4_mtrace_dfs_release),

    owner: unsafe { THIS_MODULE },
};

unsafe extern "C" fn sof_ipc4_priority_mask_dfs_read(
    file: *mut file,
    to: *mut c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let priv_ = (*file).private_data as *mut sof_mtrace_priv;
    let mut i: c_int;
    let ret: ssize_t;
    let mut offset: c_int;
    let mut remaining: c_int;
    let buf: *mut c_char;

    /*
     * one entry (14 char + new line = 15):
     * " 0: 000001ef"
     *
     * 16 * 15 + 1 = 241
     */
    buf = kzalloc(241, GFP_KERNEL);
    if buf.is_null() {
        return -ENOMEM as ssize_t;
    }

    i = 0;
    while i < MAX_ALLOWED_LIBRARIES as c_int {
        offset = strlen(buf) as c_int;
        remaining = 241 - offset;
        snprintf(
            buf.add(offset as usize),
            remaining as size_t,
            b"%2d: 0x%08x\n\0".as_ptr() as *const c_char,
            i,
            (*priv_).state_info.logs_priorities_mask[i as usize],
        );
        i += 1;
    }

    ret = simple_read_from_buffer(to, count, ppos, buf as *const c_void, strlen(buf));

    kfree(buf as *mut c_void);
    ret
}

unsafe extern "C" fn sof_ipc4_priority_mask_dfs_write(
    file: *mut file,
    from: *const c_char,
    count: size_t,
    _ppos: *mut loff_t,
) -> ssize_t {
    let priv_ = (*file).private_data as *mut sof_mtrace_priv;
    let mut id: c_uint = 0;
    let buf: *mut c_char;
    let mut mask: u32 = 0;
    let mut ret: c_int;

    /*
     * To update Nth mask entry, write:
     * "N,0x1234" or "N,1234" to the debugfs file
     * The mask will be interpreted as hexadecimal number
     */
    buf = memdup_user_nul(from, count);
    if IS_ERR(buf as *const c_void) {
        return PTR_ERR(buf as *const c_void) as ssize_t;
    }

    ret = sscanf(
        buf,
        b"%u,0x%x\0".as_ptr() as *const c_char,
        ptr::addr_of_mut!(id),
        ptr::addr_of_mut!(mask),
    );
    if ret != 2 {
        ret = sscanf(
            buf,
            b"%u,%x\0".as_ptr() as *const c_char,
            ptr::addr_of_mut!(id),
            ptr::addr_of_mut!(mask),
        );
        if ret != 2 {
            ret = -EINVAL;
            kfree(buf as *mut c_void);
            return ret as ssize_t;
        }
    }

    if id >= MAX_ALLOWED_LIBRARIES as c_uint {
        ret = -EINVAL;
        kfree(buf as *mut c_void);
        return ret as ssize_t;
    }

    (*priv_).state_info.logs_priorities_mask[id as usize] = mask;
    ret = count as c_int;

    kfree(buf as *mut c_void);
    ret as ssize_t
}

static sof_dfs_priority_mask_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(sof_ipc4_priority_mask_dfs_read),
    write: Some(sof_ipc4_priority_mask_dfs_write),
    llseek: Some(default_llseek),
    release: None,

    owner: unsafe { THIS_MODULE },
};

unsafe fn mtrace_debugfs_create(sdev: *mut snd_sof_dev) -> c_int {
    let priv_ = (*sdev).fw_trace_data as *mut sof_mtrace_priv;
    let dfs_root: *mut dentry;
    let mut dfs_name = [0 as c_char; 100];
    let mut i: c_int;

    dfs_root = debugfs_create_dir(b"mtrace\0".as_ptr() as *const c_char, (*sdev).debugfs_root);
    if IS_ERR_OR_NULL(dfs_root as *const c_void) {
        return 0;
    }

    /* Create files for the logging parameters */
    debugfs_create_u32(
        b"aging_timer_period\0".as_ptr() as *const c_char,
        0o644,
        dfs_root,
        ptr::addr_of_mut!((*priv_).state_info.aging_timer_period),
    );
    debugfs_create_u32(
        b"fifo_full_timer_period\0".as_ptr() as *const c_char,
        0o644,
        dfs_root,
        ptr::addr_of_mut!((*priv_).state_info.fifo_full_timer_period),
    );
    debugfs_create_file(
        b"logs_priorities_mask\0".as_ptr() as *const c_char,
        0o644,
        dfs_root,
        priv_ as *mut c_void,
        ptr::addr_of!(sof_dfs_priority_mask_fops),
    );

    /* Separate log files per core */
    i = 0;
    while i < (*sdev).num_cores {
        snprintf(
            dfs_name.as_mut_ptr(),
            dfs_name.len(),
            b"core%d\0".as_ptr() as *const c_char,
            i,
        );
        debugfs_create_file(
            dfs_name.as_ptr(),
            0o444,
            dfs_root,
            cores_ptr(priv_, i) as *mut c_void,
            ptr::addr_of!(sof_dfs_mtrace_fops),
        );
        i += 1;
    }

    0
}

unsafe fn ipc4_mtrace_enable(sdev: *mut snd_sof_dev) -> c_int {
    let priv_ = (*sdev).fw_trace_data as *mut sof_mtrace_priv;
    let iops = (*(*sdev).ipc).ops;
    let mut msg = core::mem::MaybeUninit::<sof_ipc4_msg>::zeroed().assume_init();
    let mut system_time: u64;
    let mut ret: c_int;

    if (*priv_).mtrace_state as c_int != sof_mtrace_state::SOF_MTRACE_DISABLED as c_int {
        return 0;
    }

    msg.primary = SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MOD_ID(SOF_IPC4_MOD_INIT_BASEFW_MOD_ID);
    msg.primary |= SOF_IPC4_MOD_INSTANCE(SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID);
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_FW_PARAM_SYSTEM_TIME);

    /*
     * local_clock() is used to align with dmesg, so both kernel and firmware logs have
     * the same base and a minor delta due to the IPC. system time is in us format but
     * local_clock() returns the time in ns, so convert to ns.
     */
    system_time = div64_u64(local_clock(), NSEC_PER_USEC);
    msg.data_size = size_of::<u64>();
    msg.data_ptr = ptr::addr_of_mut!(system_time) as *mut c_void;
    ret = ((*iops).set_get_data.unwrap())(sdev, ptr::addr_of_mut!(msg), msg.data_size, true);
    if ret != 0 {
        return ret;
    }

    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_FW_PARAM_ENABLE_LOGS);

    (*priv_).state_info.enable = 1;

    msg.data_size = size_of::<sof_log_state_info>();
    msg.data_ptr = ptr::addr_of_mut!((*priv_).state_info) as *mut c_void;

    (*priv_).mtrace_state = sof_mtrace_state::SOF_MTRACE_INITIALIZING;
    ret = ((*iops).set_get_data.unwrap())(sdev, ptr::addr_of_mut!(msg), msg.data_size, true);
    if ret != 0 {
        (*priv_).mtrace_state = sof_mtrace_state::SOF_MTRACE_DISABLED;
        return ret;
    }

    (*priv_).mtrace_state = sof_mtrace_state::SOF_MTRACE_ENABLED;

    0
}

unsafe fn ipc4_mtrace_disable(sdev: *mut snd_sof_dev) {
    let priv_ = (*sdev).fw_trace_data as *mut sof_mtrace_priv;
    let iops = (*(*sdev).ipc).ops;
    let mut msg = core::mem::MaybeUninit::<sof_ipc4_msg>::zeroed().assume_init();
    let mut i: c_int;

    if (*priv_).mtrace_state as c_int == sof_mtrace_state::SOF_MTRACE_DISABLED as c_int {
        return;
    }

    msg.primary = SOF_IPC4_MSG_TARGET(SOF_IPC4_MODULE_MSG);
    msg.primary |= SOF_IPC4_MSG_DIR(SOF_IPC4_MSG_REQUEST);
    msg.primary |= SOF_IPC4_MOD_ID(SOF_IPC4_MOD_INIT_BASEFW_MOD_ID);
    msg.primary |= SOF_IPC4_MOD_INSTANCE(SOF_IPC4_MOD_INIT_BASEFW_INSTANCE_ID);
    msg.extension = SOF_IPC4_MOD_EXT_MSG_PARAM_ID(SOF_IPC4_FW_PARAM_ENABLE_LOGS);

    (*priv_).state_info.enable = 0;

    msg.data_size = size_of::<sof_log_state_info>();
    msg.data_ptr = ptr::addr_of_mut!((*priv_).state_info) as *mut c_void;
    ((*iops).set_get_data.unwrap())(sdev, ptr::addr_of_mut!(msg), msg.data_size, true);

    (*priv_).mtrace_state = sof_mtrace_state::SOF_MTRACE_DISABLED;

    i = 0;
    while i < (*sdev).num_cores {
        let core_data = cores_ptr(priv_, i);

        (*core_data).host_read_ptr = 0;
        (*core_data).dsp_write_ptr = 0;
        wake_up(ptr::addr_of_mut!((*core_data).trace_sleep));
        i += 1;
    }
}

/*
 * Each DSP core logs to a dedicated slot.
 * Parse the slot descriptors at debug_box offset to find the debug log slots
 * and map them to cores.
 * There are 15 slots and therefore 15 descriptors to check (MAX_MTRACE_SLOTS)
 */
unsafe fn sof_mtrace_find_core_slots(sdev: *mut snd_sof_dev) {
    let priv_ = (*sdev).fw_trace_data as *mut sof_mtrace_priv;
    let mut core_data: *mut sof_mtrace_core_data;
    let mut slot_desc_type_offset: u32;
    let mut type_: u32 = 0;
    let mut core: u32;
    let mut i: c_int;

    i = 0;
    while i < SOF_IPC4_MAX_DEBUG_SLOTS {
        /* The type is the second u32 in the slot descriptor */
        slot_desc_type_offset = (*sdev).debug_box.offset;
        slot_desc_type_offset += SOF_IPC4_DEBUG_DESCRIPTOR_SIZE * i as u32 + size_of::<u32>() as u32;
        sof_mailbox_read(
            sdev,
            slot_desc_type_offset,
            ptr::addr_of_mut!(type_) as *mut c_void,
            size_of::<u32>(),
        );

        if type_ & SOF_MTRACE_SLOT_TYPE_MASK == SOF_IPC4_DEBUG_SLOT_DEBUG_LOG {
            core = type_ & SOF_MTRACE_SLOT_CORE_MASK;

            if core >= (*sdev).num_cores as u32 {
                dev_dbg(
                    (*sdev).dev,
                    b"core%u is invalid for slot%d\n\0".as_ptr() as *const c_char,
                    core,
                    i,
                );
                i += 1;
                continue;
            }

            core_data = cores_ptr(priv_, core as c_int);
            /*
             * The area reserved for descriptors have the same size
             * as a slot.
             * In other words: slot0 starts at
             * debug_box + SOF_MTRACE_SLOT_SIZE offset
             */
            (*core_data).slot_offset = (*sdev).debug_box.offset;
            (*core_data).slot_offset += SOF_IPC4_DEBUG_SLOT_SIZE * (i as u32 + 1);
            dev_dbg(
                (*sdev).dev,
                b"slot%d is used for core%u\n\0".as_ptr() as *const c_char,
                i,
                core,
            );
            if (*core_data).delayed_pos_update {
                sof_ipc4_mtrace_update_pos(sdev, core as c_int);
                (*core_data).delayed_pos_update = false;
            }
        } else if type_ != 0 {
            dev_dbg(
                (*sdev).dev,
                b"slot%d is not a log slot (%#x)\n\0".as_ptr() as *const c_char,
                i,
                type_,
            );
        }
        i += 1;
    }
}

unsafe fn ipc4_mtrace_init(sdev: *mut snd_sof_dev) -> c_int {
    let ipc4_data = (*sdev).private as *mut sof_ipc4_fw_data;
    let priv_: *mut sof_mtrace_priv;
    let mut i: c_int;
    let mut ret: c_int;

    if !(*sdev).fw_trace_data.is_null() {
        dev_err(
            (*sdev).dev,
            b"fw_trace_data has been already allocated\n\0".as_ptr() as *const c_char,
        );
        return -EBUSY;
    }

    if (*ipc4_data).mtrace_log_bytes == 0 || (*ipc4_data).mtrace_type != SOF_IPC4_MTRACE_INTEL_CAVS_2
    {
        (*sdev).fw_trace_is_supported = false;
        return 0;
    }

    priv_ = devm_kzalloc(
        (*sdev).dev,
        size_of::<sof_mtrace_priv>()
            + size_of::<sof_mtrace_core_data>() * (*sdev).num_cores as usize,
        GFP_KERNEL,
    ) as *mut sof_mtrace_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*sdev).fw_trace_data = priv_ as *mut c_void;

    /* Set initial values for mtrace parameters */
    (*priv_).state_info.aging_timer_period = DEFAULT_AGING_TIMER_PERIOD_MS;
    (*priv_).state_info.fifo_full_timer_period = DEFAULT_FIFO_FULL_TIMER_PERIOD_MS;
    /* Only enable basefw logs initially (index 0 is always basefw) */
    (*priv_).state_info.logs_priorities_mask[0] = DEFAULT_LOGS_PRIORITIES_MASK;

    i = 0;
    while i < (*sdev).num_cores {
        let core_data = cores_ptr(priv_, i);

        init_waitqueue_head(ptr::addr_of_mut!((*core_data).trace_sleep));
        mutex_init(ptr::addr_of_mut!((*core_data).buffer_lock));
        (*core_data).sdev = sdev;
        (*core_data).id = i;
        i += 1;
    }

    ret = ipc4_mtrace_enable(sdev);
    if ret != 0 {
        /*
         * Mark firmware tracing as not supported and return 0 to not
         * block the whole audio stack
         */
        (*sdev).fw_trace_is_supported = false;
        dev_dbg(
            (*sdev).dev,
            b"initialization failed, fw tracing is disabled\n\0".as_ptr() as *const c_char,
        );
        return 0;
    }

    sof_mtrace_find_core_slots(sdev);

    ret = mtrace_debugfs_create(sdev);
    if ret != 0 {
        ipc4_mtrace_disable(sdev);
    }

    ret
}

unsafe fn ipc4_mtrace_free(sdev: *mut snd_sof_dev) {
    ipc4_mtrace_disable(sdev);
}

unsafe fn sof_ipc4_mtrace_update_pos_all_cores(sdev: *mut snd_sof_dev) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < (*sdev).num_cores {
        sof_ipc4_mtrace_update_pos(sdev, i);
        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sof_ipc4_mtrace_update_pos(sdev: *mut snd_sof_dev, core: c_int) -> c_int {
    let priv_ = (*sdev).fw_trace_data as *mut sof_mtrace_priv;
    let core_data: *mut sof_mtrace_core_data;

    if !(*sdev).fw_trace_is_supported
        || (*priv_).mtrace_state as c_int == sof_mtrace_state::SOF_MTRACE_DISABLED as c_int
    {
        return 0;
    }

    if core >= (*sdev).num_cores {
        return -EINVAL;
    }

    core_data = cores_ptr(priv_, core);

    if (*core_data).slot_offset == SOF_IPC4_INVALID_SLOT_OFFSET {
        (*core_data).delayed_pos_update = true;
        return 0;
    }

    /* Read out the dsp_write_ptr from the slot for this core */
    sof_mailbox_read(
        sdev,
        (*core_data).slot_offset + size_of::<u32>() as u32,
        ptr::addr_of_mut!((*core_data).dsp_write_ptr) as *mut c_void,
        4,
    );
    (*core_data).dsp_write_ptr -= (*core_data).dsp_write_ptr % 4;

    if sof_debug_check_flag(SOF_DBG_PRINT_DMA_POSITION_UPDATE_LOGS) {
        dev_dbg(
            (*sdev).dev,
            b"core%d, host read: %#x, dsp write: %#x\0".as_ptr() as *const c_char,
            core,
            (*core_data).host_read_ptr,
            (*core_data).dsp_write_ptr,
        );
    }

    wake_up(ptr::addr_of_mut!((*core_data).trace_sleep));

    0
}

unsafe fn ipc4_mtrace_fw_crashed(sdev: *mut snd_sof_dev) {
    /*
     * The DSP might not be able to send SOF_IPC4_NOTIFY_LOG_BUFFER_STATUS
     * messages anymore, so check the log buffer status on all
     * cores and process any pending messages.
     */
    sof_ipc4_mtrace_update_pos_all_cores(sdev);
}

unsafe fn ipc4_mtrace_resume(sdev: *mut snd_sof_dev) -> c_int {
    ipc4_mtrace_enable(sdev)
}

unsafe fn ipc4_mtrace_suspend(sdev: *mut snd_sof_dev, _pm_state: pm_message_t) {
    ipc4_mtrace_disable(sdev);
}

#[repr(C)]
pub struct sof_ipc_fw_tracing_ops {
    pub init: Option<unsafe fn(*mut snd_sof_dev) -> c_int>,
    pub free: Option<unsafe fn(*mut snd_sof_dev)>,
    pub fw_crashed: Option<unsafe fn(*mut snd_sof_dev)>,
    pub suspend: Option<unsafe fn(*mut snd_sof_dev, pm_message_t)>,
    pub resume: Option<unsafe fn(*mut snd_sof_dev) -> c_int>,
}

#[no_mangle]
pub static ipc4_mtrace_ops: sof_ipc_fw_tracing_ops = sof_ipc_fw_tracing_ops {
    init: Some(ipc4_mtrace_init),
    free: Some(ipc4_mtrace_free),
    fw_crashed: Some(ipc4_mtrace_fw_crashed),
    suspend: Some(ipc4_mtrace_suspend),
    resume: Some(ipc4_mtrace_resume),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
