/*
 * Board setup routines for the Emerson KSI8560
 *
 * Author: Alexandr Smirnov <asmirnov@ru.mvista.com>
 *
 * Based on mpc85xx_ads.c maintained by Kumar Gala
 *
 * 2008 (c) MontaVista, Software, Inc.  This file is licensed under
 * the terms of the GNU General Public License version 2.  This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

// C headers and build-time dependencies are supplied by the surrounding kernel.

const KSI8560_CPLD_HVR: usize = 0x04; // Hardware Version Register
const KSI8560_CPLD_PVR: usize = 0x08; // PLD Version Register
const KSI8560_CPLD_RCR1: usize = 0x30; // Reset Command Register 1
const KSI8560_CPLD_RCR1_CPUHR: u8 = 0x80; // CPU Hard Reset

static mut cpld_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" {
    fn out_8(addr: *mut u8, value: u8);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn mpic_alloc(a: *mut core::ffi::c_void, b: i32, c: i32, d: i32, e: i32, f: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn mpic_init(mpic: *mut core::ffi::c_void);
    fn mpc85xx_cpm2_pic_init();
    fn of_find_compatible_node(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, c: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn of_iomap(node: *mut core::ffi::c_void, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut core::ffi::c_void);
    fn cpm2_reset();
    fn cpm2_set_pin(port: i32, pin: i32, flags: i32);
    fn cpm2_clk_setup(clock: i32, brg: i32, mode: i32);
    fn mfspr(spr: i32) -> u32;
    fn in_8(addr: *mut u8) -> u8;
    fn seq_printf(m: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn mpic_get_irq(x: *mut core::ffi::c_void) -> i32;
}

#[repr(C)]
struct cpm_pin {
    port: i32,
    pin: i32,
    flags: i32,
}

#[cfg(feature = "CONFIG_CPM2")]
static mut ksi8560_pins: [cpm_pin; 18] = [
    cpm_pin { port: 3, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 30, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 3, pin: 31, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 26, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 27, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 3, pin: 28, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 14, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 15, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 16, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 17, flags: CPM_PIN_INPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 18, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 19, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 20, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 21, flags: CPM_PIN_OUTPUT | CPM_PIN_PRIMARY },
    cpm_pin { port: 0, pin: 26, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 27, flags: CPM_PIN_INPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 28, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
    cpm_pin { port: 0, pin: 29, flags: CPM_PIN_OUTPUT | CPM_PIN_SECONDARY },
];

unsafe fn machine_restart(_cmd: *mut core::ffi::c_char) -> ! {
    if !cpld_base.is_null() {
        out_8(cpld_base.add(KSI8560_CPLD_RCR1) as *mut u8, KSI8560_CPLD_RCR1_CPUHR);
    } else {
        printk(b"Can't find CPLD base, hang forever\0".as_ptr() as *const _,);
    }
    loop {}
}

unsafe fn ksi8560_pic_init() {
    let mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN, 0, 256, b" OpenPIC  \0".as_ptr() as *const _);
    BUG_ON(mpic.is_null());
    mpic_init(mpic);
    mpc85xx_cpm2_pic_init();
}

#[cfg(feature = "CONFIG_CPM2")]
unsafe fn init_ioports() {
    let mut i = 0;
    while i < ksi8560_pins.len() {
        let pin = &ksi8560_pins[i];
        cpm2_set_pin(pin.port, pin.pin, pin.flags);
        i += 1;
    }
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC1, CPM_BRG1, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_SCC2, CPM_BRG2, CPM_CLK_TX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK9, CPM_CLK_RX);
    cpm2_clk_setup(CPM_CLK_FCC1, CPM_CLK10, CPM_CLK_TX);
}

unsafe fn ksi8560_setup_arch() {
    let cpld = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"emerson,KSI8560-cpld\0".as_ptr() as *const _);
    if !cpld.is_null() {
        cpld_base = of_iomap(cpld, 0);
    } else {
        printk(b"Can't find CPLD in device tree\0".as_ptr() as *const _,);
    }
    of_node_put(cpld);
    if !ppc_md.progress.is_none() { ppc_md.progress.unwrap()(b"ksi8560_setup_arch()\0".as_ptr() as *const _, 0); }
    #[cfg(feature = "CONFIG_CPM2")]
    { cpm2_reset(); init_ioports(); }
}

unsafe fn ksi8560_show_cpuinfo(m: *mut core::ffi::c_void) {
    let pvid = mfspr(SPRN_PVR);
    let svid = mfspr(SPRN_SVR);
    seq_printf(m, b"Vendor\t\t: Emerson Network Power\n\0".as_ptr() as *const _);
    seq_printf(m, b"Board\t\t: KSI8560\n\0".as_ptr() as *const _);
    if !cpld_base.is_null() {
        seq_printf(m, b"Hardware rev\t: %d\n\0".as_ptr() as *const _, in_8(cpld_base.add(KSI8560_CPLD_HVR) as *mut u8));
        seq_printf(m, b"CPLD rev\t: %d\n\0".as_ptr() as *const _, in_8(cpld_base.add(KSI8560_CPLD_PVR) as *mut u8));
    } else { seq_printf(m, b"Unknown Hardware and CPLD revs\n\0".as_ptr() as *const _); }
    seq_printf(m, b"PVR\t\t: 0x%x\n\0".as_ptr() as *const _, pvid);
    seq_printf(m, b"SVR\t\t: 0x%x\n\0".as_ptr() as *const _, svid);
    let phid1 = mfspr(SPRN_HID1);
    seq_printf(m, b"PLL setting\t: 0x%x\n\0".as_ptr() as *const _, (phid1 >> 24) & 0x3f);
}

// machine_device_initcall(ksi8560, mpc85xx_common_publish_devices);
// define_machine(ksi8560) { .name = "KSI8560", .compatible = "emerson,KSI8560", ... };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
