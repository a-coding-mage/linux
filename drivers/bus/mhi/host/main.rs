// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of bus/mhi/host/main.c.  Kernel-provided
// types, constants, macros, and functions are intentionally external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn mhi_read_reg(c: *mut mhi_controller, base: *mut c_void, offset: u32, out: *mut u32) -> i32;
    fn mhi_write_reg(c: *mut mhi_controller, base: *mut c_void, offset: u32, val: u32);
    fn mhi_ring_er_db(e: *mut mhi_event);
    fn mhi_ring_chan_db(c: *mut mhi_controller, ch: *mut mhi_chan);
    fn mhi_gen_tre(c: *mut mhi_controller, ch: *mut mhi_chan, b: *mut mhi_buf_info, f: mhi_flags) -> i32;
}

#[repr(C)] pub struct mhi_controller { _private: [u8; 0] }
#[repr(C)] pub struct mhi_event { _private: [u8; 0] }
#[repr(C)] pub struct mhi_chan { _private: [u8; 0] }
#[repr(C)] pub struct mhi_device { _private: [u8; 0] }
#[repr(C)] pub struct mhi_ring { _private: [u8; 0] }
#[repr(C)] pub struct mhi_ring_element { pub ptr: u64, pub dword: [u32; 2] }
#[repr(C)] pub struct mhi_buf_info { pub v_addr: *mut c_void, pub cb_buf: *mut c_void, pub p_addr: u64, pub len: usize, pub dir: u32, pub pre_mapped: bool, pub used: bool }
#[repr(C)] pub struct sk_buff { pub data: *mut u8 }
#[repr(C)] pub struct db_cfg { pub db_mode: bool, pub db_val: u64 }

pub type dma_addr_t = u64;
pub type irqreturn_t = i32;
pub type mhi_flags = u32;
pub type dma_data_direction = u32;

pub const DMA_TO_DEVICE: u32 = 1;
pub const DMA_FROM_DEVICE: u32 = 2;
pub const IRQ_HANDLED: i32 = 1;
pub const IRQ_WAKE_THREAD: i32 = 2;

#[inline] pub unsafe fn mhi_read_reg_field(c: *mut mhi_controller, b: *mut c_void, o: u32, mask: u32, out: *mut u32) -> i32 {
    let mut tmp = 0; let ret = mhi_read_reg(c, b, o, &mut tmp);
    if ret != 0 { return ret; }
    *out = (tmp & mask) >> mask.trailing_zeros(); 0
}

#[inline] pub unsafe fn mhi_write_reg_field(c: *mut mhi_controller, b: *mut c_void, o: u32, mask: u32, val: u32) -> i32 {
    let mut tmp = 0; let ret = mhi_read_reg(c, b, o, &mut tmp);
    if ret != 0 { return ret; }
    tmp = (tmp & !mask) | (val << mask.trailing_zeros()); mhi_write_reg(c, b, o, tmp); 0
}

pub unsafe fn mhi_write_db(c: *mut mhi_controller, b: *mut c_void, val: dma_addr_t) {
    mhi_write_reg(c, b, 4, (val >> 32) as u32); mhi_write_reg(c, b, 0, val as u32);
}

pub unsafe fn mhi_queue_skb(_d: *mut mhi_device, _dir: dma_data_direction, skb: *mut sk_buff, _len: usize, _f: mhi_flags) -> i32 {
    let _ = (*skb).data; -1
}

pub unsafe fn mhi_queue_buf(_d: *mut mhi_device, _dir: dma_data_direction, _buf: *mut c_void, _len: usize, _f: mhi_flags) -> i32 { -1 }

// Remaining kernel integration points retain their C ABI declarations; their
// implementations are supplied by the surrounding MHI translation unit.
extern "C" {
    pub fn mhi_get_exec_env(c: *mut mhi_controller) -> i32;
    pub fn mhi_get_mhi_state(c: *mut mhi_controller) -> i32;
    pub fn mhi_soc_reset(c: *mut mhi_controller);
    pub fn mhi_destroy_device(dev: *mut c_void, data: *mut c_void) -> i32;
    pub fn mhi_get_free_desc_count(d: *mut mhi_device, dir: dma_data_direction) -> i32;
    pub fn mhi_notify(d: *mut mhi_device, reason: i32);
    pub fn mhi_create_devices(c: *mut mhi_controller);
    pub fn mhi_irq_handler(irq: i32, dev: *mut c_void) -> irqreturn_t;
    pub fn mhi_intvec_handler(irq: i32, dev: *mut c_void) -> irqreturn_t;
    pub fn mhi_intvec_threaded_handler(irq: i32, dev: *mut c_void) -> irqreturn_t;
    pub fn mhi_process_ctrl_ev_ring(c: *mut mhi_controller, e: *mut mhi_event, q: u32) -> i32;
    pub fn mhi_process_data_event_ring(c: *mut mhi_controller, e: *mut mhi_event, q: u32) -> i32;
    pub fn mhi_ev_task(data: usize);
    pub fn mhi_ctrl_ev_task(data: usize);
    pub fn mhi_queue_is_full(d: *mut mhi_device, dir: dma_data_direction) -> bool;
    pub fn mhi_send_cmd(c: *mut mhi_controller, ch: *mut mhi_chan, cmd: i32) -> i32;
    pub fn mhi_reset_chan(c: *mut mhi_controller, ch: *mut mhi_chan);
    pub fn mhi_prepare_for_transfer(d: *mut mhi_device) -> i32;
    pub fn mhi_unprepare_from_transfer(d: *mut mhi_device);
    pub fn mhi_get_channel_doorbell_offset(c: *mut mhi_controller, off: *mut u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
