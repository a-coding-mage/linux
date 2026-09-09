/*
** asm/blinken.h -- m68k blinkenlights support (currently hp300 only)
**
** (c) 1998 Phil Blundell <philb@gnu.org>
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
*/

// Dependencies supplied by the corresponding assembly/platform headers.
extern "C" {
    static mut MACH_IS_HP300: bool;
    static mut hp300_ledstate: u8;
    fn out_8(address: u32, value: u8);
}

pub const HP300_LEDS: u32 = 0xf001_ffff;

pub unsafe extern "C" fn blinken_leds(on: i32, off: i32) {
    if MACH_IS_HP300 {
        hp300_ledstate |= on as u8;
        hp300_ledstate &= !(off as u8);
        out_8(HP300_LEDS, !hp300_ledstate);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
