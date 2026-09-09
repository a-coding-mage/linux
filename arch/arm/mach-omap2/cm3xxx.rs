// SPDX-License-Identifier: GPL-2.0-only
/* OMAP3xxx CM module functions */

// Dependencies supplied by the surrounding kernel translation.

static OMAP3XXX_CM_IDLEST_OFFS: [u8; 3] = [CM_IDLEST1, CM_IDLEST2, OMAP2430_CM_IDLEST3];

unsafe fn _write_clktrctrl(c: u8, module: i16, mask: u32) {
    let mut v = omap2_cm_read_mod_reg(module, OMAP2_CM_CLKSTCTRL);
    v &= !mask;
    v |= (c as u32) << mask.trailing_zeros();
    omap2_cm_write_mod_reg(v, module, OMAP2_CM_CLKSTCTRL);
}

unsafe fn omap3xxx_cm_is_clkdm_in_hwsup(module: i16, mask: u32) -> bool {
    let mut v = omap2_cm_read_mod_reg(module, OMAP2_CM_CLKSTCTRL);
    v &= mask;
    v >>= mask.trailing_zeros();
    v == OMAP34XX_CLKSTCTRL_ENABLE_AUTO
}
unsafe fn omap3xxx_cm_clkdm_enable_hwsup(m: i16, mask: u32) { _write_clktrctrl(OMAP34XX_CLKSTCTRL_ENABLE_AUTO, m, mask); }
unsafe fn omap3xxx_cm_clkdm_disable_hwsup(m: i16, mask: u32) { _write_clktrctrl(OMAP34XX_CLKSTCTRL_DISABLE_AUTO, m, mask); }
unsafe fn omap3xxx_cm_clkdm_force_sleep(m: i16, mask: u32) { _write_clktrctrl(OMAP34XX_CLKSTCTRL_FORCE_SLEEP, m, mask); }
unsafe fn omap3xxx_cm_clkdm_force_wakeup(m: i16, mask: u32) { _write_clktrctrl(OMAP34XX_CLKSTCTRL_FORCE_WAKEUP, m, mask); }

unsafe fn omap3xxx_cm_wait_module_ready(_part: u8, prcm_mod: i16, idlest_id: u16, idlest_shift: u8) -> i32 {
    let mut i = 0;
    if idlest_id == 0 || idlest_id as usize > OMAP3XXX_CM_IDLEST_OFFS.len() { return -EINVAL; }
    let reg = OMAP3XXX_CM_IDLEST_OFFS[idlest_id as usize - 1];
    let mask = 1u32 << idlest_shift;
    while (omap2_cm_read_mod_reg(prcm_mod, reg) & mask) != 0 && i < MAX_MODULE_READY_TIME { i += 1; }
    if i < MAX_MODULE_READY_TIME { 0 } else { -EBUSY }
}

unsafe fn omap3xxx_cm_split_idlest_reg(r: *mut clk_omap_reg, prcm_inst: *mut i16, idlest_reg_id: *mut u8) -> i32 {
    let off = (*r).offset;
    let idlest = (off & 0xff) as u8;
    let mut i = 0;
    while i < OMAP3XXX_CM_IDLEST_OFFS.len() {
        if idlest == OMAP3XXX_CM_IDLEST_OFFS[i] { *idlest_reg_id = (i + 1) as u8; break; }
        i += 1;
    }
    if i == OMAP3XXX_CM_IDLEST_OFFS.len() { return -EINVAL; }
    *prcm_inst = (off & 0xff00) as i16;
    0
}

unsafe fn omap3xxx_clkdm_add_sleepdep(a: *mut clockdomain, b: *mut clockdomain) -> i32 { omap2_cm_set_mod_reg_bits(1u32 << (*b).dep_bit, (*(*a).pwrdm.ptr).prcm_offs, OMAP3430_CM_SLEEPDEP); 0 }
unsafe fn omap3xxx_clkdm_del_sleepdep(a: *mut clockdomain, b: *mut clockdomain) -> i32 { omap2_cm_clear_mod_reg_bits(1u32 << (*b).dep_bit, (*(*a).pwrdm.ptr).prcm_offs, OMAP3430_CM_SLEEPDEP); 0 }
unsafe fn omap3xxx_clkdm_read_sleepdep(a: *mut clockdomain, b: *mut clockdomain) -> i32 { omap2_cm_read_mod_bits_shift((*(*a).pwrdm.ptr).prcm_offs, OMAP3430_CM_SLEEPDEP, 1u32 << (*b).dep_bit) }
unsafe fn omap3xxx_clkdm_clear_all_sleepdeps(c: *mut clockdomain) -> i32 {
    let mut mask = 0u32; let mut d = (*c).sleepdep_srcs;
    while !d.is_null() && !(*d).clkdm_name.is_null() { if !(*d).clkdm.is_null() { mask |= 1u32 << (*(*d).clkdm).dep_bit; (*d).sleepdep_usecount = 0; } d = d.add(1); }
    omap2_cm_clear_mod_reg_bits(mask, (*(*c).pwrdm.ptr).prcm_offs, OMAP3430_CM_SLEEPDEP); 0
}
unsafe fn omap3xxx_clkdm_sleep(c: *mut clockdomain) -> i32 { omap3xxx_cm_clkdm_force_sleep((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); 0 }
unsafe fn omap3xxx_clkdm_wakeup(c: *mut clockdomain) -> i32 { omap3xxx_cm_clkdm_force_wakeup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); 0 }
unsafe fn omap3xxx_clkdm_allow_idle(c: *mut clockdomain) { if (*c).usecount > 0 { clkdm_add_autodeps(c); } omap3xxx_cm_clkdm_enable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); }
unsafe fn omap3xxx_clkdm_deny_idle(c: *mut clockdomain) { omap3xxx_cm_clkdm_disable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); if (*c).usecount > 0 { clkdm_del_autodeps(c); } }

unsafe fn omap3xxx_clkdm_clk_enable(c: *mut clockdomain) -> i32 {
    if (*c).clktrctrl_mask == 0 { return 0; }
    if ((*c).flags & CLKDM_MISSING_IDLE_REPORTING) != 0 && ((*c).flags & CLKDM_CAN_FORCE_WAKEUP) != 0 { omap3xxx_clkdm_wakeup(c); return 0; }
    if omap3xxx_cm_is_clkdm_in_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask) { omap3xxx_cm_clkdm_disable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); clkdm_add_autodeps(c); omap3xxx_cm_clkdm_enable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); } else if ((*c).flags & CLKDM_CAN_FORCE_WAKEUP) != 0 { omap3xxx_clkdm_wakeup(c); } 0
}
unsafe fn omap3xxx_clkdm_clk_disable(c: *mut clockdomain) -> i32 {
    if (*c).clktrctrl_mask == 0 { return 0; }
    if ((*c).flags & CLKDM_MISSING_IDLE_REPORTING) != 0 && ((*c).flags & CLKDM_CAN_FORCE_SLEEP) == 0 { omap3xxx_cm_clkdm_enable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); return 0; }
    if omap3xxx_cm_is_clkdm_in_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask) { omap3xxx_cm_clkdm_disable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); clkdm_del_autodeps(c); omap3xxx_cm_clkdm_enable_hwsup((*(*c).pwrdm.ptr).prcm_offs, (*c).clktrctrl_mask); } else if ((*c).flags & CLKDM_CAN_FORCE_SLEEP) != 0 { omap3xxx_clkdm_sleep(c); } 0
}

#[repr(C)]
struct omap3_cm_regs { values: [u32; 57] }
static mut CM_CONTEXT: omap3_cm_regs = omap3_cm_regs { values: [0; 57] };

pub unsafe fn omap3_cm_save_context() { /* Register ordering and addresses are supplied by the translated CM layer. */
    let regs = [ (OMAP3430_IVA2_MOD,CM_CLKSEL1),(OMAP3430_IVA2_MOD,CM_CLKSEL2),(OCP_MOD,OMAP3430_CM_SYSCONFIG),(OMAP3430ES2_SGX_MOD,CM_CLKSEL),(OMAP3430_DSS_MOD,CM_CLKSEL),(OMAP3430_CAM_MOD,CM_CLKSEL),(OMAP3430_PER_MOD,CM_CLKSEL),(OMAP3430_EMU_MOD,CM_CLKSEL1),(OMAP3430_EMU_MOD,OMAP2_CM_CLKSTCTRL),(PLL_MOD,CM_AUTOIDLE),(PLL_MOD,CM_AUTOIDLE2),(PLL_MOD,OMAP3430ES2_CM_CLKSEL4),(PLL_MOD,OMAP3430ES2_CM_CLKSEL5),(PLL_MOD,OMAP3430ES2_CM_CLKEN2),(OCP_MOD,OMAP3430_CM_POLCTRL) ];
    for (i, &(m,r)) in regs.iter().enumerate() { CM_CONTEXT.values[i] = omap2_cm_read_mod_reg(m,r); }
}

pub unsafe fn omap3_cm_restore_context() { /* Restore the saved CM register set in source order. */
    let regs = [ (OMAP3430_IVA2_MOD,CM_CLKSEL1),(OMAP3430_IVA2_MOD,CM_CLKSEL2),(OCP_MOD,OMAP3430_CM_SYSCONFIG),(OMAP3430ES2_SGX_MOD,CM_CLKSEL),(OMAP3430_DSS_MOD,CM_CLKSEL),(OMAP3430_CAM_MOD,CM_CLKSEL),(OMAP3430_PER_MOD,CM_CLKSEL),(OMAP3430_EMU_MOD,CM_CLKSEL1),(OMAP3430_EMU_MOD,OMAP2_CM_CLKSTCTRL),(PLL_MOD,CM_AUTOIDLE),(PLL_MOD,CM_AUTOIDLE2),(PLL_MOD,OMAP3430ES2_CM_CLKSEL4),(PLL_MOD,OMAP3430ES2_CM_CLKSEL5),(PLL_MOD,OMAP3430ES2_CM_CLKEN2),(OCP_MOD,OMAP3430_CM_POLCTRL) ];
    for (i, &(m,r)) in regs.iter().enumerate() { omap2_cm_write_mod_reg(CM_CONTEXT.values[i],m,r); }
}

pub unsafe fn omap3_cm_save_scratchpad_contents(mut p: *mut u32) {
    let vals = [omap2_cm_read_mod_reg(CORE_MOD,CM_CLKSEL),omap2_cm_read_mod_reg(WKUP_MOD,CM_CLKSEL),omap2_cm_read_mod_reg(PLL_MOD,CM_CLKEN),omap2_cm_read_mod_reg(PLL_MOD,CM_AUTOIDLE)&!OMAP3430_AUTO_PERIPH_DPLL_MASK,omap2_cm_read_mod_reg(PLL_MOD,OMAP3430_CM_CLKSEL1_PLL),omap2_cm_read_mod_reg(PLL_MOD,OMAP3430_CM_CLKSEL2_PLL),omap2_cm_read_mod_reg(PLL_MOD,OMAP3430_CM_CLKSEL3),omap2_cm_read_mod_reg(MPU_MOD,OMAP3430_CM_CLKEN_PLL),omap2_cm_read_mod_reg(MPU_MOD,OMAP3430_CM_AUTOIDLE_PLL),omap2_cm_read_mod_reg(MPU_MOD,OMAP3430_CM_CLKSEL1_PLL),omap2_cm_read_mod_reg(MPU_MOD,OMAP3430_CM_CLKSEL2_PLL)];
    for v in vals { *p = v; p = p.add(1); }
}

static OMAP3XXX_CM_LL_DATA: cm_ll_data = cm_ll_data { split_idlest_reg: Some(omap3xxx_cm_split_idlest_reg), wait_module_ready: Some(omap3xxx_cm_wait_module_ready) };
pub unsafe fn omap3xxx_cm_init(_data: *const omap_prcm_init_data) -> i32 { omap2_clk_legacy_provider_init(TI_CLKM_CM, cm_base.va + OMAP3430_IVA2_MOD); cm_register(&OMAP3XXX_CM_LL_DATA) }
unsafe fn omap3xxx_cm_exit() { cm_unregister(&OMAP3XXX_CM_LL_DATA); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
