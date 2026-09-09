// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

// Linux dependencies supplied by the surrounding kernel translation.

const EDAC_LLCC: &str = "qcom_llcc";
const LLCC_ERP_PANIC_ON_UE: i32 = 1;
const TRP_SYN_REG_CNT: usize = 6;
const DRP_SYN_REG_CNT: usize = 8;
const LLCC_LB_CNT_MASK: u32 = 0xf000_0000;
const LLCC_LB_CNT_SHIFT: u32 = 28;
const ECC_DB_ERR_COUNT_MASK: u32 = 0x1f;
const ECC_DB_ERR_WAYS_MASK: u32 = 0xffff_0000;
const ECC_DB_ERR_WAYS_SHIFT: u32 = 0x10;
const ECC_SB_ERR_COUNT_MASK: u32 = 0x00ff_0000;
const ECC_SB_ERR_COUNT_SHIFT: u32 = 0x10;
const ECC_SB_ERR_WAYS_MASK: u32 = 0x0000_ffff;
const SB_ECC_ERROR: u32 = 1 << 0;
const DB_ECC_ERROR: u32 = 1 << 1;
const DRP_TRP_INT_CLEAR: u32 = 0x3;
const DRP_TRP_CNT_CLEAR: u32 = 0x3;
const SB_ERROR_THRESHOLD: u32 = 0x1;
const SB_ERROR_THRESHOLD_SHIFT: u32 = 24;
const SB_DB_TRP_INTERRUPT_ENABLE: u32 = 0x3;
const TRP0_INTERRUPT_ENABLE: u32 = 0x1;
const DRP0_INTERRUPT_ENABLE: u32 = 1 << 6;
const SB_DB_DRP_INTERRUPT_ENABLE: u32 = 0x3;
const ECC_POLL_MSEC: u32 = 5000;

const LLCC_DRAM_CE: usize = 0;
const LLCC_DRAM_UE: usize = 1;
const LLCC_TRAM_CE: usize = 2;
const LLCC_TRAM_UE: usize = 3;

#[repr(C)]
pub struct llcc_edac_reg_data {
    pub name: &'static str,
    pub reg_cnt: usize,
    pub count_mask: u32,
    pub ways_mask: u32,
    pub count_shift: u32,
    pub ways_shift: u32,
}

static EDAC_REG_DATA: [llcc_edac_reg_data; 4] = [
    llcc_edac_reg_data { name: "DRAM Single-bit", reg_cnt: DRP_SYN_REG_CNT, count_mask: ECC_SB_ERR_COUNT_MASK, ways_mask: ECC_SB_ERR_WAYS_MASK, count_shift: ECC_SB_ERR_COUNT_SHIFT, ways_shift: 0 },
    llcc_edac_reg_data { name: "DRAM Double-bit", reg_cnt: DRP_SYN_REG_CNT, count_mask: ECC_DB_ERR_COUNT_MASK, ways_mask: ECC_DB_ERR_WAYS_MASK, count_shift: 0, ways_shift: ECC_DB_ERR_WAYS_SHIFT },
    llcc_edac_reg_data { name: "TRAM Single-bit", reg_cnt: TRP_SYN_REG_CNT, count_mask: ECC_SB_ERR_COUNT_MASK, ways_mask: ECC_SB_ERR_WAYS_MASK, count_shift: ECC_SB_ERR_COUNT_SHIFT, ways_shift: 0 },
    llcc_edac_reg_data { name: "TRAM Double-bit", reg_cnt: TRP_SYN_REG_CNT, count_mask: ECC_DB_ERR_COUNT_MASK, ways_mask: ECC_DB_ERR_WAYS_MASK, count_shift: 0, ways_shift: ECC_DB_ERR_WAYS_SHIFT },
];

#[repr(C)] pub struct llcc_edac_reg_offset {
    pub cmn_interrupt_0_enable: u32, pub trp_interrupt_0_enable: u32,
    pub drp_ecc_error_cfg: u32, pub drp_interrupt_enable: u32,
    pub drp_interrupt_clear: u32, pub drp_ecc_error_cntr_clear: u32,
    pub trp_interrupt_0_clear: u32, pub trp_ecc_error_cntr_clear: u32,
    pub drp_ecc_sb_err_syn0: u32, pub drp_ecc_db_err_syn0: u32,
    pub drp_ecc_error_status1: u32, pub drp_ecc_error_status0: u32,
    pub trp_ecc_sb_err_syn0: u32, pub trp_ecc_db_err_syn0: u32,
    pub trp_ecc_error_status1: u32, pub trp_ecc_error_status0: u32,
    pub drp_interrupt_status: u32, pub trp_interrupt_0_status: u32,
}

#[repr(C)] pub struct llcc_drv_data {
    pub edac_reg_offset: *mut llcc_edac_reg_offset,
    pub bcast_regmap: *mut regmap,
    pub regmaps: *mut *mut regmap,
    pub num_banks: u32, pub ecc_irq_configured: bool, pub ecc_irq: i32,
}
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct device { pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct edac_device_ctl_info { pub dev: *mut device, pub poll_msec: u32, pub edac_check: Option<unsafe extern "C" fn(*mut edac_device_ctl_info)> }

type irqreturn_t = i32;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn edac_printk(level: i32, driver: &str, fmt: &str, ...);
    fn edac_device_handle_ce(ctl: *mut edac_device_ctl_info, a: u32, b: u32, msg: &str);
    fn edac_device_handle_ue(ctl: *mut edac_device_ctl_info, a: u32, b: u32, msg: &str);
    fn edac_device_alloc_ctl_info(a: u32, name: &str, n: u32, unit: &str, banks: u32, x: u32, idx: i32) -> *mut edac_device_ctl_info;
    fn edac_device_alloc_index() -> i32;
    fn edac_device_add_device(ctl: *mut edac_device_ctl_info) -> i32;
    fn edac_device_free_ctl_info(ctl: *mut edac_device_ctl_info);
    fn edac_device_del_device(dev: *mut device);
    fn dev_name(dev: *mut device) -> &'static str;
    fn devm_request_irq(dev: *mut device, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t, flags: u32, name: &str, data: *mut edac_device_ctl_info) -> i32;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut edac_device_ctl_info);
    fn dev_get_drvdata(dev: *mut device) -> *mut edac_device_ctl_info;
}

#[repr(C)] pub struct qcom_llcc_syn_regs { pub synd_reg: u32, pub count_status_reg: u32, pub ways_status_reg: u32 }

unsafe fn qcom_llcc_core_setup(drv: *mut llcc_drv_data, map: *mut regmap) -> i32 {
    let o = (*drv).edac_reg_offset;
    let mut ret = regmap_update_bits(map, (*o).cmn_interrupt_0_enable, TRP0_INTERRUPT_ENABLE, TRP0_INTERRUPT_ENABLE); if ret != 0 { return ret; }
    ret = regmap_update_bits(map, (*o).trp_interrupt_0_enable, SB_DB_TRP_INTERRUPT_ENABLE, SB_DB_TRP_INTERRUPT_ENABLE); if ret != 0 { return ret; }
    ret = regmap_write(map, (*o).drp_ecc_error_cfg, SB_ERROR_THRESHOLD << SB_ERROR_THRESHOLD_SHIFT); if ret != 0 { return ret; }
    ret = regmap_update_bits(map, (*o).cmn_interrupt_0_enable, DRP0_INTERRUPT_ENABLE, DRP0_INTERRUPT_ENABLE); if ret != 0 { return ret; }
    regmap_write(map, (*o).drp_interrupt_enable, SB_DB_DRP_INTERRUPT_ENABLE)
}

unsafe fn qcom_llcc_clear_error_status(err_type: i32, drv: *mut llcc_drv_data) -> i32 {
    let o = (*drv).edac_reg_offset; let (irq, cnt) = match err_type as usize { LLCC_DRAM_CE | LLCC_DRAM_UE => ((*o).drp_interrupt_clear, (*o).drp_ecc_error_cntr_clear), LLCC_TRAM_CE | LLCC_TRAM_UE => ((*o).trp_interrupt_0_clear, (*o).trp_ecc_error_cntr_clear), _ => return -22 };
    let ret = regmap_write((*drv).bcast_regmap, irq, DRP_TRP_INT_CLEAR); if ret != 0 { return ret; } regmap_write((*drv).bcast_regmap, cnt, DRP_TRP_CNT_CLEAR)
}

unsafe fn get_reg_offsets(drv: *mut llcc_drv_data, err_type: i32, r: *mut qcom_llcc_syn_regs) {
    let o = (*drv).edac_reg_offset; match err_type as usize {
        LLCC_DRAM_CE => { (*r).synd_reg=(*o).drp_ecc_sb_err_syn0; (*r).count_status_reg=(*o).drp_ecc_error_status1; (*r).ways_status_reg=(*o).drp_ecc_error_status0; },
        LLCC_DRAM_UE => { (*r).synd_reg=(*o).drp_ecc_db_err_syn0; (*r).count_status_reg=(*o).drp_ecc_error_status1; (*r).ways_status_reg=(*o).drp_ecc_error_status0; },
        LLCC_TRAM_CE => { (*r).synd_reg=(*o).trp_ecc_sb_err_syn0; (*r).count_status_reg=(*o).trp_ecc_error_status1; (*r).ways_status_reg=(*o).trp_ecc_error_status0; },
        LLCC_TRAM_UE => { (*r).synd_reg=(*o).trp_ecc_db_err_syn0; (*r).count_status_reg=(*o).trp_ecc_error_status1; (*r).ways_status_reg=(*o).trp_ecc_error_status0; }, _ => {}
    }
}

// Dump Syndrome registers data for Tag RAM, Data RAM bit errors
unsafe fn dump_syn_reg_values(drv: *mut llcc_drv_data, bank: u32, err_type: i32) -> i32 {
    let d=&EDAC_REG_DATA[err_type as usize]; let mut r=qcom_llcc_syn_regs{synd_reg:0,count_status_reg:0,ways_status_reg:0}; get_reg_offsets(drv,err_type,&mut r);
    let mut v=0; for i in 0..d.reg_cnt { let ret=regmap_read(*(*drv).regmaps.add(bank as usize),r.synd_reg+(i as u32)*4,&mut v); if ret!=0 { return qcom_llcc_clear_error_status(err_type,drv); } }
    let mut c=0; let ret=regmap_read(*(*drv).regmaps.add(bank as usize),r.count_status_reg,&mut c); if ret==0 { c=(c&d.count_mask)>>d.count_shift; }
    let mut w=0; let ret2=if ret==0 { regmap_read(*(*drv).regmaps.add(bank as usize),r.ways_status_reg,&mut w) } else { ret }; if ret2==0 { w=(w&d.ways_mask)>>d.ways_shift; } qcom_llcc_clear_error_status(err_type,drv)
}

unsafe fn dump_syn_reg(_ctl: *mut edac_device_ctl_info, err_type: i32, bank: u32, drv: *mut llcc_drv_data) -> i32 {
    let ret=dump_syn_reg_values(drv,bank,err_type); if ret!=0{return ret;} 0
}

unsafe extern "C" fn llcc_ecc_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t { let ctl=data as *mut edac_device_ctl_info; let drv=(*(*ctl).dev).platform_data as *mut llcc_drv_data; let mut rc=IRQ_NONE; for i in 0..(*drv).num_banks { let mut e=0; if regmap_read(*(*drv).regmaps.add(i as usize),(*(*drv).edac_reg_offset).drp_interrupt_status,&mut e)==0 && e&(SB_ECC_ERROR|DB_ECC_ERROR)!=0 { dump_syn_reg_values(drv,i,if e&SB_ECC_ERROR!=0 {LLCC_DRAM_CE as i32} else {LLCC_DRAM_UE as i32}); rc=IRQ_HANDLED; } } rc }

unsafe extern "C" fn llcc_ecc_check(ctl: *mut edac_device_ctl_info) { llcc_ecc_irq_handler(0,ctl as *mut core::ffi::c_void); }

unsafe extern "C" fn qcom_llcc_edac_probe(pdev: *mut platform_device) -> i32 {
    let drv=(*pdev).dev.platform_data as *mut llcc_drv_data;
    if !(*drv).ecc_irq_configured { let ret=qcom_llcc_core_setup(drv,(*drv).bcast_regmap); if ret!=0{return ret;} }
    let ctl=edac_device_alloc_ctl_info(0,"qcom-llcc",1,"bank",(*drv).num_banks,1,edac_device_alloc_index()); if ctl.is_null(){return -12;}
    (*ctl).dev=&mut (*pdev).dev; (*ctl).poll_msec=0; (*ctl).edac_check=None;
    let irq=(*drv).ecc_irq; let mut ret=0;
    if irq>0 { ret=devm_request_irq(&mut (*pdev).dev,irq,llcc_ecc_irq_handler,0x4,"llcc_ecc",ctl); if ret==0 { /* edac_op_state = EDAC_OPSTATE_INT */ } }
    if irq<=0 || ret!=0 { (*ctl).poll_msec=ECC_POLL_MSEC; (*ctl).edac_check=Some(llcc_ecc_check); /* edac_op_state = EDAC_OPSTATE_POLL */ }
    ret=edac_device_add_device(ctl); if ret!=0 { edac_device_free_ctl_info(ctl); return ret; } platform_set_drvdata(pdev,ctl); ret
}

unsafe extern "C" fn qcom_llcc_edac_remove(pdev: *mut platform_device) { let ctl=dev_get_drvdata(&mut (*pdev).dev); edac_device_del_device((*ctl).dev); edac_device_free_ctl_info(ctl); }

#[repr(C)] pub struct platform_device_id { pub name: &'static str }
static QCOM_LLCC_EDAC_ID_TABLE: [platform_device_id; 2] = [platform_device_id{name:"qcom_llcc_edac"}, platform_device_id{name:""}];
#[repr(C)] pub struct platform_driver { pub probe: unsafe extern "C" fn(*mut platform_device)->i32, pub remove: unsafe extern "C" fn(*mut platform_device), pub name: &'static str, pub id_table: *const platform_device_id }
static mut QCOM_LLCC_EDAC_DRIVER: platform_driver = platform_driver { probe:qcom_llcc_edac_probe, remove:qcom_llcc_edac_remove, name:"qcom_llcc_edac", id_table:QCOM_LLCC_EDAC_ID_TABLE.as_ptr() };

// module_platform_driver(qcom_llcc_edac_driver);
// MODULE_DESCRIPTION("QCOM EDAC driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
