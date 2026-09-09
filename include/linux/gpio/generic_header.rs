/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

pub const GPIO_GENERIC_BIG_ENDIAN: usize = 1usize << 0;
pub const GPIO_GENERIC_UNREADABLE_REG_SET: usize = 1usize << 1; /* reg_set is unreadable */
pub const GPIO_GENERIC_UNREADABLE_REG_DIR: usize = 1usize << 2; /* reg_dir is unreadable */
pub const GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER: usize = 1usize << 3;
pub const GPIO_GENERIC_READ_OUTPUT_REG_SET: usize = 1usize << 4; /* reg_set stores output value */
pub const GPIO_GENERIC_NO_OUTPUT: usize = 1usize << 5; /* only input */
pub const GPIO_GENERIC_NO_SET_ON_INPUT: usize = 1usize << 6;
pub const GPIO_GENERIC_PINCTRL_BACKEND: usize = 1usize << 7; /* Call pinctrl direction setters */
pub const GPIO_GENERIC_NO_INPUT: usize = 1usize << 8; /* only output */

/*
 * struct gpio_generic_chip_config - Generic GPIO chip configuration data
 * @dev: Parent device of the new GPIO chip (compulsory).
 * @sz: Size (width) of the MMIO registers in bytes, typically 1, 2 or 4.
 * @dat: MMIO address for the register to READ the value of the GPIO lines.
 * @set: MMIO address for the register to SET the value of the GPIO lines.
 * @clr: MMIO address for the register to CLEAR the value of the GPIO lines.
 * @dirout: MMIO address for the register to set the line as OUTPUT.
 * @dirin: MMIO address for the register to set this line as INPUT.
 * @flags: Different flags that will affect the behaviour of the device.
 */
#[repr(C)]
pub struct gpio_generic_chip_config {
    pub dev: *mut device,
    pub sz: ::core::ffi::c_ulong,
    pub dat: *mut ::core::ffi::c_void,
    pub set: *mut ::core::ffi::c_void,
    pub clr: *mut ::core::ffi::c_void,
    pub dirout: *mut ::core::ffi::c_void,
    pub dirin: *mut ::core::ffi::c_void,
    pub flags: ::core::ffi::c_ulong,
}

/* Generic GPIO chip implementation. */
#[repr(C)]
pub struct gpio_generic_chip {
    pub gc: gpio_chip,
    pub read_reg: Option<unsafe extern "C" fn(reg: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong>,
    pub write_reg: Option<unsafe extern "C" fn(reg: *mut ::core::ffi::c_void, data: ::core::ffi::c_ulong)>,
    pub be_bits: bool,
    pub reg_dat: *mut ::core::ffi::c_void,
    pub reg_set: *mut ::core::ffi::c_void,
    pub reg_clr: *mut ::core::ffi::c_void,
    pub reg_dir_out: *mut ::core::ffi::c_void,
    pub reg_dir_in: *mut ::core::ffi::c_void,
    pub dir_unreadable: bool,
    pub pinctrl: bool,
    pub bits: ::core::ffi::c_int,
    pub lock: raw_spinlock_t,
    pub sdata: ::core::ffi::c_ulong,
    pub sdir: ::core::ffi::c_ulong,
}

pub unsafe fn to_gpio_generic_chip(gc: *mut gpio_chip) -> *mut gpio_generic_chip {
    container_of!(gc, gpio_generic_chip, gc)
}

unsafe extern "C" {
    pub fn gpio_generic_chip_init(
        chip: *mut gpio_generic_chip,
        cfg: *const gpio_generic_chip_config,
    ) -> ::core::ffi::c_int;
}

pub unsafe fn gpio_generic_chip_set(
    chip: *mut gpio_generic_chip,
    offset: ::core::ffi::c_uint,
    value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if WARN_ON((*chip).gc.set.is_none()) {
        return -EOPNOTSUPP;
    }
    ((*chip).gc.set.unwrap())(&mut (*chip).gc, offset, value)
}

pub unsafe fn gpio_generic_read_reg(
    chip: *mut gpio_generic_chip,
    reg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_ulong {
    if WARN_ON((*chip).read_reg.is_none()) {
        return 0;
    }
    ((*chip).read_reg.unwrap())(reg)
}

pub unsafe fn gpio_generic_write_reg(
    chip: *mut gpio_generic_chip,
    reg: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_ulong,
) {
    if WARN_ON((*chip).write_reg.is_none()) {
        return;
    }
    ((*chip).write_reg.unwrap())(reg, val);
}

#[inline]
pub unsafe fn gpio_generic_chip_lock(gen_gc: *mut gpio_generic_chip) {
    raw_spin_lock(&mut (*gen_gc).lock);
}

#[inline]
pub unsafe fn gpio_generic_chip_unlock(gen_gc: *mut gpio_generic_chip) {
    raw_spin_unlock(&mut (*gen_gc).lock);
}

#[inline]
pub unsafe fn gpio_generic_chip_lock_irqsave(
    gen_gc: *mut gpio_generic_chip,
    flags: ::core::ffi::c_ulong,
) {
    raw_spin_lock_irqsave(&mut (*gen_gc).lock, flags);
}

#[inline]
pub unsafe fn gpio_generic_chip_unlock_irqrestore(
    gen_gc: *mut gpio_generic_chip,
    flags: ::core::ffi::c_ulong,
) {
    raw_spin_unlock_irqrestore(&mut (*gen_gc).lock, flags);
}

/* DEFINE_LOCK_GUARD_1(gpio_generic_lock, struct gpio_generic_chip, ...)
 * DEFINE_LOCK_GUARD_1(gpio_generic_lock_irqsave, struct gpio_generic_chip, ...,
 *                     unsigned long flags)
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
