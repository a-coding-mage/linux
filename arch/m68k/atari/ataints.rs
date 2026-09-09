/* arch/m68k/atari/ataints.c -- Atari Linux interrupt handling code */

// Kernel and architecture symbols referenced below are supplied by other files.

static mut FREE_VME_VEC_BITMAP: ::core::ffi::c_int = 0;

pub unsafe extern "C" fn falcon_hblhandler() {
    // The original handler is assembly: set the saved IPL to 2, then rte.
    core::arch::asm!("orw #0x200, (%sp)", "rte", options(noreturn));
}

unsafe extern "C" fn atari_irq_startup(data: *mut irq_data) -> ::core::ffi::c_uint {
    let irq = (*data).irq;
    m68k_irq_startup(data);
    atari_turnon_irq(irq);
    atari_enable_irq(irq);
    0
}

unsafe extern "C" fn atari_irq_shutdown(data: *mut irq_data) {
    let irq = (*data).irq;
    atari_disable_irq(irq);
    atari_turnoff_irq(irq);
    m68k_irq_shutdown(data);
    if irq == IRQ_AUTO_4 {
        vectors[VEC_INT4 as usize] = falcon_hblhandler as *mut _;
    }
}

unsafe extern "C" fn atari_irq_enable(data: *mut irq_data) { atari_enable_irq((*data).irq); }
unsafe extern "C" fn atari_irq_disable(data: *mut irq_data) { atari_disable_irq((*data).irq); }

static mut ATARI_IRQ_CHIP: irq_chip = irq_chip {
    name: b"atari\0".as_ptr() as *const _,
    irq_startup: Some(atari_irq_startup), irq_shutdown: Some(atari_irq_shutdown),
    irq_enable: Some(atari_irq_enable), irq_disable: Some(atari_irq_disable),
};

#[repr(C)]
struct mfptimerbase {
    mfp: *mut MFP, mfp_mask: u8, mfp_data: u8, int_mask: u16,
    handler_irq: ::core::ffi::c_int, mfptimer_irq: ::core::ffi::c_int,
    server_irq: ::core::ffi::c_int, name: *mut i8,
}

static mut STMFP_BASE: mfptimerbase = mfptimerbase {
    mfp: &raw mut st_mfp, mfp_mask: 0, mfp_data: 0, int_mask: 0,
    handler_irq: IRQ_MFP_TIMD, mfptimer_irq: IRQ_MFP_TIMER1, server_irq: 0,
    name: b"MFP Timer D\0".as_ptr() as *mut i8,
};

unsafe extern "C" fn mfp_timer_d_handler(_irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t {
    let base = dev_id as *mut mfptimerbase;
    let mut mach_irq = (*base).mfptimer_irq;
    let mut ints = (*base).int_mask;
    while ints != 0 {
        if ints & 1 != 0 { generic_handle_irq(mach_irq as _); }
        mach_irq += 1;
        ints >>= 1;
    }
    IRQ_HANDLED
}

unsafe extern "C" fn atari_mfptimer_enable(data: *mut irq_data) {
    let mfp_num = (*data).irq - IRQ_MFP_TIMER1;
    STMFP_BASE.int_mask |= 1u16 << mfp_num;
    atari_enable_irq(IRQ_MFP_TIMD);
}
unsafe extern "C" fn atari_mfptimer_disable(data: *mut irq_data) {
    let mfp_num = (*data).irq - IRQ_MFP_TIMER1;
    STMFP_BASE.int_mask &= !(1u16 << mfp_num);
    if STMFP_BASE.int_mask == 0 { atari_disable_irq(IRQ_MFP_TIMD); }
}

static mut ATARI_MFPTIMER_CHIP: irq_chip = irq_chip {
    name: b"timer_d\0".as_ptr() as *const _, irq_startup: None, irq_shutdown: None,
    irq_enable: Some(atari_mfptimer_enable), irq_disable: Some(atari_mfptimer_disable),
};

static mut ENAT_CPLD: *mut u8 = core::ptr::null_mut();

unsafe fn enat_map() {
    if ENAT_CPLD.is_null() { ENAT_CPLD = ioremap(ATARI_ETHERNAT_PHYS_ADDR + 0x23, 2) as *mut u8; }
}
unsafe extern "C" fn atari_ethernat_startup(data: *mut irq_data) -> ::core::ffi::c_uint {
    let enat_num = 140 - (*data).irq + 1; m68k_irq_startup(data); enat_map();
    if enat_num == 1 { *ENAT_CPLD |= 1 << enat_num; } 0
}
unsafe extern "C" fn atari_ethernat_enable(data: *mut irq_data) { let n=140-(*data).irq+1; enat_map(); *ENAT_CPLD |= 1<<n; }
unsafe extern "C" fn atari_ethernat_disable(data: *mut irq_data) { let n=140-(*data).irq+1; enat_map(); *ENAT_CPLD &= !(1<<n); }
unsafe extern "C" fn atari_ethernat_shutdown(data: *mut irq_data) { let n=140-(*data).irq+1; if !ENAT_CPLD.is_null() { *ENAT_CPLD &= !(1<<n); iounmap(ENAT_CPLD as _); ENAT_CPLD=core::ptr::null_mut(); } }

static mut ATARI_ETHERNAT_CHIP: irq_chip = irq_chip {
    name: b"ethernat\0".as_ptr() as *const _, irq_startup: Some(atari_ethernat_startup),
    irq_shutdown: Some(atari_ethernat_shutdown), irq_enable: Some(atari_ethernat_enable),
    irq_disable: Some(atari_ethernat_disable),
};

pub unsafe extern "C" fn atari_init_IRQ() {
    m68k_setup_user_interrupt(VEC_USER, NUM_ATARI_SOURCES - IRQ_USER);
    m68k_setup_irq_controller(&raw mut ATARI_IRQ_CHIP, handle_simple_irq, 1, NUM_ATARI_SOURCES - 1);
    st_mfp.vec_adr = 0x40; st_mfp.int_en_a=0; st_mfp.int_en_b=0; st_mfp.int_mk_a=0xff; st_mfp.int_mk_b=0xff;
    if ATARIHW_PRESENT(TT_MFP) { tt_mfp.vec_adr=0x50; tt_mfp.int_en_a=0; tt_mfp.int_en_b=0; tt_mfp.int_mk_a=0xff; tt_mfp.int_mk_b=0xff; }
    if ATARIHW_PRESENT(SCC) && !atari_SCC_reset_done { atari_scc.cha_a_ctrl=9; MFPDELAY(); atari_scc.cha_a_ctrl=0xc0; }
    if ATARIHW_PRESENT(SCU) { tt_scu.sys_mask=0; tt_scu.vme_mask=0x60; } else { vectors[VEC_INT2 as usize]=falcon_hblhandler as *mut _; vectors[VEC_INT4 as usize]=falcon_hblhandler as *mut _; }
    if ATARIHW_PRESENT(PCM_8BIT) && ATARIHW_PRESENT(MICROWIRE) { atari_microwire_cmd(MW_LM1992_PSG_HIGH); }
    stdma_init(); sound_ym.rd_data_reg_sel=7; sound_ym.wd_data=0xff;
    m68k_setup_irq_controller(&raw mut ATARI_MFPTIMER_CHIP, handle_simple_irq, IRQ_MFP_TIMER1, 8);
    irq_set_status_flags(IRQ_MFP_TIMER1, IRQ_IS_POLLED); irq_set_status_flags(IRQ_MFP_TIMER2, IRQ_IS_POLLED);
    st_mfp.tim_dt_d=254; st_mfp.tim_ct_cd=(st_mfp.tim_ct_cd & 0xf0)|0x6;
    if request_irq(IRQ_MFP_TIMD, mfp_timer_d_handler, IRQF_SHARED, STMFP_BASE.name, &raw mut STMFP_BASE) != 0 { pr_err!("Couldn't register %s interrupt\n", STMFP_BASE.name); }
    m68k_setup_irq_controller(&raw mut ATARI_ETHERNAT_CHIP, handle_simple_irq, 139, 2);
}

pub unsafe extern "C" fn atari_register_vme_int() -> ::core::ffi::c_uint {
    let mut i=0; while i<32 && (FREE_VME_VEC_BITMAP & (1<<i)) != 0 { i+=1; }
    if i==16 { return 0; } FREE_VME_VEC_BITMAP |= 1<<i; (VME_SOURCE_BASE+i) as _
}
pub unsafe extern "C" fn atari_unregister_vme_int(mut irq: ::core::ffi::c_uint) {
    if irq>=VME_SOURCE_BASE && irq<VME_SOURCE_BASE+VME_MAX_SOURCES { irq-=VME_SOURCE_BASE; FREE_VME_VEC_BITMAP &= !(1<<irq); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
