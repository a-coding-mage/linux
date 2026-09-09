// SPDX-License-Identifier: GPL-2.0
/*
 * Setup code for SH7720, SH7721.
 *
 * Copyright (C) 2007 Markus Brunner, Mark Jonas
 * Copyright (C) 2009 Paul Mundt
 *
 * Based on arch/sh/kernel/cpu/sh4/setup-sh7750.c.
 */

// Kernel headers and build-time configuration are supplied by the surrounding tree.

static mut RTC_RESOURCES: [resource; 2] = [
    resource { start: 0xa413fec0, end: 0xa413fec0 + 0x28 - 1, flags: IORESOURCE_IO, ..resource::default() },
    resource { start: evt2irq(0x480), flags: IORESOURCE_IRQ, ..resource::default() },
];

static mut RTC_INFO: sh_rtc_platform_info = sh_rtc_platform_info {
    capabilities: RTC_CAP_4_DIGIT_YEAR,
    ..sh_rtc_platform_info::default()
};

static mut RTC_DEVICE: platform_device = platform_device {
    name: "sh-rtc",
    id: -1,
    num_resources: ARRAY_SIZE(RTC_RESOURCES),
    resource: RTC_RESOURCES.as_mut_ptr(),
    dev: device { platform_data: &mut RTC_INFO as *mut _ as *mut core::ffi::c_void, ..device::default() },
    ..platform_device::default()
};

static mut SCIF0_PLATFORM_DATA: plat_sci_port = plat_sci_port {
    type_: PORT_SCIF, ops: &sh7720_sci_port_ops, regtype: SCIx_SH7705_SCIF_REGTYPE,
    ..plat_sci_port::default()
};
static mut SCIF0_RESOURCES: [resource; 2] = [
    DEFINE_RES_MEM!(0xa4430000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xc00)),
];
static mut SCIF0_DEVICE: platform_device = platform_device {
    name: "sh-sci", id: 0, resource: SCIF0_RESOURCES.as_mut_ptr(),
    num_resources: ARRAY_SIZE(SCIF0_RESOURCES),
    dev: device { platform_data: &mut SCIF0_PLATFORM_DATA as *mut _ as *mut core::ffi::c_void, ..device::default() },
    ..platform_device::default()
};

static mut SCIF1_PLATFORM_DATA: plat_sci_port = plat_sci_port {
    type_: PORT_SCIF, ops: &sh7720_sci_port_ops, regtype: SCIx_SH7705_SCIF_REGTYPE,
    ..plat_sci_port::default()
};
static mut SCIF1_RESOURCES: [resource; 2] = [
    DEFINE_RES_MEM!(0xa4438000, 0x100), DEFINE_RES_IRQ!(evt2irq(0xc20)),
];
static mut SCIF1_DEVICE: platform_device = platform_device {
    name: "sh-sci", id: 1, resource: SCIF1_RESOURCES.as_mut_ptr(),
    num_resources: ARRAY_SIZE(SCIF1_RESOURCES),
    dev: device { platform_data: &mut SCIF1_PLATFORM_DATA as *mut _ as *mut core::ffi::c_void, ..device::default() },
    ..platform_device::default()
};

static mut USB_OHCI_RESOURCES: [resource; 2] = [
    resource { start: 0xA4428000, end: 0xA44280FF, flags: IORESOURCE_MEM, ..resource::default() },
    resource { start: evt2irq(0xa60), end: evt2irq(0xa60), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut USB_OHCI_DMA_MASK: u64 = 0xffffffffu64;
static mut USB_OHCI_PDATA: usb_ohci_pdata = usb_ohci_pdata::default();
static mut USB_OHCI_DEVICE: platform_device = platform_device {
    name: "ohci-platform", id: -1,
    dev: device { dma_mask: &mut USB_OHCI_DMA_MASK, coherent_dma_mask: 0xffffffff, platform_data: &mut USB_OHCI_PDATA as *mut _ as *mut core::ffi::c_void, ..device::default() },
    num_resources: ARRAY_SIZE(USB_OHCI_RESOURCES), resource: USB_OHCI_RESOURCES.as_mut_ptr(), ..platform_device::default()
};

static mut USBF_RESOURCES: [resource; 2] = [
    resource { name: "sh_udc", start: 0xA4420000, end: 0xA44200FF, flags: IORESOURCE_MEM, ..resource::default() },
    resource { name: "sh_udc", start: evt2irq(0xa20), end: evt2irq(0xa20), flags: IORESOURCE_IRQ, ..resource::default() },
];
static mut USBF_DEVICE: platform_device = platform_device {
    name: "sh_udc", id: -1,
    dev: device { dma_mask: core::ptr::null_mut(), coherent_dma_mask: 0xffffffff, ..device::default() },
    num_resources: ARRAY_SIZE(USBF_RESOURCES), resource: USBF_RESOURCES.as_mut_ptr(), ..platform_device::default()
};

static mut CMT_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 0x1f, ..sh_timer_config::default() };
static mut CMT_RESOURCES: [resource; 2] = [DEFINE_RES_MEM!(0x044a0000, 0x60), DEFINE_RES_IRQ!(evt2irq(0xf00))];
static mut CMT_DEVICE: platform_device = platform_device { name: "sh-cmt-32", id: 0, dev: device { platform_data: &mut CMT_PLATFORM_DATA as *mut _ as *mut core::ffi::c_void, ..device::default() }, resource: CMT_RESOURCES.as_mut_ptr(), num_resources: ARRAY_SIZE(CMT_RESOURCES), ..platform_device::default() };

static mut TMU0_PLATFORM_DATA: sh_timer_config = sh_timer_config { channels_mask: 7, ..sh_timer_config::default() };
static mut TMU0_RESOURCES: [resource; 4] = [DEFINE_RES_MEM!(0xa412fe90, 0x28), DEFINE_RES_IRQ!(evt2irq(0x400)), DEFINE_RES_IRQ!(evt2irq(0x420)), DEFINE_RES_IRQ!(evt2irq(0x440))];
static mut TMU0_DEVICE: platform_device = platform_device { name: "sh-tmu-sh3", id: 0, dev: device { platform_data: &mut TMU0_PLATFORM_DATA as *mut _ as *mut core::ffi::c_void, ..device::default() }, resource: TMU0_RESOURCES.as_mut_ptr(), num_resources: ARRAY_SIZE(TMU0_RESOURCES), ..platform_device::default() };

static mut SH7720_DEVICES: [*mut platform_device; 7] = [&mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut CMT_DEVICE, &mut TMU0_DEVICE, &mut RTC_DEVICE, &mut USB_OHCI_DEVICE, &mut USBF_DEVICE];

unsafe fn sh7720_devices_setup() -> i32 {
    platform_add_devices(SH7720_DEVICES.as_mut_ptr(), ARRAY_SIZE(SH7720_DEVICES))
}

// arch_initcall(sh7720_devices_setup)
static mut SH7720_EARLY_DEVICES: [*mut platform_device; 4] = [&mut SCIF0_DEVICE, &mut SCIF1_DEVICE, &mut CMT_DEVICE, &mut TMU0_DEVICE];

unsafe fn plat_early_device_setup() {
    sh_early_platform_add_devices(SH7720_EARLY_DEVICES.as_mut_ptr(), ARRAY_SIZE(SH7720_EARLY_DEVICES));
}

#[repr(usize)]
enum InterruptSource { UNUSED = 0, TMU0, TMU1, TMU2, RTC, WDT, REF_RCMI, SIM, IRQ0, IRQ1, IRQ2, IRQ3, USBF_SPD, TMU_SUNI, IRQ5, IRQ4, DMAC1, LCDC, SSL, ADC, DMAC2, USBFI, CMT, SCIF0, SCIF1, PINT07, PINT815, TPU, IIC, SIOF0, SIOF1, MMC, PCC, USBHI, AFEIF, H_UDI }

static mut VECTORS: [intc_vect; 43] = [
    INTC_VECT!(TMU0, 0x400), INTC_VECT!(TMU1, 0x420), INTC_VECT!(TMU2, 0x440), INTC_VECT!(RTC, 0x480), INTC_VECT!(RTC, 0x4a0), INTC_VECT!(RTC, 0x4c0),
    INTC_VECT!(SIM, 0x4e0), INTC_VECT!(SIM, 0x500), INTC_VECT!(SIM, 0x520), INTC_VECT!(SIM, 0x540), INTC_VECT!(WDT, 0x560), INTC_VECT!(REF_RCMI, 0x580), INTC_VECT!(TMU_SUNI, 0x6c0), INTC_VECT!(USBF_SPD, 0x6e0), INTC_VECT!(DMAC1, 0x800), INTC_VECT!(DMAC1, 0x820), INTC_VECT!(DMAC1, 0x840), INTC_VECT!(DMAC1, 0x860), INTC_VECT!(LCDC, 0x900),
    // CONFIG_CPU_SUBTYPE_SH7720 includes the following vector.
    INTC_VECT!(SSL, 0x980),
    INTC_VECT!(USBFI, 0xa20), INTC_VECT!(USBFI, 0xa40), INTC_VECT!(USBHI, 0xa60), INTC_VECT!(DMAC2, 0xb80), INTC_VECT!(DMAC2, 0xba0), INTC_VECT!(ADC, 0xbe0), INTC_VECT!(SCIF0, 0xc00), INTC_VECT!(SCIF1, 0xc20), INTC_VECT!(PINT07, 0xc80), INTC_VECT!(PINT815, 0xca0), INTC_VECT!(SIOF0, 0xd00), INTC_VECT!(SIOF1, 0xd20), INTC_VECT!(TPU, 0xd80), INTC_VECT!(TPU, 0xda0), INTC_VECT!(TPU, 0xdc0), INTC_VECT!(TPU, 0xde0), INTC_VECT!(IIC, 0xe00), INTC_VECT!(MMC, 0xe80), INTC_VECT!(MMC, 0xea0), INTC_VECT!(MMC, 0xec0), INTC_VECT!(MMC, 0xee0), INTC_VECT!(CMT, 0xf00), INTC_VECT!(PCC, 0xf60), INTC_VECT!(AFEIF, 0xfe0),
];

static mut PRIO_REGISTERS: [intc_prio_reg; 10] = [
    intc_prio_reg { set: 0xA414FEE2, first: 0, width: 16, shift: 4, priorities: [TMU0, TMU1, TMU2, RTC] }, intc_prio_reg { set: 0xA414FEE4, first: 0, width: 16, shift: 4, priorities: [WDT, REF_RCMI, SIM, 0] }, intc_prio_reg { set: 0xA4140016, first: 0, width: 16, shift: 4, priorities: [IRQ3, IRQ2, IRQ1, IRQ0] }, intc_prio_reg { set: 0xA4140018, first: 0, width: 16, shift: 4, priorities: [USBF_SPD, TMU_SUNI, IRQ5, IRQ4] }, intc_prio_reg { set: 0xA414001A, first: 0, width: 16, shift: 4, priorities: [DMAC1, 0, LCDC, SSL] }, intc_prio_reg { set: 0xA4080000, first: 0, width: 16, shift: 4, priorities: [ADC, DMAC2, USBFI, CMT] }, intc_prio_reg { set: 0xA4080002, first: 0, width: 16, shift: 4, priorities: [SCIF0, SCIF1, 0, 0] }, intc_prio_reg { set: 0xA4080004, first: 0, width: 16, shift: 4, priorities: [PINT07, PINT815, TPU, IIC] }, intc_prio_reg { set: 0xA4080006, first: 0, width: 16, shift: 4, priorities: [SIOF0, SIOF1, MMC, PCC] }, intc_prio_reg { set: 0xA4080008, first: 0, width: 16, shift: 4, priorities: [0, USBHI, 0, AFEIF] },
];

static mut INTC_DESC: intc_desc = DECLARE_INTC_DESC!("sh7720", VECTORS, PRIO_REGISTERS);

unsafe fn plat_irq_setup() {
    register_intc_controller(&mut INTC_DESC);
    plat_irq_setup_sh3();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
