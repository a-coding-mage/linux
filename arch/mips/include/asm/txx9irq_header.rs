/*
 * include/asm-mips/txx9irq.h
 * TX39/TX49 interrupt controller definitions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency supplied by the surrounding translation unit: irq.h.

#[cfg(feature = "CONFIG_IRQ_MIPS_CPU")]
pub const TXX9_IRQ_BASE: usize = MIPS_CPU_IRQ_BASE + 8;

#[cfg(all(
    not(feature = "CONFIG_IRQ_MIPS_CPU"),
    feature = "CONFIG_I8259"
))]
pub const TXX9_IRQ_BASE: usize = I8259A_IRQ_BASE + 16;

#[cfg(all(
    not(feature = "CONFIG_IRQ_MIPS_CPU"),
    not(feature = "CONFIG_I8259")
))]
pub const TXX9_IRQ_BASE: usize = 0;

pub const TXx9_MAX_IR: usize = 32;

extern "C" {
    pub fn txx9_irq_init(baseaddr: core::ffi::c_ulong);
    pub fn txx9_irq() -> core::ffi::c_int;
    pub fn txx9_irq_set_pri(
        irc_irq: core::ffi::c_int,
        new_pri: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
