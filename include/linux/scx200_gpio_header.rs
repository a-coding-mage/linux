/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn scx200_gpio_configure(index: u32, set: u32, clear: u32) -> u32;

    pub static mut scx200_gpio_base: u32;
    pub static mut scx200_gpio_shadow: [core::ffi::c_ulong; 2];
    pub static mut scx200_gpio_ops: nsc_gpio_ops;
}

/* Defined by the surrounding dependency. */
#[allow(non_camel_case_types)]
pub type nsc_gpio_ops = core::ffi::c_void;

#[inline]
pub fn scx200_gpio_present() -> bool {
    unsafe { scx200_gpio_base != 0 }
}

/* Definitions to make sure I do the same thing in all functions */

/* The following operations are supplied by the surrounding kernel code. */
extern "C" {
    fn inl(ioaddr: u16) -> u32;
    fn set_bit(index: u32, address: *mut core::ffi::c_ulong);
    fn clear_bit(index: u32, address: *mut core::ffi::c_ulong);
    fn change_bit(index: u32, address: *mut core::ffi::c_ulong);
}

#[inline]
pub unsafe fn scx200_gpio_get(mut index: u32) -> i32 {
    let bank: u32 = index >> 5;
    let ioaddr: u16 = (scx200_gpio_base + 0x10 * bank + 0x04) as u16;
    index &= 31;

    if (inl(ioaddr) & (1u32 << index)) != 0 { 1 } else { 0 }
}

/* return the value driven on the GPIO signal (the value that will be
   driven if the GPIO is configured as an output, it might not be the
   state of the GPIO right now if the GPIO is configured as an input) */
#[inline]
pub unsafe fn scx200_gpio_current(mut index: u32) -> i32 {
    let bank: u32 = index >> 5;
    index &= 31;

    if (scx200_gpio_shadow[bank as usize] & ((1 as core::ffi::c_ulong) << index)) != 0 { 1 } else { 0 }
}

/* The C __SCx200_GPIO_OUT macro performs an x86 outsl volatile inline
   assembly operation.  It remains an explicit external operation here. */
extern "C" {
    fn __scx200_gpio_out(ioaddr: u16, shadow: *mut core::ffi::c_ulong);
}

#[inline]
pub unsafe fn scx200_gpio_set_high(mut index: u32) {
    let bank: u32 = index >> 5;
    let ioaddr: u16 = (scx200_gpio_base + 0x10 * bank) as u16;
    let shadow: *mut core::ffi::c_ulong = scx200_gpio_shadow.as_mut_ptr().add(bank as usize);
    index &= 31;
    set_bit(index, shadow);
    __scx200_gpio_out(ioaddr, shadow);
}

/* drive the GPIO signal low */
#[inline]
pub unsafe fn scx200_gpio_set_low(mut index: u32) {
    let bank: u32 = index >> 5;
    let ioaddr: u16 = (scx200_gpio_base + 0x10 * bank) as u16;
    let shadow: *mut core::ffi::c_ulong = scx200_gpio_shadow.as_mut_ptr().add(bank as usize);
    index &= 31;
    clear_bit(index, shadow);
    __scx200_gpio_out(ioaddr, shadow);
}

/* drive the GPIO signal to state */
#[inline]
pub unsafe fn scx200_gpio_set(mut index: u32, state: i32) {
    let bank: u32 = index >> 5;
    let ioaddr: u16 = (scx200_gpio_base + 0x10 * bank) as u16;
    let shadow: *mut core::ffi::c_ulong = scx200_gpio_shadow.as_mut_ptr().add(bank as usize);
    index &= 31;
    if state != 0 { set_bit(index, shadow); } else { clear_bit(index, shadow); }
    __scx200_gpio_out(ioaddr, shadow);
}

/* toggle the GPIO signal */
#[inline]
pub unsafe fn scx200_gpio_change(mut index: u32) {
    let bank: u32 = index >> 5;
    let ioaddr: u16 = (scx200_gpio_base + 0x10 * bank) as u16;
    let shadow: *mut core::ffi::c_ulong = scx200_gpio_shadow.as_mut_ptr().add(bank as usize);
    index &= 31;
    change_bit(index, shadow);
    __scx200_gpio_out(ioaddr, shadow);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
