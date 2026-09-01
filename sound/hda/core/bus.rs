// SPDX-License-Identifier: GPL-2.0-only
/*
 * HD-audio core bus driver
 */

use core::ffi::c_void;
use core::mem;
use core::ptr;

type u32 = u32;

const SNDRV_DMA_TYPE_DEV: u32 = 0;
const HDA_UNSOL_QUEUE_SIZE: u32 = 64;
const EINVAL: i32 = 22;
const EAGAIN: i32 = 11;
const EBUSY: i32 = 16;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
}

#[repr(C)]
pub struct hdac_bus_ops {
    pub command: Option<unsafe extern "C" fn(*mut hdac_bus, u32) -> i32>,
    pub get_response: Option<unsafe extern "C" fn(*mut hdac_bus, u32, *mut u32) -> i32>,
    pub link_power: Option<unsafe extern "C" fn(*mut hdac_device, bool)>,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub ops: *const hdac_bus_ops,
    pub dma_type: u32,
    pub stream_list: list_head,
    pub codec_list: list_head,
    pub unsol_work: work_struct,
    pub reg_lock: spinlock_t,
    pub cmd_mutex: mutex,
    pub lock: mutex,
    pub hlink_list: list_head,
    pub rirb_wq: wait_queue_head_t,
    pub irq: i32,
    pub addr_offset: u32,
    pub sdo_limit: u32,
    pub sync_write: bool,
    pub unsol_wp: u32,
    pub unsol_rp: u32,
    pub unsol_queue: [u32; (HDA_UNSOL_QUEUE_SIZE as usize) * 2],
    pub caddr_tbl: [*mut hdac_device; 16],
    pub codec_powered: usize,
    pub num_codecs: u32,
}

#[repr(C)]
pub struct hdac_device {
    pub bus: *mut hdac_bus,
    pub addr: u32,
    pub list: list_head,
    pub registered: bool,
    pub dev: device,
}

#[repr(C)]
pub struct hdac_driver {
    pub unsol_event: Option<unsafe extern "C" fn(*mut hdac_device, u32)>,
}

extern "C" {
    fn snd_hdac_bus_send_cmd(bus: *mut hdac_bus, cmd: u32) -> i32;
    fn snd_hdac_bus_get_response(bus: *mut hdac_bus, addr: u32, res: *mut u32) -> i32;
    fn snd_hdac_bus_link_power(codec: *mut hdac_device, enable: bool);

    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn INIT_WORK(work: *mut work_struct, func: unsafe extern "C" fn(*mut work_struct));
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn init_waitqueue_head(wq_head: *mut wait_queue_head_t);
    fn list_empty(head: *const list_head) -> i32;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn trace_hda_send_cmd(bus: *mut hdac_bus, cmd: u32);
    fn trace_hda_get_response(bus: *mut hdac_bus, addr: u32, res: u32);
    fn trace_hda_unsol_event(bus: *mut hdac_bus, res: u32, res_ex: u32);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn drv_to_hdac_driver(drv: *mut device_driver) -> *mut hdac_driver;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn set_bit(nr: u32, addr: *mut usize);
    fn list_del_init(entry: *mut list_head);
    fn clear_bit(nr: u32, addr: *mut usize);
    fn flush_work(work: *mut work_struct) -> bool;
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

unsafe fn WARN_ON(condition: bool) -> bool {
    condition
}

static default_ops: hdac_bus_ops = hdac_bus_ops {
    command: Some(snd_hdac_bus_send_cmd),
    get_response: Some(snd_hdac_bus_get_response),
    link_power: Some(snd_hdac_bus_link_power),
};

/**
 * snd_hdac_bus_init - initialize a HD-audio bas bus
 * @bus: the pointer to bus object
 * @dev: device pointer
 * @ops: bus verb operators
 *
 * Returns 0 if successful, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_init(
    bus: *mut hdac_bus,
    dev: *mut device,
    ops: *const hdac_bus_ops,
) -> i32 {
    memset(bus as *mut c_void, 0, mem::size_of_val(&*bus));
    (*bus).dev = dev;
    if !ops.is_null() {
        (*bus).ops = ops;
    } else {
        (*bus).ops = &default_ops;
    }
    (*bus).dma_type = SNDRV_DMA_TYPE_DEV;
    INIT_LIST_HEAD(&mut (*bus).stream_list);
    INIT_LIST_HEAD(&mut (*bus).codec_list);
    INIT_WORK(&mut (*bus).unsol_work, snd_hdac_bus_process_unsol_events);
    spin_lock_init(&mut (*bus).reg_lock);
    mutex_init(&mut (*bus).cmd_mutex);
    mutex_init(&mut (*bus).lock);
    INIT_LIST_HEAD(&mut (*bus).hlink_list);
    init_waitqueue_head(&mut (*bus).rirb_wq);
    (*bus).irq = -1;
    (*bus).addr_offset = 0;

    /*
     * Default value of '8' is as per the HD audio specification (Rev 1.0a).
     * Following relation is used to derive STRIPE control value.
     *  For sample rate <= 48K:
     *   { ((num_channels * bits_per_sample) / number of SDOs) >= 8 }
     *  For sample rate > 48K:
     *   { ((num_channels * bits_per_sample * rate/48000) /
     *	number of SDOs) >= 8 }
     */
    (*bus).sdo_limit = 8;

    0
}

/**
 * snd_hdac_bus_exit - clean up a HD-audio bas bus
 * @bus: the pointer to bus object
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_exit(bus: *mut hdac_bus) {
    WARN_ON(list_empty(&(*bus).stream_list) == 0);
    WARN_ON(list_empty(&(*bus).codec_list) == 0);
    cancel_work_sync(&mut (*bus).unsol_work);
}

/**
 * snd_hdac_bus_exec_verb - execute a HD-audio verb on the given bus
 * @bus: bus object
 * @addr: the HDAC device address
 * @cmd: HD-audio encoded verb
 * @res: pointer to store the response, NULL if performing asynchronously
 *
 * Returns 0 if successful, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_exec_verb(
    bus: *mut hdac_bus,
    addr: u32,
    cmd: u32,
    res: *mut u32,
) -> i32 {
    mutex_lock(&mut (*bus).cmd_mutex);
    let ret = snd_hdac_bus_exec_verb_unlocked(bus, addr, cmd, res);
    mutex_unlock(&mut (*bus).cmd_mutex);
    ret
}

/**
 * snd_hdac_bus_exec_verb_unlocked - unlocked version
 * @bus: bus object
 * @addr: the HDAC device address
 * @cmd: HD-audio encoded verb
 * @res: pointer to store the response, NULL if performing asynchronously
 *
 * Returns 0 if successful, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_exec_verb_unlocked(
    bus: *mut hdac_bus,
    addr: u32,
    cmd: u32,
    mut res: *mut u32,
) -> i32 {
    let mut tmp: u32 = 0;
    let mut err: i32;

    if cmd == !0u32 {
        return -EINVAL;
    }

    if !res.is_null() {
        *res = (-1i32) as u32;
    } else if (*bus).sync_write {
        res = &mut tmp;
    }
    loop {
        trace_hda_send_cmd(bus, cmd);
        err = ((*(*bus).ops).command.unwrap())(bus, cmd);
        if err != -EAGAIN {
            break;
        }
        /* process pending verbs */
        err = ((*(*bus).ops).get_response.unwrap())(bus, addr, &mut tmp);
        if err != 0 {
            break;
        }
    }
    if err == 0 && !res.is_null() {
        err = ((*(*bus).ops).get_response.unwrap())(bus, addr, res);
        trace_hda_get_response(bus, addr, *res);
    }
    err
}

/**
 * snd_hdac_bus_queue_event - add an unsolicited event to queue
 * @bus: the BUS
 * @res: unsolicited event (lower 32bit of RIRB entry)
 * @res_ex: codec addr and flags (upper 32bit or RIRB entry)
 *
 * Adds the given event to the queue.  The events are processed in
 * the workqueue asynchronously.  Call this function in the interrupt
 * hanlder when RIRB receives an unsolicited event.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_queue_event(bus: *mut hdac_bus, res: u32, res_ex: u32) {
    let mut wp: u32;

    if bus.is_null() {
        return;
    }

    trace_hda_unsol_event(bus, res, res_ex);
    wp = ((*bus).unsol_wp + 1) % HDA_UNSOL_QUEUE_SIZE;
    (*bus).unsol_wp = wp;

    wp <<= 1;
    (*bus).unsol_queue[wp as usize] = res;
    (*bus).unsol_queue[(wp + 1) as usize] = res_ex;

    schedule_work(&mut (*bus).unsol_work);
}

/*
 * process queued unsolicited events
 */
unsafe extern "C" fn snd_hdac_bus_process_unsol_events(work: *mut work_struct) {
    let bus = (work as *mut u8).sub(mem::offset_of!(hdac_bus, unsol_work)) as *mut hdac_bus;
    let mut codec: *mut hdac_device;
    let mut drv: *mut hdac_driver;
    let mut rp: u32;
    let mut caddr: u32;
    let mut res: u32;

    spin_lock_irq(&mut (*bus).reg_lock);
    while (*bus).unsol_rp != (*bus).unsol_wp {
        rp = ((*bus).unsol_rp + 1) % HDA_UNSOL_QUEUE_SIZE;
        (*bus).unsol_rp = rp;
        rp <<= 1;
        res = (*bus).unsol_queue[rp as usize];
        caddr = (*bus).unsol_queue[(rp + 1) as usize];
        if (caddr & (1 << 4)) == 0 {
            /* no unsolicited event? */
            continue;
        }
        codec = (*bus).caddr_tbl[(caddr & 0x0f) as usize];
        if codec.is_null() || !(*codec).registered {
            continue;
        }
        spin_unlock_irq(&mut (*bus).reg_lock);
        drv = drv_to_hdac_driver((*codec).dev.driver);
        if let Some(unsol_event) = (*drv).unsol_event {
            unsol_event(codec, res);
        }
        spin_lock_irq(&mut (*bus).reg_lock);
    }
    spin_unlock_irq(&mut (*bus).reg_lock);
}

/**
 * snd_hdac_bus_add_device - Add a codec to bus
 * @bus: HDA core bus
 * @codec: HDA core device to add
 *
 * Adds the given codec to the list in the bus.  The caddr_tbl array
 * and codec_powered bits are updated, as well.
 * Returns zero if success, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_add_device(
    bus: *mut hdac_bus,
    codec: *mut hdac_device,
) -> i32 {
    if !(*bus).caddr_tbl[(*codec).addr as usize].is_null() {
        dev_err(
            (*bus).dev,
            b"address 0x%x is already occupied\n\0".as_ptr() as *const i8,
            (*codec).addr,
        );
        return -EBUSY;
    }

    list_add_tail(&mut (*codec).list, &mut (*bus).codec_list);
    (*bus).caddr_tbl[(*codec).addr as usize] = codec;
    set_bit((*codec).addr, &mut (*bus).codec_powered);
    (*bus).num_codecs += 1;
    0
}

/**
 * snd_hdac_bus_remove_device - Remove a codec from bus
 * @bus: HDA core bus
 * @codec: HDA core device to remove
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_bus_remove_device(bus: *mut hdac_bus, codec: *mut hdac_device) {
    WARN_ON(bus != (*codec).bus);
    if list_empty(&(*codec).list) != 0 {
        return;
    }
    list_del_init(&mut (*codec).list);
    (*bus).caddr_tbl[(*codec).addr as usize] = ptr::null_mut();
    clear_bit((*codec).addr, &mut (*bus).codec_powered);
    (*bus).num_codecs -= 1;
    flush_work(&mut (*bus).unsol_work);
}

/*
 * CONFIG_SND_HDA_ALIGNED_MMIO:
 * Helpers for aligned read/write of mmio space, for Tegra.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_hdac_aligned_read(addr: *mut c_void, mask: u32) -> u32 {
    let aligned_addr = ((addr as usize) & !0x3usize) as *mut c_void;
    let shift = (((addr as usize) & 0x3usize) << 3) as u32;
    let v: u32;

    v = readl(aligned_addr);
    (v >> shift) & mask
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_aligned_write(val: u32, addr: *mut c_void, mask: u32) {
    let aligned_addr = ((addr as usize) & !0x3usize) as *mut c_void;
    let shift = (((addr as usize) & 0x3usize) << 3) as u32;
    let mut v: u32;

    v = readl(aligned_addr);
    v &= !(mask << shift);
    v |= val << shift;
    writel(v, aligned_addr);
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_codec_link_up(codec: *mut hdac_device) {
    let bus: *mut hdac_bus = (*codec).bus;

    if let Some(link_power) = (*(*bus).ops).link_power {
        link_power(codec, true);
    } else {
        snd_hdac_bus_link_power(codec, true);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_hdac_codec_link_down(codec: *mut hdac_device) {
    let bus: *mut hdac_bus = (*codec).bus;

    if let Some(link_power) = (*(*bus).ops).link_power {
        link_power(codec, false);
    } else {
        snd_hdac_bus_link_power(codec, false);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
