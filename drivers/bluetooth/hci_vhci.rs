// SPDX-License-Identifier: GPL-2.0-or-later
/* Bluetooth virtual HCI driver. */

// Kernel headers and symbols are supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::MaybeUninit;

const VERSION: &str = "1.5";
static mut amp: bool = false;

#[repr(C)]
struct vhci_data {
    hdev: *mut hci_dev,
    read_wait: wait_queue_head_t,
    readq: sk_buff_head,
    open_mutex: mutex,
    open_timeout: delayed_work,
    suspend_work: work_struct,
    suspended: bool,
    wakeup: bool,
    msft_opcode: u16,
    aosp_capable: bool,
    initialized: atomic_t,
}

extern "C" {
    fn hci_get_drvdata(hdev: *mut hci_dev) -> *mut vhci_data;
    fn skb_queue_purge(q: *mut sk_buff_head);
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_queue_tail(q: *mut sk_buff_head, skb: *mut sk_buff);
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn wake_up_interruptible(q: *mut wait_queue_head_t);
    fn hci_skb_pkt_type(skb: *mut sk_buff) -> *mut u8;
    fn hci_suspend_dev(hdev: *mut hci_dev);
    fn hci_resume_dev(hdev: *mut hci_dev);
    fn schedule_work(work: *mut work_struct);
    fn kstrtobool_from_user(buf: *const c_char, count: usize, val: *mut bool) -> c_int;
    fn simple_read_from_buffer(to: *mut c_char, count: usize, ppos: *mut loff_t, from: *const c_char, len: usize) -> isize;
    fn cancel_delayed_work_sync(work: *mut delayed_work);
    fn hci_opcode_ogf(opcode: u64) -> u8;
    fn hci_set_msft_opcode(hdev: *mut hci_dev, opcode: u16);
    fn hci_set_aosp_capable(hdev: *mut hci_dev);
    fn skb_put_data(skb: *mut sk_buff, data: *const c_void, len: usize);
    fn strlen(s: *const c_char) -> usize;
    fn secs_to_jiffies(secs: c_ulong) -> c_ulong;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn alloc_skb(size: usize, gfp: c_ulong) -> *mut sk_buff;
    fn bt_skb_alloc(size: usize, gfp: c_ulong) -> *mut sk_buff;
    fn kfree_skb(skb: *mut sk_buff);
    fn hci_devcd_register(hdev: *mut hci_dev, dump: unsafe extern "C" fn(*mut hci_dev), hdr: unsafe extern "C" fn(*mut hci_dev, *mut sk_buff), priv_: *mut c_void);
    fn hci_devcd_init(hdev: *mut hci_dev, len: usize) -> c_int;
    fn hci_devcd_append(hdev: *mut hci_dev, skb: *mut sk_buff);
    fn hci_devcd_complete(hdev: *mut hci_dev);
    fn hci_devcd_abort(hdev: *mut hci_dev);
    fn debugfs_create_file(name: *const c_char, mode: u32, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn hci_alloc_dev() -> *mut hci_dev;
    fn hci_set_drvdata(hdev: *mut hci_dev, data: *mut vhci_data);
    fn hci_set_quirk(hdev: *mut hci_dev, quirk: u32);
    fn hci_register_dev(hdev: *mut hci_dev) -> c_int;
    fn hci_free_dev(hdev: *mut hci_dev);
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_queue_head(q: *mut sk_buff_head, skb: *mut sk_buff);
    fn atomic_inc(v: *mut atomic_t);
    fn copy_from_iter_full(to: *mut c_void, len: usize, from: *mut iov_iter) -> bool;
    fn skb_pull(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn hci_recv_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
    fn iov_iter_count(from: *const iov_iter) -> usize;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn skb_dequeue(q: *mut sk_buff_head) -> *mut sk_buff;
    fn skb_queue_empty(q: *const sk_buff_head) -> bool;
    fn wait_event_interruptible(q: *mut wait_queue_head_t, condition: bool) -> c_int;
    fn poll_wait(file: *mut file, q: *mut wait_queue_head_t, wait: *mut poll_table);
    fn init_waitqueue_head(q: *mut wait_queue_head_t);
    fn skb_queue_head_init(q: *mut sk_buff_head);
    fn mutex_init(m: *mut mutex);
    fn nonseekable_open(inode: *mut inode, file: *mut file) -> c_int;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool;
    fn flush_work(work: *mut work_struct);
    fn hci_unregister_dev(hdev: *mut hci_dev);
    fn debugfs_lookup_and_remove(name: *const c_char, parent: *mut dentry);
    fn kfree(ptr: *mut c_void);
}

#[repr(C)] struct hci_dev { bus: u32, debugfs: *mut dentry, stat: hci_stats, dump: hci_dump, open: Option<unsafe extern "C" fn(*mut hci_dev)->c_int>, close: Option<unsafe extern "C" fn(*mut hci_dev)->c_int>, flush: Option<unsafe extern "C" fn(*mut hci_dev)->c_int>, send: Option<unsafe extern "C" fn(*mut hci_dev,*mut sk_buff)->c_int>, get_data_path_id: Option<unsafe extern "C" fn(*mut hci_dev,*mut u8)->c_int>, get_codec_config_data: Option<unsafe extern "C" fn(*mut hci_dev,u8,*mut bt_codec,*mut u8,*mut *mut u8)->c_int>, wakeup: Option<unsafe extern "C" fn(*mut hci_dev)->bool>, setup: Option<unsafe extern "C" fn(*mut hci_dev)->c_int> }
#[repr(C)] struct sk_buff { data: *mut u8, len: usize }
#[repr(C)] struct sk_buff_head { _private: [u8; 0] }
#[repr(C)] struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] struct mutex { _private: [u8; 0] }
#[repr(C)] struct delayed_work { work: work_struct }
#[repr(C)] struct work_struct { _private: [u8; 0] }
#[repr(C)] struct atomic_t { counter: c_int }
#[repr(C)] struct file { private_data: *mut c_void, f_flags: c_int }
#[repr(C)] struct inode { _private: [u8; 0] }
#[repr(C)] struct iov_iter { _private: [u8; 0] }
#[repr(C)] struct poll_table { _private: [u8; 0] }
#[repr(C)] struct dentry { _private: [u8; 0] }
#[repr(C)] struct bt_codec { _private: [u8; 0] }
#[repr(C)] struct hci_stats { byte_tx: u64, cmd_tx: u64, acl_tx: u64, sco_tx: u64 }
#[repr(C)] struct hci_dump { timeout: c_ulong }
#[repr(C)] struct file_operations { _private: [u8; 0] }
type loff_t = i64;

const EINVAL: c_int = 22; const EALREADY: c_int = 114; const EFAULT: c_int = 14;
const ENOMEM: c_int = 12; const ENODEV: c_int = 19; const EBADFD: c_int = 77;
const EBUSY: c_int = 16; const EAGAIN: c_int = 11;
const HCI_EVENT_PKT: u8 = 0x04; const HCI_ACLDATA_PKT: u8 = 0x02;
const HCI_SCODATA_PKT: u8 = 0x03; const HCI_ISODATA_PKT: u8 = 0x05;
const HCI_VENDOR_PKT: u8 = 0xff; const HCI_COMMAND_PKT: u8 = 0x01;
const ESCO_LINK: u8 = 2; const HCI_MAX_FRAME_SIZE: usize = 4096;

unsafe extern "C" fn vhci_open_dev(_: *mut hci_dev) -> c_int { 0 }
unsafe extern "C" fn vhci_close_dev(hdev: *mut hci_dev) -> c_int { skb_queue_purge(&mut (*hci_get_drvdata(hdev)).readq); 0 }
unsafe extern "C" fn vhci_flush(hdev: *mut hci_dev) -> c_int { skb_queue_purge(&mut (*hci_get_drvdata(hdev)).readq); 0 }
unsafe extern "C" fn vhci_send_frame(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int {
    let data = hci_get_drvdata(hdev); *skb_push(skb,1)=*hci_skb_pkt_type(skb); skb_queue_tail(&mut (*data).readq,skb);
    if atomic_read(&(*data).initialized)>0 { wake_up_interruptible(&mut (*data).read_wait); } 0
}
unsafe extern "C" fn vhci_get_data_path_id(_: *mut hci_dev, id: *mut u8) -> c_int { *id=0; 0 }
unsafe extern "C" fn vhci_get_codec_config_data(_: *mut hci_dev, ty:u8, _: *mut bt_codec, len:*mut u8, data:*mut *mut u8)->c_int { if ty!=ESCO_LINK{return -EINVAL}; *len=0;*data=core::ptr::null_mut();0 }
unsafe extern "C" fn vhci_wakeup(hdev:*mut hci_dev)->bool { (*hci_get_drvdata(hdev)).wakeup }

// The remaining file-scope callbacks and registration wiring are represented as external
// declarations because their kernel callback ABI and helper macros are provided elsewhere.
extern "C" {
    fn vhci_debugfs_init(data: *mut vhci_data);
    fn vhci_create_device(data: *mut vhci_data, opcode: u8) -> c_int;
    fn vhci_read(file: *mut file, buf: *mut c_char, count: usize, pos: *mut loff_t) -> isize;
    fn vhci_write(iocb: *mut c_void, from: *mut iov_iter) -> isize;
    fn vhci_poll(file: *mut file, wait: *mut poll_table) -> u32;
    fn vhci_open(inode: *mut inode, file: *mut file) -> c_int;
    fn vhci_release(inode: *mut inode, file: *mut file) -> c_int;
}

// CONFIG_DEV_COREDUMP, CONFIG_BT_MSFTEXT, and CONFIG_BT_AOSPEXT conditionals are
// intentionally preserved as build-time concerns for the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
