// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2005-2007  Kristian Hoegsberg <krh@bitplanet.net>
 */

// Linux kernel dependencies and symbols from core.h are intentionally external.

const DEFAULT_SPLIT_TIMEOUT: i32 = 2 * 8000;
const BIB_BUS_NAME: u32 = 0x31333934;
const NODE_CAPABILITIES: u32 = 0x0c0083c0;

#[inline] const fn bib_crc(v: u32) -> u32 { v << 0 }
#[inline] const fn bib_crc_length(v: u32) -> u32 { v << 16 }
#[inline] const fn bib_info_length(v: u32) -> u32 { v << 24 }
#[inline] const fn bib_link_speed(v: u32) -> u32 { v << 0 }
#[inline] const fn bib_generation(v: u32) -> u32 { v << 4 }
#[inline] const fn bib_max_rom(v: u32) -> u32 { v << 8 }
#[inline] const fn bib_max_receive(v: u32) -> u32 { v << 12 }
#[inline] const fn bib_cyc_clk_acc(v: u32) -> u32 { v << 16 }
const BIB_PMC: u32 = 1 << 27;
const BIB_BMC: u32 = 1 << 28;
const BIB_ISC: u32 = 1 << 29;
const BIB_CMC: u32 = 1 << 30;
const BIB_IRMC: u32 = 1 << 31;

static mut CARD_MUTEX: core::mem::MaybeUninit<mutex> = core::mem::MaybeUninit::uninit();
static mut CARD_LIST: list_head = list_head::new();
static mut DESCRIPTOR_LIST: list_head = list_head::new();
static mut DESCRIPTOR_COUNT: i32 = 0;
static mut TMP_CONFIG_ROM: [u32; 256] = [0; 256];
static mut CONFIG_ROM_LENGTH: usize = 1 + 4 + 1 + 1;

pub unsafe fn fw_compute_block_crc(block: *mut u32) -> i32 {
    let length = ((u32::from_be((*block).to_be()) >> 16) & 0xff) as i32;
    let crc = crc_itu_t(0, block.add(1) as *const u8, (length * 4) as usize);
    *block |= (crc as u32).to_be();
    length
}

unsafe fn generate_config_rom(card: *mut fw_card, config_rom: *mut u32) {
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut length: i32;
    (*config_rom.add(0)) = (bib_crc_length(4) | bib_info_length(4) | bib_crc(0)).to_be();
    *config_rom.add(1) = BIB_BUS_NAME.to_be();
    *config_rom.add(2) = (bib_link_speed((*card).link_speed) |
        bib_generation((*card).config_rom_generation.wrapping_add(1) % 14 + 2) |
        bib_max_rom(2) | bib_max_receive((*card).max_receive) |
        BIB_BMC | BIB_ISC | BIB_CMC | BIB_IRMC).to_be();
    *config_rom.add(3) = ((*card).guid >> 32) as u32 .to_be();
    *config_rom.add(4) = (*card).guid as u32 .to_be();
    *config_rom.add(6) = NODE_CAPABILITIES.to_be();
    i = 7; j = 7 + DESCRIPTOR_COUNT;
    let mut desc = (*DESCRIPTOR_LIST.next as *mut fw_descriptor);
    while desc != core::ptr::addr_of_mut!(DESCRIPTOR_LIST) as *mut fw_descriptor {
        if (*desc).immediate > 0 { *config_rom.add(i as usize) = (*desc).immediate.to_be(); i += 1; }
        *config_rom.add(i as usize) = ((*desc).key + (j - i) as u32).to_be(); i += 1;
        j += (*desc).length as i32;
        desc = (*(*desc).link.next).container_of();
    }
    *config_rom.add(5) = (((i - 5 - 1) << 16) as u32).to_be();
    desc = (*DESCRIPTOR_LIST.next as *mut fw_descriptor);
    while desc != core::ptr::addr_of_mut!(DESCRIPTOR_LIST) as *mut fw_descriptor {
        for k in 0..(*desc).length { *config_rom.add((i + k as i32) as usize) = (*desc).data.add(k).read().to_be(); }
        i += (*desc).length as i32;
        desc = (*(*desc).link.next).container_of();
    }
    i = 0; length = 0;
    while i < j { length = fw_compute_block_crc(config_rom.add(i as usize)); i += length + 1; }
    WARN_ON(j as usize != CONFIG_ROM_LENGTH);
}

unsafe fn update_config_roms() {
    let mut card = CARD_LIST.next as *mut fw_card;
    while card != core::ptr::addr_of_mut!(CARD_LIST) as *mut fw_card {
        generate_config_rom(card, TMP_CONFIG_ROM.as_mut_ptr());
        ((*(*card).driver).set_config_rom)(card, TMP_CONFIG_ROM.as_mut_ptr(), CONFIG_ROM_LENGTH);
        card = (*(*card).link.next).container_of();
    }
}

unsafe fn required_space(desc: *mut fw_descriptor) -> usize { (*desc).length + 1 + if (*desc).immediate > 0 { 1 } else { 0 } }

#[no_mangle] pub unsafe extern "C" fn fw_core_add_descriptor(desc: *mut fw_descriptor) -> i32 {
    if (*desc).length < 1 || (*desc).length > 256 { return -EINVAL; }
    let mut i = 0usize;
    while i < (*desc).length { let block_len = (*desc).data.add(i).read() >> 16; if block_len >= ((*desc).length - i) as u32 { return -EINVAL; } i += block_len as usize + 1; }
    if i != (*desc).length { return -EINVAL; }
    guard_mutex(&mut CARD_MUTEX);
    if CONFIG_ROM_LENGTH + required_space(desc) > 256 { return -EBUSY; }
    list_add_tail(&mut (*desc).link, &mut DESCRIPTOR_LIST); CONFIG_ROM_LENGTH += required_space(desc); DESCRIPTOR_COUNT += 1;
    if (*desc).immediate > 0 { DESCRIPTOR_COUNT += 1; } update_config_roms(); 0
}

#[no_mangle] pub unsafe extern "C" fn fw_core_remove_descriptor(desc: *mut fw_descriptor) {
    guard_mutex(&mut CARD_MUTEX); list_del(&mut (*desc).link); CONFIG_ROM_LENGTH -= required_space(desc); DESCRIPTOR_COUNT -= 1;
    if (*desc).immediate > 0 { DESCRIPTOR_COUNT -= 1; } update_config_roms();
}

unsafe fn reset_bus(card: *mut fw_card, short_reset: bool) -> i32 {
    let reg = if short_reset { 5 } else { 1 }; let bit = if short_reset { PHY_BUS_SHORT_RESET } else { PHY_BUS_RESET };
    trace_bus_reset_initiate((*card).index, (*card).generation, short_reset); ((*(*card).driver).update_phy_reg)(card, reg, 0, bit)
}

#[no_mangle] pub unsafe extern "C" fn fw_schedule_bus_reset(card: *mut fw_card, delayed: bool, short_reset: bool) {
    trace_bus_reset_schedule((*card).index, (*card).generation, short_reset); (*card).br_short = short_reset; fw_card_get(card);
    if !queue_delayed_work(fw_workqueue, &mut (*card).br_work, if delayed { msecs_to_jiffies(10) } else { 0 }) { fw_card_put(card); }
}

unsafe fn br_work(work: *mut work_struct) {
    let card = from_work!(work, fw_card, br_work.work);
    if (*card).reset_jiffies != 0 && time_is_after_jiffies64((*card).reset_jiffies + secs_to_jiffies(2)) {
        trace_bus_reset_postpone((*card).index, (*card).generation, (*card).br_short);
        if !queue_delayed_work(fw_workqueue, &mut (*card).br_work, secs_to_jiffies(2)) { fw_card_put(card); } return;
    }
    fw_send_phy_config(card, FW_PHY_CONFIG_NO_NODE_ID, (*card).generation, FW_PHY_CONFIG_CURRENT_GAP_COUNT); reset_bus(card, (*card).br_short); fw_card_put(card);
}

// The remaining driver/workqueue routines retain their C control flow and use
// external kernel/core symbols supplied by the surrounding translation unit.
// (Declarations below intentionally remain source-level interfaces.)

unsafe fn allocate_broadcast_channel(card: *mut fw_card, generation: i32) {
    let mut channel = 0; let mut bandwidth = 0;
    if !(*card).broadcast_channel_allocated { fw_iso_resource_manage(card, generation, 1u64 << 31, &mut channel, &mut bandwidth, true); if channel != 31 { fw_notice(card, "failed to allocate broadcast channel\n"); return; } (*card).broadcast_channel_allocated = true; }
    device_for_each_child((*card).device, generation as isize as *mut core::ffi::c_void, fw_device_set_broadcast_channel);
}

#[no_mangle] pub unsafe extern "C" fn fw_schedule_bm_work(card: *mut fw_card, delay: ulong) { fw_card_get(card); if !schedule_delayed_work(&mut (*card).bm_work, delay) { fw_card_put(card); } }

#[no_mangle] pub unsafe extern "C" fn fw_card_read_cycle_time(card: *mut fw_card, cycle_time: *mut u32) -> i32 {
    if (*card).driver.read_csr as usize == dummy_read_csr as usize { return -ENODEV; }
    *cycle_time = ((*(*card).driver).read_csr)(card, CSR_CYCLE_TIME); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
