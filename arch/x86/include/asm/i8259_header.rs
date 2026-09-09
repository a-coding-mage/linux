/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/delay.h and asm/io.h.

unsafe extern "C" {
    pub static mut cached_irq_mask: ::core::ffi::c_uint;

    pub static mut i8259A_lock: raw_spinlock_t;
    pub static mut i8259A_chip: irq_chip;
    pub static mut legacy_pic: *mut legacy_pic;
    pub static mut null_legacy_pic: legacy_pic;

    pub fn inb(port: ::core::ffi::c_uint) -> ::core::ffi::c_uchar;
    pub fn outb(value: ::core::ffi::c_uchar, port: ::core::ffi::c_uint);
    pub fn udelay(usecs: ::core::ffi::c_uint);
}

#[macro_export]
macro_rules! __byte {
    ($x:expr, $y:expr) => {
        unsafe {
            *((&mut $y as *mut _ as *mut ::core::ffi::c_uchar).add($x as usize))
        }
    };
}

#[macro_export]
macro_rules! cached_master_mask {
    () => {
        $crate::__byte!(0, $crate::cached_irq_mask)
    };
}

#[macro_export]
macro_rules! cached_slave_mask {
    () => {
        $crate::__byte!(1, $crate::cached_irq_mask)
    };
}

/* i8259A PIC registers */
pub const PIC_MASTER_CMD: ::core::ffi::c_uint = 0x20;
pub const PIC_MASTER_IMR: ::core::ffi::c_uint = 0x21;
pub const PIC_MASTER_ISR: ::core::ffi::c_uint = PIC_MASTER_CMD;
pub const PIC_MASTER_POLL: ::core::ffi::c_uint = PIC_MASTER_ISR;
pub const PIC_MASTER_OCW3: ::core::ffi::c_uint = PIC_MASTER_ISR;
pub const PIC_SLAVE_CMD: ::core::ffi::c_uint = 0xa0;
pub const PIC_SLAVE_IMR: ::core::ffi::c_uint = 0xa1;
pub const PIC_ELCR1: ::core::ffi::c_uint = 0x4d0;
pub const PIC_ELCR2: ::core::ffi::c_uint = 0x4d1;

/* i8259A PIC related value */
pub const PIC_CASCADE_IR: ::core::ffi::c_uint = 2;
pub const MASTER_ICW4_DEFAULT: ::core::ffi::c_uint = 0x01;
pub const SLAVE_ICW4_DEFAULT: ::core::ffi::c_uint = 0x01;
pub const PIC_ICW4_AEOI: ::core::ffi::c_uint = 2;

/* the PIC may need a careful delay on some platforms, hence specific calls */
#[inline]
pub unsafe fn inb_pic(port: ::core::ffi::c_uint) -> ::core::ffi::c_uchar {
    let value = unsafe { inb(port) };

    /*
     * delay for some accesses to PIC on motherboard or in chipset
     * must be at least one microsecond, so be safe here:
     */
    unsafe { udelay(2) };

    value
}

#[inline]
pub unsafe fn outb_pic(value: ::core::ffi::c_uchar, port: ::core::ffi::c_uint) {
    unsafe { outb(value, port) };
    /*
     * delay for some accesses to PIC on motherboard or in chipset
     * must be at least one microsecond, so be safe here:
     */
    unsafe { udelay(2) };
}

#[repr(C)]
pub struct legacy_pic {
    pub nr_legacy_irqs: ::core::ffi::c_int,
    pub chip: *mut irq_chip,
    pub mask: Option<unsafe extern "C" fn(irq: ::core::ffi::c_uint)>,
    pub unmask: Option<unsafe extern "C" fn(irq: ::core::ffi::c_uint)>,
    pub mask_all: Option<unsafe extern "C" fn()>,
    pub restore_mask: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn(auto_eoi: ::core::ffi::c_int)>,
    pub probe: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub irq_pending:
        Option<unsafe extern "C" fn(irq: ::core::ffi::c_uint) -> ::core::ffi::c_int>,
    pub make_irq: Option<unsafe extern "C" fn(irq: ::core::ffi::c_uint)>,
}

pub unsafe extern "C" fn legacy_pic_pcat_compat();

#[inline]
pub unsafe fn has_legacy_pic() -> bool {
    unsafe { legacy_pic != &raw mut null_legacy_pic }
}

#[inline]
pub unsafe fn nr_legacy_irqs() -> ::core::ffi::c_int {
    unsafe { (*legacy_pic).nr_legacy_irqs }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
