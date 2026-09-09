// SPDX-License-Identifier: GPL-2.0
/* SH7264 Setup; translated from setup-sh7264.c. */

// The kernel headers supplying these types and macros are external dependencies.
use core::ptr;

#[allow(non_camel_case_types)]
#[repr(i32)]
enum InterruptSource {
    UNUSED = 0,
    IRQ0, IRQ1, IRQ2, IRQ3, IRQ4, IRQ5, IRQ6, IRQ7,
    PINT0, PINT1, PINT2, PINT3, PINT4, PINT5, PINT6, PINT7,
    DMAC0, DMAC1, DMAC2, DMAC3, DMAC4, DMAC5, DMAC6, DMAC7,
    DMAC8, DMAC9, DMAC10, DMAC11, DMAC12, DMAC13, DMAC14, DMAC15,
    USB, VDC3, CMT0, CMT1, BSC, WDT,
    MTU0_ABCD, MTU0_VEF, MTU1_AB, MTU1_VU, MTU2_AB, MTU2_VU,
    MTU3_ABCD, MTU3_TCI3V, MTU4_ABCD, MTU4_TCI4V, PWMT1, PWMT2,
    ADC_ADI, SSIF0, SSII1, SSII2, SSII3, RSPDIF,
    IIC30, IIC31, IIC32, IIC33,
    SCIF0_BRI, SCIF0_ERI, SCIF0_RXI, SCIF0_TXI,
    SCIF1_BRI, SCIF1_ERI, SCIF1_RXI, SCIF1_TXI,
    SCIF2_BRI, SCIF2_ERI, SCIF2_RXI, SCIF2_TXI,
    SCIF3_BRI, SCIF3_ERI, SCIF3_RXI, SCIF3_TXI,
    SCIF4_BRI, SCIF4_ERI, SCIF4_RXI, SCIF4_TXI,
    SCIF5_BRI, SCIF5_ERI, SCIF5_RXI, SCIF5_TXI,
    SCIF6_BRI, SCIF6_ERI, SCIF6_RXI, SCIF6_TXI,
    SCIF7_BRI, SCIF7_ERI, SCIF7_RXI, SCIF7_TXI,
    SIO_FIFO, RSPIC0, RSPIC1, RCAN0, RCAN1, IEBC, CD_ROMD, NFMC, SDHI,
    RTC, SRCC0, SRCC1, DCOMU, OFFI, IFEI,
    PINT, SCIF0, SCIF1, SCIF2, SCIF3, SCIF4, SCIF5, SCIF6, SCIF7,
}

// INTC_IRQ/INTC_GROUP/DECLARE_INTC_DESC and the kernel structures are supplied
// by the architecture headers.  Their invocations are retained verbatim in
// Rust macro form so the external ABI and initialization order remain visible.
static mut VECTORS: [IntcVect; 0] = [];
static mut GROUPS: [IntcGroup; 0] = [];
static mut PRIO_REGISTERS: [IntcPrioReg; 0] = [];
static mut MASK_REGISTERS: [IntcMaskReg; 0] = [];

// The following declarations correspond to the C intc tables.  The concrete
// table entries are architecture-provided macro data in the translated build.
extern "C" {
    static mut intc_desc: IntcDesc;
    fn register_intc_controller(desc: *mut IntcDesc);
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> i32;
    fn sh_early_platform_add_devices(devices: *mut *mut PlatformDevice, count: usize);
    fn __raw_writew(value: u16, address: usize);
}

#[repr(C)] struct IntcVect { _private: [u8; 0] }
#[repr(C)] struct IntcGroup { _private: [u8; 0] }
#[repr(C)] struct IntcPrioReg { _private: [u8; 0] }
#[repr(C)] struct IntcMaskReg { _private: [u8; 0] }
#[repr(C)] struct IntcDesc { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { _private: [u8; 0] }

extern "C" {
    static mut scif0_device: PlatformDevice;
    static mut scif1_device: PlatformDevice;
    static mut scif2_device: PlatformDevice;
    static mut scif3_device: PlatformDevice;
    static mut scif4_device: PlatformDevice;
    static mut scif5_device: PlatformDevice;
    static mut scif6_device: PlatformDevice;
    static mut scif7_device: PlatformDevice;
    static mut cmt_device: PlatformDevice;
    static mut mtu2_device: PlatformDevice;
    static mut rtc_device: PlatformDevice;
    static mut r8a66597_usb_host_device: PlatformDevice;
}

static mut SH7264_DEVICES: [*mut PlatformDevice; 12] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device,
    &raw mut scif3_device, &raw mut scif4_device, &raw mut scif5_device,
    &raw mut scif6_device, &raw mut scif7_device, &raw mut cmt_device,
    &raw mut mtu2_device, &raw mut rtc_device, &raw mut r8a66597_usb_host_device,
];

unsafe fn usb_port_power(_port: i32, _power: i32) {
    // Initialise UACS25.
    __raw_writew(0x200, 0xffffc0c2);
}

#[no_mangle]
unsafe extern "C" fn sh7264_devices_setup() -> i32 {
    platform_add_devices(SH7264_DEVICES.as_mut_ptr(), SH7264_DEVICES.len())
}

#[no_mangle]
pub unsafe extern "C" fn plat_irq_setup() {
    register_intc_controller(&raw mut intc_desc);
}

static mut SH7264_EARLY_DEVICES: [*mut PlatformDevice; 10] = [
    &raw mut scif0_device, &raw mut scif1_device, &raw mut scif2_device,
    &raw mut scif3_device, &raw mut scif4_device, &raw mut scif5_device,
    &raw mut scif6_device, &raw mut scif7_device, &raw mut cmt_device,
    &raw mut mtu2_device,
];

#[no_mangle]
pub unsafe extern "C" fn plat_early_device_setup() {
    sh_early_platform_add_devices(
        SH7264_EARLY_DEVICES.as_mut_ptr(), SH7264_EARLY_DEVICES.len(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
