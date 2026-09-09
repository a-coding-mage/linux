/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 *  Machine specific IO port address definition for generic.
 *  Written by Osamu Tomita <tomita@cinet.co.jp>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/* i8253A PIT registers */
pub const PIT_MODE: u32 = 0x43;
pub const PIT_CH0: u32 = 0x40;
pub const PIT_CH2: u32 = 0x42;

pub const PIT_LATCH: u32 = (PIT_TICK_RATE + HZ / 2) / HZ;

unsafe extern "C" {
    pub static mut i8253_lock: raw_spinlock_t;
    pub static mut i8253_clockevent: clock_event_device;
    pub fn clockevent_i8253_init(oneshot: bool);
    pub fn clockevent_i8253_disable();
    pub fn setup_pit_timer();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
