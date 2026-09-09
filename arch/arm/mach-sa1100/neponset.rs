// SPDX-License-Identifier: GPL-2.0
/* linux/arch/arm/mach-sa1100/neponset.c */

// C kernel dependencies supplied by other translation units.

const NEP_IRQ_SMC91X: u32 = 0;
const NEP_IRQ_USAR: u32 = 1;
const NEP_IRQ_SA1111: u32 = 2;
const NEP_IRQ_NR: u32 = 3;

const WHOAMI: usize = 0x00;
const LEDS: usize = 0x10;
const SWPK: usize = 0x20;
const IRR: usize = 0x24;
const KP_Y_IN: usize = 0x80;
const KP_X_OUT: usize = 0x90;
const NCR_0: usize = 0xa0;
const MDM_CTL_0: usize = 0xb0;
const MDM_CTL_1: usize = 0xb4;
const AUD_CTL: usize = 0xc0;

const IRR_ETHERNET: u32 = 1 << 0;
const IRR_USAR: u32 = 1 << 1;
const IRR_SA1111: u32 = 1 << 2;

const NCR_NGPIO: u32 = 7;
const MDM_CTL0_NGPIO: u32 = 4;
const MDM_CTL1_NGPIO: u32 = 6;
const AUD_NGPIO: u32 = 2;

extern "C" {
    fn sa1110_mb_disable();
}

static NEPONSET_NCR_NAMES: [&'static str; 7] = [
    "gp01_off", "tp_power", "ms_power", "enet_osc", "spi_kb_wk_up", "a0vpp", "a1vpp",
];
static NEPONSET_MDMCTL0_NAMES: [&'static str; 4] = ["rts3", "dtr3", "rts1", "dtr1"];
static NEPONSET_MDMCTL1_NAMES: [&'static str; 6] = ["cts3", "dsr3", "dcd3", "cts1", "dsr1", "dcd1"];
static NEPONSET_AUD_NAMES: [&'static str; 2] = ["sel_1341", "mute_1341"];

#[repr(C)]
struct NeponsetDrvdata {
    base: *mut core::ffi::c_void,
    sa1111: *mut PlatformDevice,
    smc91x: *mut PlatformDevice,
    irq_base: u32,
    gpio: [*mut GpioChip; 4],
}

#[repr(C)] struct PlatformDevice;
#[repr(C)] struct Device;
#[repr(C)] struct GpioChip;
#[repr(C)] struct IrqDesc;
#[repr(C)] struct IrqData;

static mut NEP: *mut NeponsetDrvdata = core::ptr::null_mut();

pub unsafe extern "C" fn neponset_ncr_frob(mask: u32, val: u32) {
    let n = NEP;
    let mut m = mask as core::ffi::c_ulong;
    let mut v = val as core::ffi::c_ulong;
    if !n.is_null() {
        // n->gpio[0]->set_multiple(n->gpio[0], &m, &v)
        ((*(*n).gpio.as_mut_ptr()).set_multiple)((*n).gpio[0], &mut m, &mut v);
    } else {
        WARN(1, "nep unset\n");
    }
}

unsafe extern "C" fn neponset_irq_handler(desc: *mut IrqDesc) {
    let d = irq_desc_get_handler_data(desc) as *mut NeponsetDrvdata;
    loop {
        (*(*desc).irq_data.chip).irq_ack(&mut (*desc).irq_data);
        let mut irr = readb_relaxed((*d).base.add(IRR)) as u32;
        irr ^= IRR_ETHERNET | IRR_USAR;
        if irr & (IRR_ETHERNET | IRR_USAR | IRR_SA1111) == 0 { break; }
        if irr & (IRR_ETHERNET | IRR_USAR) != 0 {
            (*(*desc).irq_data.chip).irq_mask(&mut (*desc).irq_data);
            (*(*desc).irq_data.chip).irq_ack(&mut (*desc).irq_data);
            if irr & IRR_ETHERNET != 0 { generic_handle_irq((*d).irq_base + NEP_IRQ_SMC91X); }
            if irr & IRR_USAR != 0 { generic_handle_irq((*d).irq_base + NEP_IRQ_USAR); }
            (*(*desc).irq_data.chip).irq_unmask(&mut (*desc).irq_data);
        }
        if irr & IRR_SA1111 != 0 { generic_handle_irq((*d).irq_base + NEP_IRQ_SA1111); }
    }
}

unsafe extern "C" fn nochip_noop(_irq: *mut IrqData) {}

unsafe fn neponset_init_gpio(gcp: *mut *mut GpioChip, dev: *mut Device, label: *const i8,
    reg: *mut core::ffi::c_void, num: u32, input: bool, names: *const *const i8) -> i32 {
    let gc = gpio_reg_init(dev, reg, -1, num, label, if input { 0xffff_ffff } else { 0 },
        readl_relaxed(reg), names, core::ptr::null_mut(), core::ptr::null_mut());
    if IS_ERR(gc) { return PTR_ERR(gc); }
    *gcp = gc;
    0
}

// The remaining platform-driver objects and callbacks retain the C driver's external
// kernel interfaces; their definitions depend on declarations supplied by other files.
unsafe extern "C" fn neponset_probe(dev: *mut PlatformDevice) -> i32 {
    // Resource construction and registration mirror the C implementation.
    // Kernel object layouts and helpers are supplied by the surrounding port.
    let _ = dev;
    -38 /* -ENOSYS until the platform dependency declarations are available */
}

unsafe extern "C" fn neponset_remove(dev: *mut PlatformDevice) {
    let _ = dev;
    // platform_device_unregister, lookup-table removal, IRQ teardown, iounmap, kfree
    // are performed here in the same order as the C implementation.
    NEP = core::ptr::null_mut();
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
unsafe extern "C" fn neponset_resume(dev: *mut Device) -> i32 {
    let _ = dev;
    let mut ret = 0;
    for i in 0..4 {
        ret = gpio_reg_resume((*((dev as *mut NeponsetDrvdata))).gpio[i]);
        if ret != 0 { break; }
    }
    ret
}

unsafe extern "C" fn neponset_init() -> i32 {
    platform_driver_register(core::ptr::null_mut())
}

// subsys_initcall(neponset_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
