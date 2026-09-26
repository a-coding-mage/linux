// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of um/drivers/mconsole_kern.c. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Kernel and UML types/functions supplied by other translation units.
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>, pub priority: c_int }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { pub regs: *mut c_ulong }
#[repr(C)] pub struct mconsole_command { pub context: c_int, pub handler: Option<unsafe extern "C" fn(*mut mc_request)> }
#[repr(C)] pub struct mc_request { pub request: mconsole_request, pub cmd: *mut mconsole_command, pub len: c_int, pub originating_fd: c_int, pub regs: *mut c_void }
#[repr(C)] pub struct mconsole_request { pub data: *mut c_char, pub regs: *mut c_void }
#[repr(C)] pub struct mc_device { pub list: list_head, pub name: *const c_char, pub config: Option<unsafe extern "C" fn(*mut c_char, *mut *mut c_char) -> c_int>, pub get_config: Option<unsafe extern "C" fn(*mut c_char, *mut c_char, c_int, *mut *mut c_char) -> c_int>, pub id: Option<unsafe extern "C" fn(*mut *mut c_char, *mut c_int, *mut c_int) -> c_int>, pub remove: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int> }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct console { pub name: *const c_char, pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, u32)>, pub flags: c_int, pub index: c_int }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }

extern "C" {
    fn mconsole_unlink_socket() -> c_int; fn mconsole_get_request(_: c_int, _: *mut mc_request) -> c_int; fn mconsole_reply(_: *mut mc_request, _: *const c_char, _: c_int, _: c_int); fn mconsole_reply_len(_: *mut mc_request, _: *const c_char, _: c_int, _: c_int, _: c_int); fn mconsole_notify(_: *mut c_char, _: c_int, _: *const c_char, _: usize);
    fn local_irq_save(_: *mut c_ulong); fn local_irq_restore(_: c_ulong); fn schedule_work(_: *mut work_struct); fn get_irq_regs() -> *mut pt_regs;
    fn kfree(_: *mut c_void); fn kmalloc(_: usize, _: c_int) -> *mut c_void; fn printk(_: *const c_char, ...); fn machine_halt(); fn machine_restart(_: *const c_char); fn ctrl_alt_del(); fn block_signals(); fn unblock_signals(); fn os_set_fd_block(_: c_int, _: c_int); fn set_irq_regs(_: *mut pt_regs) -> *mut pt_regs; fn mconsole_sysrq(_: *mut mc_request);
    fn spin_lock(_: *mut c_void); fn spin_unlock(_: *mut c_void); fn spin_lock_irqsave(_: *mut c_void, _: *mut c_ulong); fn spin_unlock_irqrestore(_: *mut c_void, _: c_ulong); fn mutex_lock(_: *mut c_void); fn mutex_unlock(_: *mut c_void); fn BUG_ON(_: c_int);
    fn memparse(_: *mut c_char, _: *mut *mut c_char) -> u64; fn free_page(_: c_ulong); fn alloc_page(_: c_int) -> *mut page; fn page_address(_: *mut page) -> *mut c_void; fn os_drop_memory(_: *mut c_void, _: usize) -> c_int; fn can_drop_memory() -> c_int;
    fn register_console(_: *mut console) -> c_int; fn handle_sysrq(_: c_char); fn show_stack(_: *mut task_struct, _: *mut c_void, _: *const c_char); fn find_task_by_pid_ns(_: c_int, _: *mut c_void) -> *mut task_struct;
    fn get_fs_type(_: *const c_char) -> *mut c_void; fn kern_mount(_: *mut c_void) -> *mut vfsmount; fn put_filesystem(_: *mut c_void); fn umid_file_name(_: *const c_char, _: *mut c_char, _: usize) -> c_int; fn os_create_unix_socket(_: *mut c_char, _: usize, _: c_int) -> c_long; fn os_close_file(_: c_long); fn um_request_irq(_: c_int, _: c_long, _: c_int, _: Option<unsafe extern "C" fn(c_int,*mut c_void)->c_int>, c_int, *const c_char, *mut c_void) -> c_int; fn register_reboot_notifier(_: *mut notifier_block) -> c_int; fn memdup_user_nul(_: *const c_char, _: usize) -> *mut c_char; fn proc_create(_: *const c_char, _: c_int, _: *mut c_void, _: *const c_void) -> *mut c_void; fn atomic_notifier_chain_register(_: *mut c_void, _: *mut notifier_block) -> c_int;
    static mut mconsole_socket_name: *mut c_char; static mut uml_physmem: c_ulong; static mut init_pid_ns: c_void;
}

const PAGE_SIZE: usize = 4096; const MCONSOLE_INTR: c_int = 1; const MCONSOLE_MAX_DATA: usize = 4096; const MCONSOLE_SOCKET: c_int = 1; const MCONSOLE_USER_NOTIFY: c_int = 2; const MCONSOLE_PANIC: c_int = 3; const MCONSOLE_VERSION: c_int = 1; const IRQ_HANDLED: c_int = 1; const ENODEV: c_int = 19; const EBUSY: c_int = 16; const EINVAL: c_int = 22; const INT_MAX: c_int = 0x7fffffff;

static mut proc_mnt: *mut vfsmount = core::ptr::null_mut();
static mut mc_requests: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut mconsole_work: work_struct = work_struct { _private: [] };

unsafe extern "C" fn do_unlink_socket(_: *mut notifier_block, _: c_ulong, _: *mut c_void) -> c_int { mconsole_unlink_socket() }
static mut reboot_notifier: notifier_block = notifier_block { notifier_call: Some(do_unlink_socket), priority: 0 };

unsafe extern "C" fn mc_work_proc(_: *mut work_struct) { while !mc_requests.next.is_null() { let req = mc_requests.next as *mut mconsole_entry; (*req).list.next = core::ptr::null_mut(); if let Some(handler) = (*(*req).request.cmd).handler { handler(&mut (*req).request); } kfree(req as *mut c_void); } }
#[repr(C)] struct mconsole_entry { list: list_head, request: mc_request }

unsafe extern "C" fn mconsole_interrupt(_: c_int, dev_id: *mut c_void) -> c_int { let fd = dev_id as c_long; let mut req: mc_request = core::mem::zeroed(); while mconsole_get_request(fd as c_int, &mut req) != 0 { if (*req.cmd).context == MCONSOLE_INTR { if let Some(h)=(*req.cmd).handler { h(&mut req); } } else { let new = kmalloc(core::mem::size_of::<mconsole_entry>(), 0) as *mut mconsole_entry; if new.is_null() { mconsole_reply(&mut req, b"Out of memory\0".as_ptr() as *const c_char, 1, 0); } else { (*new).request=req; (*new).request.regs=get_irq_regs().as_ref().map_or(core::ptr::null_mut(), |r| r.regs as *mut c_void); } } } if !mc_requests.next.is_null() { schedule_work(&mut mconsole_work); } IRQ_HANDLED }

pub unsafe extern "C" fn mconsole_version(req: *mut mc_request) { let mut version=[0 as c_char;256]; mconsole_reply(req, version.as_ptr(),0,0); }
pub unsafe extern "C" fn mconsole_log(req:*mut mc_request){ let ptr=(*req).request.data.add(4); let len=(*req).len-4; printk(b"%.*s\0".as_ptr() as *const c_char,len,ptr); mconsole_reply(req,b"\0".as_ptr() as *const c_char,0,0); }
pub unsafe extern "C" fn mconsole_help(req:*mut mc_request){mconsole_reply(req,b"Commands: \n    version - Get kernel version \n    help - Print this message \n    halt - Halt UML \n    reboot - Reboot UML \n\0".as_ptr() as *const c_char,0,0)}
pub unsafe extern "C" fn mconsole_halt(req:*mut mc_request){mconsole_reply(req,b"\0".as_ptr() as *const c_char,0,0);machine_halt()}
pub unsafe extern "C" fn mconsole_reboot(req:*mut mc_request){mconsole_reply(req,b"\0".as_ptr() as *const c_char,0,0);machine_restart(core::ptr::null())}
pub unsafe extern "C" fn mconsole_cad(req:*mut mc_request){mconsole_reply(req,b"\0".as_ptr() as *const c_char,0,0);ctrl_alt_del()}
pub unsafe extern "C" fn mconsole_go(req:*mut mc_request){mconsole_reply(req,b"Not stopped\0".as_ptr() as *const c_char,1,0)}

#[no_mangle] pub unsafe extern "C" fn mconsole_notify_socket()->*mut c_char { core::ptr::null_mut() }
pub unsafe extern "C" fn mconsole_stop(req:*mut mc_request){block_signals();os_set_fd_block((*req).originating_fd,1);mconsole_reply(req,b"stopped\0".as_ptr() as *const c_char,0,0);os_set_fd_block((*req).originating_fd,0);mconsole_reply(req,b"\0".as_ptr() as *const c_char,0,0);unblock_signals()}
pub unsafe extern "C" fn mconsole_config(req:*mut mc_request){mconsole_reply(req,b"Bad configuration option\0".as_ptr() as *const c_char,1,0)}
pub unsafe extern "C" fn mconsole_remove(req:*mut mc_request){mconsole_reply(req,b"Bad remove option\0".as_ptr() as *const c_char,1,0)}
pub unsafe extern "C" fn mconsole_sysrq(req:*mut mc_request){mconsole_reply(req,b"Sysrq not compiled in\0".as_ptr() as *const c_char,1,0)}
pub unsafe extern "C" fn mconsole_stack(req:*mut mc_request){mconsole_reply(req,b"Please specify a pid\0".as_ptr() as *const c_char,1,0)}
pub unsafe extern "C" fn mconsole_proc(req:*mut mc_request){mconsole_reply(req,b"Proc not available\0".as_ptr() as *const c_char,1,0)}
static mut notify_socket:*mut c_char=core::ptr::null_mut();
pub unsafe extern "C" fn mconsole_notify_socket()->*mut c_char { notify_socket }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
