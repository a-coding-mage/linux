// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/*
 * Microsemi MIPS SoC support
 *
 * Copyright (c) 2017 Microsemi Corporation
 */

const DEVCPU_GCB_CHIP_REGS_CHIP_ID: usize = 0x71070000;
const CHIP_ID_PART_ID: u32 = ((1u32 << (27 - 12 + 1)) - 1) << 12;

const OCELOT_PART_ID: u32 = 0x7514u32 << 12;

const UART_UART: usize = 0x70100000;

extern "C" {
    static mut fw_arg0: u32;
    static mut fw_arg1: u32;
    static mut arcs_cmdline: *mut core::ffi::c_char;
    static mut late_time_init: Option<unsafe extern "C" fn()>;
    static __dtb_ocelot_pcb123_begin: u8;

    fn write_c0_entryhi(value: usize);
    fn mtc0_tlbw_hazard();
    fn tlb_probe();
    fn tlb_probe_hazard();
    fn read_c0_index() -> i32;
    fn __raw_readl(address: *const core::ffi::c_void) -> u32;
    fn strlen(string: *const core::ffi::c_char) -> usize;
    fn strscpy(destination: *mut core::ffi::c_char, source: *const core::ffi::c_char);
    fn ioremap(address: usize, size: usize) -> *mut core::ffi::c_void;
    fn setup_8250_early_printk_port(base: usize, register_shift: i32, clock: u32);
}

unsafe extern "C" fn ocelot_detect() -> bool {
    let mut rev: u32;
    let idx: i32;

    /* Look for the TLB entry set up by redboot before trying to use it */
    write_c0_entryhi(DEVCPU_GCB_CHIP_REGS_CHIP_ID);
    mtc0_tlbw_hazard();
    tlb_probe();
    tlb_probe_hazard();
    idx = read_c0_index();
    if idx < 0 {
        return false;
    }

    /* A TLB entry exists, lets assume its usable and check the CHIP ID */
    rev = __raw_readl(DEVCPU_GCB_CHIP_REGS_CHIP_ID as *const core::ffi::c_void);

    if (rev & CHIP_ID_PART_ID) != OCELOT_PART_ID {
        return false;
    }

    /* Copy command line from bootloader early for Initrd detection */
    if fw_arg0 < 10 && (fw_arg1 & 0xFFF00000) == 0x80000000 {
        let prom_argc: u32 = fw_arg0;
        let prom_argv: *const *const core::ffi::c_char = fw_arg1 as *const *const core::ffi::c_char;

        if prom_argc > 1 && strlen(*prom_argv.add(1)) > 0 {
            /* ignore all built-in args if any f/w args given */
            strscpy(arcs_cmdline, *prom_argv.add(1));
        }
    }

    true
}

unsafe extern "C" fn ocelot_earlyprintk_init() {
    let uart_base: *mut core::ffi::c_void;

    uart_base = ioremap(UART_UART, 0x20);
    setup_8250_early_printk_port(uart_base as usize, 2, 50000);
}

unsafe extern "C" fn ocelot_late_init() {
    ocelot_earlyprintk_init();
}

unsafe extern "C" fn ocelot_fixup_fdt(
    fdt: *const core::ffi::c_void,
    _match_data: *const core::ffi::c_void,
) -> *const core::ffi::c_void {
    /* This has to be done so late because ioremap needs to work */
    late_time_init = Some(ocelot_late_init);

    fdt
}

#[repr(C)]
pub struct MipsMachine {
    pub fdt: *const u8,
    pub fixup_fdt: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> *const core::ffi::c_void,
    pub detect: unsafe extern "C" fn() -> bool,
}

#[no_mangle]
pub static mut ocelot: MipsMachine = MipsMachine {
    fdt: unsafe { &__dtb_ocelot_pcb123_begin },
    fixup_fdt: ocelot_fixup_fdt,
    detect: ocelot_detect,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
