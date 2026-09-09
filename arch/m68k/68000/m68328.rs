/***************************************************************************/

/*
 *  m68328.c - 68328/68EZ328/68VZ328 specific config
 *
 *  Copyright (C) 1993 Hamish Macdonald
 *  Copyright (C) 1999 D. Jeff Dionne
 *  Copyright (C) 2001 Georges Menie, Ken Desmet
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 * VZ Support/Fixes             Evan Stawnyczy <e@lineo.ca>
 */

/***************************************************************************/

// Linux and machine-specific declarations supplied by the surrounding tree.
extern "C" {
    fn local_irq_disable();

    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_hwclk: Option<unsafe extern "C" fn() -> i32>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;

    fn hw_timer_init();
    fn m68328_hwclk() -> i32;
    fn init_ucsimm(command: *mut u8, len: i32);
    fn init_dragen2(command: *mut u8, len: i32);
}

/***************************************************************************/

unsafe extern "C" fn m68328_reset() {
    local_irq_disable();
    core::arch::asm!(
        "moveal #0x10c00000, %a0;\n\t",
        "moveb #0, 0xFFFFF300;\n\t",
        "moveal 0(%a0), %sp;\n\t",
        "moveal 4(%a0), %a0;\n\t",
        "jmp (%a0);",
        options(noreturn)
    );
}

/***************************************************************************/

pub unsafe extern "C" fn config_BSP(command: *mut u8, len: i32) {
    mach_sched_init = Some(hw_timer_init);
    mach_hwclk = Some(m68328_hwclk);
    mach_reset = Some(m68328_reset);

    // CONFIG_PILOT && CONFIG_M68328
    // mach_sched_init = None;
    // CONFIG_UCSIMM
    // init_ucsimm(command, len);
    // CONFIG_UCDIMM
    // init_ucsimm(command, len);
    // CONFIG_DRAGEN2
    // init_dragen2(command, len);
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
