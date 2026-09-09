// SPDX-License-Identifier: GPL-2.0-only
/* OMAP4 PRM module functions; dependencies are supplied by the surrounding kernel translation. */

static mut OMAP_PRM_CONTEXT: omap_prm_irq_context = omap_prm_irq_context { irq_enable: 0, pm_ctrl: 0 };

#[repr(C)]
struct omap_prm_irq_context { irq_enable: c_ulong, pm_ctrl: c_ulong }

#[repr(C)]
struct omap4_vp { irqstatus_mpu: u32, tranxdone_status: u32 }

static mut OMAP4_VP: [omap4_vp; 3] = [
    omap4_vp { irqstatus_mpu: OMAP4_PRM_IRQSTATUS_MPU_2_OFFSET, tranxdone_status: OMAP4430_VP_MPU_TRANXDONE_ST_MASK },
    omap4_vp { irqstatus_mpu: OMAP4_PRM_IRQSTATUS_MPU_OFFSET, tranxdone_status: OMAP4430_VP_IVA_TRANXDONE_ST_MASK },
    omap4_vp { irqstatus_mpu: OMAP4_PRM_IRQSTATUS_MPU_OFFSET, tranxdone_status: OMAP4430_VP_CORE_TRANXDONE_ST_MASK },
];

unsafe fn omap4_prm_read_inst_reg(inst: i16, reg: u16) -> u32 { readl_relaxed(prm_base.va.add((inst as isize + reg as isize) as usize)) }
unsafe fn omap4_prm_write_inst_reg(val: u32, inst: i16, reg: u16) { writel_relaxed(val, prm_base.va.add((inst as isize + reg as isize) as usize)); }
unsafe fn omap4_prm_rmw_inst_reg_bits(mask: u32, bits: u32, inst: i16, reg: i16) -> u32 {
    let mut v = omap4_prm_read_inst_reg(inst, reg as u16); v &= !mask; v |= bits; omap4_prm_write_inst_reg(v, inst, reg as u16); v
}

unsafe fn omap4_prm_vp_check_txdone(vp_id: u8) -> u32 {
    let vp = &OMAP4_VP[vp_id as usize];
    omap4_prminst_read_inst_reg(OMAP4430_PRM_PARTITION, OMAP4430_PRM_OCP_SOCKET_INST, vp.irqstatus_mpu) & vp.tranxdone_status
}
unsafe fn omap4_prm_vp_clear_txdone(vp_id: u8) { let vp = &OMAP4_VP[vp_id as usize]; omap4_prminst_write_inst_reg(vp.tranxdone_status, OMAP4430_PRM_PARTITION, OMAP4430_PRM_OCP_SOCKET_INST, vp.irqstatus_mpu); }

pub unsafe fn omap4_prm_vcvp_read(offset: u8) -> u32 { let inst = omap4_prmst_get_prm_dev_inst(); if inst == PRM_INSTANCE_UNKNOWN { 0 } else { omap4_prminst_read_inst_reg(OMAP4430_PRM_PARTITION, inst, offset as u16) } }
pub unsafe fn omap4_prm_vcvp_write(val: u32, offset: u8) { let inst = omap4_prmst_get_prm_dev_inst(); if inst != PRM_INSTANCE_UNKNOWN { omap4_prminst_write_inst_reg(val, OMAP4430_PRM_PARTITION, inst, offset as u16); } }
pub unsafe fn omap4_prm_vcvp_rmw(mask: u32, bits: u32, offset: u8) -> u32 { let inst = omap4_prmst_get_prm_dev_inst(); if inst == PRM_INSTANCE_UNKNOWN { 0 } else { omap4_prminst_rmw_inst_reg_bits(mask, bits, OMAP4430_PRM_PARTITION, inst, offset as u16) } }

unsafe fn read_pending_irq_reg(en: u16, st: u16) -> u32 { omap4_prm_read_inst_reg(OMAP4430_PRM_OCP_SOCKET_INST, en) & omap4_prm_read_inst_reg(OMAP4430_PRM_OCP_SOCKET_INST, st) }
unsafe fn omap44xx_prm_read_pending_irqs(events: *mut c_ulong) { for i in 0..omap4_prcm_irq_setup.nr_regs { *events.add(i as usize) = read_pending_irq_reg(omap4_prcm_irq_setup.mask + i * 4, omap4_prcm_irq_setup.ack + i * 4) as c_ulong; } }
unsafe fn omap44xx_prm_ocp_barrier() { omap4_prm_read_inst_reg(OMAP4430_PRM_OCP_SOCKET_INST, OMAP4_REVISION_PRM_OFFSET); }
unsafe fn omap44xx_prm_save_and_clear_irqen(saved: *mut u32) { for i in 0..omap4_prcm_irq_setup.nr_regs { let r = omap4_prcm_irq_setup.mask + i * 4; *saved.add(i as usize) = omap4_prm_read_inst_reg(OMAP4430_PRM_OCP_SOCKET_INST, r); omap4_prm_write_inst_reg(0, OMAP4430_PRM_OCP_SOCKET_INST, r); } omap44xx_prm_ocp_barrier(); }
unsafe fn omap44xx_prm_restore_irqen(saved: *mut u32) { for i in 0..omap4_prcm_irq_setup.nr_regs { omap4_prm_write_inst_reg(*saved.add(i as usize), OMAP4430_PRM_OCP_SOCKET_INST, omap4_prcm_irq_setup.mask + i * 4); } }

unsafe fn omap44xx_prm_reconfigure_io_chain() {
    let mut i = 0; let inst = omap4_prmst_get_prm_dev_inst(); if inst == PRM_INSTANCE_UNKNOWN { return; }
    omap4_prm_rmw_inst_reg_bits(OMAP4430_WUCLK_CTRL_MASK, OMAP4430_WUCLK_CTRL_MASK, inst, omap4_prcm_irq_setup.pm_ctrl as i16);
    omap_test_timeout!(((omap4_prm_read_inst_reg(inst, omap4_prcm_irq_setup.pm_ctrl) & OMAP4430_WUCLK_STATUS_MASK) >> OMAP4430_WUCLK_STATUS_SHIFT) == 1, MAX_IOPAD_LATCH_TIME, i);
    if i == MAX_IOPAD_LATCH_TIME { pr_warn!("PRM: I/O chain clock line assertion timed out\n"); }
    omap4_prm_rmw_inst_reg_bits(OMAP4430_WUCLK_CTRL_MASK, 0, inst, omap4_prcm_irq_setup.pm_ctrl as i16);
    omap_test_timeout!(((omap4_prm_read_inst_reg(inst, omap4_prcm_irq_setup.pm_ctrl) & OMAP4430_WUCLK_STATUS_MASK) >> OMAP4430_WUCLK_STATUS_SHIFT) == 0, MAX_IOPAD_LATCH_TIME, i);
    if i == MAX_IOPAD_LATCH_TIME { pr_warn!("PRM: I/O chain clock line deassertion timed out\n"); }
}
unsafe fn omap44xx_prm_enable_io_wakeup() { let inst = omap4_prmst_get_prm_dev_inst(); if inst != PRM_INSTANCE_UNKNOWN { omap4_prm_rmw_inst_reg_bits(OMAP4430_GLOBAL_WUEN_MASK, OMAP4430_GLOBAL_WUEN_MASK, inst, omap4_prcm_irq_setup.pm_ctrl as i16); } }

// The remaining power-domain and registration interfaces retain the original callback shape.
pub unsafe fn omap44xx_prm_read_reset_sources() -> u32 { let inst = omap4_prmst_get_prm_dev_inst(); if inst == PRM_INSTANCE_UNKNOWN { return 0; } let v = omap4_prm_read_inst_reg(inst, OMAP4_RM_RSTST); let mut r = 0; for &(a,b) in &[(OMAP4430_GLOBAL_WARM_SW_RST_SHIFT, OMAP_GLOBAL_WARM_RST_SRC_ID_SHIFT),(OMAP4430_GLOBAL_COLD_RST_SHIFT, OMAP_GLOBAL_COLD_RST_SRC_ID_SHIFT),(OMAP4430_MPU_SECURITY_VIOL_RST_SHIFT, OMAP_SECU_VIOL_RST_SRC_ID_SHIFT),(OMAP4430_MPU_WDT_RST_SHIFT, OMAP_MPU_WD_RST_SRC_ID_SHIFT),(OMAP4430_SECURE_WDT_RST_SHIFT, OMAP_SECU_WD_RST_SRC_ID_SHIFT),(OMAP4430_EXTERNAL_WARM_RST_SHIFT, OMAP_EXTWARM_RST_SRC_ID_SHIFT),(OMAP4430_VDD_MPU_VOLT_MGR_RST_SHIFT, OMAP_VDD_MPU_VM_RST_SRC_ID_SHIFT),(OMAP4430_VDD_IVA_VOLT_MGR_RST_SHIFT, OMAP_VDD_IVA_VM_RST_SRC_ID_SHIFT),(OMAP4430_VDD_CORE_VOLT_MGR_RST_SHIFT, OMAP_VDD_CORE_VM_RST_SRC_ID_SHIFT),(OMAP4430_ICEPICK_RST_SHIFT, OMAP_ICEPICK_RST_SRC_ID_SHIFT),(OMAP4430_C2C_RST_SHIFT, OMAP_C2C_RST_SRC_ID_SHIFT)] { if v & (1u32 << a) != 0 { r |= 1u32 << b; } } r }

pub unsafe fn omap44xx_prm_was_any_context_lost_old(part: u8, inst: i16, idx: u16) -> bool { omap4_prminst_read_inst_reg(part, inst, idx) != 0 }
pub unsafe fn omap44xx_prm_clear_context_loss_flags_old(part: u8, inst: i16, idx: u16) { omap4_prminst_write_inst_reg(0xffff_ffff, part, inst, idx); }

unsafe fn omap4_pwrdm_set_next_pwrst(p: *mut powerdomain, s: u8) -> i32 { omap4_prminst_rmw_inst_reg_bits(OMAP_POWERSTATE_MASK, (s as u32) << OMAP_POWERSTATE_SHIFT, (*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTCTRL); 0 }
unsafe fn omap4_pwrdm_read_next_pwrst(p: *mut powerdomain) -> i32 { (omap4_prminst_read_inst_reg((*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTCTRL) & OMAP_POWERSTATE_MASK >> OMAP_POWERSTATE_SHIFT) as i32 }
unsafe fn omap4_pwrdm_read_pwrst(p: *mut powerdomain) -> i32 { (omap4_prminst_read_inst_reg((*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTST) & OMAP_POWERSTATEST_MASK >> OMAP_POWERSTATEST_SHIFT) as i32 }
unsafe fn omap4_pwrdm_read_prev_pwrst(p: *mut powerdomain) -> i32 { (omap4_prminst_read_inst_reg((*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTST) & OMAP4430_LASTPOWERSTATEENTERED_MASK >> OMAP4430_LASTPOWERSTATEENTERED_SHIFT) as i32 }
unsafe fn omap4_pwrdm_set_lowpwrstchange(p: *mut powerdomain) -> i32 { omap4_prminst_rmw_inst_reg_bits(OMAP4430_LOWPOWERSTATECHANGE_MASK, 1 << OMAP4430_LOWPOWERSTATECHANGE_SHIFT, (*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTCTRL); 0 }
unsafe fn omap4_pwrdm_clear_all_prev_pwrst(p: *mut powerdomain) -> i32 { omap4_prminst_rmw_inst_reg_bits(OMAP4430_LASTPOWERSTATEENTERED_MASK, OMAP4430_LASTPOWERSTATEENTERED_MASK, (*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTST); 0 }
unsafe fn omap4_pwrdm_read_logic_pwrst(p: *mut powerdomain) -> i32 { (omap4_prminst_read_inst_reg((*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTST) & OMAP4430_LOGICSTATEST_MASK >> OMAP4430_LOGICSTATEST_SHIFT) as i32 }
unsafe fn omap4_pwrdm_read_logic_retst(p: *mut powerdomain) -> i32 { (omap4_prminst_read_inst_reg((*p).prcm_partition, (*p).prcm_offs, OMAP4_PM_PWSTCTRL) & OMAP4430_LOGICRETSTATE_MASK >> OMAP4430_LOGICRETSTATE_SHIFT) as i32 }
unsafe fn omap4_pwrdm_read_prev_logic_pwrst(p: *mut powerdomain) -> i32 { let s=omap4_pwrdm_read_prev_pwrst(p); if s==PWRDM_POWER_OFF {PWRDM_POWER_OFF} else if s!=PWRDM_POWER_RET {PWRDM_POWER_RET} else {omap4_pwrdm_read_logic_retst(p)} }
unsafe fn omap4_check_vcvp() -> i32 { if prm_features & PRM_HAS_VOLTAGE != 0 {1} else {0} }

// Remaining declarations mirror the source's powerdomain callback table and platform registration hooks.
extern "C" { pub fn prm_register(d: *mut prm_ll_data) -> i32; pub fn prm_unregister(d: *mut prm_ll_data); }

unsafe fn prm_save_context() { OMAP_PRM_CONTEXT.irq_enable=omap4_prm_read_inst_reg(AM43XX_PRM_OCP_SOCKET_INST,omap4_prcm_irq_setup.mask) as c_ulong; OMAP_PRM_CONTEXT.pm_ctrl=omap4_prm_read_inst_reg(AM43XX_PRM_DEVICE_INST,omap4_prcm_irq_setup.pm_ctrl) as c_ulong; }
unsafe fn prm_restore_context() { omap4_prm_write_inst_reg(OMAP_PRM_CONTEXT.irq_enable as u32,OMAP4430_PRM_OCP_SOCKET_INST,omap4_prcm_irq_setup.mask); omap4_prm_write_inst_reg(OMAP_PRM_CONTEXT.pm_ctrl as u32,AM43XX_PRM_DEVICE_INST,omap4_prcm_irq_setup.pm_ctrl); }
unsafe fn cpu_notifier(_nb:*mut notifier_block,cmd:c_ulong,_v:*mut c_void)->i32 { match cmd { CPU_CLUSTER_PM_ENTER=>if enable_off_mode {prm_save_context()}, CPU_CLUSTER_PM_EXIT=>if enable_off_mode {prm_restore_context()}, _=>{} } NOTIFY_OK }

static mut prm_init_data: *const omap_prcm_init_data = core::ptr::null();
pub unsafe fn omap44xx_prm_init(data:*const omap_prcm_init_data)->i32 { static mut nb:notifier_block=notifier_block{notifier_call:None}; omap_prm_base_init(); prm_init_data=data; if (*data).flags&PRM_HAS_IO_WAKEUP!=0 {prm_features|=PRM_HAS_IO_WAKEUP;} if (*data).flags&PRM_HAS_VOLTAGE!=0 {prm_features|=PRM_HAS_VOLTAGE;} omap4_prminst_set_prm_dev_inst((*data).device_inst_offset); if of_device_is_compatible((*data).np,"ti,am4-prcm") {omap4_prcm_irq_setup.nr_irqs=1;omap4_prcm_irq_setup.nr_regs=1;omap4_prcm_irq_setup.pm_ctrl=AM43XX_PRM_IO_PMCTRL_OFFSET;omap4_prcm_irq_setup.ack=AM43XX_PRM_IRQSTATUS_MPU_OFFSET;omap4_prcm_irq_setup.mask=AM43XX_PRM_IRQENABLE_MPU_OFFSET;} if soc_is_am43xx() {nb.notifier_call=Some(cpu_notifier);cpu_pm_register_notifier(&mut nb);} prm_register(core::ptr::null_mut()) }
unsafe fn omap44xx_prm_late_init()->i32 { if prm_features&PRM_HAS_IO_WAKEUP==0{return 0;} let irq=of_irq_get((*prm_init_data).np,0); if irq==-EPROBE_DEFER{return irq;} omap4_prcm_irq_setup.irq=irq;omap44xx_prm_enable_io_wakeup();omap_prcm_register_chain_handler(&mut omap4_prcm_irq_setup) }
pub unsafe fn omap44xx_prm_exit(){prm_unregister(core::ptr::null_mut());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
