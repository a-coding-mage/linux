// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// pr_fmt(fmt) = "sead3: " fmt

// Maximum 384MB RAM at physical address 0, preceding any I/O.
static mut MEM_REGIONS: [yamon_mem_region; 2] = [
    yamon_mem_region { start: 0, size: SZ_256M + SZ_128M },
    yamon_mem_region { start: 0, size: 0 },
];

const SEAD_CONFIG: usize = CKSEG1ADDR(0x1b100110);
const SEAD_CONFIG_GIC_PRESENT: u32 = BIT(1);

const MIPS_REVISION: usize = CKSEG1ADDR(0x1fc00010);
const MIPS_REVISION_MACHINE: u32 = 0xf << 4;
const MIPS_REVISION_MACHINE_SEAD3: u32 = 0x4 << 4;

unsafe fn sead3_detect() -> bool {
    let rev: u32 = __raw_readl(MIPS_REVISION as *const core::ffi::c_void);
    (rev & MIPS_REVISION_MACHINE) == MIPS_REVISION_MACHINE_SEAD3
}

unsafe fn append_memory(fdt: *mut core::ffi::c_void) -> i32 {
    yamon_dt_append_memory(fdt, MEM_REGIONS.as_ptr())
}

unsafe fn remove_gic(fdt: *mut core::ffi::c_void) -> i32 {
    const CPU_EHCI_INT: u32 = 2;
    const CPU_UART_INT: u32 = 4;
    const CPU_ETH_INT: u32 = 6;
    let mut gic_off: i32;
    let mut cpu_off: i32;
    let mut uart_off: i32;
    let mut eth_off: i32;
    let mut ehci_off: i32;
    let mut err: i32;
    let cfg: u32;
    let cpu_phandle: u32;

    /* leave the GIC node intact if a GIC is present */
    cfg = __raw_readl(SEAD_CONFIG as *const u32);
    if (cfg & SEAD_CONFIG_GIC_PRESENT) != 0 {
        return 0;
    }

    gic_off = fdt_node_offset_by_compatible(fdt, -1, b"mti,gic\0".as_ptr() as *const i8);
    if gic_off < 0 {
        pr_err!("unable to find DT GIC node: {}\n", gic_off);
        return gic_off;
    }
    err = fdt_nop_node(fdt, gic_off);
    if err != 0 {
        pr_err!("unable to nop GIC node\n");
        return err;
    }

    cpu_off = fdt_node_offset_by_compatible(fdt, -1, b"mti,cpu-interrupt-controller\0".as_ptr() as *const i8);
    if cpu_off < 0 {
        pr_err!("unable to find CPU intc node: {}\n", cpu_off);
        return cpu_off;
    }
    cpu_phandle = fdt_get_phandle(fdt, cpu_off);
    if cpu_phandle == 0 {
        pr_err!("unable to get CPU intc phandle\n");
        return -EINVAL;
    }

    uart_off = fdt_node_offset_by_compatible(fdt, -1, b"ns16550a\0".as_ptr() as *const i8);
    while uart_off >= 0 {
        err = fdt_setprop_u32(fdt, uart_off, b"interrupt-parent\0".as_ptr() as *const i8, cpu_phandle);
        if err != 0 { pr_warn!("unable to set UART interrupt-parent: {}\n", err); return err; }
        err = fdt_setprop_u32(fdt, uart_off, b"interrupts\0".as_ptr() as *const i8, CPU_UART_INT);
        if err != 0 { pr_err!("unable to set UART interrupts property: {}\n", err); return err; }
        uart_off = fdt_node_offset_by_compatible(fdt, uart_off, b"ns16550a\0".as_ptr() as *const i8);
    }
    if uart_off != -FDT_ERR_NOTFOUND { pr_err!("error searching for UART DT node: {}\n", uart_off); return uart_off; }

    eth_off = fdt_node_offset_by_compatible(fdt, -1, b"smsc,lan9115\0".as_ptr() as *const i8);
    if eth_off < 0 { pr_err!("unable to find ethernet DT node: {}\n", eth_off); return eth_off; }
    err = fdt_setprop_u32(fdt, eth_off, b"interrupt-parent\0".as_ptr() as *const i8, cpu_phandle);
    if err != 0 { pr_err!("unable to set ethernet interrupt-parent: {}\n", err); return err; }
    err = fdt_setprop_u32(fdt, eth_off, b"interrupts\0".as_ptr() as *const i8, CPU_ETH_INT);
    if err != 0 { pr_err!("unable to set ethernet interrupts property: {}\n", err); return err; }

    ehci_off = fdt_node_offset_by_compatible(fdt, -1, b"generic-ehci\0".as_ptr() as *const i8);
    if ehci_off < 0 { pr_err!("unable to find EHCI DT node: {}\n", ehci_off); return ehci_off; }
    err = fdt_setprop_u32(fdt, ehci_off, b"interrupt-parent\0".as_ptr() as *const i8, cpu_phandle);
    if err != 0 { pr_err!("unable to set EHCI interrupt-parent: {}\n", err); return err; }
    err = fdt_setprop_u32(fdt, ehci_off, b"interrupts\0".as_ptr() as *const i8, CPU_EHCI_INT);
    if err != 0 { pr_err!("unable to set EHCI interrupts property: {}\n", err); return err; }
    0
}

static SEAD3_FDT_FIXUPS: [mips_fdt_fixup; 5] = [
    mips_fdt_fixup { func: yamon_dt_append_cmdline, description: b"append command line\0".as_ptr() as *const i8 },
    mips_fdt_fixup { func: append_memory, description: b"append memory\0".as_ptr() as *const i8 },
    mips_fdt_fixup { func: remove_gic, description: b"remove GIC when not present\0".as_ptr() as *const i8 },
    mips_fdt_fixup { func: yamon_dt_serial_config, description: b"append serial configuration\0".as_ptr() as *const i8 },
    mips_fdt_fixup { func: None, description: core::ptr::null() },
];

unsafe fn sead3_fixup_fdt(fdt: *const core::ffi::c_void, _match_data: *const core::ffi::c_void) -> *const core::ffi::c_void {
    static mut FDT_BUF: [u8; 16 << 10] = [0; 16 << 10];
    if fdt_check_header(fdt) != 0 { panic!("Corrupt DT"); }
    BUG_ON(fdt_node_check_compatible(fdt, 0, b"mti,sead-3\0".as_ptr() as *const i8));
    fw_init_cmdline();
    let err = apply_mips_fdt_fixups(FDT_BUF.as_mut_ptr() as *mut core::ffi::c_void, FDT_BUF.len(), fdt, SEAD3_FDT_FIXUPS.as_ptr());
    if err != 0 { panic!("Unable to fixup FDT: {}", err); }
    FDT_BUF.as_ptr() as *const core::ffi::c_void
}

unsafe fn sead3_measure_hpt_freq() -> u32 {
    let status_reg = 0xbf000410 as *mut core::ffi::c_void;
    let mut freq: u32;
    let mut orig: u32;
    let mut tick: u32 = 0;
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    orig = readl(status_reg) & 0x2;
    while (readl(status_reg) & 0x2) == orig {}
    orig ^= 0x2;
    write_c0_count(0);
    while tick < 100 {
        while (readl(status_reg) & 0x2) == orig {}
        orig ^= 0x2;
        tick += 1;
    }
    freq = read_c0_count();
    local_irq_restore(flags);
    freq
}

extern "C" {
    static mut __dtb_sead3_begin: u8;
}

// MIPS_MACHINE(sead3)
static mut SEAD3_MACHINE: mips_machine = mips_machine {
    fdt: unsafe { &__dtb_sead3_begin },
    detect: sead3_detect,
    fixup_fdt: sead3_fixup_fdt,
    measure_hpt_freq: sead3_measure_hpt_freq,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
