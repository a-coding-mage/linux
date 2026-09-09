// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM33XX PRM functions
 *
 * Copyright (C) 2011-2012 Texas Instruments Incorporated - https://www.ti.com/
 */

// Dependencies supplied by the surrounding kernel translation.

/* Read a register in a PRM instance */
unsafe fn am33xx_prm_read_reg(inst: i16, idx: u16) -> u32 {
    readl_relaxed(prm_base.va.add((inst as isize + idx as isize) as usize))
}

/* Write into a register in a PRM instance */
unsafe fn am33xx_prm_write_reg(val: u32, inst: i16, idx: u16) {
    writel_relaxed(val, prm_base.va.add((inst as isize + idx as isize) as usize));
}

/* Read-modify-write a register in PRM. Caller must lock */
unsafe fn am33xx_prm_rmw_reg_bits(mask: u32, bits: u32, inst: i16, idx: i16) -> u32 {
    let mut v = am33xx_prm_read_reg(inst, idx as u16);
    v &= !mask;
    v |= bits;
    am33xx_prm_write_reg(v, inst, idx as u16);
    v
}

unsafe fn am33xx_prm_is_hardreset_asserted(shift: u8, _part: u8, inst: i16, rstctrl_offs: u16) -> i32 {
    let mut v = am33xx_prm_read_reg(inst, rstctrl_offs);
    v &= 1u32 << shift;
    v >>= shift;
    v as i32
}

unsafe fn am33xx_prm_assert_hardreset(shift: u8, _part: u8, inst: i16, rstctrl_offs: u16) -> i32 {
    let mask = 1u32 << shift;
    am33xx_prm_rmw_reg_bits(mask, mask, inst, rstctrl_offs as i16);
    0
}

unsafe fn am33xx_prm_deassert_hardreset(shift: u8, st_shift: u8, _part: u8,
                                         inst: i16, rstctrl_offs: u16, rstst_offs: u16) -> i32 {
    let mut c: i32;
    let mut mask = 1u32 << st_shift;
    if am33xx_prm_is_hardreset_asserted(shift, 0, inst, rstctrl_offs) == 0 { return -EEXIST; }
    am33xx_prm_rmw_reg_bits(0xffff_ffff, mask, inst, rstst_offs as i16);
    mask = 1u32 << shift;
    am33xx_prm_rmw_reg_bits(mask, 0, inst, rstctrl_offs as i16);
    c = 0;
    while am33xx_prm_is_hardreset_asserted(st_shift, 0, inst, rstst_offs) != 0 && c < MAX_MODULE_HARDRESET_WAIT { c += 1; }
    if c == MAX_MODULE_HARDRESET_WAIT { -EBUSY } else { 0 }
}

unsafe fn am33xx_pwrdm_set_next_pwrst(pwrdm: *mut powerdomain, pwrst: u8) -> i32 {
    am33xx_prm_rmw_reg_bits(OMAP_POWERSTATE_MASK, (pwrst as u32) << OMAP_POWERSTATE_SHIFT, (*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs);
    0
}
unsafe fn am33xx_pwrdm_read_next_pwrst(pwrdm: *mut powerdomain) -> i32 { let mut v = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs as u16); v &= OMAP_POWERSTATE_MASK; (v >> OMAP_POWERSTATE_SHIFT) as i32 }
unsafe fn am33xx_pwrdm_read_pwrst(pwrdm: *mut powerdomain) -> i32 { let mut v = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstst_offs as u16); v &= OMAP_POWERSTATEST_MASK; (v >> OMAP_POWERSTATEST_SHIFT) as i32 }
unsafe fn am33xx_pwrdm_set_lowpwrstchange(pwrdm: *mut powerdomain) -> i32 { am33xx_prm_rmw_reg_bits(AM33XX_LOWPOWERSTATECHANGE_MASK, 1 << AM33XX_LOWPOWERSTATECHANGE_SHIFT, (*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); 0 }
unsafe fn am33xx_pwrdm_clear_all_prev_pwrst(pwrdm: *mut powerdomain) -> i32 { am33xx_prm_rmw_reg_bits(AM33XX_LASTPOWERSTATEENTERED_MASK, AM33XX_LASTPOWERSTATEENTERED_MASK, (*pwrdm).prcm_offs, (*pwrdm).pwrstst_offs); 0 }
unsafe fn am33xx_pwrdm_set_logic_retst(pwrdm: *mut powerdomain, pwrst: u8) -> i32 { let m = (*pwrdm).logicretstate_mask; if m == 0 { return -EINVAL; } am33xx_prm_rmw_reg_bits(m, (pwrst as u32) << __ffs(m), (*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); 0 }
unsafe fn am33xx_pwrdm_read_logic_pwrst(pwrdm: *mut powerdomain) -> i32 { let mut v = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstst_offs); v &= AM33XX_LOGICSTATEST_MASK; (v >> AM33XX_LOGICSTATEST_SHIFT) as i32 }
unsafe fn am33xx_pwrdm_read_logic_retst(pwrdm: *mut powerdomain) -> i32 { let m = (*pwrdm).logicretstate_mask; if m == 0 { return -EINVAL; } let mut v = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); v &= m; (v >> __ffs(m)) as i32 }

unsafe fn am33xx_pwrdm_set_mem_onst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> i32 { let m = (*pwrdm).mem_on_mask[bank as usize]; if m == 0 { return -EINVAL; } am33xx_prm_rmw_reg_bits(m, (pwrst as u32) << __ffs(m), (*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); 0 }
unsafe fn am33xx_pwrdm_set_mem_retst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> i32 { let m = (*pwrdm).mem_ret_mask[bank as usize]; if m == 0 { return -EINVAL; } am33xx_prm_rmw_reg_bits(m, (pwrst as u32) << __ffs(m), (*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); 0 }
unsafe fn am33xx_pwrdm_read_mem_pwrst(pwrdm: *mut powerdomain, bank: u8) -> i32 { let m = (*pwrdm).mem_pwrst_mask[bank as usize]; if m == 0 { return -EINVAL; } let mut v = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstst_offs); v &= m; (v >> __ffs(m)) as i32 }
unsafe fn am33xx_pwrdm_read_mem_retst(pwrdm: *mut powerdomain, bank: u8) -> i32 { let m = (*pwrdm).mem_retst_mask[bank as usize]; if m == 0 { return -EINVAL; } let mut v = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); v &= m; (v >> __ffs(m)) as i32 }

unsafe fn am33xx_pwrdm_wait_transition(pwrdm: *mut powerdomain) -> i32 {
    let mut c: u32 = 0;
    while (am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstst_offs) & OMAP_INTRANSITION_MASK) != 0 && { c += 1; c <= PWRDM_TRANSITION_BAILOUT } { udelay(1); }
    if c > PWRDM_TRANSITION_BAILOUT { pr_err!("powerdomain: {}: waited too long to complete transition\n", (*pwrdm).name); return -EAGAIN; }
    pr_debug!("powerdomain: completed transition in {} loops\n", c); 0
}

unsafe fn am33xx_check_vcvp() -> i32 { 0 }
unsafe fn am33xx_prm_global_sw_reset() { let mut mask = AM33XX_RST_GLOBAL_WARM_SW_MASK; if prm_reboot_mode == REBOOT_COLD { mask = AM33XX_RST_GLOBAL_COLD_SW_MASK; } am33xx_prm_rmw_reg_bits(mask, mask, AM33XX_PRM_DEVICE_MOD, AM33XX_PRM_RSTCTRL_OFFSET as i16); let _ = am33xx_prm_read_reg(AM33XX_PRM_DEVICE_MOD, AM33XX_PRM_RSTCTRL_OFFSET); }
unsafe fn am33xx_pwrdm_save_context(pwrdm: *mut powerdomain) { (*pwrdm).context = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); (*pwrdm).context &= !AM33XX_LOWPOWERSTATECHANGE_MASK; }
unsafe fn am33xx_pwrdm_restore_context(pwrdm: *mut powerdomain) { let st = am33xx_prm_read_reg((*pwrdm).prcm_offs, (*pwrdm).pwrstst_offs); am33xx_prm_write_reg((*pwrdm).context, (*pwrdm).prcm_offs, (*pwrdm).pwrstctrl_offs); let st = st & OMAP_POWERSTATEST_MASK; let ctrl = OMAP_POWERSTATEST_MASK & (*pwrdm).context; if st != ctrl { am33xx_pwrdm_wait_transition(pwrdm); } }

static mut am33xx_pwrdm_operations: pwrdm_ops = pwrdm_ops {
    pwrdm_set_next_pwrst: Some(am33xx_pwrdm_set_next_pwrst), pwrdm_read_next_pwrst: Some(am33xx_pwrdm_read_next_pwrst), pwrdm_read_pwrst: Some(am33xx_pwrdm_read_pwrst), pwrdm_set_logic_retst: Some(am33xx_pwrdm_set_logic_retst), pwrdm_read_logic_pwrst: Some(am33xx_pwrdm_read_logic_pwrst), pwrdm_read_logic_retst: Some(am33xx_pwrdm_read_logic_retst), pwrdm_clear_all_prev_pwrst: Some(am33xx_pwrdm_clear_all_prev_pwrst), pwrdm_set_lowpwrstchange: Some(am33xx_pwrdm_set_lowpwrstchange), pwrdm_read_mem_pwrst: Some(am33xx_pwrdm_read_mem_pwrst), pwrdm_read_mem_retst: Some(am33xx_pwrdm_read_mem_retst), pwrdm_set_mem_onst: Some(am33xx_pwrdm_set_mem_onst), pwrdm_set_mem_retst: Some(am33xx_pwrdm_set_mem_retst), pwrdm_wait_transition: Some(am33xx_pwrdm_wait_transition), pwrdm_has_voltdm: Some(am33xx_check_vcvp), pwrdm_save_context: Some(am33xx_pwrdm_save_context), pwrdm_restore_context: Some(am33xx_pwrdm_restore_context),
};
static mut am33xx_prm_ll_data: prm_ll_data = prm_ll_data { assert_hardreset: Some(am33xx_prm_assert_hardreset), deassert_hardreset: Some(am33xx_prm_deassert_hardreset), is_hardreset_asserted: Some(am33xx_prm_is_hardreset_asserted), reset_system: Some(am33xx_prm_global_sw_reset) };
pub unsafe fn am33xx_prm_init(_data: *const omap_prcm_init_data) -> i32 { prm_register(&am33xx_prm_ll_data) }
unsafe fn am33xx_prm_exit() { prm_unregister(&am33xx_prm_ll_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
