/*
 * linux/arch/m68k/amiga/amisound.c
 *
 * amiga sound driver for Linux/m68k
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Linux kernel and Amiga hardware declarations are supplied by the surrounding
// translation unit.

use core::mem::size_of;

extern "C" {
    static mut amiga_custom: AmigaCustom;
    static mut amiga_audio_min_period: u16;
    static mut amiga_audio_period: u16;
    static amiga_colorclock: c_ulong;
    static mut jiffies: c_ulong;

    fn amiga_chip_alloc_res(size: usize, resource: *mut resource) -> *mut u16;
    fn amifb_video_off();
    fn pr_crit(message: *const u8, ...);
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn timer_delete(timer: *mut timer_list);
    fn add_timer(timer: *mut timer_list);
}

type c_ulong = usize;

#[repr(C)]
pub struct resource {
    pub name: *const u8,
}

#[repr(C)]
pub struct timer_list {
    pub expires: c_ulong,
}

// The complete hardware definitions are provided by asm/amigahw.h.
#[repr(C)]
pub struct AudioChannel {
    pub audlc: *mut u16,
    pub audlen: u16,
    pub audper: u16,
    pub audvol: u16,
}

#[repr(C)]
pub struct AmigaCustom {
    pub aud: [AudioChannel; 4],
    pub dmacon: u16,
}

static mut snd_data: *mut u16 = core::ptr::null_mut();
static sine_data: [i8; 20] = [
    0, 39, 75, 103, 121, 127, 121, 103, 75, 39,
    0, -39, -75, -103, -121, -127, -121, -103, -75, -39,
];
const DATA_SIZE: usize = sine_data.len();

const MAX_PERIOD: u32 = 65535;

/* The minimum period may be modified by the frame-buffer device. */
#[no_mangle]
pub static mut AMIGA_AUDIO_MIN_PERIOD: u16 = 124;

/* Current period (set by dmasound.c). */
#[no_mangle]
pub static mut AMIGA_AUDIO_PERIOD: u16 = MAX_PERIOD as u16;

static mut clock_constant: c_ulong = 0;

// DEFINE_TIMER(sound_timer, nosound)
static mut sound_timer: timer_list = timer_list { expires: 0 };

pub unsafe fn amiga_init_sound() {
    let mut beep_res = resource {
        name: b"Beep\0".as_ptr(),
    };

    snd_data = amiga_chip_alloc_res(size_of::<[i8; 20]>(), &mut beep_res);
    if snd_data.is_null() {
        pr_crit(b"amiga init_sound: failed to allocate chipmem\n\0".as_ptr());
        return;
    }
    core::ptr::copy_nonoverlapping(
        sine_data.as_ptr() as *const u16,
        snd_data,
        size_of::<[i8; 20]>() / size_of::<u16>(),
    );

    clock_constant = (amiga_colorclock + DATA_SIZE / 2) / DATA_SIZE;

    // Without CONFIG_FB_AMIGA, turn video off and enable high quality sound.
    #[cfg(not(CONFIG_FB_AMIGA))]
    amifb_video_off();
}

pub unsafe fn amiga_mksound(hz: u32, ticks: u32) {
    let mut flags: c_ulong = 0;

    if snd_data.is_null() {
        return;
    }

    local_irq_save(&mut flags);
    timer_delete(&mut sound_timer);

    if hz > 20 && hz < 32767 {
        let mut period = clock_constant / hz as c_ulong;

        if period < AMIGA_AUDIO_MIN_PERIOD as c_ulong {
            period = AMIGA_AUDIO_MIN_PERIOD as c_ulong;
        }
        if period > MAX_PERIOD as c_ulong {
            period = MAX_PERIOD as c_ulong;
        }

        // Set pointer to data, period, length, and volume.
        amiga_custom.aud[2].audlc = snd_data;
        amiga_custom.aud[2].audlen = (size_of::<[i8; 20]>() / 2) as u16;
        amiga_custom.aud[2].audper = period as u16;
        amiga_custom.aud[2].audvol = 32; // 50% of maxvol

        if ticks != 0 {
            sound_timer.expires = jiffies + ticks as c_ulong;
            add_timer(&mut sound_timer);
        }

        // Turn on DMA for audio channel 2.
        amiga_custom.dmacon = DMAF_SETCLR | DMAF_AUD2;
    } else {
        nosound(core::ptr::null_mut());
    }

    local_irq_restore(flags);
}

unsafe fn nosound(_unused: *mut timer_list) {
    // Turn off DMA for audio channel 2.
    amiga_custom.dmacon = DMAF_AUD2;
    // Restore the period to its previous value after beeping.
    amiga_custom.aud[2].audper = AMIGA_AUDIO_PERIOD;
}

const DMAF_SETCLR: u16 = 0x8000;
const DMAF_AUD2: u16 = 0x0004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
