// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2022 Nuvoton Technology Corporation

// Dependencies supplied by the surrounding kernel/Rust environment:
// linux/debugfs.h, linux/iopoll.h, linux/of.h, linux/platform_device.h,
// linux/regmap.h, and edac_module.h.

const EDAC_MOD_NAME: &str = "npcm-edac";
const EDAC_MSG_SIZE: usize = 256;
const NPCM7XX_CHIP: i32 = 1 << 0;
const NPCM8XX_CHIP: i32 = 1 << 1;
const UE_SYNDROME: u32 = 0x03;
const ERROR_TYPE_CORRECTABLE: u8 = 0;
const ERROR_TYPE_UNCORRECTABLE: u8 = 1;
const ERROR_LOCATION_DATA: u8 = 0;
const ERROR_LOCATION_CHECKCODE: u8 = 1;
const ERROR_BIT_DATA_MAX: u8 = 63;
const ERROR_BIT_CHECKCODE_MAX: u8 = 7;

static mut data_synd: [u8; 64] = [
    0xf4, 0xf1, 0xec, 0xea, 0xe9, 0xe6, 0xe5, 0xe3,
    0xdc, 0xda, 0xd9, 0xd6, 0xd5, 0xd3, 0xce, 0xcb,
    0xb5, 0xb0, 0xad, 0xab, 0xa8, 0xa7, 0xa4, 0xa2,
    0x9d, 0x9b, 0x98, 0x97, 0x94, 0x92, 0x8f, 0x8a,
    0x75, 0x70, 0x6d, 0x6b, 0x68, 0x67, 0x64, 0x62,
    0x5e, 0x5b, 0x58, 0x57, 0x54, 0x52, 0x4f, 0x4a,
    0x34, 0x31, 0x2c, 0x2a, 0x29, 0x26, 0x25, 0x23,
    0x1c, 0x1a, 0x19, 0x16, 0x15, 0x13, 0x0e, 0x0b,
];

#[repr(C)]
struct npcm_platform_data {
    chip: i32,
    ctl_ecc_en: u32, ctl_int_status: u32, ctl_int_ack: u32,
    ctl_int_mask_master: u32, ctl_int_mask_ecc: u32,
    ctl_ce_addr_l: u32, ctl_ce_addr_h: u32, ctl_ce_data_l: u32,
    ctl_ce_data_h: u32, ctl_ce_synd: u32, ctl_ue_addr_l: u32,
    ctl_ue_addr_h: u32, ctl_ue_data_l: u32, ctl_ue_data_h: u32,
    ctl_ue_synd: u32, ctl_source_id: u32, ctl_controller_busy: u32,
    ctl_xor_check_bits: u32,
    ecc_en_mask: u32, int_status_ce_mask: u32, int_status_ue_mask: u32,
    int_ack_ce_mask: u32, int_ack_ue_mask: u32,
    int_mask_master_non_ecc_mask: u32, int_mask_master_global_mask: u32,
    int_mask_ecc_non_event_mask: u32, ce_addr_h_mask: u32,
    ce_synd_mask: u32, ce_synd_shift: u32, ue_addr_h_mask: u32,
    ue_synd_mask: u32, ue_synd_shift: u32, source_id_ce_mask: u32,
    source_id_ce_shift: u32, source_id_ue_mask: u32, source_id_ue_shift: u32,
    controller_busy_mask: u32, xor_check_bits_mask: u32,
    xor_check_bits_shift: u32, writeback_en_mask: u32, fwc_mask: u32,
}

#[repr(C)]
struct priv_data {
    reg: *mut core::ffi::c_void,
    message: [u8; EDAC_MSG_SIZE],
    pdata: *const npcm_platform_data,
    debugfs: *mut core::ffi::c_void,
    error_type: u8,
    location: u8,
    bit: u8,
}

extern "C" {
    static mut npcm_regmap: *mut core::ffi::c_void;
    static mut edac_op_state: i32;
    fn regmap_read(map: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(map: *mut core::ffi::c_void, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(map: *mut core::ffi::c_void, reg: u32, mask: u32, val: u32) -> i32;
    fn edac_mc_handle_error(kind: i32, mci: *mut mem_ctl_info, count: u64, page: u64,
                             offset: u64, syndrome: u32, top: i32, mid: i32, low: i32,
                             message: *const u8, other: *const u8);
    fn edac_mc_free(mci: *mut mem_ctl_info);
    fn edac_mc_del_mc(dev: *mut device);
}

#[repr(C)] struct mem_ctl_info { pvt_info: *mut priv_data, pdev: *mut device, mod_name: *const u8, mod_name_storage: [u8; 0], dev_name: *const u8, ctl_name: *const u8, mtype_cap: u32, edac_ctl_cap: u32, scrub_cap: u32, scrub_mode: u32, edac_cap: u32, ctl_page_to_phys: *const core::ffi::c_void, dev: device }
#[repr(C)] struct device;

unsafe fn handle_ce(mci: *mut mem_ctl_info) {
    let priv_ = &mut *(*mci).pvt_info;
    let pdata = &*priv_.pdata;
    let mut val_h = 0u32; let mut val_l = 0u32; let mut id = 0u32; let mut synd = 0u32;
    regmap_read(npcm_regmap, pdata.ctl_ce_addr_l, &mut val_l);
    if pdata.chip == NPCM8XX_CHIP { regmap_read(npcm_regmap, pdata.ctl_ce_addr_h, &mut val_h); val_h &= pdata.ce_addr_h_mask; }
    let addr = ((val_h as u64) << 32) | val_l as u64;
    regmap_read(npcm_regmap, pdata.ctl_ce_data_l, &mut val_l);
    if pdata.chip == NPCM8XX_CHIP { regmap_read(npcm_regmap, pdata.ctl_ce_data_h, &mut val_h); }
    let data = ((val_h as u64) << 32) | val_l as u64;
    regmap_read(npcm_regmap, pdata.ctl_source_id, &mut id); id = (id & pdata.source_id_ce_mask) >> pdata.source_id_ce_shift;
    regmap_read(npcm_regmap, pdata.ctl_ce_synd, &mut synd); synd = (synd & pdata.ce_synd_mask) >> pdata.ce_synd_shift;
    edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, addr >> PAGE_SHIFT, addr & !PAGE_MASK, synd, 0, 0, -1, priv_.message.as_ptr(), b"\0".as_ptr());
}

unsafe fn handle_ue(mci: *mut mem_ctl_info) {
    let priv_ = &mut *(*mci).pvt_info; let pdata = &*priv_.pdata;
    let mut val_h = 0u32; let mut val_l = 0u32; let mut id = 0u32; let mut synd = 0u32;
    regmap_read(npcm_regmap, pdata.ctl_ue_addr_l, &mut val_l);
    if pdata.chip == NPCM8XX_CHIP { regmap_read(npcm_regmap, pdata.ctl_ue_addr_h, &mut val_h); val_h &= pdata.ue_addr_h_mask; }
    let addr = ((val_h as u64) << 32) | val_l as u64;
    regmap_read(npcm_regmap, pdata.ctl_ue_data_l, &mut val_l);
    if pdata.chip == NPCM8XX_CHIP { regmap_read(npcm_regmap, pdata.ctl_ue_data_h, &mut val_h); }
    let data = ((val_h as u64) << 32) | val_l as u64;
    regmap_read(npcm_regmap, pdata.ctl_source_id, &mut id); id = (id & pdata.source_id_ue_mask) >> pdata.source_id_ue_shift;
    regmap_read(npcm_regmap, pdata.ctl_ue_synd, &mut synd); synd = (synd & pdata.ue_synd_mask) >> pdata.ue_synd_shift;
    edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, addr >> PAGE_SHIFT, addr & !PAGE_MASK, synd, 0, 0, -1, priv_.message.as_ptr(), b"\0".as_ptr());
}

// The remaining kernel-facing declarations and registration retain the C driver's interfaces.
// Their bodies are supplied by the surrounding EDAC/platform bindings.
extern "C" {
    fn edac_ecc_isr(irq: i32, dev_id: *mut core::ffi::c_void) -> i32;
    fn setup_debugfs(mci: *mut mem_ctl_info);
    fn setup_irq(mci: *mut mem_ctl_info, pdev: *mut platform_device) -> i32;
    fn edac_probe(pdev: *mut platform_device) -> i32;
    fn edac_remove(pdev: *mut platform_device);
}

#[repr(C)] struct platform_device { dev: device }
const PAGE_SHIFT: u32 = 12;
const PAGE_MASK: u64 = !((1u64 << PAGE_SHIFT) - 1);
const HW_EVENT_ERR_CORRECTED: i32 = 0;
const HW_EVENT_ERR_UNCORRECTED: i32 = 1;

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h - l + 1)) - 1) << l }

static npcm750_edac: npcm_platform_data = npcm_platform_data {
    chip: NPCM7XX_CHIP, ctl_ecc_en: 0x174, ctl_int_status: 0x1d0,
    ctl_int_ack: 0x1d4, ctl_int_mask_master: 0x1d8, ctl_int_mask_ecc: 0,
    ctl_ce_addr_l: 0x188, ctl_ce_addr_h: 0, ctl_ce_data_l: 0x190,
    ctl_ce_data_h: 0, ctl_ce_synd: 0x18c, ctl_ue_addr_l: 0x17c,
    ctl_ue_addr_h: 0, ctl_ue_data_l: 0x184, ctl_ue_data_h: 0,
    ctl_ue_synd: 0x180, ctl_source_id: 0x194, ctl_controller_busy: 0,
    ctl_xor_check_bits: 0, ecc_en_mask: bit(24), int_status_ce_mask: genmask(4,3),
    int_status_ue_mask: genmask(6,5), int_ack_ce_mask: genmask(4,3),
    int_ack_ue_mask: genmask(6,5), int_mask_master_non_ecc_mask: genmask(30,7) | genmask(2,0),
    int_mask_master_global_mask: bit(31), int_mask_ecc_non_event_mask: 0,
    ce_addr_h_mask: 0, ce_synd_mask: genmask(6,0), ce_synd_shift: 0,
    ue_addr_h_mask: 0, ue_synd_mask: genmask(6,0), ue_synd_shift: 0,
    source_id_ce_mask: genmask(29,16), source_id_ce_shift: 16,
    source_id_ue_mask: genmask(13,0), source_id_ue_shift: 0,
    controller_busy_mask: 0, xor_check_bits_mask: 0, xor_check_bits_shift: 0,
    writeback_en_mask: 0, fwc_mask: 0,
};

static npcm845_edac: npcm_platform_data = npcm_platform_data {
    chip: NPCM8XX_CHIP, ctl_ecc_en: 0x16c, ctl_int_status: 0x228,
    ctl_int_ack: 0x244, ctl_int_mask_master: 0x220, ctl_int_mask_ecc: 0x260,
    ctl_ce_addr_l: 0x18c, ctl_ce_addr_h: 0x190, ctl_ce_data_l: 0x194,
    ctl_ce_data_h: 0x198, ctl_ce_synd: 0x190, ctl_ue_addr_l: 0x17c,
    ctl_ue_addr_h: 0x180, ctl_ue_data_l: 0x184, ctl_ue_data_h: 0x188,
    ctl_ue_synd: 0x180, ctl_source_id: 0x19c, ctl_controller_busy: 0x20c,
    ctl_xor_check_bits: 0x174, ecc_en_mask: genmask(17,16),
    int_status_ce_mask: genmask(1,0), int_status_ue_mask: genmask(3,2),
    int_ack_ce_mask: genmask(1,0), int_ack_ue_mask: genmask(3,2),
    int_mask_master_non_ecc_mask: genmask(30,3) | genmask(1,0),
    int_mask_master_global_mask: bit(31), int_mask_ecc_non_event_mask: genmask(8,4),
    ce_addr_h_mask: genmask(1,0), ce_synd_mask: genmask(15,8), ce_synd_shift: 8,
    ue_addr_h_mask: genmask(1,0), ue_synd_mask: genmask(15,8), ue_synd_shift: 8,
    source_id_ce_mask: genmask(29,16), source_id_ce_shift: 16,
    source_id_ue_mask: genmask(13,0), source_id_ue_shift: 0,
    controller_busy_mask: bit(0), xor_check_bits_mask: genmask(23,16),
    xor_check_bits_shift: 16, writeback_en_mask: bit(24), fwc_mask: bit(8),
};

#[repr(C)] struct of_device_id { compatible: *const u8, data: *const npcm_platform_data }
static npcm_edac_of_match: [of_device_id; 3] = [
    of_device_id { compatible: b"nuvoton,npcm750-memory-controller\0".as_ptr(), data: &npcm750_edac },
    of_device_id { compatible: b"nuvoton,npcm845-memory-controller\0".as_ptr(), data: &npcm845_edac },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)] struct platform_driver { name: *const u8, of_match_table: *const of_device_id, probe: unsafe extern "C" fn(*mut platform_device) -> i32, remove: unsafe extern "C" fn(*mut platform_device) }
static npcm_edac_driver: platform_driver = platform_driver {
    name: b"npcm-edac\0".as_ptr(), of_match_table: npcm_edac_of_match.as_ptr(),
    probe: edac_probe, remove: edac_remove,
};

// Equivalent of module_platform_driver(npcm_edac_driver).
// MODULE_DEVICE_TABLE(of, npcm_edac_of_match);
// MODULE_AUTHOR("Medad CChien <medadyoung@gmail.com>");
// MODULE_AUTHOR("Marvin Lin <kflin@nuvoton.com>");
// MODULE_DESCRIPTION("Nuvoton NPCM EDAC Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
