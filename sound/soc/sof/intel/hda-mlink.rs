// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2022 Intel Corporation
//

/*
 * Management of HDaudio multi-link (capabilities, power, coupling)
 */

// Rust translation of include dependencies:
// <sound/hdaudio_ext.h>, <sound/hda_register.h>, <sound/hda-mlink.h>
// <linux/bitfield.h>, <linux/module.h>, <linux/string_choices.h>

// Original C condition: #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_MLINK)

use core::ffi::c_void;
use core::ptr::null_mut;

type u16 = u16;
type u32 = u32;

extern "C" {
    static AZX_ML_HDA_LCAP_ALT: u32;
    static AZX_ML_HDA_LCAP_INTC: u32;
    static AZX_ML_HDA_LCAP_OFLS: u32;
    static AZX_ML_HDA_LCAP_LSS: u32;
    static AZX_ML_HDA_LCAP_SLCOUNT: u32;
    static AZX_REG_ML_LEPTR_ID: u32;
    static AZX_REG_ML_LEPTR_PTR: u32;
    static AZX_REG_ML_LEPTR_ID_SDW: i32;
    static AZX_REG_ML_LEPTR_ID_INTEL_DMIC: i32;
    static AZX_REG_ML_LEPTR_ID_INTEL_SSP: i32;
    static AZX_REG_ML_LEPTR_ID_INTEL_UAOL: i32;
    static AZX_REG_ML_LSYNC_SYNCPRD: u32;
    static AZX_REG_ML_LSYNC_SYNCPU: u32;
    static AZX_REG_ML_LSYNC_CMDSYNC: u32;
    static AZX_REG_ML_LSYNC_SYNCGO: u32;
    static AZX_REG_ML_LSYNC_CMDSYNC_SHIFT: i32;
    static AZX_ML_LCTL_INTEN: u32;
    static AZX_ML_LCTL_INTSTS: u32;
    static AZX_ML_LCTL_OFLEN: u32;
    static AZX_ML_LCTL_CPA_SHIFT: i32;
    static AZX_ML_LCTL_SPA_SHIFT: i32;
    static AZX_REG_ML_LCAP: isize;
    static AZX_REG_ML_LSDIID: isize;
    static AZX_REG_ML_LEPTR: isize;
    static AZX_REG_ML_LCTL: isize;
    static AZX_REG_ML_LSYNC: isize;
    static AZX_REG_ML_MLCD: isize;
    static AZX_REG_ML_LOSIDV: isize;
    static AZX_ML_BASE: isize;
    static AZX_ML_INTERVAL: isize;
    static EINVAL: i32;
    static EIO: i32;
    static EAGAIN: i32;
    static ENOMEM: i32;
    static ENODEV: i32;
    static HDA_BUS_ML_LINK_HDA: hda_bus_ml_link_type;
    static HDA_BUS_ML_LINK_SDW: hda_bus_ml_link_type;
    static HDA_BUS_ML_LINK_UAOL: hda_bus_ml_link_type;
    static HDA_BUS_ML_LINK_OTHER: hda_bus_ml_link_type;

    fn readl(addr: *const c_void) -> u32;
    fn readw(addr: *const c_void) -> u16;
    fn writel(val: u32, addr: *mut c_void);
    fn writew(val: u16, addr: *mut c_void);
    fn usleep_range(min: u32, max: u32);
    fn mutex_init(lock: *mut mutex);
    fn mutex_destroy(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn kzalloc_obj_hdac_ext2_link() -> *mut hdac_ext2_link;
    fn kfree(ptr: *mut c_void);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
    fn snd_hdac_ext_bus_link_power_up(hlink: *mut hdac_ext_link) -> i32;
    fn snd_hdac_ext_bus_link_power_down(hlink: *mut hdac_ext_link) -> i32;
    fn u16p_replace_bits(ptr: *mut u16, val: i32, mask: u16);
    fn __fls(word: i32) -> i32;
    fn __ffs(word: i32) -> i32;
    fn AZX_REG_ML_LSDIID_OFFSET(sublink: i32) -> isize;
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
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hdac_bus {
    pub mlcap: *mut c_void,
    pub hlink_list: list_head,
    pub dev: *mut device,
    pub remap_addr: *mut c_void,
}

#[repr(C)]
pub struct hdac_ext_link {
    pub list: list_head,
    pub index: i32,
    pub bus: *mut hdac_bus,
    pub ml_addr: *mut c_void,
    pub lcaps: u32,
    pub lsdiid: u16,
    pub ref_count: i32,
}

#[repr(C)]
pub struct hda_bus_ml_link_type {
    _private: [u8; 0],
}

/* worst-case number of sublinks is used for sublink refcount array allocation only */
const HDAML_MAX_SUBLINKS: usize = 32;

/**
 * struct hdac_ext2_link - HDAudio extended+alternate link
 *
 * @hext_link:		hdac_ext_link
 * @alt:		flag set for alternate extended links
 * @intc:		boolean for interrupt capable
 * @ofls:		boolean for offload support
 * @lss:		boolean for link synchronization capabilities
 * @slcount:		sublink count
 * @elid:		extended link ID (AZX_REG_ML_LEPTR_ID_ defines)
 * @elver:		extended link version
 * @leptr:		extended link pointer
 * @eml_lock:		mutual exclusion to access shared registers e.g. CPA/SPA bits
 * in LCTL register
 * @sublink_ref_count:	array of refcounts, required to power-manage sublinks independently
 * @base_ptr:		pointer to shim/ip/shim_vs space
 * @instance_offset:	offset between each of @slcount instances managed by link
 * @shim_offset:	offset to SHIM register base
 * @ip_offset:		offset to IP register base
 * @shim_vs_offset:	offset to vendor-specific (VS) SHIM base
 * @mic_privacy_mask:	bitmask of sublinks where mic privacy is applied
 */
#[repr(C)]
pub struct hdac_ext2_link {
    pub hext_link: hdac_ext_link,

    /* read directly from LCAP register */
    pub alt: bool,
    pub intc: bool,
    pub ofls: bool,
    pub lss: bool,
    pub slcount: i32,
    pub elid: i32,
    pub elver: i32,
    pub leptr: u32,

    pub eml_lock: mutex, /* prevent concurrent access to e.g. CPA/SPA */
    pub sublink_ref_count: [i32; HDAML_MAX_SUBLINKS],

    /* internal values computed from LCAP contents */
    pub base_ptr: *mut c_void,
    pub instance_offset: u32,
    pub shim_offset: u32,
    pub ip_offset: u32,
    pub shim_vs_offset: u32,

    pub mic_privacy_mask: usize,
}

unsafe fn hdac_ext_link_to_ext2(h: *mut hdac_ext_link) -> *mut hdac_ext2_link {
    h as *mut hdac_ext2_link
}

const AZX_REG_SDW_INSTANCE_OFFSET: u32 = 0x8000;
const AZX_REG_SDW_SHIM_OFFSET: u32 = 0x0;
const AZX_REG_SDW_IP_OFFSET: u32 = 0x100;
const AZX_REG_SDW_VS_SHIM_OFFSET: u32 = 0x6000;
fn AZX_REG_SDW_SHIM_PCMSyCM(y: i32) -> u32 {
    0x16 + 0x4 * y as u32
}

/* only one instance supported */
const AZX_REG_INTEL_DMIC_SHIM_OFFSET: u32 = 0x0;
const AZX_REG_INTEL_DMIC_IP_OFFSET: u32 = 0x100;
const AZX_REG_INTEL_DMIC_VS_SHIM_OFFSET: u32 = 0x6000;

const AZX_REG_INTEL_SSP_INSTANCE_OFFSET: u32 = 0x1000;
const AZX_REG_INTEL_SSP_SHIM_OFFSET: u32 = 0x0;
const AZX_REG_INTEL_SSP_IP_OFFSET: u32 = 0x100;
const AZX_REG_INTEL_SSP_VS_SHIM_OFFSET: u32 = 0xC00;

/* only one instance supported */
const AZX_REG_INTEL_UAOL_SHIM_OFFSET: u32 = 0x0;
const AZX_REG_INTEL_UAOL_IP_OFFSET: u32 = 0x100;
const AZX_REG_INTEL_UAOL_VS_SHIM_OFFSET: u32 = 0xC00;

/* Microphone privacy */
const AZX_REG_INTEL_VS_SHIM_PVCCS: u32 = 0x10;
const AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHGIE: u32 = 1 << 0;
const AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHG: u32 = 1 << 8;
const AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTS: u32 = 1 << 9;
const AZX_REG_INTEL_VS_SHIM_PVCCS_FMDIS: u32 = 1 << 10;

fn BIT(nr: i32) -> u32 {
    1u32 << nr
}

fn GENMASK(h: i32, l: i32) -> u32 {
    (!0u32 << l) & (!0u32 >> (31 - h))
}

unsafe fn FIELD_GET(mask: u32, reg: u32) -> u32 {
    (reg & mask) >> mask.trailing_zeros()
}

unsafe fn ptr_add(base: *mut c_void, offset: isize) -> *mut c_void {
    (base as *mut u8).offset(offset) as *mut c_void
}

/* HDAML section - this part follows sequences in the hardware specification,
 * including naming conventions and the use of the hdaml_ prefix.
 * The code is intentionally minimal with limited dependencies on frameworks or
 * helpers. Locking and scanning lists is handled at a higher level
 */

unsafe fn hdaml_lnk_enum(
    dev: *mut device,
    h2link: *mut hdac_ext2_link,
    remap_addr: *mut c_void,
    ml_addr: *mut c_void,
    link_idx: i32,
) -> i32 {
    let hlink = &mut (*h2link).hext_link as *mut hdac_ext_link;
    let mut base_offset: u32;

    (*hlink).lcaps = readl(ptr_add(ml_addr, AZX_REG_ML_LCAP));

    (*h2link).alt = FIELD_GET(AZX_ML_HDA_LCAP_ALT, (*hlink).lcaps) != 0;

    /* handle alternate extensions */
    if !(*h2link).alt {
        (*h2link).slcount = 1;

        /*
         * LSDIID is initialized by hardware for HDaudio link,
         * it needs to be setup by software for alternate links
         */
        (*hlink).lsdiid = readw(ptr_add(ml_addr, AZX_REG_ML_LSDIID));

        dev_dbg(dev, b"Link %d: HDAudio - lsdiid=%d\n\0".as_ptr(), link_idx, (*hlink).lsdiid as i32);

        return 0;
    }

    (*h2link).intc = FIELD_GET(AZX_ML_HDA_LCAP_INTC, (*hlink).lcaps) != 0;
    (*h2link).ofls = FIELD_GET(AZX_ML_HDA_LCAP_OFLS, (*hlink).lcaps) != 0;
    (*h2link).lss = FIELD_GET(AZX_ML_HDA_LCAP_LSS, (*hlink).lcaps) != 0;

    /* read slcount (increment due to zero-based hardware representation */
    (*h2link).slcount = FIELD_GET(AZX_ML_HDA_LCAP_SLCOUNT, (*hlink).lcaps) as i32 + 1;
    dev_dbg(dev, b"Link %d: HDAudio extended - sublink count %d\n\0".as_ptr(), link_idx, (*h2link).slcount);

    /* find IP ID and offsets */
    (*h2link).leptr = readl(ptr_add(ml_addr, AZX_REG_ML_LEPTR));

    (*h2link).elid = FIELD_GET(AZX_REG_ML_LEPTR_ID, (*h2link).leptr) as i32;

    base_offset = FIELD_GET(AZX_REG_ML_LEPTR_PTR, (*h2link).leptr);
    (*h2link).base_ptr = ptr_add(remap_addr, base_offset as isize);

    match (*h2link).elid {
        x if x == AZX_REG_ML_LEPTR_ID_SDW => {
            (*h2link).instance_offset = AZX_REG_SDW_INSTANCE_OFFSET;
            (*h2link).shim_offset = AZX_REG_SDW_SHIM_OFFSET;
            (*h2link).ip_offset = AZX_REG_SDW_IP_OFFSET;
            (*h2link).shim_vs_offset = AZX_REG_SDW_VS_SHIM_OFFSET;
            dev_dbg(dev, b"Link %d: HDAudio extended - SoundWire alternate link, leptr.ptr %#x\n\0".as_ptr(), link_idx, base_offset);
        }
        x if x == AZX_REG_ML_LEPTR_ID_INTEL_DMIC => {
            (*h2link).shim_offset = AZX_REG_INTEL_DMIC_SHIM_OFFSET;
            (*h2link).ip_offset = AZX_REG_INTEL_DMIC_IP_OFFSET;
            (*h2link).shim_vs_offset = AZX_REG_INTEL_DMIC_VS_SHIM_OFFSET;
            dev_dbg(dev, b"Link %d: HDAudio extended - INTEL DMIC alternate link, leptr.ptr %#x\n\0".as_ptr(), link_idx, base_offset);
        }
        x if x == AZX_REG_ML_LEPTR_ID_INTEL_SSP => {
            (*h2link).instance_offset = AZX_REG_INTEL_SSP_INSTANCE_OFFSET;
            (*h2link).shim_offset = AZX_REG_INTEL_SSP_SHIM_OFFSET;
            (*h2link).ip_offset = AZX_REG_INTEL_SSP_IP_OFFSET;
            (*h2link).shim_vs_offset = AZX_REG_INTEL_SSP_VS_SHIM_OFFSET;
            dev_dbg(dev, b"Link %d: HDAudio extended - INTEL SSP alternate link, leptr.ptr %#x\n\0".as_ptr(), link_idx, base_offset);
        }
        x if x == AZX_REG_ML_LEPTR_ID_INTEL_UAOL => {
            (*h2link).shim_offset = AZX_REG_INTEL_UAOL_SHIM_OFFSET;
            (*h2link).ip_offset = AZX_REG_INTEL_UAOL_IP_OFFSET;
            (*h2link).shim_vs_offset = AZX_REG_INTEL_UAOL_VS_SHIM_OFFSET;
            dev_dbg(dev, b"Link %d: HDAudio extended - INTEL UAOL alternate link, leptr.ptr %#x\n\0".as_ptr(), link_idx, base_offset);
        }
        _ => {
            dev_err(dev, b"Link %d: HDAudio extended - Unsupported alternate link, leptr.id=%#02x value\n\0".as_ptr(), link_idx, (*h2link).elid);
            return -EINVAL;
        }
    }
    0
}

/*
 * Hardware recommendations are to wait ~10us before checking any hardware transition
 * reported by bits changing status.
 * This value does not need to be super-precise, a slack of 5us is perfectly acceptable.
 * The worst-case is about 1ms before reporting an issue
 */
const HDAML_POLL_DELAY_MIN_US: u32 = 10;
const HDAML_POLL_DELAY_SLACK_US: u32 = 5;
const HDAML_POLL_DELAY_RETRY: i32 = 100;

unsafe fn check_sublink_power(lctl: *mut u32, sublink: i32, enabled: bool) -> i32 {
    let mask = BIT(sublink) << AZX_ML_LCTL_CPA_SHIFT;
    let mut retry = HDAML_POLL_DELAY_RETRY;
    let mut val: u32;

    usleep_range(HDAML_POLL_DELAY_MIN_US, HDAML_POLL_DELAY_MIN_US + HDAML_POLL_DELAY_SLACK_US);
    loop {
        val = readl(lctl as *const c_void);
        if enabled {
            if val & mask != 0 {
                return 0;
            }
        } else if val & mask == 0 {
            return 0;
        }
        usleep_range(HDAML_POLL_DELAY_MIN_US, HDAML_POLL_DELAY_MIN_US + HDAML_POLL_DELAY_SLACK_US);

        retry -= 1;
        if retry == 0 {
            break;
        }
    }

    -EIO
}

unsafe fn hdaml_link_init(lctl: *mut u32, sublink: i32) -> i32 {
    let mut val: u32;
    let mask = BIT(sublink) << AZX_ML_LCTL_SPA_SHIFT;

    val = readl(lctl as *const c_void);
    val |= mask;

    writel(val, lctl as *mut c_void);

    check_sublink_power(lctl, sublink, true)
}

unsafe fn hdaml_link_shutdown(lctl: *mut u32, sublink: i32) -> i32 {
    let mut val: u32;
    let mask: u32;

    val = readl(lctl as *const c_void);
    mask = BIT(sublink) << AZX_ML_LCTL_SPA_SHIFT;
    val &= !mask;

    writel(val, lctl as *mut c_void);

    check_sublink_power(lctl, sublink, false)
}

unsafe fn hdaml_link_enable_interrupt(lctl: *mut u32, enable: bool) {
    let mut val: u32;

    val = readl(lctl as *const c_void);
    if enable {
        val |= AZX_ML_LCTL_INTEN;
    } else {
        val &= !AZX_ML_LCTL_INTEN;
    }

    writel(val, lctl as *mut c_void);
}

unsafe fn hdaml_link_check_interrupt(lctl: *mut u32) -> bool {
    let val: u32;

    val = readl(lctl as *const c_void);

    val & AZX_ML_LCTL_INTSTS != 0
}

unsafe fn hdaml_wait_bit(base: *mut c_void, offset: i32, mask: u32, target: u32) -> i32 {
    let mut timeout = HDAML_POLL_DELAY_RETRY;
    let mut reg_read: u32;

    loop {
        reg_read = readl(ptr_add(base, offset as isize));
        if (reg_read & mask) == target {
            return 0;
        }

        timeout -= 1;
        usleep_range(HDAML_POLL_DELAY_MIN_US, HDAML_POLL_DELAY_MIN_US + HDAML_POLL_DELAY_SLACK_US);
        if timeout == 0 {
            break;
        }
    }

    -EAGAIN
}

unsafe fn hdaml_link_set_syncprd(lsync: *mut u32, syncprd: u32) {
    let mut val: u32;

    val = readl(lsync as *const c_void);
    val &= !AZX_REG_ML_LSYNC_SYNCPRD;
    val |= syncprd & AZX_REG_ML_LSYNC_SYNCPRD;

    /*
     * set SYNCPU but do not wait. The bit is cleared by hardware when
     * the link becomes active.
     */
    val |= AZX_REG_ML_LSYNC_SYNCPU;

    writel(val, lsync as *mut c_void);
}

unsafe fn hdaml_link_wait_syncpu(lsync: *mut u32) -> i32 {
    hdaml_wait_bit(lsync as *mut c_void, 0, AZX_REG_ML_LSYNC_SYNCPU, 0)
}

unsafe fn hdaml_link_sync_arm(lsync: *mut u32, sublink: i32) {
    let mut val: u32;

    val = readl(lsync as *const c_void);
    val |= AZX_REG_ML_LSYNC_CMDSYNC << sublink;

    writel(val, lsync as *mut c_void);
}

unsafe fn hdaml_link_sync_go(lsync: *mut u32) {
    let mut val: u32;

    val = readl(lsync as *const c_void);
    val |= AZX_REG_ML_LSYNC_SYNCGO;

    writel(val, lsync as *mut c_void);
}

unsafe fn hdaml_link_check_cmdsync(lsync: *mut u32, cmdsync_mask: u32) -> bool {
    let val: u32;

    val = readl(lsync as *const c_void);

    (val & cmdsync_mask) != 0
}

unsafe fn hdaml_link_get_lsdiid(lsdiid: *mut u16) -> u16 {
    readw(lsdiid as *const c_void)
}

unsafe fn hdaml_link_set_lsdiid(lsdiid: *mut u16, dev_num: i32) {
    let mut val: u16;

    val = readw(lsdiid as *const c_void);
    val |= BIT(dev_num) as u16;

    writew(val, lsdiid as *mut c_void);
}

unsafe fn hdaml_shim_map_stream_ch(
    pcmsycm: *mut u16,
    lchan: i32,
    hchan: i32,
    stream_id: i32,
    dir: i32,
) {
    let mut val: u16;

    val = readw(pcmsycm as *const c_void);

    u16p_replace_bits(&mut val, lchan, GENMASK(3, 0) as u16);
    u16p_replace_bits(&mut val, hchan, GENMASK(7, 4) as u16);
    u16p_replace_bits(&mut val, stream_id, GENMASK(13, 8) as u16);
    u16p_replace_bits(&mut val, dir, BIT(15) as u16);

    writew(val, pcmsycm as *mut c_void);
}

unsafe fn hdaml_lctl_offload_enable(lctl: *mut u32, enable: bool) {
    let mut val = readl(lctl as *const c_void);

    if enable {
        val |= AZX_ML_LCTL_OFLEN;
    } else {
        val &= !AZX_ML_LCTL_OFLEN;
    }

    writel(val, lctl as *mut c_void);
}

/* END HDAML section */

unsafe fn hda_ml_alloc_h2link(bus: *mut hdac_bus, index: i32) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;
    let ret: i32;

    h2link = kzalloc_obj_hdac_ext2_link();
    if h2link.is_null() {
        return -ENOMEM;
    }

    /* basic initialization */
    hlink = &mut (*h2link).hext_link;

    (*hlink).index = index;
    (*hlink).bus = bus;
    (*hlink).ml_addr = ptr_add((*bus).mlcap, AZX_ML_BASE + AZX_ML_INTERVAL * index as isize);

    ret = hdaml_lnk_enum((*bus).dev, h2link, (*bus).remap_addr, (*hlink).ml_addr, index);
    if ret < 0 {
        kfree(h2link as *mut c_void);
        return ret;
    }

    mutex_init(&mut (*h2link).eml_lock);

    list_add_tail(&mut (*hlink).list, &mut (*bus).hlink_list);

    /*
     * HDaudio regular links are powered-on by default, the
     * refcount needs to be initialized.
     */
    if !(*h2link).alt {
        (*hlink).ref_count = 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_init(bus: *mut hdac_bus) -> i32 {
    let link_count: u32;
    let mut ret: i32;
    let mut i: i32;

    if (*bus).mlcap.is_null() {
        return 0;
    }

    /* Enumeration is a one time operation, skip if already done */
    if !list_empty(&(*bus).hlink_list) {
        return 0;
    }

    link_count = readl(ptr_add((*bus).mlcap, AZX_REG_ML_MLCD)) + 1;

    dev_dbg((*bus).dev, b"HDAudio Multi-Link count: %d\n\0".as_ptr(), link_count);

    i = 0;
    while i < link_count as i32 {
        ret = hda_ml_alloc_h2link(bus, i);
        if ret < 0 {
            hda_bus_ml_free(bus);
            return ret;
        }
        i += 1;
    }
    0
}
// EXPORT_SYMBOL_NS(hda_bus_ml_init, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_free(bus: *mut hdac_bus) {
    let mut hlink: *mut hdac_ext_link;
    let mut h2link: *mut hdac_ext2_link;

    if (*bus).mlcap.is_null() {
        return;
    }

    hlink = (*bus).hlink_list.next as *mut hdac_ext_link;
    while !hlink.is_null() && (&mut (*hlink).list as *mut list_head) != &mut (*bus).hlink_list {
        let next = (*hlink).list.next as *mut hdac_ext_link;
        list_del(&mut (*hlink).list);
        h2link = hdac_ext_link_to_ext2(hlink);

        mutex_destroy(&mut (*h2link).eml_lock);
        kfree(h2link as *mut c_void);
        hlink = next;
    }
}
// EXPORT_SYMBOL_NS(hda_bus_ml_free, "SND_SOC_SOF_HDA_MLINK");

unsafe fn find_ext2_link(bus: *mut hdac_bus, alt: bool, elid: i32) -> *mut hdac_ext2_link {
    let mut hlink: *mut hdac_ext_link;

    hlink = (*bus).hlink_list.next as *mut hdac_ext_link;
    while !hlink.is_null() && (&mut (*hlink).list as *mut list_head) != &mut (*bus).hlink_list {
        let h2link = hdac_ext_link_to_ext2(hlink);

        if (*h2link).alt == alt && (*h2link).elid == elid {
            return h2link;
        }
        hlink = (*hlink).list.next as *mut hdac_ext_link;
    }

    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_get_count(bus: *mut hdac_bus, alt: bool, elid: i32) -> i32 {
    let h2link: *mut hdac_ext2_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return 0;
    }

    (*h2link).slcount
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_get_count, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_enable_interrupt_unlocked(
    bus: *mut hdac_bus,
    alt: bool,
    elid: i32,
    enable: bool,
) {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return;
    }

    if !(*h2link).intc {
        return;
    }

    hlink = &mut (*h2link).hext_link;

    hdaml_link_enable_interrupt(ptr_add((*hlink).ml_addr, AZX_REG_ML_LCTL) as *mut u32, enable);
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_enable_interrupt_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_enable_interrupt(
    bus: *mut hdac_bus,
    alt: bool,
    elid: i32,
    enable: bool,
) {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return;
    }

    if !(*h2link).intc {
        return;
    }

    hlink = &mut (*h2link).hext_link;

    mutex_lock(&mut (*h2link).eml_lock);
    hdaml_link_enable_interrupt(ptr_add((*hlink).ml_addr, AZX_REG_ML_LCTL) as *mut u32, enable);
    mutex_unlock(&mut (*h2link).eml_lock);
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_enable_interrupt, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_check_interrupt(bus: *mut hdac_bus, alt: bool, elid: i32) -> bool {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return false;
    }

    if !(*h2link).intc {
        return false;
    }

    hlink = &mut (*h2link).hext_link;

    hdaml_link_check_interrupt(ptr_add((*hlink).ml_addr, AZX_REG_ML_LCTL) as *mut u32)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_check_interrupt, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_set_syncprd_unlocked(
    bus: *mut hdac_bus,
    alt: bool,
    elid: i32,
    syncprd: u32,
) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return 0;
    }

    if !(*h2link).lss {
        return 0;
    }

    hlink = &mut (*h2link).hext_link;

    hdaml_link_set_syncprd(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSYNC) as *mut u32, syncprd);

    0
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_set_syncprd_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_set_syncprd_unlocked(bus: *mut hdac_bus, syncprd: u32) -> i32 {
    hdac_bus_eml_set_syncprd_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, syncprd)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_set_syncprd_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_wait_syncpu_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return 0;
    }

    if !(*h2link).lss {
        return 0;
    }

    hlink = &mut (*h2link).hext_link;

    hdaml_link_wait_syncpu(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSYNC) as *mut u32)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_wait_syncpu_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_wait_syncpu_unlocked(bus: *mut hdac_bus) -> i32 {
    hdac_bus_eml_wait_syncpu_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_wait_syncpu_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sync_arm_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return;
    }

    if !(*h2link).lss {
        return;
    }

    hlink = &mut (*h2link).hext_link;

    hdaml_link_sync_arm(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSYNC) as *mut u32, sublink);
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sync_arm_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_sync_arm_unlocked(bus: *mut hdac_bus, sublink: i32) {
    hdac_bus_eml_sync_arm_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, sublink);
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_sync_arm_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sync_go_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return 0;
    }

    if !(*h2link).lss {
        return 0;
    }

    hlink = &mut (*h2link).hext_link;

    hdaml_link_sync_go(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSYNC) as *mut u32);

    0
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sync_go_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_sync_go_unlocked(bus: *mut hdac_bus) -> i32 {
    hdac_bus_eml_sync_go_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_sync_go_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_check_cmdsync_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32) -> bool {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;
    let cmdsync_mask: u32;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return false;
    }

    if !(*h2link).lss {
        return false;
    }

    hlink = &mut (*h2link).hext_link;

    cmdsync_mask = GENMASK(AZX_REG_ML_LSYNC_CMDSYNC_SHIFT + (*h2link).slcount - 1, AZX_REG_ML_LSYNC_CMDSYNC_SHIFT);

    hdaml_link_check_cmdsync(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSYNC) as *mut u32, cmdsync_mask)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_check_cmdsync_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_check_cmdsync_unlocked(bus: *mut hdac_bus) -> bool {
    hdac_bus_eml_check_cmdsync_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_check_cmdsync_unlocked, "SND_SOC_SOF_HDA_MLINK");

unsafe fn hdac_bus_eml_power_up_base(
    bus: *mut hdac_bus,
    alt: bool,
    elid: i32,
    sublink: i32,
    eml_lock: bool,
) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;
    let mut ret = 0;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return -ENODEV;
    }

    if sublink >= (*h2link).slcount {
        return -EINVAL;
    }

    hlink = &mut (*h2link).hext_link;

    if eml_lock {
        mutex_lock(&mut (*h2link).eml_lock);
    }

    if !alt {
        (*hlink).ref_count += 1;
        if (*hlink).ref_count > 1 {
            goto_skip_init(bus, h2link, eml_lock);
            return ret;
        }
    } else {
        (*h2link).sublink_ref_count[sublink as usize] += 1;
        if (*h2link).sublink_ref_count[sublink as usize] > 1 {
            goto_skip_init(bus, h2link, eml_lock);
            return ret;
        }
    }

    ret = hdaml_link_init(ptr_add((*hlink).ml_addr, AZX_REG_ML_LCTL) as *mut u32, sublink);
    if ((*h2link).mic_privacy_mask & BIT(sublink) as usize) != 0 && ret == 0 {
        let pvccs = ptr_add(
            (*h2link).base_ptr,
            ((*h2link).shim_vs_offset + sublink as u32 * (*h2link).instance_offset + AZX_REG_INTEL_VS_SHIM_PVCCS) as isize,
        ) as *mut u16;
        let val = readw(pvccs as *const c_void);

        writew(val | AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHGIE as u16, pvccs as *mut c_void);

        if (val as u32 & AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTS) != 0 {
            dev_dbg((*bus).dev, b"sublink %d (%d:%d): Mic privacy is enabled\n\0".as_ptr(), sublink, alt as i32, elid);
        }
    }

    goto_skip_init(bus, h2link, eml_lock);
    ret
}

unsafe fn goto_skip_init(_bus: *mut hdac_bus, h2link: *mut hdac_ext2_link, eml_lock: bool) {
    if eml_lock {
        mutex_unlock(&mut (*h2link).eml_lock);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_up(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32 {
    hdac_bus_eml_power_up_base(bus, alt, elid, sublink, true)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_power_up, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_up_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32 {
    hdac_bus_eml_power_up_base(bus, alt, elid, sublink, false)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_power_up_unlocked, "SND_SOC_SOF_HDA_MLINK");

unsafe fn hdac_bus_eml_power_down_base(
    bus: *mut hdac_bus,
    alt: bool,
    elid: i32,
    sublink: i32,
    eml_lock: bool,
) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;
    let mut ret = 0;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return -ENODEV;
    }

    if sublink >= (*h2link).slcount {
        return -EINVAL;
    }

    hlink = &mut (*h2link).hext_link;

    if eml_lock {
        mutex_lock(&mut (*h2link).eml_lock);
    }

    if !alt {
        (*hlink).ref_count -= 1;
        if (*hlink).ref_count > 0 {
            goto_skip_shutdown(h2link, eml_lock);
            return ret;
        }
    } else {
        (*h2link).sublink_ref_count[sublink as usize] -= 1;
        if (*h2link).sublink_ref_count[sublink as usize] > 0 {
            goto_skip_shutdown(h2link, eml_lock);
            return ret;
        }
    }

    if ((*h2link).mic_privacy_mask & BIT(sublink) as usize) != 0 {
        let pvccs = ptr_add(
            (*h2link).base_ptr,
            ((*h2link).shim_vs_offset + sublink as u32 * (*h2link).instance_offset + AZX_REG_INTEL_VS_SHIM_PVCCS) as isize,
        ) as *mut u16;

        writew(readw(pvccs as *const c_void) & !(AZX_REG_INTEL_VS_SHIM_PVCCS_MDSTSCHGIE as u16), pvccs as *mut c_void);
    }

    ret = hdaml_link_shutdown(ptr_add((*hlink).ml_addr, AZX_REG_ML_LCTL) as *mut u32, sublink);

    goto_skip_shutdown(h2link, eml_lock);
    ret
}

unsafe fn goto_skip_shutdown(h2link: *mut hdac_ext2_link, eml_lock: bool) {
    if eml_lock {
        mutex_unlock(&mut (*h2link).eml_lock);
    }
}

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_down(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32 {
    hdac_bus_eml_power_down_base(bus, alt, elid, sublink, true)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_power_down, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_power_down_unlocked(bus: *mut hdac_bus, alt: bool, elid: i32, sublink: i32) -> i32 {
    hdac_bus_eml_power_down_base(bus, alt, elid, sublink, false)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_power_down_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_power_up_unlocked(bus: *mut hdac_bus, sublink: i32) -> i32 {
    hdac_bus_eml_power_up_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, sublink)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_power_up_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_power_down_unlocked(bus: *mut hdac_bus, sublink: i32) -> i32 {
    hdac_bus_eml_power_down_unlocked(bus, true, AZX_REG_ML_LEPTR_ID_SDW, sublink)
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_power_down_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_get_lsdiid_unlocked(bus: *mut hdac_bus, sublink: i32, lsdiid: *mut u16) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if h2link.is_null() {
        return -ENODEV;
    }

    hlink = &mut (*h2link).hext_link;

    *lsdiid = hdaml_link_get_lsdiid(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSDIID_OFFSET(sublink)) as *mut u16);

    0
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_get_lsdiid_unlocked, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_set_lsdiid(bus: *mut hdac_bus, sublink: i32, dev_num: i32) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if h2link.is_null() {
        return -ENODEV;
    }

    hlink = &mut (*h2link).hext_link;

    mutex_lock(&mut (*h2link).eml_lock);
    hdaml_link_set_lsdiid(ptr_add((*hlink).ml_addr, AZX_REG_ML_LSDIID_OFFSET(sublink)) as *mut u16, dev_num);
    mutex_unlock(&mut (*h2link).eml_lock);

    0
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_set_lsdiid, "SND_SOC_SOF_HDA_MLINK");

/*
 * the 'y' parameter comes from the PCMSyCM hardware register naming. 'y' refers to the
 * PDI index, i.e. the FIFO used for RX or TX
 */
#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_map_stream_ch(
    bus: *mut hdac_bus,
    sublink: i32,
    y: i32,
    channel_mask: i32,
    stream_id: i32,
    dir: i32,
) -> i32 {
    let h2link: *mut hdac_ext2_link;
    let pcmsycm: *mut u16;
    let hchan: i32;
    let lchan: i32;
    let val: u16;

    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if h2link.is_null() {
        return -ENODEV;
    }

    pcmsycm = ptr_add(
        (*h2link).base_ptr,
        ((*h2link).shim_offset + (*h2link).instance_offset * sublink as u32 + AZX_REG_SDW_SHIM_PCMSyCM(y)) as isize,
    ) as *mut u16;

    if channel_mask != 0 {
        hchan = __fls(channel_mask);
        lchan = __ffs(channel_mask);
    } else {
        hchan = 0;
        lchan = 0;
    }

    mutex_lock(&mut (*h2link).eml_lock);
    hdaml_shim_map_stream_ch(pcmsycm, lchan, hchan, stream_id, dir);
    mutex_unlock(&mut (*h2link).eml_lock);

    val = readw(pcmsycm as *const c_void);

    dev_dbg((*bus).dev, b"sublink %d channel_mask %#x stream_id %d dir %d pcmscm %#x\n\0".as_ptr(), sublink, channel_mask, stream_id, dir, val as i32);

    0
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_map_stream_ch, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_reset_losidv(bus: *mut hdac_bus) {
    let mut hlink: *mut hdac_ext_link;

    /* Reset stream-to-link mapping */
    hlink = (*bus).hlink_list.next as *mut hdac_ext_link;
    while !hlink.is_null() && (&mut (*hlink).list as *mut list_head) != &mut (*bus).hlink_list {
        writel(0, ptr_add((*hlink).ml_addr, AZX_REG_ML_LOSIDV));
        hlink = (*hlink).list.next as *mut hdac_ext_link;
    }
}
// EXPORT_SYMBOL_NS(hda_bus_ml_reset_losidv, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_link_get_type(hlink: *mut hdac_ext_link) -> hda_bus_ml_link_type {
    let h2link = hdac_ext_link_to_ext2(hlink);

    if !(*h2link).alt {
        return HDA_BUS_ML_LINK_HDA;
    }

    match (*h2link).elid {
        x if x == AZX_REG_ML_LEPTR_ID_SDW => HDA_BUS_ML_LINK_SDW,
        x if x == AZX_REG_ML_LEPTR_ID_INTEL_UAOL => HDA_BUS_ML_LINK_UAOL,
        _ => HDA_BUS_ML_LINK_OTHER,
    }
}
// EXPORT_SYMBOL_NS(hda_bus_ml_link_get_type, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_resume(bus: *mut hdac_bus) -> i32 {
    let mut hlink: *mut hdac_ext_link;
    let ret: i32;

    /* power up links that were active before suspend */
    hlink = (*bus).hlink_list.next as *mut hdac_ext_link;
    while !hlink.is_null() && (&mut (*hlink).list as *mut list_head) != &mut (*bus).hlink_list {
        let h2link = hdac_ext_link_to_ext2(hlink);

        if !(*h2link).alt && (*hlink).ref_count != 0 {
            ret = snd_hdac_ext_bus_link_power_up(hlink);
            if ret < 0 {
                return ret;
            }
        }
        hlink = (*hlink).list.next as *mut hdac_ext_link;
    }
    0
}
// EXPORT_SYMBOL_NS(hda_bus_ml_resume, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hda_bus_ml_suspend(bus: *mut hdac_bus) -> i32 {
    let mut hlink: *mut hdac_ext_link;
    let ret: i32;

    hlink = (*bus).hlink_list.next as *mut hdac_ext_link;
    while !hlink.is_null() && (&mut (*hlink).list as *mut list_head) != &mut (*bus).hlink_list {
        let h2link = hdac_ext_link_to_ext2(hlink);

        if !(*h2link).alt {
            ret = snd_hdac_ext_bus_link_power_down(hlink);
            if ret < 0 {
                return ret;
            }
        }
        hlink = (*hlink).list.next as *mut hdac_ext_link;
    }
    0
}
// EXPORT_SYMBOL_NS(hda_bus_ml_suspend, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_get_mutex(bus: *mut hdac_bus, alt: bool, elid: i32) -> *mut mutex {
    let h2link: *mut hdac_ext2_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return null_mut();
    }

    &mut (*h2link).eml_lock
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_get_mutex, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_ssp_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link {
    let h2link: *mut hdac_ext2_link;

    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_SSP);
    if h2link.is_null() {
        return null_mut();
    }

    &mut (*h2link).hext_link
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_ssp_get_hlink, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_dmic_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link {
    let h2link: *mut hdac_ext2_link;

    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_INTEL_DMIC);
    if h2link.is_null() {
        return null_mut();
    }

    &mut (*h2link).hext_link
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_dmic_get_hlink, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_sdw_get_hlink(bus: *mut hdac_bus) -> *mut hdac_ext_link {
    let h2link: *mut hdac_ext2_link;

    h2link = find_ext2_link(bus, true, AZX_REG_ML_LEPTR_ID_SDW);
    if h2link.is_null() {
        return null_mut();
    }

    &mut (*h2link).hext_link
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_sdw_get_hlink, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_enable_offload(bus: *mut hdac_bus, alt: bool, elid: i32, enable: bool) {
    let h2link: *mut hdac_ext2_link;
    let hlink: *mut hdac_ext_link;

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() || !(*h2link).ofls {
        return;
    }

    hlink = &mut (*h2link).hext_link;

    mutex_lock(&mut (*h2link).eml_lock);
    hdaml_lctl_offload_enable(ptr_add((*hlink).ml_addr, AZX_REG_ML_LCTL) as *mut u32, enable);
    mutex_unlock(&mut (*h2link).eml_lock);
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_enable_offload, "SND_SOC_SOF_HDA_MLINK");

#[no_mangle]
pub unsafe extern "C" fn hdac_bus_eml_set_mic_privacy_mask(
    bus: *mut hdac_bus,
    alt: bool,
    elid: i32,
    mask: usize,
) {
    let h2link: *mut hdac_ext2_link;

    if mask == 0 {
        return;
    }

    h2link = find_ext2_link(bus, alt, elid);
    if h2link.is_null() {
        return;
    }

    if __fls(mask as i32) > (*h2link).slcount {
        dev_warn(
            (*bus).dev,
            b"%s: invalid sublink mask for %d:%d, slcount %d: %#lx\n\0".as_ptr(),
            b"hdac_bus_eml_set_mic_privacy_mask\0".as_ptr(),
            alt as i32,
            elid,
            (*h2link).slcount,
            mask,
        );
        return;
    }

    dev_dbg((*bus).dev, b"sublink mask for %d:%d, slcount %d: %#lx\n\0".as_ptr(), alt as i32, elid, (*h2link).slcount, mask);

    (*h2link).mic_privacy_mask = mask;
}
// EXPORT_SYMBOL_NS(hdac_bus_eml_set_mic_privacy_mask, "SND_SOC_SOF_HDA_MLINK");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
