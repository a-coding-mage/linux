/*
 * linux/arch/m68k/atari/atasound.c
 *
 * ++Geert: Moved almost all stuff to linux/drivers/sound/
 *
 * The author of atari_nosound, atari_mksound and atari_microwire_cmd is
 * unknown. (++roman: That's me... :-)
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 * 1998-05-31 ++andreas: atari_mksound rewritten to always use the envelope,
 *                         no timer, atari_nosound removed.
 */

// Declarations supplied by the Linux Atari platform headers.
#[repr(C)]
pub struct AtariMicrowire {
    pub mask: u16,
    pub data: u16,
}

#[repr(C)]
pub struct SoundYm {
    pub rd_data_reg_sel: u8,
    pub wd_data: u8,
}

extern "C" {
    pub static mut tt_microwire: AtariMicrowire;
    pub static mut sound_ym: SoundYm;
    pub fn local_irq_save(flags: *mut usize);
    pub fn local_irq_restore(flags: usize);
}

// Supplied by the Atari hardware definitions.
pub const MW_LM1992_ADDR: u16 = 0;
pub const HZ: usize = 0;

/*
 * stuff from the old atasound.c
 */

pub unsafe fn atari_microwire_cmd(cmd: i32) {
    tt_microwire.mask = 0x7ff;
    tt_microwire.data = MW_LM1992_ADDR | cmd as u16;

    /* Busy wait for data being completely sent :-( */
    while tt_microwire.mask != 0x7ff {}
}

/* PSG base frequency */
pub const PSG_FREQ: u32 = 125000;
/* PSG envelope base frequency times 10 */
pub const PSG_ENV_FREQ_10: u32 = 78125;

pub unsafe fn atari_mksound(hz: u32, ticks: u32) {
    /* Generates sound of some frequency for some number of clock
       ticks.  */
    let mut flags: usize = 0;
    let mut tmp: u8;
    let mut period: u32;

    local_irq_save(&mut flags);

    /* Disable generator A in mixer control.  */
    sound_ym.rd_data_reg_sel = 7;
    tmp = sound_ym.rd_data_reg_sel;
    tmp |= 0o11;
    sound_ym.wd_data = tmp;

    if hz != 0 {
        /* Convert from frequency value to PSG period value (base
           frequency 125 kHz).  */
        period = PSG_FREQ / hz;

        if period > 0xfff {
            period = 0xfff;
        }

        /* Set generator A frequency to hz.  */
        sound_ym.rd_data_reg_sel = 0;
        sound_ym.wd_data = (period & 0xff) as u8;
        sound_ym.rd_data_reg_sel = 1;
        sound_ym.wd_data = ((period >> 8) & 0xf) as u8;
        if ticks != 0 {
            /* Set length of envelope (max 8 sec).  */
            let mut length = (ticks * PSG_ENV_FREQ_10) / HZ as u32 / 10;

            if length > 0xffff {
                length = 0xffff;
            }
            sound_ym.rd_data_reg_sel = 11;
            sound_ym.wd_data = (length & 0xff) as u8;
            sound_ym.rd_data_reg_sel = 12;
            sound_ym.wd_data = (length >> 8) as u8;
            /* Envelope form: max -> min single.  */
            sound_ym.rd_data_reg_sel = 13;
            sound_ym.wd_data = 0;
            /* Use envelope for generator A.  */
            sound_ym.rd_data_reg_sel = 8;
            sound_ym.wd_data = 0x10;
        } else {
            /* Set generator A level to maximum, no envelope.  */
            sound_ym.rd_data_reg_sel = 8;
            sound_ym.wd_data = 15;
        }
        /* Turn on generator A in mixer control.  */
        sound_ym.rd_data_reg_sel = 7;
        tmp &= !1;
        sound_ym.wd_data = tmp;
    }
    local_irq_restore(flags);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
