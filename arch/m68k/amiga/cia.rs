/*
 *  linux/arch/m68k/amiga/cia.c - CIA support
 *
 *  Copyright (C) 1996 Roman Zippel
 *
 *  The concept of some functions bases on the original Amiga OS function
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel and Amiga hardware code.

use core::ffi::c_void;

type U8 = u8;
type U16 = u16;
type CInt = i32;
type CUInt = u32;
type CULong = usize;
type IrqreturnT = CInt;

#[repr(C)]
pub struct CIA {
    pub icr: U8,
}

#[repr(C)]
pub struct Custom {
    pub intreq: U16,
    pub intena: U16,
}

#[repr(C)]
pub struct irq_data {
    pub irq: CUInt,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const i8,
    pub irq_enable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>,
}

extern "C" {
    static mut ciaa: CIA;
    static mut ciab: CIA;
    static mut amiga_custom: Custom;

    fn local_irq_save(flags: *mut CULong);
    fn local_irq_restore(flags: CULong);
    fn generic_handle_irq(irq: CInt);
    fn m68k_setup_irq_controller(
        chip: *mut irq_chip,
        handler: unsafe extern "C" fn(*mut irq_data),
        irq: CInt,
        count: CInt,
    );
    fn handle_simple_irq(data: *mut irq_data);
    fn m68k_irq_startup_irq(irq: CInt);
    fn request_irq(
        irq: CInt,
        handler: unsafe extern "C" fn(CInt, *mut c_void) -> IrqreturnT,
        flags: CInt,
        name: *const i8,
        dev_id: *mut c_void,
    ) -> CInt;
    fn pr_err(fmt: *const i8, ...);
}

// Constants supplied by the included Amiga IRQ and hardware headers.
extern "C" {
    static IF_PORTS: U16;
    static IF_EXTER: U16;
    static IF_SETCLR: U16;
    static IRQ_AMIGA_PORTS: CInt;
    static IRQ_AMIGA_EXTER: CInt;
    static IRQ_AMIGA_CIAA: CInt;
    static IRQ_AMIGA_CIAB: CInt;
    static IRQ_AUTO_2: CInt;
    static IRQ_AUTO_6: CInt;
    static CIA_ICR_SETCLR: U8;
    static CIA_ICR_ALL: U8;
    static CIA_IRQS: CInt;
    static IRQF_SHARED: CInt;
}

#[repr(C)]
pub struct ciabase {
    pub cia: *mut CIA,
    pub icr_mask: U8,
    pub icr_data: U8,
    pub int_mask: U16,
    pub handler_irq: CInt,
    pub cia_irq: CInt,
    pub server_irq: CInt,
    pub name: *mut i8,
}

#[no_mangle]
pub static mut ciaa_base: ciabase = ciabase {
    cia: unsafe { &raw mut ciaa },
    icr_mask: 0,
    icr_data: 0,
    int_mask: 0,
    handler_irq: 0,
    cia_irq: 0,
    server_irq: 0,
    name: core::ptr::null_mut(),
};

#[no_mangle]
pub static mut ciab_base: ciabase = ciabase {
    cia: unsafe { &raw mut ciab },
    icr_mask: 0,
    icr_data: 0,
    int_mask: 0,
    handler_irq: 0,
    cia_irq: 0,
    server_irq: 0,
    name: core::ptr::null_mut(),
};

/*
 *  Cause or clear CIA interrupts, return old interrupt status.
 */
pub unsafe extern "C" fn cia_set_irq(base: *mut ciabase, mask: U8) -> U8 {
    let base_ref = &mut *base;
    let old = {
        base_ref.icr_data |= (*base_ref.cia).icr;
        base_ref.icr_data
    };
    if mask & CIA_ICR_SETCLR != 0 {
        base_ref.icr_data |= mask;
    } else {
        base_ref.icr_data &= !mask;
    }
    if base_ref.icr_data & base_ref.icr_mask != 0 {
        amiga_custom.intreq = IF_SETCLR | base_ref.int_mask;
    }
    old & base_ref.icr_mask
}

/*
 *  Enable or disable CIA interrupts, return old interrupt mask,
 */
pub unsafe extern "C" fn cia_able_irq(base: *mut ciabase, mask: U8) -> U8 {
    let base_ref = &mut *base;
    let old = base_ref.icr_mask;
    base_ref.icr_data |= (*base_ref.cia).icr;
    (*base_ref.cia).icr = mask;
    if mask & CIA_ICR_SETCLR != 0 {
        base_ref.icr_mask |= mask;
    } else {
        base_ref.icr_mask &= !mask;
    }
    base_ref.icr_mask &= CIA_ICR_ALL;
    if base_ref.icr_data & base_ref.icr_mask != 0 {
        amiga_custom.intreq = IF_SETCLR | base_ref.int_mask;
    }
    old
}

unsafe extern "C" fn cia_handler(_irq: CInt, dev_id: *mut c_void) -> IrqreturnT {
    let base = dev_id as *mut ciabase;
    let mut mach_irq;
    let mut ints;
    let mut flags: CULong = 0;

    /* Interrupts get disabled while the timer irq flag is cleared and
     * the timer interrupt serviced.
     */
    mach_irq = (*base).cia_irq;
    local_irq_save(&mut flags);
    ints = cia_set_irq(base, CIA_ICR_ALL);
    amiga_custom.intreq = (*base).int_mask;
    if ints & 1 != 0 {
        generic_handle_irq(mach_irq);
    }
    local_irq_restore(flags);
    mach_irq += 1;
    ints >>= 1;
    while ints != 0 {
        if ints & 1 != 0 {
            generic_handle_irq(mach_irq);
        }
        mach_irq += 1;
        ints >>= 1;
    }
    1
}

unsafe extern "C" fn cia_irq_enable(data: *mut irq_data) {
    let irq = (*data).irq;
    let mask: U8;
    if irq >= IRQ_AMIGA_CIAB as CUInt {
        mask = 1u8 << (irq - IRQ_AMIGA_CIAB as CUInt);
        cia_set_irq(&raw mut ciab_base, mask);
        cia_able_irq(&raw mut ciab_base, CIA_ICR_SETCLR | mask);
    } else {
        mask = 1u8 << (irq - IRQ_AMIGA_CIAA as CUInt);
        cia_set_irq(&raw mut ciaa_base, mask);
        cia_able_irq(&raw mut ciaa_base, CIA_ICR_SETCLR | mask);
    }
}

unsafe extern "C" fn cia_irq_disable(data: *mut irq_data) {
    let irq = (*data).irq;
    if irq >= IRQ_AMIGA_CIAB as CUInt {
        cia_able_irq(&raw mut ciab_base, 1u8 << (irq - IRQ_AMIGA_CIAB as CUInt));
    } else {
        cia_able_irq(&raw mut ciaa_base, 1u8 << (irq - IRQ_AMIGA_CIAA as CUInt));
    }
}

static mut cia_irq_chip: irq_chip = irq_chip {
    name: b"cia\0".as_ptr() as *const i8,
    irq_enable: Some(cia_irq_enable),
    irq_disable: Some(cia_irq_disable),
};

/*
 * Override auto irq 2 & 6 and use them as general chain
 * for external interrupts, we link the CIA interrupt sources
 * into this chain.
 */
unsafe extern "C" fn auto_irq_enable(data: *mut irq_data) {
    match (*data).irq as CInt {
        x if x == IRQ_AUTO_2 => amiga_custom.intena = IF_SETCLR | IF_PORTS,
        x if x == IRQ_AUTO_6 => amiga_custom.intena = IF_SETCLR | IF_EXTER,
        _ => {}
    }
}

unsafe extern "C" fn auto_irq_disable(data: *mut irq_data) {
    match (*data).irq as CInt {
        x if x == IRQ_AUTO_2 => amiga_custom.intena = IF_PORTS,
        x if x == IRQ_AUTO_6 => amiga_custom.intena = IF_EXTER,
        _ => {}
    }
}

static mut auto_irq_chip: irq_chip = irq_chip {
    name: b"auto\0".as_ptr() as *const i8,
    irq_enable: Some(auto_irq_enable),
    irq_disable: Some(auto_irq_disable),
};

pub unsafe extern "C" fn cia_init_IRQ(base: *mut ciabase) {
    m68k_setup_irq_controller(&raw mut cia_irq_chip, handle_simple_irq,
                              (*base).cia_irq, CIA_IRQS);

    /* clear any pending interrupt and turn off all interrupts */
    cia_set_irq(base, CIA_ICR_ALL);
    cia_able_irq(base, CIA_ICR_ALL);

    /* override auto int and install CIA handler */
    m68k_setup_irq_controller(&raw mut auto_irq_chip, handle_simple_irq,
                              (*base).handler_irq, 1);
    m68k_irq_startup_irq((*base).handler_irq);
    if request_irq((*base).handler_irq, cia_handler, IRQF_SHARED,
                   (*base).name, base as *mut c_void) != 0 {
        pr_err(b"Couldn't register %s interrupt\n\0".as_ptr() as *const i8,
               (*base).name);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
