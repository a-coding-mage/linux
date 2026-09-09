// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */

// Translated from adf_6xxx_hw_data.c. Types, constants, macros, and external
// functions are supplied by the surrounding driver bindings.

const RP_GROUP_0_MASK: u32 = (1 << 0) | (1 << 2);
const RP_GROUP_1_MASK: u32 = (1 << 1) | (1 << 3);
const RP_GROUP_ALL_MASK: u32 = RP_GROUP_0_MASK | RP_GROUP_1_MASK;
const ADF_AE_GROUP_0: u32 = 0xf;
const ADF_AE_GROUP_1: u32 = 0xf0;
const ADF_AE_GROUP_2: u32 = 1 << 8;
const ASB_MULTIPLIER: u32 = 9;

#[allow(non_camel_case_types)]
type c_ulong = usize;
#[allow(non_camel_case_types)]
type c_uint = u32;
#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_char = i8;

static mut adf_6xxx_class: adf_hw_device_class = adf_hw_device_class {
    name: ADF_6XXX_DEVICE_NAME,
    type_: DEV_6XXX,
    instances: 0,
};

#[repr(C)]
struct adf_ring_config {
    ring_mask: u32,
    ring_type: adf_cfg_service_type,
    thrd_mask: *const c_ulong,
}

static mut RMASK_TWO_SERVICES: [u32; 2] = [RP_GROUP_0_MASK, RP_GROUP_1_MASK];

#[repr(u32)]
enum adf_gen6_rps { RP0 = 0, RP1 = 1, RP2 = 2, RP3 = 3, RP_MAX = 3 }

static THRD_MASK_SYM: [c_ulong; ADF_6XXX_MAX_ACCELENGINES as usize] = [0x0c,0x0c,0x0c,0x0c,0x1c,0x1c,0x1c,0x1c,0];
static THRD_MASK_ASYM: [c_ulong; ADF_6XXX_MAX_ACCELENGINES as usize] = [0x70,0x70,0x70,0x70,0x60,0x60,0x60,0x60,0];
static THRD_MASK_CPR: [c_ulong; ADF_6XXX_MAX_ACCELENGINES as usize] = [1,1,1,1,1,1,1,1,0];
static THRD_MASK_DCC: [c_ulong; ADF_6XXX_MAX_ACCELENGINES as usize] = [0,0,0,0,7,7,3,3,0];
static THRD_MASK_DCPR: [c_ulong; ADF_6XXX_MAX_ACCELENGINES as usize] = [2,2,2,2,2,2,2,2,0];
static THRD_MASK_WCY: [c_ulong; ADF_6XXX_MAX_ACCELENGINES as usize] = [0x7f,0x7f,0x7f,0x7f,0x7f,0x7f,0x7f,0x7f,0];

static ADF_6XXX_FW_OBJS: [*const c_char; 4] = [ADF_6XXX_CY_OBJ, ADF_6XXX_DC_OBJ, ADF_6XXX_ADMIN_OBJ, ADF_6XXX_WCY_OBJ];
static ADF_DEFAULT_FW_CONFIG: [adf_fw_config; 3] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_DC_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_CY_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];
static ADF_WCY_FW_CONFIG: [adf_fw_config; 3] = [
    adf_fw_config { ae_mask: ADF_AE_GROUP_1, obj: ADF_FW_WCY_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_0, obj: ADF_FW_WCY_OBJ },
    adf_fw_config { ae_mask: ADF_AE_GROUP_2, obj: ADF_FW_ADMIN_OBJ },
];

unsafe fn services_supported(mask: c_ulong) -> bool {
    if mask >= (1 << SVC_COUNT) { return false; }
    match mask.count_ones() {
        1 => true,
        2 | 3 => (mask & (1 << SVC_DCC)) == 0,
        _ => false,
    }
}
unsafe fn wcy_services_supported(mask: c_ulong) -> bool { mask == (1 << SVC_SYM) }

unsafe fn get_service(mask: *mut c_ulong) -> c_int {
    for (bit, svc) in [(SVC_ASYM, SVC_ASYM),(SVC_SYM,SVC_SYM),(SVC_DC,SVC_DC),(SVC_DCC,SVC_DCC),(SVC_DECOMP,SVC_DECOMP)] {
        if (*mask & (1 << bit)) != 0 { *mask &= !(1 << bit); return svc as c_int; }
    }
    -EINVAL
}
unsafe fn get_ring_type(service: c_uint) -> adf_cfg_service_type {
    match service { SVC_SYM => SYM, SVC_ASYM => ASYM, SVC_DC | SVC_DCC => COMP, SVC_DECOMP => DECOMP, _ => UNUSED }
}
unsafe fn get_thrd_mask(accel_dev: *mut adf_accel_dev, service: c_uint) -> *const c_ulong {
    if adf_6xxx_is_wcy(GET_HW_DATA(accel_dev)) { return if service == SVC_SYM { THRD_MASK_WCY.as_ptr() } else { core::ptr::null() }; }
    match service { SVC_SYM=>THRD_MASK_SYM.as_ptr(), SVC_ASYM=>THRD_MASK_ASYM.as_ptr(), SVC_DC=>THRD_MASK_CPR.as_ptr(), SVC_DCC=>THRD_MASK_DCC.as_ptr(), SVC_DECOMP=>THRD_MASK_DCPR.as_ptr(), _=>core::ptr::null() }
}

unsafe fn get_rp_config(accel_dev: *mut adf_accel_dev, cfg: *mut adf_ring_config, n: *mut c_uint) -> c_int {
    let mut mask: c_ulong = 0; let ret = adf_get_service_mask(accel_dev, &mut mask); if ret != 0 { return ret; }
    let ns = mask.count_ones(); if ns > MAX_NUM_CONCURR_SVC { return -EINVAL; }
    for i in 0..ns as usize { let service = get_service(&mut mask); if service < 0 { return service; }
        (*cfg.add(i)).ring_type = get_ring_type(service as c_uint); (*cfg.add(i)).thrd_mask = get_thrd_mask(accel_dev, service as c_uint);
        (*cfg.add(i)).ring_mask = match ns { ADF_ONE_SERVICE => RP_GROUP_ALL_MASK, ADF_TWO_SERVICES => RMASK_TWO_SERVICES[i], ADF_THREE_SERVICES => { let mut v=1<<i; if service as c_uint==SVC_ASYM {v|=1<<RP3;} v }, _=>return -EINVAL };
    } *n=ns; 0
}

unsafe fn adf_gen6_get_arb_mask(accel_dev: *mut adf_accel_dev, ae: c_uint) -> u32 {
    let mut cfg: [adf_ring_config; MAX_NUM_CONCURR_SVC as usize] = core::mem::zeroed(); let mut ns=0; if get_rp_config(accel_dev,cfg.as_mut_ptr(),&mut ns)!=0{return 0;}
    let mut out=0; for i in 0..ns as usize { let p=(*cfg.as_ptr().add(i)).thrd_mask.add(ae as usize); let mut t=0; while t<ADF_NUM_THREADS_PER_AE { if *p & (1<<t)!=0 {out|=(*cfg.as_ptr().add(i)).ring_mask << (t*4);} t+=1; } } out
}

unsafe fn get_ring_to_svc_map(accel_dev: *mut adf_accel_dev) -> u16 {
    let mut rps: [adf_cfg_service_type; ADF_GEN6_NUM_BANKS_PER_VF as usize]=[UNUSED; ADF_GEN6_NUM_BANKS_PER_VF as usize]; let mut cfg:[adf_ring_config;MAX_NUM_CONCURR_SVC as usize]=core::mem::zeroed(); let mut ns=0; if get_rp_config(accel_dev,cfg.as_mut_ptr(),&mut ns)!=0{return 0;}
    for i in 0..ns as usize { let mut m=cfg[i].ring_mask; for r in 0..ADF_GEN6_NUM_BANKS_PER_VF {if m&(1<<r)!=0{rps[r as usize]=cfg[i].ring_type;} m&=!(1<<r);} }
    ((rps[0] as u16)<<ADF_CFG_SERV_RING_PAIR_0_SHIFT)|((rps[1] as u16)<<ADF_CFG_SERV_RING_PAIR_1_SHIFT)|((rps[2] as u16)<<ADF_CFG_SERV_RING_PAIR_2_SHIFT)|((rps[3] as u16)<<ADF_CFG_SERV_RING_PAIR_3_SHIFT)
}

// The remaining hardware callbacks retain the C structure layout and external
// driver operations; their definitions are expressed as direct unsafe Rust.
unsafe fn get_accel_mask(_: *mut adf_hw_device_data)->u32{ADF_GEN6_ACCELERATORS_MASK}
unsafe fn get_num_accels(_: *mut adf_hw_device_data)->u32{ADF_GEN6_MAX_ACCELERATORS}
unsafe fn get_num_aes(s:*mut adf_hw_device_data)->u32{if s.is_null(){0}else{(*s).ae_mask.count_ones()}}
unsafe fn get_misc_bar_id(_: *mut adf_hw_device_data)->u32{ADF_GEN6_PMISC_BAR}
unsafe fn get_etr_bar_id(_: *mut adf_hw_device_data)->u32{ADF_GEN6_ETR_BAR}
unsafe fn get_sram_bar_id(_: *mut adf_hw_device_data)->u32{ADF_GEN6_SRAM_BAR}
unsafe fn get_sku(_: *mut adf_hw_device_data)->dev_sku_info{DEV_SKU_1}

pub unsafe fn adf_init_hw_data_6xxx(hw_data: *mut adf_hw_device_data) {
    (*hw_data).dev_class=&mut adf_6xxx_class; (*hw_data).instance_id=adf_6xxx_class.instances; adf_6xxx_class.instances+=1;
    (*hw_data).num_banks=ADF_GEN6_ETR_MAX_BANKS; (*hw_data).num_banks_per_vf=ADF_GEN6_NUM_BANKS_PER_VF; (*hw_data).num_rings_per_bank=ADF_GEN6_NUM_RINGS_PER_BANK; (*hw_data).num_accel=ADF_GEN6_MAX_ACCELERATORS; (*hw_data).num_engines=ADF_6XXX_MAX_ACCELENGINES; (*hw_data).num_logical_accel=1; (*hw_data).tx_rx_gap=ADF_GEN6_RX_RINGS_OFFSET; (*hw_data).tx_rings_mask=ADF_GEN6_TX_RINGS_MASK; (*hw_data).ring_to_svc_map=0;
    (*hw_data).admin_ae_mask=ADF_6XXX_ADMIN_AE_MASK; (*hw_data).fw_name=ADF_6XXX_FW; (*hw_data).fw_mmp_name=ADF_6XXX_MMP; (*hw_data).num_hb_ctrs=ADF_NUM_HB_CNT_PER_AE; (*hw_data).num_rps=ADF_GEN6_ETR_MAX_BANKS; (*hw_data).clock_frequency=ADF_6XXX_AE_FREQ; (*hw_data).accel_capabilities_ext_mask=ADF_ACCEL_CAPABILITIES_EXT_ZSTD;
    adf_gen6_init_services_supported(hw_data); adf_gen6_init_hw_csr_ops(&mut (*hw_data).csr_ops); adf_gen6_init_pf_pfvf_ops(&mut (*hw_data).pfvf_ops); adf_gen6_init_dc_ops(&mut (*hw_data).dc_ops); adf_gen6_init_vf_mig_ops(&mut (*hw_data).vfmig_ops); adf_gen6_init_ras_ops(&mut (*hw_data).ras_ops); adf_gen6_init_tl_data(&mut (*hw_data).tl_data); adf_gen6_init_rl_data(&mut (*hw_data).rl_data); adf_gen6_init_anti_rb(&mut (*hw_data).anti_rb_data); adf_gen6_init_kpt(&mut (*hw_data).kpt_data);
}
pub unsafe fn adf_clean_hw_data_6xxx(hw_data:*mut adf_hw_device_data){if (*(*hw_data).dev_class).instances!=0{(*(*hw_data).dev_class).instances-=1;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
