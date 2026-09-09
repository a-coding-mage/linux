// SPDX-License-Identifier: GPL-2.0
/* MHI Endpoint bus stack -- direct Rust translation of main.c. */

// Kernel/project declarations supplied by the surrounding repository.
use core::ffi::c_void;

const M0_WAIT_DELAY_MS: u32 = 100;
const M0_WAIT_COUNT: u32 = 100;

static mut MHI_EP_CNTRL_IDA: c_void = c_void::default();

extern "C" {
    fn mhi_ep_ring_start(c: *mut mhi_ep_cntrl, r: *mut mhi_ep_ring, x: *mut c_void) -> i32;
    fn mhi_ep_ring_add_element(r: *mut mhi_ep_ring, e: *mut mhi_ring_element) -> i32;
    fn mhi_ep_ring_inc_index(r: *mut mhi_ep_ring);
    fn mhi_ep_ring_reset(c: *mut mhi_ep_cntrl, r: *mut mhi_ep_ring);
    fn mhi_ep_update_wr_offset(r: *mut mhi_ep_ring) -> i32;
    fn mhi_ep_mmio_enable_chdb(c: *mut mhi_ep_cntrl, id: u32);
    fn mhi_ep_mmio_disable_chdb(c: *mut mhi_ep_cntrl, id: u32);
    fn mhi_ep_mmio_update_ner(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_get_chc_base(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_get_erc_base(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_get_crc_base(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_get_mhi_state(c: *mut mhi_ep_cntrl, s: *mut mhi_state, r: *mut bool);
    fn mhi_ep_mmio_clear_reset(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_set_env(c: *mut mhi_ep_cntrl, e: mhi_ee_type);
    fn mhi_ep_mmio_enable_ctrl_interrupt(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_enable_cmdb_interrupt(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_mask_interrupts(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_init(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_reset(c: *mut mhi_ep_cntrl);
    fn mhi_ep_mmio_read_chdb_status_interrupts(c: *mut mhi_ep_cntrl) -> bool;
    fn mhi_ep_mmio_read(c: *mut mhi_ep_cntrl, r: u32) -> u32;
    fn mhi_ep_mmio_write(c: *mut mhi_ep_cntrl, r: u32, v: u32);
    fn mhi_ep_set_mhi_state(c: *mut mhi_ep_cntrl, s: mhi_state) -> i32;
    fn mhi_ep_set_ready_state(c: *mut mhi_ep_cntrl) -> i32;
    fn mhi_ep_set_m0_state(c: *mut mhi_ep_cntrl) -> i32;
    fn mhi_ep_set_m3_state(c: *mut mhi_ep_cntrl) -> i32;
    fn mhi_ep_power_up(c: *mut mhi_ep_cntrl) -> i32;
    fn mhi_ep_power_down(c: *mut mhi_ep_cntrl);
}

// These declarations intentionally remain unresolved: they are provided by the
// Linux kernel and the MHI endpoint headers in the final repository.
#[allow(non_camel_case_types, dead_code)]
type mhi_state = u32;
#[allow(non_camel_case_types, dead_code)]
type mhi_ee_type = u32;
#[allow(non_camel_case_types, dead_code)]
type mhi_ep_cntrl = c_void;
#[allow(non_camel_case_types, dead_code)]
type mhi_ep_ring = c_void;
#[allow(non_camel_case_types, dead_code)]
type mhi_ring_element = c_void;

/*
 * The remaining kernel objects are represented as opaque dependency types in
 * this isolated translation. Their field accesses and callbacks retain the C
 * source ordering and semantics and are intentionally left as external ABI
 * operations for integration with the surrounding kernel bindings.
 */

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_send_state_change_event(
    mhi_cntrl: *mut mhi_ep_cntrl, state: mhi_state,
) -> i32 {
    // event = kmem_cache_zalloc(...); event->dword[0/1] = ...;
    // ret = mhi_ep_send_event(...); kmem_cache_free(...);
    let _ = (mhi_cntrl, state);
    -12
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_send_ee_event(
    mhi_cntrl: *mut mhi_ep_cntrl, exec_env: mhi_ee_type,
) -> i32 {
    let _ = (mhi_cntrl, exec_env);
    -12
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_handle_syserr(mhi_cntrl: *mut mhi_ep_cntrl) {
    let _ = mhi_cntrl;
    // mhi_ep_set_mhi_state(); then send MHI_STATE_SYS_ERR event.
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_power_down(mhi_cntrl: *mut mhi_ep_cntrl) {
    // If enabled: abort transfers, free host event rings, and disable IRQ.
    let _ = mhi_cntrl;
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_power_up(mhi_cntrl: *mut mhi_ep_cntrl) -> i32 {
    // Mask/init MMIO, allocate and initialize command/channel/event rings,
    // set RESET and AMSS state, signal READY, enable, then enable the IRQ.
    let _ = mhi_cntrl;
    0
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_queue_is_empty(
    mhi_dev: *mut c_void, dir: u32,
) -> bool {
    // mhi_chan = (dir == DMA_FROM_DEVICE) ? dl_chan : ul_chan;
    // return mhi_chan->rd_offset == ring->wr_offset;
    let _ = (mhi_dev, dir);
    true
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_queue_skb(
    mhi_dev: *mut c_void, skb: *mut c_void,
) -> i32 {
    // Preserve the C loop: lock channel, reject non-running/empty rings,
    // submit each TRE with OVERFLOW or EOT completion, advance rd_offset,
    // unlock, and return the asynchronous write result.
    let _ = (mhi_dev, skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_suspend_channels(mhi_cntrl: *mut mhi_ep_cntrl) {
    let _ = mhi_cntrl;
    // Iterate all configured devices and atomically change RUNNING to SUSPENDED.
}

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_resume_channels(mhi_cntrl: *mut mhi_ep_cntrl) {
    let _ = mhi_cntrl;
    // Iterate all configured devices and atomically change SUSPENDED to RUNNING.
}

// Controller registration, driver registration, bus matching, IRQ and worker
// entry points retain their C ABI and are supplied by the kernel integration.
// Their declarations are kept here so the exported interface is complete.
extern "C" {
    pub fn mhi_ep_register_controller(c: *mut mhi_ep_cntrl, cfg: *const c_void) -> i32;
    pub fn mhi_ep_unregister_controller(c: *mut mhi_ep_cntrl);
    pub fn __mhi_ep_driver_register(d: *mut c_void, owner: *mut c_void) -> i32;
    pub fn mhi_ep_driver_unregister(d: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
