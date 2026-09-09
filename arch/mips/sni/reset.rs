// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/mips/sni/process.c
 *
 *  Reset a SNI machine.
 */

// C dependencies: linux/delay.h, asm/io.h, asm/reboot.h, and asm/sni.h.

unsafe extern "C" {
    fn inb_p(port: u16) -> u8;
    fn outb_p(value: u8, port: u16);
    fn udelay(usecs: u64);
    fn local_irq_disable();
    static PCIMT_CSWCSM: usize;
}

/*
 * This routine reboots the machine by asking the keyboard
 * controller to pulse the reset-line low. We try that for a while,
 * and if it doesn't work, we do some other stupid things.
 */
unsafe fn kb_wait() {
    let mut i: i32;

    i = 0;
    while i < 0x10000 {
        if (inb_p(0x64) & 0x02) == 0 {
            break;
        }
        i += 1;
    }
}

/* XXX This ends up at the ARC firmware prompt ...  */
pub unsafe fn sni_machine_restart(_command: *mut u8) {
    let mut i: i32;

    /* This does a normal via the keyboard controller like a PC.
       We can do that easier ...  */
    local_irq_disable();
    loop {
        i = 0;
        while i < 100 {
            kb_wait();
            udelay(50);
            outb_p(0xfe, 0x64); /* pulse reset low */
            udelay(50);
            i += 1;
        }
    }
}

pub unsafe fn sni_machine_power_off() {
    core::ptr::write_volatile(PCIMT_CSWCSM as *mut u8, 0xfd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
