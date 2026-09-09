// SPDX-License-Identifier: GPL-2.0-only
/* OMAP4 CM instance functions. */

// C headers and build-time configuration are supplied by the surrounding kernel translation.

const OMAP4430_IDLEST_SHIFT: u32 = 16;
const OMAP4430_IDLEST_MASK: u32 = 0x3 << 16;
const OMAP4430_CLKTRCTRL_SHIFT: u32 = 0;
const OMAP4430_CLKTRCTRL_MASK: u32 = 0x3;
const OMAP4430_MODULEMODE_SHIFT: u32 = 0;
const OMAP4430_MODULEMODE_MASK: u32 = 0x3;

const CLKCTRL_IDLEST_FUNCTIONAL: u32 = 0x0;
const CLKCTRL_IDLEST_INTRANSITION: u32 = 0x1;
const CLKCTRL_IDLEST_INTERFACE_IDLE: u32 = 0x2;
const CLKCTRL_IDLEST_DISABLED: u32 = 0x3;

static mut _cm_bases: [omap_domain_base; OMAP4_MAX_PRCM_PARTITIONS as usize] =
    [omap_domain_base { va: core::ptr::null_mut(), pa: 0 }; OMAP4_MAX_PRCM_PARTITIONS as usize];

unsafe fn omap_cm_base_init() {
    core::ptr::copy_nonoverlapping(&prm_base, &mut _cm_bases[OMAP4430_PRM_PARTITION as usize], 1);
    core::ptr::copy_nonoverlapping(&cm_base, &mut _cm_bases[OMAP4430_CM1_PARTITION as usize], 1);
    core::ptr::copy_nonoverlapping(&cm2_base, &mut _cm_bases[OMAP4430_CM2_PARTITION as usize], 1);
    core::ptr::copy_nonoverlapping(&prcm_mpu_base, &mut _cm_bases[OMAP4430_PRCM_MPU_PARTITION as usize], 1);
}

unsafe fn omap4_cminst_read_inst_reg(part: u8, inst: u16, idx: u16) -> u32 {
    debug_assert!((part as u32) < OMAP4_MAX_PRCM_PARTITIONS &&
                  part as u32 != OMAP4430_INVALID_PRCM_PARTITION && !_cm_bases[part as usize].va.is_null());
    core::ptr::read_volatile(_cm_bases[part as usize].va.add(inst as usize + idx as usize) as *const u32)
}

unsafe fn omap4_cminst_write_inst_reg(val: u32, part: u8, inst: u16, idx: u16) {
    debug_assert!((part as u32) < OMAP4_MAX_PRCM_PARTITIONS &&
                  part as u32 != OMAP4430_INVALID_PRCM_PARTITION && !_cm_bases[part as usize].va.is_null());
    core::ptr::write_volatile(_cm_bases[part as usize].va.add(inst as usize + idx as usize) as *mut u32, val);
}

unsafe fn _clkctrl_idlest(part: u8, inst: u16, off: u16) -> u32 {
    (omap4_cminst_read_inst_reg(part, inst, off) & OMAP4430_IDLEST_MASK) >> OMAP4430_IDLEST_SHIFT
}
unsafe fn _is_module_ready(part: u8, inst: u16, off: u16) -> bool {
    let v = _clkctrl_idlest(part, inst, off);
    v == CLKCTRL_IDLEST_FUNCTIONAL || v == CLKCTRL_IDLEST_INTERFACE_IDLE
}
unsafe fn omap4_cminst_rmw_inst_reg_bits(mask: u32, bits: u32, part: u8, inst: u16, idx: i16) -> u32 {
    let mut v = omap4_cminst_read_inst_reg(part, inst, idx as u16);
    v &= !mask; v |= bits;
    omap4_cminst_write_inst_reg(v, part, inst, idx as u16); v
}
unsafe fn omap4_cminst_set_inst_reg_bits(bits: u32, part: u8, inst: u16, idx: i16) -> u32 { omap4_cminst_rmw_inst_reg_bits(bits, bits, part, inst, idx) }
unsafe fn omap4_cminst_clear_inst_reg_bits(bits: u32, part: u8, inst: u16, idx: i16) -> u32 { omap4_cminst_rmw_inst_reg_bits(bits, 0, part, inst, idx) }
unsafe fn omap4_cminst_read_inst_reg_bits(part: u8, inst: u16, idx: i16, mask: u32) -> u32 {
    let v = omap4_cminst_read_inst_reg(part, inst, idx as u16) & mask;
    v >> mask.trailing_zeros()
}

unsafe fn _clktrctrl_write(c: u8, part: u8, inst: u16, cdoffs: u16) {
    let idx = cdoffs + OMAP4_CM_CLKSTCTRL;
    let mut v = omap4_cminst_read_inst_reg(part, inst, idx);
    v &= !OMAP4430_CLKTRCTRL_MASK; v |= (c as u32) << OMAP4430_CLKTRCTRL_SHIFT;
    omap4_cminst_write_inst_reg(v, part, inst, idx);
}
unsafe fn omap4_cminst_is_clkdm_in_hwsup(part: u8, inst: u16, cdoffs: u16) -> bool {
    (omap4_cminst_read_inst_reg(part, inst, cdoffs + OMAP4_CM_CLKSTCTRL) & OMAP4430_CLKTRCTRL_MASK) >> OMAP4430_CLKTRCTRL_SHIFT == OMAP34XX_CLKSTCTRL_ENABLE_AUTO
}
unsafe fn omap4_cminst_clkdm_enable_hwsup(p:u8,i:u16,c:u16){_clktrctrl_write(OMAP34XX_CLKSTCTRL_ENABLE_AUTO,p,i,c)}
unsafe fn omap4_cminst_clkdm_disable_hwsup(p:u8,i:u16,c:u16){_clktrctrl_write(OMAP34XX_CLKSTCTRL_DISABLE_AUTO,p,i,c)}
unsafe fn omap4_cminst_clkdm_force_wakeup(p:u8,i:u16,c:u16){_clktrctrl_write(OMAP34XX_CLKSTCTRL_FORCE_WAKEUP,p,i,c)}
unsafe fn omap4_cminst_clkdm_force_sleep(p:u8,i:u16,c:u16){_clktrctrl_write(OMAP34XX_CLKSTCTRL_FORCE_SLEEP,p,i,c)}

unsafe fn omap4_cminst_wait_module_ready(part:u8, inst:i16, off:u16, _shift:u8)->i32 {
    let mut i=0; while i < MAX_MODULE_READY_TIME && !_is_module_ready(part,inst as u16,off) { i+=1; } if i < MAX_MODULE_READY_TIME {0} else {-EBUSY}
}
unsafe fn omap4_cminst_wait_module_idle(part:u8, inst:i16, off:u16, _shift:u8)->i32 {
    let mut i=0; while i < MAX_MODULE_DISABLE_TIME && _clkctrl_idlest(part,inst as u16,off)!=CLKCTRL_IDLEST_DISABLED { i+=1; } if i < MAX_MODULE_DISABLE_TIME {0} else {-EBUSY}
}
unsafe fn omap4_cminst_module_enable(mode:u8,part:u8,inst:u16,off:u16){let mut v=omap4_cminst_read_inst_reg(part,inst,off);v&=!OMAP4430_MODULEMODE_MASK;v|=(mode as u32)<<OMAP4430_MODULEMODE_SHIFT;omap4_cminst_write_inst_reg(v,part,inst,off)}
unsafe fn omap4_cminst_module_disable(part:u8,inst:u16,off:u16){let mut v=omap4_cminst_read_inst_reg(part,inst,off);v&=!OMAP4430_MODULEMODE_MASK;omap4_cminst_write_inst_reg(v,part,inst,off)}

unsafe fn omap4_clkdm_add_wkup_sleep_dep(a:*mut clockdomain,b:*mut clockdomain)->i32 { omap4_cminst_set_inst_reg_bits(1u32<<(*b).dep_bit,(*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs+OMAP4_CM_STATICDEP as i16); 0 }
unsafe fn omap4_clkdm_del_wkup_sleep_dep(a:*mut clockdomain,b:*mut clockdomain)->i32 { omap4_cminst_clear_inst_reg_bits(1u32<<(*b).dep_bit,(*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs+OMAP4_CM_STATICDEP as i16); 0 }
unsafe fn omap4_clkdm_read_wkup_sleep_dep(a:*mut clockdomain,b:*mut clockdomain)->i32 { omap4_cminst_read_inst_reg_bits((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs+OMAP4_CM_STATICDEP as i16,1u32<<(*b).dep_bit) as i32 }
unsafe fn omap4_clkdm_clear_all_wkup_sleep_deps(a:*mut clockdomain)->i32 { if (*a).prcm_partition==0{return 0}; let mut mask=0; let mut cd=(*a).wkdep_srcs; while !cd.is_null() && !(*cd).clkdm_name.is_null(){if !(*cd).clkdm.is_null(){mask|=1u32<<(*(*cd).clkdm).dep_bit;(*cd).wkdep_usecount=0;}cd=cd.add(1);} omap4_cminst_clear_inst_reg_bits(mask,(*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs+OMAP4_CM_STATICDEP as i16);0 }
unsafe fn omap4_clkdm_sleep(a:*mut clockdomain)->i32 { if (*a).flags&CLKDM_CAN_HWSUP!=0 {omap4_cminst_clkdm_enable_hwsup((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs)} else if (*a).flags&CLKDM_CAN_FORCE_SLEEP!=0 {omap4_cminst_clkdm_force_sleep((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs)} else{return -EINVAL};0 }
unsafe fn omap4_clkdm_wakeup(a:*mut clockdomain)->i32{omap4_cminst_clkdm_force_wakeup((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs);0}
unsafe fn omap4_clkdm_allow_idle(a:*mut clockdomain){omap4_cminst_clkdm_enable_hwsup((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs)}
unsafe fn omap4_clkdm_deny_idle(a:*mut clockdomain){if (*a).flags&CLKDM_CAN_FORCE_WAKEUP!=0{omap4_clkdm_wakeup(a);}else{omap4_cminst_clkdm_disable_hwsup((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs)}}
unsafe fn omap4_clkdm_clk_enable(a:*mut clockdomain)->i32{if (*a).flags&CLKDM_CAN_FORCE_WAKEUP!=0{omap4_clkdm_wakeup(a)}else{0}}
unsafe fn omap4_clkdm_clk_disable(a:*mut clockdomain)->i32{if (*a).prcm_partition==0{return 0} if (*a).flags&CLKDM_MISSING_IDLE_REPORTING!=0&&(*a).flags&CLKDM_CAN_FORCE_SLEEP==0{omap4_clkdm_allow_idle(a);return 0} if !omap4_cminst_is_clkdm_in_hwsup((*a).prcm_partition,(*a).cm_inst,(*a).clkdm_offs)&&(*a).flags&CLKDM_CAN_FORCE_SLEEP!=0{omap4_clkdm_sleep(a);}0}

unsafe fn omap4_cminst_xlate_clkctrl(part:u8,inst:u16,offset:u16)->u32{_cm_bases[part as usize].pa.wrapping_add(inst as u32).wrapping_add(offset as u32)}

// Clockdomain operations are direct translations; dependent clockdomain types and operation tables are supplied externally.
unsafe fn omap4_clkdm_save_context(clkdm: *mut clockdomain)->i32 { (*clkdm).context=omap4_cminst_read_inst_reg((*clkdm).prcm_partition,(*clkdm).cm_inst,(*clkdm).clkdm_offs+OMAP4_CM_CLKSTCTRL)&OMAP4430_MODULEMODE_MASK; 0 }
unsafe fn omap4_clkdm_restore_context(clkdm:*mut clockdomain)->i32 { match (*clkdm).context { OMAP34XX_CLKSTCTRL_DISABLE_AUTO=>omap4_cminst_clkdm_disable_hwsup((*clkdm).prcm_partition,(*clkdm).cm_inst,(*clkdm).clkdm_offs), OMAP34XX_CLKSTCTRL_FORCE_SLEEP=>omap4_cminst_clkdm_force_sleep((*clkdm).prcm_partition,(*clkdm).cm_inst,(*clkdm).clkdm_offs), OMAP34XX_CLKSTCTRL_FORCE_WAKEUP=>omap4_cminst_clkdm_force_wakeup((*clkdm).prcm_partition,(*clkdm).cm_inst,(*clkdm).clkdm_offs), OMAP34XX_CLKSTCTRL_ENABLE_AUTO=>omap4_cminst_clkdm_enable_hwsup((*clkdm).prcm_partition,(*clkdm).cm_inst,(*clkdm).clkdm_offs), _=>{} } 0 }

extern "C" {
    static mut prm_base: omap_domain_base; static mut cm_base: omap_domain_base; static mut cm2_base: omap_domain_base; static mut prcm_mpu_base: omap_domain_base;
    fn cm_register(data: *const cm_ll_data) -> i32; fn cm_unregister(data: *const cm_ll_data);
}

#[no_mangle] pub unsafe extern "C" fn omap4_cm_init(_data:*const omap_prcm_init_data)->i32 { omap_cm_base_init(); cm_register(&omap4xxx_cm_ll_data) }
static omap4xxx_cm_ll_data: cm_ll_data = cm_ll_data { wait_module_ready: Some(omap4_cminst_wait_module_ready), wait_module_idle: Some(omap4_cminst_wait_module_idle), module_enable: Some(omap4_cminst_module_enable), module_disable: Some(omap4_cminst_module_disable), xlate_clkctrl: Some(omap4_cminst_xlate_clkctrl) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
