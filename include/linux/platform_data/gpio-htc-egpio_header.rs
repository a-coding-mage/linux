/* SPDX-License-Identifier: GPL-2.0 */
/*
 * HTC simple EGPIO irq and gpio extender
 */

/* Descriptive values for all-in or all-out htc_egpio_chip descriptors. */
pub const HTC_EGPIO_OUTPUT: usize = !0;
pub const HTC_EGPIO_INPUT: usize = 0;

/**
 * struct htc_egpio_chip - descriptor to create gpio_chip for register range
 * @reg_start: index of first register
 * @gpio_base: gpio number of first pin in this register range
 * @num_gpios: number of gpios in this register range, max BITS_PER_LONG
 *    (number of registers = DIV_ROUND_UP(num_gpios, reg_width))
 * @direction: bitfield, '0' = input, '1' = output,
 */
#[repr(C)]
pub struct htc_egpio_chip {
    pub reg_start: i32,
    pub gpio_base: i32,
    pub num_gpios: i32,
    pub direction: usize,
    pub initial_values: usize,
}

/**
 * struct htc_egpio_platform_data - description provided by the arch
 * @irq_base: beginning of available IRQs (eg, IRQ_BOARD_START)
 * @num_irqs: number of irqs
 * @reg_width: number of bits per register, either 8 or 16 bit
 * @bus_width: alignment of the registers, either 16 or 32 bit
 * @invert_acks: set if chip requires writing '0' to ack an irq, instead of '1'
 * @ack_register: location of the irq/ack register
 * @chip: pointer to array of htc_egpio_chip descriptors
 * @num_chips: number of egpio chip descriptors
 */
#[repr(C)]
pub struct htc_egpio_platform_data {
    pub bus_width: i32,
    pub reg_width: i32,
    pub irq_base: i32,
    pub num_irqs: i32,
    pub invert_acks: i32,
    pub ack_register: i32,
    pub chip: *mut htc_egpio_chip,
    pub num_chips: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
