/* SPDX-License-Identifier: GPL-2.0 */

// Dependency equivalent of <linux/linkage.h> is supplied externally.

use core::ffi::{c_char, c_int, c_uchar, c_ulong};

#[repr(C)]
pub struct rtc_time {
    _private: [u8; 0],
}

/* config.c */
unsafe extern "C" {
    pub fn sun3_init();

    /* idprom.c */
    pub fn sun3_get_model(model: *mut c_char);

    /* intersil.c */
    pub fn sun3_hwclk(set: c_int, t: *mut rtc_time) -> c_int;

    /* leds.c */
    pub fn sun3_leds(byte: c_uchar);

    /* mmu_emu.c */
    pub fn mmu_emu_init(bootmem_end: c_ulong);
    pub fn mmu_emu_handle_fault(
        vaddr: c_ulong,
        read_flag: c_int,
        kernel_fault: c_int,
    ) -> c_int;
    pub fn print_pte_vaddr(vaddr: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
