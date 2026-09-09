/*
 * ints.c - Generic interrupt controller support
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 * Copyright 1996 Roman Zippel
 * Copyright 1999 D. Jeff Dionne <jeff@rt-control.com>
 */

// C header dependencies are supplied by the surrounding kernel translation.

/* assembler routines */
extern "C" {
    fn system_call();
    fn buserr();
    fn trap();
    fn trap3();
    fn trap4();
    fn trap5();
    fn trap6();
    fn trap7();
    fn trap8();
    fn trap9();
    fn trap10();
    fn trap11();
    fn trap12();
    fn trap13();
    fn trap14();
    fn trap15();
    fn trap33();
    fn trap34();
    fn trap35();
    fn trap36();
    fn trap37();
    fn trap38();
    fn trap39();
    fn trap40();
    fn trap41();
    fn trap42();
    fn trap43();
    fn trap44();
    fn trap45();
    fn trap46();
    fn trap47();
    fn bad_interrupt(_: i32, _: *mut core::ffi::c_void) -> irqreturn_t;
    fn inthandler() -> irqreturn_t;
    fn inthandler1() -> irqreturn_t;
    fn inthandler2() -> irqreturn_t;
    fn inthandler3() -> irqreturn_t;
    fn inthandler4() -> irqreturn_t;
    fn inthandler5() -> irqreturn_t;
    fn inthandler6() -> irqreturn_t;
    fn inthandler7() -> irqreturn_t;
}

/* The 68k family did not have a good way to determine the source
 * of interrupts until later in the family.  The EC000 core does
 * not provide the vector number on the stack, we vector everything
 * into one vector and look in the blasted mask register...
 * This code is designed to be fast, almost constant time, not clean!
 */
pub unsafe extern "C" fn process_int(_vec: i32, fp: *mut pt_regs) {
    let mut irq: i32;
    let mut mask: u32;

    let mut pend: u32 = ISR;

    while pend != 0 {
        if pend & 0x0000_ffff != 0 {
            if pend & 0x0000_00ff != 0 {
                if pend & 0x0000_000f != 0 {
                    mask = 0x0000_0001;
                    irq = 0;
                } else {
                    mask = 0x0000_0010;
                    irq = 4;
                }
            } else if pend & 0x0000_0f00 != 0 {
                mask = 0x0000_0100;
                irq = 8;
            } else {
                mask = 0x0000_1000;
                irq = 12;
            }
        } else if pend & 0x00ff_0000 != 0 {
            if pend & 0x000f_0000 != 0 {
                mask = 0x0001_0000;
                irq = 16;
            } else {
                mask = 0x0010_0000;
                irq = 20;
            }
        } else if pend & 0x0f00_0000 != 0 {
            mask = 0x0100_0000;
            irq = 24;
        } else {
            mask = 0x1000_0000;
            irq = 28;
        }

        while mask & pend == 0 {
            mask <<= 1;
            irq += 1;
        }

        do_IRQ(irq, fp);
        pend &= !mask;
    }
}

unsafe extern "C" fn intc_irq_unmask(d: *mut irq_data) {
    IMR &= !(1 << (*d).irq);
}

unsafe extern "C" fn intc_irq_mask(d: *mut irq_data) {
    IMR |= 1 << (*d).irq;
}

static mut intc_irq_chip: irq_chip = irq_chip {
    name: "M68K-INTC\0".as_ptr() as *const _,
    irq_mask: Some(intc_irq_mask),
    irq_unmask: Some(intc_irq_unmask),
};

/*
 * This function should be called during kernel startup to initialize
 * the machine vector table.
 */
pub unsafe extern "C" fn trap_init() {
    let mut i: i32;

    /* set up the vectors */
    i = 72;
    while i < 256 {
        _ramvec[i as usize] = bad_interrupt as e_vector;
        i += 1;
    }

    _ramvec[32] = system_call as e_vector;

    _ramvec[65] = inthandler1 as e_vector;
    _ramvec[66] = inthandler2 as e_vector;
    _ramvec[67] = inthandler3 as e_vector;
    _ramvec[68] = inthandler4 as e_vector;
    _ramvec[69] = inthandler5 as e_vector;
    _ramvec[70] = inthandler6 as e_vector;
    _ramvec[71] = inthandler7 as e_vector;
}

pub unsafe extern "C" fn init_IRQ() {
    let mut i: i32;

    IVR = 0x40; /* Set DragonBall IVR (interrupt base) to 64 */

    /* turn off all interrupts */
    IMR = !0;

    i = 0;
    while i < NR_IRQS {
        irq_set_chip(i, &raw mut intc_irq_chip);
        irq_set_handler(i, handle_level_irq);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
