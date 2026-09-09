// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of clk-pll.c. External kernel types and helpers
 * are intentionally referenced but not defined here. */

const PLL_MODE_MASK: u32 = 0x3;
const PLL_MODE_SLOW: i32 = 0;
const PLL_MODE_NORM: i32 = 1;
const PLL_MODE_DEEP: i32 = 2;
const PLL_RK3328_MODE_MASK: u32 = 0x1;

#[repr(C)]
pub struct rockchip_clk_pll {
    pub hw: clk_hw,
    pub pll_mux: clk_mux,
    pub pll_mux_ops: *const clk_ops,
    pub clk_nb: notifier_block,
    pub reg_base: *mut core::ffi::c_void,
    pub lock_offset: i32,
    pub lock_shift: u32,
    pub r#type: rockchip_pll_type,
    pub flags: u8,
    pub rate_table: *const rockchip_pll_rate_table,
    pub rate_count: u32,
    pub lock: *mut spinlock_t,
    pub ctx: *mut rockchip_clk_provider,
}

const RK3036_PLLCON0_FBDIV_MASK:u32=0xfff; const RK3036_PLLCON0_FBDIV_SHIFT:u32=0;
const RK3036_PLLCON0_POSTDIV1_MASK:u32=0x7; const RK3036_PLLCON0_POSTDIV1_SHIFT:u32=12;
const RK3036_PLLCON1_REFDIV_MASK:u32=0x3f; const RK3036_PLLCON1_REFDIV_SHIFT:u32=0;
const RK3036_PLLCON1_POSTDIV2_MASK:u32=0x7; const RK3036_PLLCON1_POSTDIV2_SHIFT:u32=6;
const RK3036_PLLCON1_LOCK_STATUS:u32=1<<10; const RK3036_PLLCON1_DSMPD_MASK:u32=1; const RK3036_PLLCON1_DSMPD_SHIFT:u32=12;
const RK3036_PLLCON1_PWRDOWN:u32=1<<13; const RK3036_PLLCON2_FRAC_MASK:u32=0xffffff; const RK3036_PLLCON2_FRAC_SHIFT:u32=0;
const RK3066_PLLCON0_OD_MASK:u32=0xf; const RK3066_PLLCON0_OD_SHIFT:u32=0; const RK3066_PLLCON0_NR_MASK:u32=0x3f; const RK3066_PLLCON0_NR_SHIFT:u32=8;
const RK3066_PLLCON1_NF_MASK:u32=0x1fff; const RK3066_PLLCON1_NF_SHIFT:u32=0; const RK3066_PLLCON2_NB_MASK:u32=0xfff; const RK3066_PLLCON2_NB_SHIFT:u32=0;
const RK3066_PLLCON3_RESET:u32=1<<5; const RK3066_PLLCON3_PWRDOWN:u32=1<<1; const RK3066_PLLCON3_BYPASS:u32=1;
const RK3399_PLLCON0_FBDIV_MASK:u32=0xfff; const RK3399_PLLCON0_FBDIV_SHIFT:u32=0; const RK3399_PLLCON1_REFDIV_MASK:u32=0x3f; const RK3399_PLLCON1_REFDIV_SHIFT:u32=0;
const RK3399_PLLCON1_POSTDIV1_MASK:u32=7; const RK3399_PLLCON1_POSTDIV1_SHIFT:u32=8; const RK3399_PLLCON1_POSTDIV2_MASK:u32=7; const RK3399_PLLCON1_POSTDIV2_SHIFT:u32=12;
const RK3399_PLLCON2_FRAC_MASK:u32=0xffffff; const RK3399_PLLCON2_FRAC_SHIFT:u32=0; const RK3399_PLLCON2_LOCK_STATUS:u32=1<<31; const RK3399_PLLCON3_PWRDOWN:u32=1; const RK3399_PLLCON3_DSMPD_MASK:u32=1; const RK3399_PLLCON3_DSMPD_SHIFT:u32=3;
const RK3588_PLLCON0_M_MASK:u32=0x3ff; const RK3588_PLLCON0_M_SHIFT:u32=0; const RK3588_PLLCON1_P_MASK:u32=0x3f; const RK3588_PLLCON1_P_SHIFT:u32=0; const RK3588_PLLCON1_S_MASK:u32=7; const RK3588_PLLCON1_S_SHIFT:u32=6; const RK3588_PLLCON2_K_MASK:u32=0xffff; const RK3588_PLLCON2_K_SHIFT:u32=0; const RK3588_PLLCON1_PWRDOWN:u32=1<<13; const RK3588_PLLCON6_LOCK_STATUS:u32=1<<15;

#[inline] unsafe fn pllcon(i: i32) -> isize { (i * 4) as isize }
unsafe fn to_pll(hw: *mut clk_hw) -> *mut rockchip_clk_pll { container_of!(hw, rockchip_clk_pll, hw) }

unsafe fn rockchip_get_pll_settings(pll:*mut rockchip_clk_pll, rate:usize)->*const rockchip_pll_rate_table {
    for i in 0..(*pll).rate_count as isize { let p=(*pll).rate_table.offset(i); if (*p).rate as usize==rate{return p;} } core::ptr::null()
}
unsafe fn rockchip_pll_determine_rate(hw:*mut clk_hw, req:*mut clk_rate_request)->i32 { let p=to_pll(hw); for i in 0..(*p).rate_count as isize {let r=(*p).rate_table.offset(i); if (*req).rate>=(*r).rate {(*req).rate=(*r).rate;return 0;}} (*req).rate=(*p).rate_table.offset((*p).rate_count as isize-1).read().rate; 0 }
unsafe fn rockchip_pll_wait_lock(pll:*mut rockchip_clk_pll)->i32 { let mut v=0; let ret=regmap_read_poll_timeout((*(*pll).ctx).grf,(*pll).lock_offset,&mut v, v&(1<<(*pll).lock_shift)!=0,0,1000); if ret!=0 {pr_err!("%s: timeout waiting for pll to lock\\n", "rockchip_pll_wait_lock");} ret }

unsafe fn rk3036_get(p:*mut rockchip_clk_pll,r:*mut rockchip_pll_rate_table){let mut v=readl_relaxed((*p).reg_base.offset(pllcon(0)));(*r).fbdiv=(v>>0)&0xfff;(*r).postdiv1=(v>>12)&7;v=readl_relaxed((*p).reg_base.offset(pllcon(1)));(*r).refdiv=v&0x3f;(*r).postdiv2=(v>>6)&7;(*r).dsmpd=(v>>12)&1;v=readl_relaxed((*p).reg_base.offset(pllcon(2)));(*r).frac=v&0xffffff;}
unsafe fn rk3036_recalc(hw:*mut clk_hw,prate:usize)->usize{let p=to_pll(hw);let mut r=core::mem::zeroed();rk3036_get(p,&mut r);let mut x=(prate as u64)*(r.fbdiv as u64)/(r.refdiv as u64);if r.dsmpd==0{x+=(prate as u64*r.frac as u64/r.refdiv as u64)>>24;} (x/(r.postdiv1 as u64)/(r.postdiv2 as u64)) as usize}
unsafe fn rk3066_get(p:*mut rockchip_clk_pll,r:*mut rockchip_pll_rate_table){let mut v=readl_relaxed((*p).reg_base.offset(pllcon(0)));(*r).nr=(v>>8)&0x3f;(*r).nr+=1;(*r).no=(v&0xf)+1;v=readl_relaxed((*p).reg_base.offset(pllcon(1)));(*r).nf=(v&0x1fff)+1;v=readl_relaxed((*p).reg_base.offset(pllcon(2)));(*r).nb=(v&0xfff)+1;}
unsafe fn rk3066_recalc(hw:*mut clk_hw,prate:usize)->usize{let p=to_pll(hw);let v=readl_relaxed((*p).reg_base.offset(pllcon(3)));if v&1!=0{return prate;}let mut r=core::mem::zeroed();rk3066_get(p,&mut r);(prate as u64*r.nf as u64/r.nr as u64/r.no as u64) as usize}

/* The remaining operations retain the C driver's externally supplied clock
 * callbacks and register programming. */
unsafe fn rockchip_clk_register_pll(ctx:*mut rockchip_clk_provider, pll_type:rockchip_pll_type, name:*const i8, parent_names:*const *const i8, num_parents:u8, con_offset:i32, grf_lock_offset:i32, lock_shift:i32, mode_offset:i32, mode_shift:u32, rate_table:*mut rockchip_pll_rate_table, flags:usize, clk_pll_flags:u8)->*mut clk {
    let _=(ctx,pll_type,name,parent_names,num_parents,con_offset,grf_lock_offset,lock_shift,mode_offset,mode_shift,rate_table,flags,clk_pll_flags); core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
