// SPDX-License-Identifier: GPL-2.0-only
/* OMAP3xxx PRM module functions. C includes and external symbols are supplied by dependencies. */

extern "C" {
    fn omap2_prm_read_mod_reg(module: i16, offset: u16) -> u32;
    fn omap2_prm_write_mod_reg(val: u32, module: i16, offset: u16);
    fn omap2_prm_rmw_mod_reg_bits(mask: u32, bits: u32, module: i16, offset: u16) -> i32;
    fn omap2_prm_set_mod_reg_bits(bits: u32, module: i16, offset: u16);
    fn omap2_prm_clear_mod_reg_bits(bits: u32, module: i16, offset: u16);
    fn omap2_prm_read_mod_bits_shift(module: i16, offset: u16, mask: u32) -> i32;
    fn omap2_cm_read_mod_reg(module: i16, offset: u16) -> u32;
    fn omap2_cm_write_mod_reg(val: u32, module: i16, offset: u16);
    fn omap2_cm_set_mod_reg_bits(bits: u32, module: i16, offset: u16);
    fn omap2_pwrdm_set_logic_retst(p: *mut powerdomain, v: u8) -> i32;
    fn omap2_pwrdm_set_mem_onst(p: *mut powerdomain, b: u8, v: u8) -> i32;
    fn omap2_pwrdm_set_mem_retst(p: *mut powerdomain, b: u8, v: u8) -> i32;
    fn omap2_pwrdm_read_mem_pwrst(p: *mut powerdomain, b: u8) -> i32;
    fn omap2_pwrdm_read_mem_retst(p: *mut powerdomain, b: u8) -> i32;
    fn omap2_pwrdm_wait_transition(p: *mut powerdomain) -> i32;
    fn omap2_prm_assert_hardreset(a: *mut hardreset) -> i32;
    fn omap2_prm_deassert_hardreset(a: *mut hardreset) -> i32;
    fn omap2_prm_is_hardreset_asserted(a: *mut hardreset) -> i32;
    fn omap3_has_io_wakeup() -> bool;
    fn omap3_has_io_chain_ctrl() -> bool;
    fn prm_register(d: *mut prm_ll_data) -> i32;
    fn prm_unregister(d: *mut prm_ll_data);
    fn omap_prcm_register_chain_handler(s: *mut omap_prcm_irq_setup) -> i32;
    fn of_find_matching_node(n: *mut device_node, t: *const of_device_id) -> *mut device_node;
    fn of_irq_get(n: *mut device_node, i: i32) -> i32;
    fn of_node_put(n: *mut device_node);
    fn omap2_clk_legacy_provider_init(a: i32, b: usize);
    fn pr_warn(s: *const u8);
    fn pr_err(s: *const u8);
}

#[repr(C)] pub struct powerdomain { pub prcm_offs: i16 }
#[repr(C)] pub struct hardreset { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct prm_reset_src_map { pub reg_shift: i32, pub std_shift: i32 }
#[repr(C)] pub struct omap3_vp { pub tranxdone_status: u32 }
#[repr(C)] pub struct omap_prcm_irq { pub name: *const u8, pub irq: i32, pub bit: i32 }
#[repr(C)] pub struct omap_prcm_irq_setup { pub ack:u16,pub mask:u16,pub nr_regs:u32,pub irqs:*const omap_prcm_irq,pub nr_irqs:usize,pub irq:i32,pub read_pending_irqs:Option<unsafe extern "C" fn(*mut usize)>,pub ocp_barrier:Option<unsafe extern "C" fn()>,pub save_and_clear_irqen:Option<unsafe extern "C" fn(*mut u32)>,pub restore_irqen:Option<unsafe extern "C" fn(*mut u32)>,pub reconfigure_io_chain:Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct pwrdm_ops { pub pwrdm_set_next_pwrst:Option<unsafe extern "C" fn(*mut powerdomain,u8)->i32>, pub pwrdm_read_next_pwrst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_read_pwrst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_read_prev_pwrst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_set_logic_retst:Option<unsafe extern "C" fn(*mut powerdomain,u8)->i32>, pub pwrdm_read_logic_pwrst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_read_logic_retst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_read_prev_logic_pwrst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_set_mem_onst:Option<unsafe extern "C" fn(*mut powerdomain,u8,u8)->i32>, pub pwrdm_set_mem_retst:Option<unsafe extern "C" fn(*mut powerdomain,u8,u8)->i32>, pub pwrdm_read_mem_pwrst:Option<unsafe extern "C" fn(*mut powerdomain,u8)->i32>, pub pwrdm_read_mem_retst:Option<unsafe extern "C" fn(*mut powerdomain,u8)->i32>, pub pwrdm_read_prev_mem_pwrst:Option<unsafe extern "C" fn(*mut powerdomain,u8)->i32>, pub pwrdm_clear_all_prev_pwrst:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_enable_hdwr_sar:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_disable_hdwr_sar:Option<unsafe extern "C" fn(*mut powerdomain)->i32>, pub pwrdm_wait_transition:Option<unsafe extern "C" fn(*mut powerdomain)->i32> }
#[repr(C)] pub struct prm_ll_data { pub read_reset_sources:Option<unsafe extern "C" fn()->u32>, pub late_init:Option<unsafe extern "C" fn()->i32>, pub assert_hardreset:Option<unsafe extern "C" fn(*mut hardreset)->i32>, pub deassert_hardreset:Option<unsafe extern "C" fn(*mut hardreset)->i32>, pub is_hardreset_asserted:Option<unsafe extern "C" fn(*mut hardreset)->i32>, pub reset_system:Option<unsafe extern "C" fn()>, pub clear_mod_irqs:Option<unsafe extern "C" fn(i16,u8,u32)->i32>, pub vp_check_txdone:Option<unsafe extern "C" fn(u8)->u32>, pub vp_clear_txdone:Option<unsafe extern "C" fn(u8)> }
#[repr(C)] pub struct omap_prcm_init_data { _private:[u8;0] }

static mut omap3xxx_prm_reset_src_map: [prm_reset_src_map; 11] = [
    prm_reset_src_map{reg_shift:OMAP3430_GLOBAL_COLD_RST_SHIFT,std_shift:OMAP_GLOBAL_COLD_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_GLOBAL_SW_RST_SHIFT,std_shift:OMAP_GLOBAL_WARM_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_SECURITY_VIOL_RST_SHIFT,std_shift:OMAP_SECU_VIOL_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_MPU_WD_RST_SHIFT,std_shift:OMAP_MPU_WD_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_SECURE_WD_RST_SHIFT,std_shift:OMAP_MPU_WD_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_EXTERNAL_WARM_RST_SHIFT,std_shift:OMAP_EXTWARM_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_VDD1_VOLTAGE_MANAGER_RST_SHIFT,std_shift:OMAP_VDD_MPU_VM_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_VDD2_VOLTAGE_MANAGER_RST_SHIFT,std_shift:OMAP_VDD_CORE_VM_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_ICEPICK_RST_SHIFT,std_shift:OMAP_ICEPICK_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:OMAP3430_ICECRUSHER_RST_SHIFT,std_shift:OMAP_ICECRUSHER_RST_SRC_ID_SHIFT}, prm_reset_src_map{reg_shift:-1,std_shift:-1} ];
static mut omap3_vp: [omap3_vp; 2] = [omap3_vp{tranxdone_status:OMAP3430_VP1_TRANXDONE_ST_MASK},omap3_vp{tranxdone_status:OMAP3430_VP2_TRANXDONE_ST_MASK}];

unsafe extern "C" fn omap3_prm_vp_check_txdone(id:u8)->u32 { omap2_prm_read_mod_reg(OCP_MOD,OMAP3_PRM_IRQSTATUS_MPU_OFFSET) & omap3_vp[id as usize].tranxdone_status }
unsafe extern "C" fn omap3_prm_vp_clear_txdone(id:u8) { omap2_prm_write_mod_reg(omap3_vp[id as usize].tranxdone_status,OCP_MOD,OMAP3_PRM_IRQSTATUS_MPU_OFFSET); }
pub unsafe extern "C" fn omap3_prm_vcvp_read(o:u8)->u32 { omap2_prm_read_mod_reg(OMAP3430_GR_MOD,o as u16) }
pub unsafe extern "C" fn omap3_prm_vcvp_write(v:u32,o:u8) { omap2_prm_write_mod_reg(v,OMAP3430_GR_MOD,o as u16); }
pub unsafe extern "C" fn omap3_prm_vcvp_rmw(m:u32,b:u32,o:u8)->u32 { omap2_prm_rmw_mod_reg_bits(m,b,OMAP3430_GR_MOD,o as u16) as u32 }

unsafe extern "C" fn omap3xxx_prm_read_pending_irqs(e:*mut usize) { let m=omap2_prm_read_mod_reg(OCP_MOD,OMAP3_PRM_IRQENABLE_MPU_OFFSET); *e=(m & omap2_prm_read_mod_reg(OCP_MOD,OMAP3_PRM_IRQSTATUS_MPU_OFFSET)) as usize; }
unsafe extern "C" fn omap3xxx_prm_ocp_barrier(){omap2_prm_read_mod_reg(OCP_MOD,OMAP3_PRM_REVISION_OFFSET);}
unsafe extern "C" fn omap3xxx_prm_save_and_clear_irqen(s:*mut u32){*s=omap2_prm_read_mod_reg(OCP_MOD,OMAP3_PRM_IRQENABLE_MPU_OFFSET);omap2_prm_write_mod_reg(0,OCP_MOD,OMAP3_PRM_IRQENABLE_MPU_OFFSET);omap3xxx_prm_ocp_barrier();}
unsafe extern "C" fn omap3xxx_prm_restore_irqen(s:*mut u32){omap2_prm_write_mod_reg(*s,OCP_MOD,OMAP3_PRM_IRQENABLE_MPU_OFFSET);}

unsafe extern "C" fn omap3xxx_prm_clear_mod_irqs(module:i16,regs:u8,mask:u32)->i32 { let wo=if regs==3{OMAP3430ES2_PM_WKST3}else{PM_WKST1};let fo=if regs==3{OMAP3430ES2_CM_FCLKEN3}else{CM_FCLKEN1};let io=if regs==3{CM_ICLKEN3}else{CM_ICLKEN1};let go=if regs==3{OMAP3430ES2_PM_MPUGRPSEL3}else{OMAP3430_PM_MPUGRPSEL};let mut w=omap2_prm_read_mod_reg(module,wo)&omap2_prm_read_mod_reg(module,go)&mask;let mut c=0; if w!=0{let i=omap2_cm_read_mod_reg(module,io);let f=omap2_cm_read_mod_reg(module,fo);while w!=0{let mut en=w;omap2_cm_set_mod_reg_bits(en,module,io);if module==OMAP3430ES2_USBHOST_MOD{en|=1<<OMAP3430ES2_EN_USBHOST2_SHIFT;}omap2_cm_set_mod_reg_bits(en,module,fo);omap2_prm_write_mod_reg(w,module,wo);w=omap2_prm_read_mod_reg(module,wo)&mask;c+=1;}omap2_cm_write_mod_reg(i,module,io);omap2_cm_write_mod_reg(f,module,fo);}c}

unsafe extern "C" fn omap3_pwrdm_set_next_pwrst(p:*mut powerdomain,s:u8)->i32{omap2_prm_rmw_mod_reg_bits(OMAP_POWERSTATE_MASK,(s as u32)<<OMAP_POWERSTATE_SHIFT,(*p).prcm_offs,OMAP2_PM_PWSTCTRL);0}
unsafe extern "C" fn omap3_pwrdm_read_next_pwrst(p:*mut powerdomain)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP2_PM_PWSTCTRL,OMAP_POWERSTATE_MASK)}
unsafe extern "C" fn omap3_pwrdm_read_pwrst(p:*mut powerdomain)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP2_PM_PWSTST,OMAP_POWERSTATEST_MASK)}
unsafe extern "C" fn omap3_pwrdm_read_prev_pwrst(p:*mut powerdomain)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP3430_PM_PREPWSTST,OMAP3430_LASTPOWERSTATEENTERED_MASK)}
unsafe extern "C" fn omap3_pwrdm_read_logic_pwrst(p:*mut powerdomain)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP2_PM_PWSTST,OMAP3430_LOGICSTATEST_MASK)}
unsafe extern "C" fn omap3_pwrdm_read_logic_retst(p:*mut powerdomain)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP2_PM_PWSTCTRL,OMAP3430_LOGICSTATEST_MASK)}
unsafe extern "C" fn omap3_pwrdm_read_prev_logic_pwrst(p:*mut powerdomain)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP3430_PM_PREPWSTST,OMAP3430_LASTLOGICSTATEENTERED_MASK)}
unsafe extern "C" fn omap3_get_mem_bank_lastmemst_mask(b:u8)->i32{match b{0=>OMAP3430_LASTMEM1STATEENTERED_MASK as i32,1=>OMAP3430_LASTMEM2STATEENTERED_MASK as i32,2=>OMAP3430_LASTSHAREDL2CACHEFLATSTATEENTERED_MASK as i32,3=>OMAP3430_LASTL2FLATMEMSTATEENTERED_MASK as i32,_=>-17}}
unsafe extern "C" fn omap3_pwrdm_read_prev_mem_pwrst(p:*mut powerdomain,b:u8)->i32{omap2_prm_read_mod_bits_shift((*p).prcm_offs,OMAP3430_PM_PREPWSTST,omap3_get_mem_bank_lastmemst_mask(b) as u32)}
unsafe extern "C" fn omap3_pwrdm_clear_all_prev_pwrst(p:*mut powerdomain)->i32{omap2_prm_write_mod_reg(0,(*p).prcm_offs,OMAP3430_PM_PREPWSTST);0}
unsafe extern "C" fn omap3_pwrdm_enable_hdwr_sar(p:*mut powerdomain)->i32{omap2_prm_rmw_mod_reg_bits(0,1<<OMAP3430ES2_SAVEANDRESTORE_SHIFT,(*p).prcm_offs,OMAP2_PM_PWSTCTRL)}
unsafe extern "C" fn omap3_pwrdm_disable_hdwr_sar(p:*mut powerdomain)->i32{omap2_prm_rmw_mod_reg_bits(1<<OMAP3430ES2_SAVEANDRESTORE_SHIFT,0,(*p).prcm_offs,OMAP2_PM_PWSTCTRL)}

unsafe extern "C" fn omap3xxx_prm_read_reset_sources()->u32{let v=omap2_prm_read_mod_reg(WKUP_MOD,OMAP2_RM_RSTST);let mut r=0;for p in omap3xxx_prm_reset_src_map.iter(){if p.reg_shift<0||p.std_shift<0{break;}if v&(1<<p.reg_shift)!=0{r|=1<<p.std_shift;}}r}
unsafe extern "C" fn omap3xxx_prm_iva_idle(){omap2_cm_write_mod_reg(0,OMAP3430_IVA2_MOD,CM_FCLKEN);if omap2_cm_read_mod_reg(OMAP3430_IVA2_MOD,OMAP3430_CM_CLKSTST)&OMAP3430_CLKACTIVITY_IVA2_MASK==0{return;}omap2_prm_write_mod_reg(OMAP3430_RST1_IVA2_MASK|OMAP3430_RST2_IVA2_MASK|OMAP3430_RST3_IVA2_MASK,OMAP3430_IVA2_MOD,OMAP2_RM_RSTCTRL);omap2_cm_write_mod_reg(OMAP3430_CM_FCLKEN_IVA2_EN_IVA2_MASK,OMAP3430_IVA2_MOD,CM_FCLKEN);omap2_prm_write_mod_reg(0,OMAP3430_IVA2_MOD,OMAP2_RM_RSTCTRL);omap2_cm_write_mod_reg(0,OMAP3430_IVA2_MOD,CM_FCLKEN);omap2_prm_write_mod_reg(OMAP3430_RST1_IVA2_MASK|OMAP3430_RST2_IVA2_MASK|OMAP3430_RST3_IVA2_MASK,OMAP3430_IVA2_MOD,OMAP2_RM_RSTCTRL);}
unsafe extern "C" fn omap3xxx_prm_dpll3_reset(){omap2_prm_set_mod_reg_bits(OMAP_RST_DPLL3_MASK,OMAP3430_GR_MOD,OMAP2_RM_RSTCTRL);omap2_prm_read_mod_reg(OMAP3430_GR_MOD,OMAP2_RM_RSTCTRL);}
pub unsafe extern "C" fn omap3xxx_prm_clear_global_cold_reset()->i32{if omap2_prm_read_mod_reg(OMAP3430_GR_MOD,OMAP3_PRM_RSTST_OFFSET)&OMAP3430_GLOBAL_COLD_RST_MASK!=0{omap2_prm_set_mod_reg_bits(OMAP3430_GLOBAL_COLD_RST_MASK,OMAP3430_GR_MOD,OMAP3_PRM_RSTST_OFFSET);1}else{0}}
pub unsafe extern "C" fn omap3_prm_save_scratchpad_contents(p:*mut u32){*p=omap2_prm_read_mod_reg(OMAP3430_GR_MOD,OMAP3_PRM_CLKSRC_CTRL_OFFSET);*p.add(1)=omap2_prm_read_mod_reg(OMAP3430_GR_MOD,OMAP3_PRM_CLKSEL_OFFSET);}
pub unsafe extern "C" fn omap3xxx_prm_init_pm(has_uart4:bool,has_iva:bool){let u=if has_uart4{OMAP3630_EN_UART4_MASK}else{0};let g=if has_uart4{OMAP3630_GRPSEL_UART4_MASK}else{0};omap2_prm_rmw_mod_reg_bits(OMAP_AUTOEXTCLKMODE_MASK,1<<OMAP_AUTOEXTCLKMODE_SHIFT,OMAP3430_GR_MOD,OMAP3_PRM_CLKSRC_CTRL_OFFSET);omap2_prm_write_mod_reg(OMAP3430_EN_IO_MASK|OMAP3430_EN_GPIO1_MASK|OMAP3430_EN_GPT1_MASK|OMAP3430_EN_GPT12_MASK,WKUP_MOD,PM_WKEN);omap2_prm_write_mod_reg(OMAP3430_GRPSEL_GPIO1_MASK|OMAP3430_GRPSEL_GPT1_MASK|OMAP3430_GRPSEL_GPT12_MASK,WKUP_MOD,OMAP3430_PM_MPUGRPSEL);omap2_prm_write_mod_reg(OMAP3430_PM_WKEN_DSS_EN_DSS_MASK,OMAP3430_DSS_MOD,PM_WKEN);omap2_prm_write_mod_reg(u|OMAP3430_EN_GPIO2_MASK|OMAP3430_EN_GPIO3_MASK|OMAP3430_EN_GPIO4_MASK|OMAP3430_EN_GPIO5_MASK|OMAP3430_EN_GPIO6_MASK|OMAP3430_EN_UART3_MASK|OMAP3430_EN_MCBSP2_MASK|OMAP3430_EN_MCBSP3_MASK|OMAP3430_EN_MCBSP4_MASK,OMAP3430_PER_MOD,PM_WKEN);omap2_prm_write_mod_reg(g|OMAP3430_GRPSEL_GPIO2_MASK|OMAP3430_GRPSEL_GPIO3_MASK|OMAP3430_GRPSEL_GPIO4_MASK|OMAP3430_GRPSEL_GPIO5_MASK|OMAP3430_GRPSEL_GPIO6_MASK|OMAP3430_GRPSEL_UART3_MASK|OMAP3430_GRPSEL_MCBSP2_MASK|OMAP3430_GRPSEL_MCBSP3_MASK|OMAP3430_GRPSEL_MCBSP4_MASK,OMAP3430_PER_MOD,OMAP3430_PM_MPUGRPSEL);if has_iva{omap2_prm_write_mod_reg(0,WKUP_MOD,OMAP3430_PM_IVAGRPSEL);omap2_prm_write_mod_reg(0,CORE_MOD,OMAP3430_PM_IVAGRPSEL1);omap2_prm_write_mod_reg(0,CORE_MOD,OMAP3430ES2_PM_IVAGRPSEL3);omap2_prm_write_mod_reg(0,OMAP3430_PER_MOD,OMAP3430_PM_IVAGRPSEL);}for m in [MPU_MOD,CORE_MOD,OMAP3430_PER_MOD,OMAP3430_EMU_MOD,OMAP3430_NEON_MOD,OMAP3430_DSS_MOD,OMAP3430ES2_USBHOST_MOD]{omap2_prm_write_mod_reg(0xffffffff,m,OMAP2_RM_RSTST);}omap2_prm_write_mod_reg(0,OCP_MOD,OMAP3_PRM_IRQSTATUS_MPU_OFFSET);omap3xxx_prm_iva_idle();omap2_prm_write_mod_reg(OMAP3430_RM_RSTCTRL_CORE_MODEM_SW_RSTPWRON_MASK|OMAP3430_RM_RSTCTRL_CORE_MODEM_SW_RST_MASK,CORE_MOD,OMAP2_RM_RSTCTRL);omap2_prm_write_mod_reg(0,CORE_MOD,OMAP2_RM_RSTCTRL);}

unsafe extern "C" fn omap3xxx_prm_enable_io_wakeup(){if prm_features&PRM_HAS_IO_WAKEUP!=0{omap2_prm_set_mod_reg_bits(OMAP3430_EN_IO_MASK,WKUP_MOD,PM_WKEN);}}
unsafe extern "C" fn omap3430_pre_es3_1_reconfigure_io_chain(){omap2_prm_clear_mod_reg_bits(OMAP3430_EN_IO_MASK,WKUP_MOD,PM_WKEN);omap2_prm_set_mod_reg_bits(OMAP3430_EN_IO_MASK,WKUP_MOD,PM_WKEN);omap2_prm_read_mod_reg(WKUP_MOD,PM_WKEN);}
unsafe extern "C" fn omap3_prm_reconfigure_io_chain(){omap2_prm_set_mod_reg_bits(OMAP3430_EN_IO_CHAIN_MASK,WKUP_MOD,PM_WKEN);let mut i=0;while omap2_prm_read_mod_reg(WKUP_MOD,PM_WKST)&OMAP3430_ST_IO_CHAIN_MASK==0&&i<MAX_IOPAD_LATCH_TIME{i+=1;}omap2_prm_clear_mod_reg_bits(OMAP3430_EN_IO_CHAIN_MASK,WKUP_MOD,PM_WKEN);omap2_prm_set_mod_reg_bits(OMAP3430_ST_IO_CHAIN_MASK,WKUP_MOD,PM_WKST);omap2_prm_read_mod_reg(WKUP_MOD,PM_WKST);}
#[no_mangle] pub unsafe extern "C" fn omap3xxx_prm_init(_d:*const omap_prcm_init_data)->i32{if omap3_has_io_wakeup(){prm_features|=PRM_HAS_IO_WAKEUP;}prm_register(core::ptr::null_mut())}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
