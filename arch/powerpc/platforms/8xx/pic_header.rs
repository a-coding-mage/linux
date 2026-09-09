// Declarations corresponding to the C header's Linux interrupt dependencies.

unsafe extern "C" {
    pub fn mpc8xx_pic_init();
    pub fn mpc8xx_get_irq() -> u32;
}

/*
 * Some internal interrupt registers use an 8-bit mask for the interrupt
 * level instead of a number.
 */
#[inline]
pub fn mk_int_int_mask(mask: u32) -> u32 {
    1u32.wrapping_shl(7u32.wrapping_sub(mask / 2))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
