/*
 * include/asm-mips/txx9pio.h
 * TX39/TX49 PIO controller definitions.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

#[repr(C)]
pub struct Txx9PioReg {
    pub dout: u32,
    pub din: u32,
    pub dir: u32,
    pub od: u32,
    pub flag: [u32; 2],
    pub pol: u32,
    pub intc: u32,
    pub maskcpu: u32,
    pub maskext: u32,
}

unsafe extern "C" {
    pub fn txx9_gpio_init(baseaddr: usize, num: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
