// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Translated from ivpu_hw_btrs.c. Register and driver symbols are supplied by
// the surrounding driver environment.

const PLL_CDYN_DEFAULT: u32 = 0x80;
const PLL_EPP_DEFAULT: u32 = 0x80;
const PLL_REF_CLK_FREQ_MHZ: u32 = 50;
const PLL_TIMEOUT_US: u32 = 1500 * USEC_PER_MSEC;
const IDLE_TIMEOUT_US: u32 = 5 * USEC_PER_MSEC;
const TIMEOUT_US: u32 = 150 * USEC_PER_MSEC;
const MTL_CONFIG_1_TILE: u32 = 0x01;
const MTL_CONFIG_2_TILE: u32 = 0x02;
const MTL_PLL_RATIO_5_3: u32 = 0x01;
const MTL_PLL_RATIO_4_3: u32 = 0x02;
const BTRS_MTL_TILE_FUSE_ENABLE_BOTH: u32 = 0x0;
const BTRS_MTL_TILE_SKU_BOTH: u32 = 0x3630;
const BTRS_LNL_TILE_MAX_NUM: u32 = 6;
const BTRS_LNL_TILE_MAX_MASK: u32 = 0x3f;
const WEIGHTS_DEFAULT: u32 = 0xf711f711;
const WEIGHTS_ATS_DEFAULT: u32 = 0x0000f711;
const DCT_REQ: u32 = 0x2;
const DCT_ENABLE: u32 = 0x1;
const DCT_DISABLE: u32 = 0x0;

#[inline]
fn wp_config(tile: u32, ratio: u32) -> u32 { (tile << 8) | ratio }

#[repr(C)]
pub struct wp_request { pub min: u16, pub max: u16, pub target: u16, pub cfg: u16, pub epp: u16, pub cdyn: u16 }

pub unsafe fn ivpu_hw_btrs_irqs_clear_with_0_mtl(vdev: *mut ivpu_device) -> bool {
    REGB_WR32!(vdev, VPU_HW_BTRS_MTL_INTERRUPT_STAT, BTRS_MTL_ALL_IRQ_MASK);
    if REGB_RD32!(vdev, VPU_HW_BTRS_MTL_INTERRUPT_STAT) == BTRS_MTL_ALL_IRQ_MASK {
        REGB_WR32!(vdev, VPU_HW_BTRS_MTL_INTERRUPT_STAT, 0);
        return true;
    }
    false
}

unsafe fn freq_ratios_init_mtl(vdev: *mut ivpu_device) {
    let hw = (*vdev).hw; let fmin = REGB_RD32!(vdev, VPU_HW_BTRS_MTL_FMIN_FUSE);
    (*hw).pll.min_ratio = REG_GET_FLD!(VPU_HW_BTRS_MTL_FMIN_FUSE, MIN_RATIO, fmin);
    (*hw).pll.pn_ratio = REG_GET_FLD!(VPU_HW_BTRS_MTL_FMIN_FUSE, PN_RATIO, fmin);
    let fmax = REGB_RD32!(vdev, VPU_HW_BTRS_MTL_FMAX_FUSE);
    (*hw).pll.max_ratio = REG_GET_FLD!(VPU_HW_BTRS_MTL_FMAX_FUSE, MAX_RATIO, fmax);
}
unsafe fn freq_ratios_init_lnl(vdev: *mut ivpu_device) {
    let hw = (*vdev).hw; let fmin = REGB_RD32!(vdev, VPU_HW_BTRS_LNL_FMIN_FUSE);
    (*hw).pll.min_ratio = REG_GET_FLD!(VPU_HW_BTRS_LNL_FMIN_FUSE, MIN_RATIO, fmin);
    (*hw).pll.pn_ratio = REG_GET_FLD!(VPU_HW_BTRS_LNL_FMIN_FUSE, PN_RATIO, fmin);
    let fmax = REGB_RD32!(vdev, VPU_HW_BTRS_LNL_FMAX_FUSE);
    (*hw).pll.max_ratio = REG_GET_FLD!(VPU_HW_BTRS_LNL_FMAX_FUSE, MAX_RATIO, fmax);
}
pub unsafe fn ivpu_hw_btrs_freq_ratios_init(vdev: *mut ivpu_device) {
    let hw = (*vdev).hw;
    if ivpu_hw_btrs_gen(vdev) == IVPU_HW_BTRS_MTL { freq_ratios_init_mtl(vdev); } else { freq_ratios_init_lnl(vdev); }
    (*hw).pll.min_ratio = clamp_t!(u8, ivpu_pll_min_ratio, (*hw).pll.min_ratio, (*hw).pll.max_ratio);
    (*hw).pll.max_ratio = clamp_t!(u8, ivpu_pll_max_ratio, (*hw).pll.min_ratio, (*hw).pll.max_ratio);
    (*hw).pll.pn_ratio = clamp_t!(u8, (*hw).pll.pn_ratio, (*hw).pll.min_ratio, (*hw).pll.max_ratio);
    (*hw).pll.cfg_max_ratio = (*hw).pll.max_ratio; (*hw).pll.cfg_min_ratio = (*hw).pll.min_ratio;
}

fn tile_disable_check(config: u32) -> bool { config == 0 || (config <= (1 << (BTRS_LNL_TILE_MAX_NUM - 1)) && (config & (config - 1)) == 0) }
unsafe fn read_tile_config_fuse(vdev: *mut ivpu_device, out: *mut u32) -> i32 {
    let fuse = REGB_RD32!(vdev, VPU_HW_BTRS_LNL_TILE_FUSE);
    if !REG_TEST_FLD!(VPU_HW_BTRS_LNL_TILE_FUSE, VALID, fuse) { ivpu_err!(vdev, "Fuse: invalid (0x%x)\n", fuse); return -EIO; }
    let config = REG_GET_FLD!(VPU_HW_BTRS_LNL_TILE_FUSE, CONFIG, fuse);
    if !tile_disable_check(config) { ivpu_warn!(vdev, "More than 1 tile disabled, tile fuse config mask: 0x%x\n", config); }
    ivpu_dbg!(vdev, MISC, "Tile disable config mask: 0x%x\n", config); *out = config; 0
}
unsafe fn info_init_mtl(vdev: *mut ivpu_device) -> i32 { let hw=(*vdev).hw; (*hw).tile_fuse=BTRS_MTL_TILE_FUSE_ENABLE_BOTH; (*hw).sku=BTRS_MTL_TILE_SKU_BOTH; (*hw).config=wp_config(MTL_CONFIG_2_TILE, MTL_PLL_RATIO_4_3); 0 }
unsafe fn info_init_lnl(vdev: *mut ivpu_device) -> i32 { let hw=(*vdev).hw; let mut c=0; let r=read_tile_config_fuse(vdev,&mut c); if r!=0{return r;} (*hw).tile_fuse=c; (*hw).pll.profiling_freq=PLL_PROFILING_FREQ_DEFAULT; 0 }
pub unsafe fn ivpu_hw_btrs_info_init(vdev:*mut ivpu_device)->i32 { if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL {info_init_mtl(vdev)} else {info_init_lnl(vdev)} }

unsafe fn wp_request_sync(vdev:*mut ivpu_device)->i32 { if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL {REGB_POLL_FLD!(vdev,VPU_HW_BTRS_MTL_WP_REQ_CMD,SEND,0,PLL_TIMEOUT_US)} else {REGB_POLL_FLD!(vdev,VPU_HW_BTRS_LNL_WP_REQ_CMD,SEND,0,PLL_TIMEOUT_US)} }
unsafe fn wait_for_status_ready(vdev:*mut ivpu_device, enable:bool)->i32 { if IVPU_WA!(vdev,punit_disabled){return 0;} let e=if enable{1}else{0}; if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL {REGB_POLL_FLD!(vdev,VPU_HW_BTRS_MTL_VPU_STATUS,READY,e,PLL_TIMEOUT_US)} else {REGB_POLL_FLD!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS,READY,e,PLL_TIMEOUT_US)} }

unsafe fn wp_request_mtl(vdev:*mut ivpu_device, wp:&wp_request) { let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD0); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD0,MIN_RATIO,wp.min,v); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD0,MAX_RATIO,wp.max,v); REGB_WR32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD0,v); let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD1); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD1,TARGET_RATIO,wp.target,v); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD1,EPP,PLL_EPP_DEFAULT,v); REGB_WR32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD1,v); let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD2); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD2,CONFIG,wp.cfg,v); REGB_WR32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_PAYLOAD2,v); let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_CMD); v=REG_SET_FLD!(VPU_HW_BTRS_MTL_WP_REQ_CMD,SEND,v); REGB_WR32!(vdev,VPU_HW_BTRS_MTL_WP_REQ_CMD,v); }
unsafe fn wp_request_lnl(vdev:*mut ivpu_device,wp:&wp_request) { let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD0); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD0,MIN_RATIO,wp.min,v); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD0,MAX_RATIO,wp.max,v); REGB_WR32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD0,v); let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD1); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD1,TARGET_RATIO,wp.target,v); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD1,EPP,wp.epp,v); REGB_WR32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD1,v); let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD2); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD2,CONFIG,wp.cfg,v); v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD2,CDYN,wp.cdyn,v); REGB_WR32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_PAYLOAD2,v); let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_CMD); v=REG_SET_FLD!(VPU_HW_BTRS_LNL_WP_REQ_CMD,SEND,v); REGB_WR32!(vdev,VPU_HW_BTRS_LNL_WP_REQ_CMD,v); }
unsafe fn wp_request(vdev:*mut ivpu_device,wp:&wp_request){if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{wp_request_mtl(vdev,wp)}else{wp_request_lnl(vdev,wp)}}
unsafe fn wp_request_send(vdev:*mut ivpu_device,wp:&wp_request)->i32{let r=wp_request_sync(vdev);if r!=0{ivpu_err!(vdev,"Failed to sync before workpoint request: %d\n",r);return r;}wp_request(vdev,wp);let r=wp_request_sync(vdev);if r!=0{ivpu_err!(vdev,"Failed to sync after workpoint request: %d\n",r);}r}
unsafe fn prepare_wp_request(vdev:*mut ivpu_device,wp:&mut wp_request,enable:bool){let h=(*vdev).hw;wp.min=(*h).pll.min_ratio;wp.max=(*h).pll.max_ratio;if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{wp.target=if enable{(*h).pll.pn_ratio}else{0};wp.cfg=if enable{(*h).config}else{0};wp.cdyn=0;wp.epp=0;}else{wp.target=(*h).pll.pn_ratio;wp.cfg=0;wp.cdyn=if enable{PLL_CDYN_DEFAULT as u16}else{0};wp.epp=if enable{PLL_EPP_DEFAULT as u16}else{0};}}

unsafe fn wait_for_pll_lock(vdev:*mut ivpu_device,enable:bool)->i32{if ivpu_hw_btrs_gen(vdev)!=IVPU_HW_BTRS_MTL||IVPU_WA!(vdev,punit_disabled){return 0;}REGB_POLL_FLD!(vdev,VPU_HW_BTRS_MTL_PLL_STATUS,LOCK,if enable{1}else{0},PLL_TIMEOUT_US)}
unsafe fn wait_for_cdyn_deassert(vdev:*mut ivpu_device)->i32{if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{0}else{REGB_POLL_FLD!(vdev,VPU_HW_BTRS_LNL_CDYN,CDYN,0,PLL_TIMEOUT_US)}}
pub unsafe fn ivpu_hw_btrs_wp_drive(vdev:*mut ivpu_device,enable:bool)->i32{if IVPU_WA!(vdev,punit_disabled){ivpu_dbg!(vdev,PM,"Skipping workpoint request\n");return 0;}let mut wp=wp_request{min:0,max:0,target:0,cfg:0,epp:0,cdyn:0};prepare_wp_request(vdev,&mut wp,enable);ivpu_dbg!(vdev,PM,"PLL workpoint request: %u MHz, config: 0x%x, epp: 0x%x, cdyn: 0x%x\n",ivpu_hw_btrs_pll_ratio_to_mhz(vdev,wp.target as u8),wp.cfg,wp.epp,wp.cdyn);let mut r=wp_request_send(vdev,&wp);if r!=0{ivpu_err!(vdev,"Failed to send workpoint request: %d\n",r);return r;}r=wait_for_pll_lock(vdev,enable);if r!=0{ivpu_err!(vdev,"Timed out waiting for PLL lock\n");return r;}r=wait_for_status_ready(vdev,enable);if r!=0{ivpu_err!(vdev,"Timed out waiting for NPU ready status\n");return r;}if !enable{r=wait_for_cdyn_deassert(vdev);if r!=0{ivpu_err!(vdev,"Timed out waiting for CDYN deassert\n");return r;}}0}

unsafe fn d0i3_drive(vdev:*mut ivpu_device,enable:bool)->i32{let reg=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{VPU_HW_BTRS_MTL_D0I3_CONTROL}else{VPU_HW_BTRS_LNL_D0I3_CONTROL};let mut r=REGB_POLL_FLD!(vdev,reg,INPROGRESS,0,TIMEOUT_US);if r!=0{ivpu_err!(vdev,"Failed to sync before D0i3 transition: %d\n",r);return r;}let mut v=REGB_RD32!(vdev,reg);v=if enable{REG_SET_FLD!(reg,I3,v)}else{REG_CLR_FLD!(reg,I3,v)};REGB_WR32!(vdev,reg,v);r=REGB_POLL_FLD!(vdev,reg,INPROGRESS,0,TIMEOUT_US);if r!=0{ivpu_err!(vdev,"Failed to sync after D0i3 transition: %d\n",r);}r}
pub unsafe fn ivpu_hw_btrs_d0i3_enable(vdev:*mut ivpu_device)->i32{if IVPU_WA!(vdev,punit_disabled){return 0;}let r=d0i3_drive(vdev,true);if r!=0{ivpu_err!(vdev,"Failed to enable D0i3: %d\n",r);}udelay(5);r}
pub unsafe fn ivpu_hw_btrs_d0i3_disable(vdev:*mut ivpu_device)->i32{if IVPU_WA!(vdev,punit_disabled){return 0;}let r=d0i3_drive(vdev,false);if r!=0{ivpu_err!(vdev,"Failed to disable D0i3: %d\n",r);}r}
pub unsafe fn ivpu_hw_btrs_wait_for_clock_res_own_ack(vdev:*mut ivpu_device)->i32{if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{0}else{REGB_POLL_FLD!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS,CLOCK_RESOURCE_OWN_ACK,1,TIMEOUT_US)}}
pub unsafe fn ivpu_hw_btrs_set_port_arbitration_weights_lnl(vdev:*mut ivpu_device){REGB_WR32!(vdev,VPU_HW_BTRS_LNL_PORT_ARBITRATION_WEIGHTS,WEIGHTS_DEFAULT);REGB_WR32!(vdev,VPU_HW_BTRS_LNL_PORT_ARBITRATION_WEIGHTS_ATS,WEIGHTS_ATS_DEFAULT);}

// The remaining routines retain the direct register-level structure of the C implementation.
pub unsafe fn ivpu_hw_btrs_ip_reset(vdev:*mut ivpu_device)->i32{if IVPU_WA!(vdev,punit_disabled){return 0;}let reg=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{VPU_HW_BTRS_MTL_VPU_IP_RESET}else{ivpu_hw_btrs_clock_relinquish_disable_lnl(vdev);VPU_HW_BTRS_LNL_IP_RESET};let mut r=REGB_POLL_FLD!(vdev,reg,TRIGGER,0,TIMEOUT_US);if r!=0{return r;}let mut v=REGB_RD32!(vdev,reg);v=REG_SET_FLD!(reg,TRIGGER,v);REGB_WR32!(vdev,reg,v);r=REGB_POLL_FLD!(vdev,reg,TRIGGER,0,TIMEOUT_US);r}
pub unsafe fn ivpu_hw_btrs_profiling_freq_reg_set_lnl(vdev:*mut ivpu_device){let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS);v=if (*vdev).hw.pll.profiling_freq==PLL_PROFILING_FREQ_DEFAULT{REG_CLR_FLD!(VPU_HW_BTRS_LNL_VPU_STATUS,PERF_CLK,v)}else{REG_SET_FLD!(VPU_HW_BTRS_LNL_VPU_STATUS,PERF_CLK,v)};REGB_WR32!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS,v)}
pub unsafe fn ivpu_hw_btrs_ats_print_lnl(vdev:*mut ivpu_device){ivpu_dbg!(vdev,MISC,"Buttress ATS: %s\n",if REGB_RD32!(vdev,VPU_HW_BTRS_LNL_HM_ATS)!=0{"Enable"}else{"Disable"});}
pub unsafe fn ivpu_hw_btrs_clock_relinquish_disable_lnl(vdev:*mut ivpu_device){let mut v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS);v=REG_SET_FLD!(VPU_HW_BTRS_LNL_VPU_STATUS,DISABLE_CLK_RELINQUISH,v);REGB_WR32!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS,v)}
pub unsafe fn ivpu_hw_btrs_is_idle(vdev:*mut ivpu_device)->bool{if IVPU_WA!(vdev,punit_disabled){return true;}let r=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{VPU_HW_BTRS_MTL_VPU_STATUS}else{VPU_HW_BTRS_LNL_VPU_STATUS};let v=REGB_RD32!(vdev,r);REG_TEST_FLD!(r,READY,v)&&REG_TEST_FLD!(r,IDLE,v)}
pub unsafe fn ivpu_hw_btrs_wait_for_idle(vdev:*mut ivpu_device)->i32{let r=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{VPU_HW_BTRS_MTL_VPU_STATUS}else{VPU_HW_BTRS_LNL_VPU_STATUS};REGB_POLL_FLD!(vdev,r,IDLE,1,IDLE_TIMEOUT_US)}
unsafe fn pll_ratio_to_mhz_mtl(r:u8)->u32{(r as u32*PLL_REF_CLK_FREQ_MHZ*2)/3} unsafe fn pll_ratio_to_mhz_lnl(r:u8)->u32{(r as u32*PLL_REF_CLK_FREQ_MHZ)/2}
pub unsafe fn ivpu_hw_btrs_pll_ratio_to_mhz(vdev:*mut ivpu_device,r:u8)->u32{if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{pll_ratio_to_mhz_mtl(r)}else{pll_ratio_to_mhz_lnl(r)}}
pub unsafe fn ivpu_hw_btrs_pll_ratio_to_hz(vdev:*mut ivpu_device,r:u8)->u32{ivpu_hw_btrs_pll_ratio_to_mhz(vdev,r)*HZ_PER_MHZ}
pub unsafe fn ivpu_hw_btrs_current_freq_get(vdev:*mut ivpu_device)->u32{let r=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{REGB_RD32!(vdev,VPU_HW_BTRS_MTL_CURRENT_PLL)}else{REGB_RD32!(vdev,VPU_HW_BTRS_LNL_PLL_FREQ)};ivpu_hw_btrs_pll_ratio_to_mhz(vdev,r as u8)}

pub unsafe fn ivpu_hw_btrs_telemetry_offset_get(vdev:*mut ivpu_device)->u32{if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{REGB_RD32!(vdev,VPU_HW_BTRS_MTL_VPU_TELEMETRY_OFFSET)}else{REGB_RD32!(vdev,VPU_HW_BTRS_LNL_VPU_TELEMETRY_OFFSET)}}
pub unsafe fn ivpu_hw_btrs_telemetry_size_get(vdev:*mut ivpu_device)->u32{if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{REGB_RD32!(vdev,VPU_HW_BTRS_MTL_VPU_TELEMETRY_SIZE)}else{REGB_RD32!(vdev,VPU_HW_BTRS_LNL_VPU_TELEMETRY_SIZE)}}
pub unsafe fn ivpu_hw_btrs_telemetry_enable_get(vdev:*mut ivpu_device)->u32{if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{REGB_RD32!(vdev,VPU_HW_BTRS_MTL_VPU_TELEMETRY_ENABLE)}else{REGB_RD32!(vdev,VPU_HW_BTRS_LNL_VPU_TELEMETRY_ENABLE)}}
pub unsafe fn ivpu_hw_btrs_global_int_disable(vdev:*mut ivpu_device){let r=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{VPU_HW_BTRS_MTL_GLOBAL_INT_MASK}else{VPU_HW_BTRS_LNL_GLOBAL_INT_MASK};REGB_WR32!(vdev,r,1)}
pub unsafe fn ivpu_hw_btrs_global_int_enable(vdev:*mut ivpu_device){let r=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{VPU_HW_BTRS_MTL_GLOBAL_INT_MASK}else{VPU_HW_BTRS_LNL_GLOBAL_INT_MASK};REGB_WR32!(vdev,r,0)}
pub unsafe fn ivpu_hw_btrs_irq_disable(vdev:*mut ivpu_device){let(g,l)=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{(VPU_HW_BTRS_MTL_GLOBAL_INT_MASK,VPU_HW_BTRS_MTL_LOCAL_INT_MASK)}else{(VPU_HW_BTRS_LNL_GLOBAL_INT_MASK,VPU_HW_BTRS_LNL_LOCAL_INT_MASK)};REGB_WR32!(vdev,g,1);REGB_WR32!(vdev,l,u32::MAX)}
pub unsafe fn ivpu_hw_btrs_platform_read(vdev:*mut ivpu_device)->u32{let r=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_VPU_STATUS);REG_GET_FLD!(VPU_HW_BTRS_LNL_VPU_STATUS,PLATFORM,r)}

pub unsafe fn ivpu_hw_btrs_irq_enable(vdev:*mut ivpu_device){let(l,g,m)=if ivpu_hw_btrs_gen(vdev)==IVPU_HW_BTRS_MTL{(VPU_HW_BTRS_MTL_LOCAL_INT_MASK,VPU_HW_BTRS_MTL_GLOBAL_INT_MASK,BTRS_MTL_IRQ_MASK)}else{(VPU_HW_BTRS_LNL_LOCAL_INT_MASK,VPU_HW_BTRS_LNL_GLOBAL_INT_MASK,BTRS_LNL_IRQ_MASK)};REGB_WR32!(vdev,l,!m);REGB_WR32!(vdev,g,0)}
pub unsafe fn ivpu_hw_btrs_dct_get_request(vdev:*mut ivpu_device,enable:*mut bool)->i32{let v=REGB_RD32!(vdev,VPU_HW_BTRS_LNL_PCODE_MAILBOX_SHADOW);let c=REG_GET_FLD!(VPU_HW_BTRS_LNL_PCODE_MAILBOX_SHADOW,CMD,v);let p=REG_GET_FLD!(VPU_HW_BTRS_LNL_PCODE_MAILBOX_SHADOW,PARAM1,v);if c!=DCT_REQ{return -EBADR;}match p{DCT_ENABLE=>{*enable=true;0},DCT_DISABLE=>{*enable=false;0},_=>-EINVAL}}
pub unsafe fn ivpu_hw_btrs_dct_set_status(vdev:*mut ivpu_device,enable:bool,active_percent:u8){let mut v=0;v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_PCODE_MAILBOX_STATUS,CMD,DCT_REQ,v);v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_PCODE_MAILBOX_STATUS,PARAM1,if enable{DCT_ENABLE}else{DCT_DISABLE},v);v=REG_SET_FLD_NUM!(VPU_HW_BTRS_LNL_PCODE_MAILBOX_STATUS,PARAM2,active_percent,v);REGB_WR32!(vdev,VPU_HW_BTRS_LNL_PCODE_MAILBOX_STATUS,v)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
