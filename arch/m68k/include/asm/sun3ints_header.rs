/*
 * sun3ints.h -- Linux/Sun3 interrupt handling code definitions
 *
 * Erik Verbruggen (erik@bigmama.xtdnet.nl)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive for
 * more details.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, linux/interrupt.h, asm/intersil.h, asm/oplib.h,
// asm/traps.h, and asm/irq.h.

pub const SUN3_INT_VECS: u32 = 192;

unsafe extern "C" {
    pub fn sun3_enable_irq(irq: u32);
    pub fn sun3_disable_irq(irq: u32);
    pub fn sun3_init_IRQ();
    pub fn sun3_enable_interrupts();
    pub fn sun3_disable_interrupts();
    pub static mut sun3_intreg: *mut u8;
}

/* master list of VME vectors -- don't fuck with this */
pub const SUN3_VEC_FLOPPY: u32 = IRQ_USER + 0;
pub const SUN3_VEC_VMESCSI0: u32 = IRQ_USER + 0;
pub const SUN3_VEC_VMESCSI1: u32 = IRQ_USER + 1;
pub const SUN3_VEC_CG: u32 = IRQ_USER + 104;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
