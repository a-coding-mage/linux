// SPDX-License-Identifier: GPL-2.0
/*
 *	Operating System Services (OSS) chip handling
 *	Written by Joshua M. Thompson (funaho@jurai.org)
 *
 *	This chip is used in the IIfx in place of VIA #2. It acts like a fancy
 *	VIA chip with prorammable interrupt levels.
 *
 * 990502 (jmt) - Major rewrite for new interrupt architecture as well as some
 *		  recent insights into OSS operational details.
 * 990610 (jmt) - Now taking full advantage of the OSS. Interrupts are mapped
 *		  to mostly match the A/UX interrupt scheme supported on the
 *		  VIA side. Also added support for enabling the ISM irq again
 *		  since we now have a functional IOP manager.
 */

// Linux and Macintosh declarations are supplied by the surrounding crate.

extern "C" {
    static mut oss_present: i32;
    static mut oss: *mut mac_oss;

    static mut macintosh_config: *mut mac_config;

    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn generic_handle_irq(irq: i32);
    fn irq_set_chained_handler(irq: i32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn via1_irq(desc: *mut irq_desc);
    fn via_irq_enable(irq: i32);
    fn via_irq_disable(irq: i32);
}

#[repr(C)]
pub struct mac_oss {
    pub irq_level: [u8; OSS_NUM_SOURCES as usize],
    pub irq_pending: u16,
}

#[repr(C)]
pub struct mac_config {
    pub ident: i32,
}

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

pub const OSS_IRQLEV_IOPISM: i32 = IRQ_AUTO_1;
pub const OSS_IRQLEV_SCSI: i32 = IRQ_AUTO_2;
pub const OSS_IRQLEV_NUBUS: i32 = IRQ_AUTO_3;
pub const OSS_IRQLEV_IOPSCC: i32 = IRQ_AUTO_4;
pub const OSS_IRQLEV_VIA1: i32 = IRQ_AUTO_6;

/*
 * Initialize the OSS
 */
pub unsafe extern "C" fn oss_init() {
    let mut i: i32;

    if (*macintosh_config).ident != MAC_MODEL_IIFX {
        return;
    }

    oss = OSS_BASE as *mut mac_oss;
    pr_debug(b"OSS detected at %p\0".as_ptr() as *const core::ffi::c_char, oss);
    oss_present = 1;

    /* Disable all interrupts. Unlike a VIA it looks like we */
    /* do this by setting the source's interrupt level to zero. */
    i = 0;
    while i < OSS_NUM_SOURCES {
        (*oss).irq_level[i as usize] = 0;
        i += 1;
    }
}

/*
 * Handle OSS interrupts.
 * XXX how do you clear a pending IRQ? is it even necessary?
 */
unsafe extern "C" fn oss_iopism_irq(_desc: *mut irq_desc) {
    generic_handle_irq(IRQ_MAC_ADB);
}

unsafe extern "C" fn oss_scsi_irq(_desc: *mut irq_desc) {
    generic_handle_irq(IRQ_MAC_SCSI);
}

unsafe extern "C" fn oss_nubus_irq(_desc: *mut irq_desc) {
    let mut events: u16 = (*oss).irq_pending & OSS_IP_NUBUS;
    let mut irq_num: i32 = NUBUS_SOURCE_BASE + 5;
    let mut irq_bit: u16 = OSS_IP_NUBUS5;
    loop {
        if events & irq_bit != 0 {
            events &= !irq_bit;
            generic_handle_irq(irq_num);
        }
        irq_num -= 1;
        irq_bit >>= 1;
        if events == 0 {
            break;
        }
    }
}

unsafe extern "C" fn oss_iopscc_irq(_desc: *mut irq_desc) {
    generic_handle_irq(IRQ_MAC_SCC);
}

/* Register the OSS and NuBus interrupt dispatchers. */
pub unsafe extern "C" fn oss_register_interrupts() {
    irq_set_chained_handler(OSS_IRQLEV_IOPISM, oss_iopism_irq);
    irq_set_chained_handler(OSS_IRQLEV_SCSI, oss_scsi_irq);
    irq_set_chained_handler(OSS_IRQLEV_NUBUS, oss_nubus_irq);
    irq_set_chained_handler(OSS_IRQLEV_IOPSCC, oss_iopscc_irq);
    irq_set_chained_handler(OSS_IRQLEV_VIA1, via1_irq);

    /* OSS_VIA1 gets enabled here because it has no machspec interrupt. */
    (*oss).irq_level[OSS_VIA1 as usize] = OSS_IRQLEV_VIA1 as u8;
}

/* Enable an OSS interrupt. */
pub unsafe extern "C" fn oss_irq_enable(mut irq: i32) {
    match irq {
        IRQ_MAC_SCC => { (*oss).irq_level[OSS_IOPSCC as usize] = OSS_IRQLEV_IOPSCC as u8; return; }
        IRQ_MAC_ADB => { (*oss).irq_level[OSS_IOPISM as usize] = OSS_IRQLEV_IOPISM as u8; return; }
        IRQ_MAC_SCSI => { (*oss).irq_level[OSS_SCSI as usize] = OSS_IRQLEV_SCSI as u8; return; }
        IRQ_NUBUS_9 | IRQ_NUBUS_A | IRQ_NUBUS_B | IRQ_NUBUS_C | IRQ_NUBUS_D | IRQ_NUBUS_E => {
            irq -= NUBUS_SOURCE_BASE;
            (*oss).irq_level[irq as usize] = OSS_IRQLEV_NUBUS as u8;
            return;
        }
        _ => {}
    }
    if IRQ_SRC(irq) == 1 { via_irq_enable(irq); }
}

/* Disable an OSS interrupt. */
pub unsafe extern "C" fn oss_irq_disable(mut irq: i32) {
    match irq {
        IRQ_MAC_SCC => { (*oss).irq_level[OSS_IOPSCC as usize] = 0; return; }
        IRQ_MAC_ADB => { (*oss).irq_level[OSS_IOPISM as usize] = 0; return; }
        IRQ_MAC_SCSI => { (*oss).irq_level[OSS_SCSI as usize] = 0; return; }
        IRQ_NUBUS_9 | IRQ_NUBUS_A | IRQ_NUBUS_B | IRQ_NUBUS_C | IRQ_NUBUS_D | IRQ_NUBUS_E => {
            irq -= NUBUS_SOURCE_BASE;
            (*oss).irq_level[irq as usize] = 0;
            return;
        }
        _ => {}
    }
    if IRQ_SRC(irq) == 1 { via_irq_disable(irq); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
