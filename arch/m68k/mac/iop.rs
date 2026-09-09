/*
 * I/O Processor (IOP) management
 * Written and (C) 1999 by Joshua M. Thompson (funaho@jurai.org)
 *
 * Rust translation of the original implementation source.
 */

/* The original file depends on Linux, Macintosh, and mac_iop declarations. */

extern "C" {
    static mut macintosh_config: *mut MacConfig;
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn request_irq(irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_ulong, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
}

type c_int = i32;
type c_ulong = usize;
type c_char = i8;
type c_void = core::ffi::c_void;
type uint = u32;
type __u8 = u8;
type __u16 = u16;
type irqreturn_t = c_int;

#[repr(C)]
pub struct MacConfig {
    pub scc_type: c_int,
    pub adb_type: c_int,
    pub ident: c_int,
}

#[repr(C)]
pub struct mac_iop {
    pub ram_addr_lo: u8,
    pub ram_addr_hi: u8,
    pub ram_data: u8,
    pub status_ctrl: u8,
}

#[repr(C)]
pub struct iop_msg {
    pub next: *mut iop_msg,
    pub status: c_int,
    pub iop_num: c_int,
    pub channel: c_int,
    pub caller_priv: *mut c_void,
    pub message: [u8; IOP_MSG_LEN as usize],
    pub reply: [u8; IOP_MSG_LEN as usize],
    pub handler: Option<unsafe extern "C" fn(*mut iop_msg)>,
}

extern "C" {
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
}

const NUM_IOPS: usize = 2;
const NUM_IOP_CHAN: usize = 7;
const NUM_IOP_MSGS: usize = 32;
const IOP_MSG_LEN: u32 = 32;
const IOP_MSGSTATUS_UNUSED: c_int = 0;
const IOP_MSGSTATUS_WAITING: c_int = 1;
const IOP_MSGSTATUS_COMPLETE: c_int = 2;
const IOP_MSGSTATUS_UNSOL: c_int = 3;
const IOP_MSG_IDLE: u8 = 0;
const IOP_MSG_NEW: u8 = 1;
const IOP_MSG_RCVD: u8 = 2;
const IOP_MSG_COMPLETE: u8 = 3;
const IOP_AUTOINC: u8 = 0x01;
const IOP_RUN: u8 = 0x02;
const IOP_IRQ: u8 = 0x04;
const IOP_INT0: u8 = 0x08;
const IOP_INT1: u8 = 0x10;
const IOP_ADDR_ALIVE: u16 = 0;
const IOP_ADDR_SEND_MSG: u16 = 1;
const IOP_ADDR_RECV_MSG: u16 = 225;
const IOP_ADDR_SEND_STATE: u16 = 257;
const IOP_ADDR_RECV_STATE: u16 = 264;
const IOP_NUM_SCC: usize = 0;
const IOP_NUM_ISM: usize = 1;
const MAC_SCC_IOP: c_int = 1;
const MAC_ADB_IOP: c_int = 1;
const MAC_MODEL_IIFX: c_int = 1;
const IRQ_MAC_ADB: c_int = 0;
const IRQ_VIA2_0: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IRQ_HANDLED: irqreturn_t = 1;
const SCC_IOP_BASE_IIFX: usize = 0;
const SCC_IOP_BASE_QUADRA: usize = 0;
const ISM_IOP_BASE_IIFX: usize = 0;
const ISM_IOP_BASE_QUADRA: usize = 0;

pub static mut iop_scc_present: c_int = 0;
pub static mut iop_ism_present: c_int = 0;

#[repr(C)]
struct listener {
    devname: *const c_char,
    handler: Option<unsafe extern "C" fn(*mut iop_msg)>,
}

static mut iop_base: [*mut mac_iop; NUM_IOPS] = [core::ptr::null_mut(); NUM_IOPS];
static mut iop_msg_pool: [iop_msg; NUM_IOP_MSGS] = [const { iop_msg { next: core::ptr::null_mut(), status: IOP_MSGSTATUS_UNUSED, iop_num: 0, channel: 0, caller_priv: core::ptr::null_mut(), message: [0; IOP_MSG_LEN as usize], reply: [0; IOP_MSG_LEN as usize], handler: None } }; NUM_IOP_MSGS];
static mut iop_send_queue: [[*mut iop_msg; NUM_IOP_CHAN]; NUM_IOPS] = [[core::ptr::null_mut(); NUM_IOP_CHAN]; NUM_IOPS];
static mut iop_listeners: [[listener; NUM_IOP_CHAN]; NUM_IOPS] = [[const { listener { devname: core::ptr::null(), handler: None } }; NUM_IOP_CHAN]; NUM_IOPS];

unsafe fn iop_loadaddr(iop: *mut mac_iop, addr: u16) { (*iop).ram_addr_lo = addr as u8; (*iop).ram_addr_hi = (addr >> 8) as u8; }
unsafe fn iop_readb(iop: *mut mac_iop, addr: u16) -> u8 { iop_loadaddr(iop, addr); core::ptr::read_volatile(&(*iop).ram_data) }
unsafe fn iop_writeb(iop: *mut mac_iop, addr: u16, data: u8) { iop_loadaddr(iop, addr); core::ptr::write_volatile(&mut (*iop).ram_data, data); }
unsafe fn iop_stop(iop: *mut mac_iop) { (*iop).status_ctrl = IOP_AUTOINC; }
unsafe fn iop_start(iop: *mut mac_iop) { (*iop).status_ctrl = IOP_RUN | IOP_AUTOINC; }
unsafe fn iop_interrupt(iop: *mut mac_iop) { (*iop).status_ctrl = IOP_IRQ | IOP_RUN | IOP_AUTOINC; }
unsafe fn iop_alive(iop: *mut mac_iop) -> bool { let r = iop_readb(iop, IOP_ADDR_ALIVE) == 0xff; iop_writeb(iop, IOP_ADDR_ALIVE, 0); r }

unsafe fn iop_get_unused_msg() -> *mut iop_msg {
    let mut flags = 0; local_irq_save(&mut flags);
    for i in 0..NUM_IOP_MSGS { if iop_msg_pool[i].status == IOP_MSGSTATUS_UNUSED { iop_msg_pool[i].status = IOP_MSGSTATUS_WAITING; local_irq_restore(flags); return &mut iop_msg_pool[i]; } }
    local_irq_restore(flags); core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn iop_init() { let cfg = &*macintosh_config; if cfg.scc_type == MAC_SCC_IOP { iop_base[IOP_NUM_SCC] = if cfg.ident == MAC_MODEL_IIFX { SCC_IOP_BASE_IIFX as *mut mac_iop } else { SCC_IOP_BASE_QUADRA as *mut mac_iop }; iop_scc_present = 1; } if cfg.adb_type == MAC_ADB_IOP { iop_base[IOP_NUM_ISM] = if cfg.ident == MAC_MODEL_IIFX { ISM_IOP_BASE_IIFX as *mut mac_iop } else { ISM_IOP_BASE_QUADRA as *mut mac_iop }; iop_ism_present = 1; iop_stop(iop_base[IOP_NUM_ISM]); iop_start(iop_base[IOP_NUM_ISM]); iop_alive(iop_base[IOP_NUM_ISM]); } for i in 0..NUM_IOP_MSGS { iop_msg_pool[i].status = IOP_MSGSTATUS_UNUSED; } for i in 0..NUM_IOP_CHAN { iop_send_queue[IOP_NUM_SCC][i] = core::ptr::null_mut(); iop_send_queue[IOP_NUM_ISM][i] = core::ptr::null_mut(); iop_listeners[IOP_NUM_SCC][i] = listener { devname: core::ptr::null(), handler: None }; iop_listeners[IOP_NUM_ISM][i] = listener { devname: core::ptr::null(), handler: None }; } }

#[no_mangle]
pub unsafe extern "C" fn iop_register_interrupts() { if iop_ism_present != 0 { let irq = if (*macintosh_config).ident == MAC_MODEL_IIFX { IRQ_MAC_ADB } else { IRQ_VIA2_0 }; let _ = request_irq(irq, Some(iop_ism_irq), 0, b"ISM IOP\0".as_ptr() as *const c_char, IOP_NUM_ISM as *mut c_void); let _ = iop_alive(iop_base[IOP_NUM_ISM]); } }

#[no_mangle]
pub unsafe extern "C" fn iop_listen(iop_num: uint, chan: uint, handler: Option<unsafe extern "C" fn(*mut iop_msg)>, devname: *const c_char) -> c_int { if iop_num as usize >= NUM_IOPS || iop_base[iop_num as usize].is_null() || chan as usize >= NUM_IOP_CHAN || (iop_listeners[iop_num as usize][chan as usize].handler.is_some() && handler.is_some()) { return -EINVAL; } iop_listeners[iop_num as usize][chan as usize] = listener { devname, handler }; 0 }

#[no_mangle]
pub unsafe extern "C" fn iop_upload_code(iop_num: uint, mut code_start: *const u8, mut code_len: uint, shared_ram_start: u16) { if iop_num as usize >= NUM_IOPS || iop_base[iop_num as usize].is_null() { return; } iop_loadaddr(iop_base[iop_num as usize], shared_ram_start); while code_len != 0 { (*iop_base[iop_num as usize]).ram_data = *code_start; code_start = code_start.add(1); code_len -= 1; } }

#[no_mangle]
pub unsafe extern "C" fn iop_download_code(iop_num: uint, mut code_start: *mut u8, mut code_len: uint, shared_ram_start: u16) { if iop_num as usize >= NUM_IOPS || iop_base[iop_num as usize].is_null() { return; } iop_loadaddr(iop_base[iop_num as usize], shared_ram_start); while code_len != 0 { *code_start = (*iop_base[iop_num as usize]).ram_data; code_start = code_start.add(1); code_len -= 1; } }

#[no_mangle]
pub unsafe extern "C" fn iop_compare_code(iop_num: uint, mut code_start: *mut u8, mut code_len: uint, shared_ram_start: u16) -> *mut u8 { if iop_num as usize >= NUM_IOPS || iop_base[iop_num as usize].is_null() { return code_start; } iop_loadaddr(iop_base[iop_num as usize], shared_ram_start); while code_len != 0 { if *code_start != (*iop_base[iop_num as usize]).ram_data { return code_start; } code_start = code_start.add(1); code_len -= 1; } core::ptr::null_mut() }

#[no_mangle]
pub unsafe extern "C" fn iop_ism_irq(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let iop_num = dev_id as usize; let iop = iop_base[iop_num];
    let mut events = (*iop).status_ctrl & (IOP_INT0 | IOP_INT1);
    while events != 0 {
        if events & IOP_INT0 != 0 { (*iop).status_ctrl = IOP_INT0 | IOP_RUN | IOP_AUTOINC; for i in 0..NUM_IOP_CHAN { let state = iop_readb(iop, IOP_ADDR_SEND_STATE + i as u16); if state == IOP_MSG_COMPLETE { iop_handle_send(iop_num, i); } } }
        if events & IOP_INT1 != 0 { (*iop).status_ctrl = IOP_INT1 | IOP_RUN | IOP_AUTOINC; for i in 0..NUM_IOP_CHAN { let state = iop_readb(iop, IOP_ADDR_RECV_STATE + i as u16); if state == IOP_MSG_NEW { iop_handle_recv(iop_num, i); } } }
        events = (*iop).status_ctrl & (IOP_INT0 | IOP_INT1);
    }
    IRQ_HANDLED
}

unsafe fn iop_do_send(msg: *mut iop_msg) { let iop = iop_base[(*msg).iop_num as usize]; let mut offset = IOP_ADDR_SEND_MSG + (*msg).channel as u16 * IOP_MSG_LEN as u16; for b in (*msg).message { iop_writeb(iop, offset, b); offset += 1; } iop_writeb(iop, IOP_ADDR_SEND_STATE + (*msg).channel as u16, IOP_MSG_NEW); iop_interrupt(iop); }

unsafe fn iop_handle_send(iop_num: usize, chan: usize) { let iop = iop_base[iop_num]; iop_writeb(iop, IOP_ADDR_SEND_STATE + chan as u16, IOP_MSG_IDLE); let msg = iop_send_queue[iop_num][chan]; if msg.is_null() { return; } (*msg).status = IOP_MSGSTATUS_COMPLETE; let mut offset = IOP_ADDR_SEND_MSG + chan as u16 * IOP_MSG_LEN as u16; for i in 0..IOP_MSG_LEN as usize { (*msg).reply[i] = iop_readb(iop, offset); offset += 1; } if let Some(handler) = (*msg).handler { handler(msg); } (*msg).status = IOP_MSGSTATUS_UNUSED; iop_send_queue[iop_num][chan] = (*msg).next; let next = iop_send_queue[iop_num][chan]; if !next.is_null() && iop_readb(iop, IOP_ADDR_SEND_STATE + chan as u16) == IOP_MSG_IDLE { iop_do_send(next); } }

#[no_mangle]
pub unsafe extern "C" fn iop_complete_message(msg: *mut iop_msg) { let mut offset = IOP_ADDR_RECV_MSG + (*msg).channel as u16 * IOP_MSG_LEN as u16; for b in (*msg).reply { iop_writeb(iop_base[(*msg).iop_num as usize], offset, b); offset += 1; } iop_writeb(iop_base[(*msg).iop_num as usize], IOP_ADDR_RECV_STATE + (*msg).channel as u16, IOP_MSG_COMPLETE); iop_interrupt(iop_base[(*msg).iop_num as usize]); (*msg).status = IOP_MSGSTATUS_UNUSED; }

unsafe fn iop_handle_recv(iop_num: usize, chan: usize) { let msg = iop_get_unused_msg(); if msg.is_null() { return; } (*msg).iop_num = iop_num as c_int; (*msg).channel = chan as c_int; (*msg).status = IOP_MSGSTATUS_UNSOL; (*msg).handler = iop_listeners[iop_num][chan].handler; let mut offset = IOP_ADDR_RECV_MSG + chan as u16 * IOP_MSG_LEN as u16; for i in 0..IOP_MSG_LEN as usize { (*msg).message[i] = iop_readb(iop_base[iop_num], offset); offset += 1; } iop_writeb(iop_base[iop_num], IOP_ADDR_RECV_STATE + chan as u16, IOP_MSG_RCVD); if let Some(handler) = (*msg).handler { handler(msg); } else { (*msg).reply = [0; IOP_MSG_LEN as usize]; iop_complete_message(msg); } }

#[no_mangle]
pub unsafe extern "C" fn iop_send_message(iop_num: uint, chan: uint, privdata: *mut c_void, msg_len: uint, msg_data: *const u8, handler: Option<unsafe extern "C" fn(*mut iop_msg)>) -> c_int { if iop_num as usize >= NUM_IOPS || iop_base[iop_num as usize].is_null() || chan as usize >= NUM_IOP_CHAN || msg_len > IOP_MSG_LEN { return -EINVAL; } let msg = iop_get_unused_msg(); if msg.is_null() { return -ENOMEM; } (*msg).next = core::ptr::null_mut(); (*msg).status = IOP_MSGSTATUS_WAITING; (*msg).iop_num = iop_num as c_int; (*msg).channel = chan as c_int; (*msg).caller_priv = privdata; core::ptr::copy_nonoverlapping(msg_data, (*msg).message.as_mut_ptr(), msg_len as usize); (*msg).handler = handler; let q = iop_send_queue[iop_num as usize][chan as usize]; if q.is_null() { iop_send_queue[iop_num as usize][chan as usize] = msg; iop_do_send(msg); } else { let mut tail = q; while !(*tail).next.is_null() { tail = (*tail).next; } (*tail).next = msg; } 0 }

#[no_mangle]
pub unsafe extern "C" fn iop_ism_irq_poll(iop_num: uint) { let mut flags = 0; local_irq_save(&mut flags); iop_ism_irq(0, iop_num as *mut c_void); local_irq_restore(flags); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
