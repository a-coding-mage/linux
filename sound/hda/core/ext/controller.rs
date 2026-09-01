// SPDX-License-Identifier: GPL-2.0-only
/*
 *  hdac-ext-controller.c - HD-audio extended controller functions.
 *
 *  Copyright (C) 2014-2015 Intel Corp
 *  Author: Jeeja KP <jeeja.kp@intel.com>
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub ppcap: *mut c_void,
    pub mlcap: *mut c_void,
    pub hlink_list: list_head,
    pub idx: c_int,
    pub lock: mutex,
    pub cmd_dma_state: bool,
    pub codec_mask: c_ulong,
    pub codec_powered: c_ulong,
}

#[repr(C)]
pub struct hdac_ext_link {
    pub list: list_head,
    pub index: c_int,
    pub bus: *mut hdac_bus,
    pub ml_addr: *mut c_void,
    pub lcaps: u32,
    pub lsdiid: u16,
    pub slcount: u32,
    pub id: u32,
    pub ref_count: c_int,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub bus: *mut hdac_bus,
    pub addr: c_int,
}

unsafe extern "C" {
    static AZX_REG_PP_PPCTL: usize;
    static AZX_PPCTL_GPROCEN: u32;
    static AZX_PPCTL_PIE: u32;
    static AZX_REG_ML_MLCD: usize;
    static AZX_ML_BASE: usize;
    static AZX_ML_INTERVAL: usize;
    static AZX_REG_ML_LCAP: usize;
    static AZX_REG_ML_LSDIID: usize;
    static AZX_ML_HDA_LCAP_SLCOUNT: u32;
    static AZX_REG_ML_LEPTR: usize;
    static AZX_REG_ML_LEPTR_ID: u32;
    static AZX_REG_ML_LCTL: usize;
    static AZX_ML_LCTL_CPA_SHIFT: c_int;
    static AZX_ML_LCTL_SPA: u32;
    static AZX_REG_ML_LOSIDV: usize;
    static AZX_ML_LOSIDV_STREAM_MASK: u16;
    static STATESTS: c_int;
    static ENOMEM: c_int;
    static EIO: c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn readl(addr: *mut c_void) -> u32;
    fn readw(addr: *mut c_void) -> u16;
    fn snd_hdac_updatel(addr: *mut c_void, reg: usize, mask: u32, val: u32);
    fn snd_hdac_updatew(addr: *mut c_void, reg: usize, mask: u16, val: u16);
    fn hdac_ext_link_alt(hlink: *mut hdac_ext_link) -> bool;
    fn udelay(usecs: c_ulong);
    fn snd_hdac_bus_init_cmd_io(bus: *mut hdac_bus);
    fn snd_hdac_bus_stop_cmd_io(bus: *mut hdac_bus);
    fn snd_hdac_chip_readw(bus: *mut hdac_bus, reg: c_int) -> c_ulong;
    fn snd_hdac_chip_writew(bus: *mut hdac_bus, reg: c_int, val: c_ulong);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn test_bit(nr: c_int, addr: *mut c_ulong) -> bool;
    fn snd_hdac_bus_link_power(codec: *mut hdac_device, enable: bool);
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn kzalloc_obj_hdac_ext_link() -> *mut hdac_ext_link;
    fn kfree(ptr: *mut c_void);
    fn list_empty(head: *mut list_head) -> bool;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);

    /*
     * External Rust equivalents for Linux list_for_each_entry/list_first_entry
     * are required by the translated code below.
     */
    fn list_first_entry_hdac_ext_link(head: *mut list_head) -> *mut hdac_ext_link;
    fn list_next_entry_hdac_ext_link(pos: *mut hdac_ext_link) -> *mut hdac_ext_link;
    fn list_entry_is_head_hdac_ext_link(pos: *mut hdac_ext_link, head: *mut list_head) -> bool;
}

#[inline]
unsafe fn field_get(mask: u32, reg: u32) -> u32 {
    let shift = mask.trailing_zeros();
    (reg & mask) >> shift
}

#[inline]
unsafe fn byte_add(addr: *mut c_void, offset: usize) -> *mut c_void {
    (addr as *mut u8).add(offset) as *mut c_void
}

/*
 * processing pipe helpers - these helpers are useful for dealing with HDA
 * new capability of processing pipelines
 */

/**
 * snd_hdac_ext_bus_ppcap_enable - enable/disable processing pipe capability
 * @bus: the pointer to HDAC bus object
 * @enable: flag to turn on/off the capability
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_ppcap_enable(bus: *mut hdac_bus, enable: bool) {
    if (*bus).ppcap.is_null() {
        dev_err((*bus).dev, c"Address of PP capability is NULL".as_ptr());
        return;
    }

    if enable {
        snd_hdac_updatel(
            (*bus).ppcap,
            AZX_REG_PP_PPCTL,
            AZX_PPCTL_GPROCEN,
            AZX_PPCTL_GPROCEN,
        );
    } else {
        snd_hdac_updatel((*bus).ppcap, AZX_REG_PP_PPCTL, AZX_PPCTL_GPROCEN, 0);
    }
}

/**
 * snd_hdac_ext_bus_ppcap_int_enable - ppcap interrupt enable/disable
 * @bus: the pointer to HDAC bus object
 * @enable: flag to enable/disable interrupt
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_ppcap_int_enable(bus: *mut hdac_bus, enable: bool) {
    if (*bus).ppcap.is_null() {
        dev_err((*bus).dev, c"Address of PP capability is NULL\n".as_ptr());
        return;
    }

    if enable {
        snd_hdac_updatel((*bus).ppcap, AZX_REG_PP_PPCTL, AZX_PPCTL_PIE, AZX_PPCTL_PIE);
    } else {
        snd_hdac_updatel((*bus).ppcap, AZX_REG_PP_PPCTL, AZX_PPCTL_PIE, 0);
    }
}

/*
 * Multilink helpers - these helpers are useful for dealing with HDA
 * new multilink capability
 */

/**
 * snd_hdac_ext_bus_get_ml_capabilities - get multilink capability
 * @bus: the pointer to HDAC bus object
 *
 * This will parse all links and read the mlink capabilities and add them
 * in hlink_list of extended hdac bus
 * Note: this will be freed on bus exit by driver
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_get_ml_capabilities(bus: *mut hdac_bus) -> c_int {
    let mut idx: c_int;
    let link_count: u32;
    let mut hlink: *mut hdac_ext_link;
    let leptr: u32;

    link_count = readl(byte_add((*bus).mlcap, AZX_REG_ML_MLCD)) + 1;

    dev_dbg(
        (*bus).dev,
        c"In %s Link count: %d\n".as_ptr(),
        c"snd_hdac_ext_bus_get_ml_capabilities".as_ptr(),
        link_count,
    );

    idx = 0;
    while (idx as u32) < link_count {
        hlink = kzalloc_obj_hdac_ext_link();
        if hlink.is_null() {
            return -ENOMEM;
        }
        (*hlink).index = idx;
        (*hlink).bus = bus;
        (*hlink).ml_addr = byte_add(
            (*bus).mlcap,
            AZX_ML_BASE + (AZX_ML_INTERVAL * idx as usize),
        );
        (*hlink).lcaps = readl(byte_add((*hlink).ml_addr, AZX_REG_ML_LCAP));
        (*hlink).lsdiid = readw(byte_add((*hlink).ml_addr, AZX_REG_ML_LSDIID));
        (*hlink).slcount = field_get(AZX_ML_HDA_LCAP_SLCOUNT, (*hlink).lcaps) + 1;

        if hdac_ext_link_alt(hlink) {
            leptr = readl(byte_add((*hlink).ml_addr, AZX_REG_ML_LEPTR));
            (*hlink).id = field_get(AZX_REG_ML_LEPTR_ID, leptr);
        }

        /* since link in On, update the ref */
        (*hlink).ref_count = 1;

        list_add_tail(&mut (*hlink).list, &mut (*bus).hlink_list);
        idx += 1;
    }

    0
}

/**
 * snd_hdac_ext_link_free_all- free hdac extended link objects
 *
 * @bus: the pointer to HDAC bus object
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_link_free_all(bus: *mut hdac_bus) {
    let mut hlink: *mut hdac_ext_link;

    while !list_empty(&mut (*bus).hlink_list) {
        hlink = list_first_entry_hdac_ext_link(&mut (*bus).hlink_list);
        list_del(&mut (*hlink).list);
        kfree(hlink as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_get_hlink_by_id(
    bus: *mut hdac_bus,
    id: u32,
) -> *mut hdac_ext_link {
    let mut hlink: *mut hdac_ext_link;

    hlink = list_first_entry_hdac_ext_link(&mut (*bus).hlink_list);
    while !list_entry_is_head_hdac_ext_link(hlink, &mut (*bus).hlink_list) {
        if hdac_ext_link_alt(hlink) && (*hlink).id == id {
            return hlink;
        }
        hlink = list_next_entry_hdac_ext_link(hlink);
    }
    ptr::null_mut()
}

/**
 * snd_hdac_ext_bus_get_hlink_by_addr - get hlink at specified address
 * @bus: hlink's parent bus device
 * @addr: codec device address
 *
 * Returns hlink object or NULL if matching hlink is not found.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_get_hlink_by_addr(
    bus: *mut hdac_bus,
    addr: c_int,
) -> *mut hdac_ext_link {
    let mut hlink: *mut hdac_ext_link;

    hlink = list_first_entry_hdac_ext_link(&mut (*bus).hlink_list);
    while !list_entry_is_head_hdac_ext_link(hlink, &mut (*bus).hlink_list) {
        if ((*hlink).lsdiid as c_int) & (0x1 << addr) != 0 {
            return hlink;
        }
        hlink = list_next_entry_hdac_ext_link(hlink);
    }
    ptr::null_mut()
}

/**
 * snd_hdac_ext_bus_get_hlink_by_name - get hlink based on codec name
 * @bus: the pointer to HDAC bus object
 * @codec_name: codec name
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_get_hlink_by_name(
    bus: *mut hdac_bus,
    codec_name: *const c_char,
) -> *mut hdac_ext_link {
    let mut bus_idx: c_int = 0;
    let mut addr: c_int = 0;

    if sscanf(codec_name, c"ehdaudio%dD%d".as_ptr(), &mut bus_idx, &mut addr) != 2 {
        return ptr::null_mut();
    }
    if (*bus).idx != bus_idx {
        return ptr::null_mut();
    }
    if addr < 0 || addr > 31 {
        return ptr::null_mut();
    }

    snd_hdac_ext_bus_get_hlink_by_addr(bus, addr)
}

unsafe fn check_hdac_link_power_active(hlink: *mut hdac_ext_link, enable: bool) -> c_int {
    let mut timeout: c_int;
    let mut val: u32;
    let mask: c_int = 1 << AZX_ML_LCTL_CPA_SHIFT;

    udelay(3);
    timeout = 150;

    loop {
        val = readl(byte_add((*hlink).ml_addr, AZX_REG_ML_LCTL));
        if enable {
            if ((val & mask as u32) >> AZX_ML_LCTL_CPA_SHIFT) != 0 {
                return 0;
            }
        } else if ((val & mask as u32) >> AZX_ML_LCTL_CPA_SHIFT) == 0 {
            return 0;
        }
        udelay(3);
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }

    -EIO
}

/**
 * snd_hdac_ext_bus_link_power_up -power up hda link
 * @hlink: HD-audio extended link
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_up(hlink: *mut hdac_ext_link) -> c_int {
    snd_hdac_updatel(
        (*hlink).ml_addr,
        AZX_REG_ML_LCTL,
        AZX_ML_LCTL_SPA,
        AZX_ML_LCTL_SPA,
    );

    check_hdac_link_power_active(hlink, true)
}

/**
 * snd_hdac_ext_bus_link_power_down -power down hda link
 * @hlink: HD-audio extended link
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_down(hlink: *mut hdac_ext_link) -> c_int {
    snd_hdac_updatel((*hlink).ml_addr, AZX_REG_ML_LCTL, AZX_ML_LCTL_SPA, 0);

    check_hdac_link_power_active(hlink, false)
}

/**
 * snd_hdac_ext_bus_link_power_up_all -power up all hda link
 * @bus: the pointer to HDAC bus object
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_up_all(bus: *mut hdac_bus) -> c_int {
    let mut hlink: *mut hdac_ext_link;
    let mut ret: c_int;

    hlink = list_first_entry_hdac_ext_link(&mut (*bus).hlink_list);
    while !list_entry_is_head_hdac_ext_link(hlink, &mut (*bus).hlink_list) {
        ret = snd_hdac_ext_bus_link_power_up(hlink);
        if ret < 0 {
            return ret;
        }
        hlink = list_next_entry_hdac_ext_link(hlink);
    }

    0
}

/**
 * snd_hdac_ext_bus_link_power_down_all -power down all hda link
 * @bus: the pointer to HDAC bus object
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power_down_all(bus: *mut hdac_bus) -> c_int {
    let mut hlink: *mut hdac_ext_link;
    let mut ret: c_int;

    hlink = list_first_entry_hdac_ext_link(&mut (*bus).hlink_list);
    while !list_entry_is_head_hdac_ext_link(hlink, &mut (*bus).hlink_list) {
        ret = snd_hdac_ext_bus_link_power_down(hlink);
        if ret < 0 {
            return ret;
        }
        hlink = list_next_entry_hdac_ext_link(hlink);
    }

    0
}

/**
 * snd_hdac_ext_bus_link_set_stream_id - maps stream id to link output
 * @link: HD-audio ext link to set up
 * @stream: stream id
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_set_stream_id(
    link: *mut hdac_ext_link,
    stream: c_int,
) {
    snd_hdac_updatew(
        (*link).ml_addr,
        AZX_REG_ML_LOSIDV,
        (1_u16) << stream,
        (1_u16) << stream,
    );
}

/**
 * snd_hdac_ext_bus_link_clear_stream_id - maps stream id to link output
 * @link: HD-audio ext link to set up
 * @stream: stream id
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_clear_stream_id(
    link: *mut hdac_ext_link,
    stream: c_int,
) {
    snd_hdac_updatew((*link).ml_addr, AZX_REG_ML_LOSIDV, (1_u16) << stream, 0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_get(
    bus: *mut hdac_bus,
    hlink: *mut hdac_ext_link,
) -> c_int {
    let mut codec_mask: c_ulong;
    let mut ret: c_int = 0;

    mutex_lock(&mut (*bus).lock);

    /*
     * if we move from 0 to 1, count will be 1 so power up this link
     * as well, also check the dma status and trigger that
     */
    (*hlink).ref_count += 1;
    if (*hlink).ref_count == 1 {
        if !(*bus).cmd_dma_state {
            snd_hdac_bus_init_cmd_io(bus);
            (*bus).cmd_dma_state = true;
        }

        ret = snd_hdac_ext_bus_link_power_up(hlink);

        /*
         * clear the register to invalidate all the output streams
         */
        snd_hdac_updatew(
            (*hlink).ml_addr,
            AZX_REG_ML_LOSIDV,
            AZX_ML_LOSIDV_STREAM_MASK,
            0,
        );
        /*
         *  wait for 521usec for codec to report status
         *  HDA spec section 4.3 - Codec Discovery
         */
        udelay(521);
        codec_mask = snd_hdac_chip_readw(bus, STATESTS);
        dev_dbg((*bus).dev, c"codec_mask = 0x%lx\n".as_ptr(), codec_mask);
        snd_hdac_chip_writew(bus, STATESTS, codec_mask);
        if (*bus).codec_mask == 0 {
            (*bus).codec_mask = codec_mask;
        }
    }

    mutex_unlock(&mut (*bus).lock);

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_put(
    bus: *mut hdac_bus,
    hlink: *mut hdac_ext_link,
) -> c_int {
    let mut ret: c_int = 0;
    let mut hlink_tmp: *mut hdac_ext_link;
    let mut link_up: bool = false;

    mutex_lock(&mut (*bus).lock);

    /*
     * if we move from 1 to 0, count will be 0
     * so power down this link as well
     */
    (*hlink).ref_count -= 1;
    if (*hlink).ref_count == 0 {
        ret = snd_hdac_ext_bus_link_power_down(hlink);

        /*
         * now check if all links are off, if so turn off
         * cmd dma as well
         */
        hlink_tmp = list_first_entry_hdac_ext_link(&mut (*bus).hlink_list);
        while !list_entry_is_head_hdac_ext_link(hlink_tmp, &mut (*bus).hlink_list) {
            if (*hlink_tmp).ref_count != 0 {
                link_up = true;
                break;
            }
            hlink_tmp = list_next_entry_hdac_ext_link(hlink_tmp);
        }

        if !link_up {
            snd_hdac_bus_stop_cmd_io(bus);
            (*bus).cmd_dma_state = false;
        }
    }

    mutex_unlock(&mut (*bus).lock);

    ret
}

unsafe fn hdac_ext_codec_link_up(codec: *mut hdac_device) {
    let devname: *const c_char = dev_name(&mut (*codec).dev);
    let hlink: *mut hdac_ext_link =
        snd_hdac_ext_bus_get_hlink_by_name((*codec).bus, devname);

    if !hlink.is_null() {
        snd_hdac_ext_bus_link_get((*codec).bus, hlink);
    }
}

unsafe fn hdac_ext_codec_link_down(codec: *mut hdac_device) {
    let devname: *const c_char = dev_name(&mut (*codec).dev);
    let hlink: *mut hdac_ext_link =
        snd_hdac_ext_bus_get_hlink_by_name((*codec).bus, devname);

    if !hlink.is_null() {
        snd_hdac_ext_bus_link_put((*codec).bus, hlink);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hdac_ext_bus_link_power(codec: *mut hdac_device, enable: bool) {
    let bus: *mut hdac_bus = (*codec).bus;
    let oldstate: bool = test_bit((*codec).addr, &mut (*bus).codec_powered);

    if enable == oldstate {
        return;
    }

    snd_hdac_bus_link_power(codec, enable);

    if enable {
        hdac_ext_codec_link_up(codec);
    } else {
        hdac_ext_codec_link_down(codec);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
