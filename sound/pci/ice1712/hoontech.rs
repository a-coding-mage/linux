// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for Hoontech STDSP24
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

/* Dependencies from the original C source:
 * <linux/delay.h>, <linux/interrupt.h>, <linux/init.h>, <linux/slab.h>,
 * <linux/mutex.h>, <sound/core.h>, "ice1712.h", and "hoontech.h".
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

const ENOMEM: c_int = 12;

/* Hoontech-specific setting */
#[repr(C)]
pub struct hoontech_spec {
    pub boxbits: [c_uchar; 4],
    pub config: c_uint,
    pub boxconfig: [u16; 4],
}

#[repr(C)]
pub struct snd_ice1712_gpio {
    pub direction: c_uchar,
    pub write_mask: c_uchar,
}

#[repr(C)]
pub struct snd_ice1712_eeprom {
    pub gpiomask: c_uchar,
    pub gpiodir: c_uchar,
    pub gpiostate: c_uchar,
}

#[repr(C)]
pub struct snd_ice1712 {
    pub num_total_dacs: c_uint,
    pub num_total_adcs: c_uint,
    pub spec: *mut hoontech_spec,
    pub gpio_mutex: c_void,
    pub gpio: snd_ice1712_gpio,
    pub eeprom: snd_ice1712_eeprom,
    pub akm: *mut snd_akm4xxx,
    pub akm_codecs: c_uint,
}

#[repr(C)]
pub struct snd_akm4xxx_ops {
    pub lock: Option<unsafe extern "C" fn(*mut snd_akm4xxx, c_int)>,
}

#[repr(C)]
pub struct snd_akm4xxx {
    pub num_adcs: c_uint,
    pub num_dacs: c_uint,
    pub type_: c_uint,
    pub ops: snd_akm4xxx_ops,
    pub private_data: [*mut c_void; 1],
}

#[repr(C)]
pub struct snd_ak4xxx_private {
    pub caddr: c_uint,
    pub cif: c_uint,
    pub data_mask: c_uchar,
    pub clk_mask: c_uchar,
    pub cs_mask: c_uchar,
    pub cs_addr: c_uchar,
    pub cs_none: c_uchar,
    pub add_flags: c_uint,
}

#[repr(C)]
pub struct snd_ice1712_card_info {
    pub subvendor: c_uint,
    pub name: *const c_char,
    pub model: *const c_char,
    pub chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    pub mpu401_1_name: *const c_char,
    pub mpu401_2_name: *const c_char,
}

unsafe extern "C" {
    static ICE1712_STDSP24_CLOCK_BIT: c_uchar;
    static ICE1712_IREG_GPIO_DATA: c_uint;
    static ICE1712_IREG_GPIO_DIRECTION: c_uint;
    static ICE1712_IREG_GPIO_WRITE_MASK: c_uint;
    static ICE1712_STDSP24_MUTE: c_uint;
    static ICE1712_STDSP24_INSEL: c_uint;
    static ICE1712_STDSP24_DAREAR: c_uint;
    static ICE1712_STDSP24_BOX_CHN1: u16;
    static ICE1712_STDSP24_BOX_CHN2: u16;
    static ICE1712_STDSP24_BOX_CHN3: u16;
    static ICE1712_STDSP24_BOX_CHN4: u16;
    static ICE1712_STDSP24_BOX_MIDI1: u16;
    static ICE1712_STDSP24_BOX_MIDI2: u16;
    static ICE1712_STDSP24_SERIAL_DATA: c_uchar;
    static ICE1712_STDSP24_SERIAL_CLOCK: c_uchar;
    static ICE1712_STDSP24_AK4524_CS: c_uchar;
    static SND_AK4524: c_uint;
    static ICE1712_SUBDEVICE_STDSP24: c_uint;
    static ICE1712_SUBDEVICE_STDSP24_VALUE: c_uint;
    static ICE1712_SUBDEVICE_STDSP24_MEDIA7_1: c_uint;
    static ICE1712_SUBDEVICE_EVENT_EZ8: c_uint;
    static ICE1712_SUBDEVICE_STAUDIO_ADCIII: c_uint;

    fn udelay(usecs: c_uint);
    fn mdelay(msecs: c_uint);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn snd_ice1712_write(ice: *mut snd_ice1712, reg: c_uint, value: c_uchar);
    fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_akm4xxx_init(
        ak: *mut snd_akm4xxx,
        template: *const snd_akm4xxx,
        priv_: *const snd_ak4xxx_private,
        ice: *mut snd_ice1712,
    ) -> c_int;
    fn snd_ice1712_akm4xxx_build_controls(ice: *mut snd_ice1712) -> c_int;

    fn ICE1712_STDSP24_SET_ADDR(boxbits: *mut c_uchar, addr: c_int);
    fn ICE1712_STDSP24_CLOCK(boxbits: *mut c_uchar, addr: c_int, val: c_int);
    fn ICE1712_STDSP24_0_BOX(boxbits: *mut c_uchar, box_: c_int);
    fn ICE1712_STDSP24_0_DAREAR(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_1_CHN1(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_1_CHN2(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_1_CHN3(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_2_CHN4(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_2_MIDIIN(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_2_MIDI1(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_3_MIDI2(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_3_MUTE(boxbits: *mut c_uchar, activate: c_int);
    fn ICE1712_STDSP24_3_INSEL(boxbits: *mut c_uchar, activate: c_int);
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    unsafe { kzalloc(core::mem::size_of::<T>(), 0) as *mut T }
}

unsafe fn kmalloc_obj<T>() -> *mut T {
    unsafe { kmalloc(core::mem::size_of::<T>(), 0) as *mut T }
}

unsafe fn snd_ice1712_stdsp24_gpio_write(ice: *mut snd_ice1712, mut byte: c_uchar) {
    unsafe {
        byte |= ICE1712_STDSP24_CLOCK_BIT;
        udelay(100);
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, byte);
        byte &= !ICE1712_STDSP24_CLOCK_BIT;
        udelay(100);
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, byte);
        byte |= ICE1712_STDSP24_CLOCK_BIT;
        udelay(100);
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, byte);
    }
}

unsafe fn snd_ice1712_stdsp24_darear(ice: *mut snd_ice1712, activate: c_int) {
    unsafe {
        let spec: *mut hoontech_spec = (*ice).spec;

        /* guard(mutex)(&ice->gpio_mutex); */
        ICE1712_STDSP24_0_DAREAR((*spec).boxbits.as_mut_ptr(), activate);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[0]);
    }
}

unsafe fn snd_ice1712_stdsp24_mute(ice: *mut snd_ice1712, activate: c_int) {
    unsafe {
        let spec: *mut hoontech_spec = (*ice).spec;

        /* guard(mutex)(&ice->gpio_mutex); */
        ICE1712_STDSP24_3_MUTE((*spec).boxbits.as_mut_ptr(), activate);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[3]);
    }
}

unsafe fn snd_ice1712_stdsp24_insel(ice: *mut snd_ice1712, activate: c_int) {
    unsafe {
        let spec: *mut hoontech_spec = (*ice).spec;

        /* guard(mutex)(&ice->gpio_mutex); */
        ICE1712_STDSP24_3_INSEL((*spec).boxbits.as_mut_ptr(), activate);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[3]);
    }
}

unsafe fn snd_ice1712_stdsp24_box_channel(
    ice: *mut snd_ice1712,
    box_: c_int,
    chn: c_int,
    activate: c_int,
) {
    unsafe {
        let spec: *mut hoontech_spec = (*ice).spec;

        /* guard(mutex)(&ice->gpio_mutex); */

        /* select box */
        ICE1712_STDSP24_0_BOX((*spec).boxbits.as_mut_ptr(), box_);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[0]);

        /* prepare for write */
        if chn == 3 {
            ICE1712_STDSP24_2_CHN4((*spec).boxbits.as_mut_ptr(), 0);
        }
        ICE1712_STDSP24_2_MIDI1((*spec).boxbits.as_mut_ptr(), activate);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[3]);

        ICE1712_STDSP24_1_CHN1((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_1_CHN2((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_1_CHN3((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_2_CHN4((*spec).boxbits.as_mut_ptr(), 1);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[1]);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
        udelay(100);
        if chn == 3 {
            ICE1712_STDSP24_2_CHN4((*spec).boxbits.as_mut_ptr(), 0);
            snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
        } else {
            match chn {
                0 => ICE1712_STDSP24_1_CHN1((*spec).boxbits.as_mut_ptr(), 0),
                1 => ICE1712_STDSP24_1_CHN2((*spec).boxbits.as_mut_ptr(), 0),
                2 => ICE1712_STDSP24_1_CHN3((*spec).boxbits.as_mut_ptr(), 0),
                _ => {}
            }
            snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[1]);
        }
        udelay(100);
        ICE1712_STDSP24_1_CHN1((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_1_CHN2((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_1_CHN3((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_2_CHN4((*spec).boxbits.as_mut_ptr(), 1);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[1]);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
        udelay(100);

        ICE1712_STDSP24_2_MIDI1((*spec).boxbits.as_mut_ptr(), 0);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
    }
}

unsafe fn snd_ice1712_stdsp24_box_midi(
    ice: *mut snd_ice1712,
    box_: c_int,
    master: c_int,
) {
    unsafe {
        let spec: *mut hoontech_spec = (*ice).spec;

        /* guard(mutex)(&ice->gpio_mutex); */

        /* select box */
        ICE1712_STDSP24_0_BOX((*spec).boxbits.as_mut_ptr(), box_);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[0]);

        ICE1712_STDSP24_2_MIDIIN((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_2_MIDI1((*spec).boxbits.as_mut_ptr(), master);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[3]);

        udelay(100);

        ICE1712_STDSP24_2_MIDIIN((*spec).boxbits.as_mut_ptr(), 0);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);

        mdelay(10);

        ICE1712_STDSP24_2_MIDIIN((*spec).boxbits.as_mut_ptr(), 1);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[2]);
    }
}

unsafe fn snd_ice1712_stdsp24_midi2(ice: *mut snd_ice1712, activate: c_int) {
    unsafe {
        let spec: *mut hoontech_spec = (*ice).spec;

        /* guard(mutex)(&ice->gpio_mutex); */
        ICE1712_STDSP24_3_MIDI2((*spec).boxbits.as_mut_ptr(), activate);
        snd_ice1712_stdsp24_gpio_write(ice, (*spec).boxbits[3]);
    }
}

unsafe fn hoontech_init(ice: *mut snd_ice1712, staudio: bool) -> c_int {
    unsafe {
        let spec: *mut hoontech_spec;
        let mut box_: c_int;
        let mut chn: c_int;

        (*ice).num_total_dacs = 8;
        (*ice).num_total_adcs = 8;

        spec = kzalloc_obj::<hoontech_spec>();
        if spec.is_null() {
            return -ENOMEM;
        }
        (*ice).spec = spec;

        ICE1712_STDSP24_SET_ADDR((*spec).boxbits.as_mut_ptr(), 0);
        ICE1712_STDSP24_CLOCK((*spec).boxbits.as_mut_ptr(), 0, 1);
        ICE1712_STDSP24_0_BOX((*spec).boxbits.as_mut_ptr(), 0);
        ICE1712_STDSP24_0_DAREAR((*spec).boxbits.as_mut_ptr(), 0);

        ICE1712_STDSP24_SET_ADDR((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_CLOCK((*spec).boxbits.as_mut_ptr(), 1, 1);
        ICE1712_STDSP24_1_CHN1((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_1_CHN2((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_1_CHN3((*spec).boxbits.as_mut_ptr(), 1);

        ICE1712_STDSP24_SET_ADDR((*spec).boxbits.as_mut_ptr(), 2);
        ICE1712_STDSP24_CLOCK((*spec).boxbits.as_mut_ptr(), 2, 1);
        ICE1712_STDSP24_2_CHN4((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_2_MIDIIN((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_2_MIDI1((*spec).boxbits.as_mut_ptr(), 0);

        ICE1712_STDSP24_SET_ADDR((*spec).boxbits.as_mut_ptr(), 3);
        ICE1712_STDSP24_CLOCK((*spec).boxbits.as_mut_ptr(), 3, 1);
        ICE1712_STDSP24_3_MIDI2((*spec).boxbits.as_mut_ptr(), 0);
        ICE1712_STDSP24_3_MUTE((*spec).boxbits.as_mut_ptr(), 1);
        ICE1712_STDSP24_3_INSEL((*spec).boxbits.as_mut_ptr(), 0);

        /* let's go - activate only functions in first box */
        if staudio {
            (*spec).config = ICE1712_STDSP24_MUTE;
        } else {
            (*spec).config = 0;
            /* ICE1712_STDSP24_MUTE |
             * ICE1712_STDSP24_INSEL |
             * ICE1712_STDSP24_DAREAR;
             */
        }
        /*  These boxconfigs have caused problems in the past.
         *  The code is not optimal, but should now enable a working config to
         *  be achieved.
         *  ** MIDI IN can only be configured on one box **
         *  ICE1712_STDSP24_BOX_MIDI1 needs to be set for that box.
         *  Tests on a ADAC2000 box suggest the box config flags do not
         *  work as would be expected, and the inputs are crossed.
         *  Setting ICE1712_STDSP24_BOX_MIDI1 and ICE1712_STDSP24_BOX_MIDI2
         *  on the same box connects MIDI-In to both 401 uarts; both outputs
         *  are then active on all boxes.
         *  The default config here sets up everything on the first box.
         *  Alan Horstmann  5.2.2008
         */
        (*spec).boxconfig[0] = ICE1712_STDSP24_BOX_CHN1
            | ICE1712_STDSP24_BOX_CHN2
            | ICE1712_STDSP24_BOX_CHN3
            | ICE1712_STDSP24_BOX_CHN4
            | ICE1712_STDSP24_BOX_MIDI1
            | ICE1712_STDSP24_BOX_MIDI2;
        if staudio {
            (*spec).boxconfig[1] = (*spec).boxconfig[0];
            (*spec).boxconfig[2] = (*spec).boxconfig[0];
            (*spec).boxconfig[3] = (*spec).boxconfig[0];
        } else {
            (*spec).boxconfig[1] = 0;
            (*spec).boxconfig[2] = 0;
            (*spec).boxconfig[3] = 0;
        }

        snd_ice1712_stdsp24_darear(
            ice,
            if ((*spec).config & ICE1712_STDSP24_DAREAR) != 0 { 1 } else { 0 },
        );
        snd_ice1712_stdsp24_mute(
            ice,
            if ((*spec).config & ICE1712_STDSP24_MUTE) != 0 { 1 } else { 0 },
        );
        snd_ice1712_stdsp24_insel(
            ice,
            if ((*spec).config & ICE1712_STDSP24_INSEL) != 0 { 1 } else { 0 },
        );
        box_ = 0;
        while box_ < 4 {
            if ((*spec).boxconfig[box_ as usize] & ICE1712_STDSP24_BOX_MIDI2) != 0 {
                snd_ice1712_stdsp24_midi2(ice, 1);
            }
            chn = 0;
            while chn < 4 {
                snd_ice1712_stdsp24_box_channel(
                    ice,
                    box_,
                    chn,
                    if ((*spec).boxconfig[box_ as usize] & ((1u16) << chn)) != 0 {
                        1
                    } else {
                        0
                    },
                );
                chn += 1;
            }
            if ((*spec).boxconfig[box_ as usize] & ICE1712_STDSP24_BOX_MIDI1) != 0 {
                snd_ice1712_stdsp24_box_midi(ice, box_, 1);
            }
            box_ += 1;
        }

        0
    }
}

unsafe extern "C" fn snd_ice1712_hoontech_init(ice: *mut snd_ice1712) -> c_int {
    unsafe { hoontech_init(ice, false) }
}

unsafe extern "C" fn snd_ice1712_staudio_init(ice: *mut snd_ice1712) -> c_int {
    unsafe { hoontech_init(ice, true) }
}

/*
 * AK4524 access
 */

/* start callback for STDSP24 with modified hardware */
unsafe extern "C" fn stdsp24_ak4524_lock(ak: *mut snd_akm4xxx, _chip: c_int) {
    unsafe {
        let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;
        let tmp: c_uchar;
        snd_ice1712_save_gpio_status(ice);
        tmp = ICE1712_STDSP24_SERIAL_DATA
            | ICE1712_STDSP24_SERIAL_CLOCK
            | ICE1712_STDSP24_AK4524_CS;
        snd_ice1712_write(
            ice,
            ICE1712_IREG_GPIO_DIRECTION,
            (*ice).gpio.direction | tmp,
        );
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_WRITE_MASK, !tmp);
    }
}

unsafe extern "C" fn snd_ice1712_value_init(ice: *mut snd_ice1712) -> c_int {
    unsafe {
        /* Hoontech STDSP24 with modified hardware */
        static AKM_STDSP24_MV: snd_akm4xxx = snd_akm4xxx {
            num_adcs: 2,
            num_dacs: 2,
            type_: unsafe { SND_AK4524 },
            ops: snd_akm4xxx_ops {
                lock: Some(stdsp24_ak4524_lock),
            },
            private_data: [core::ptr::null_mut(); 1],
        };

        static AKM_STDSP24_MV_PRIV: snd_ak4xxx_private = snd_ak4xxx_private {
            caddr: 2,
            cif: 1, /* CIF high */
            data_mask: unsafe { ICE1712_STDSP24_SERIAL_DATA },
            clk_mask: unsafe { ICE1712_STDSP24_SERIAL_CLOCK },
            cs_mask: unsafe { ICE1712_STDSP24_AK4524_CS },
            cs_addr: unsafe { ICE1712_STDSP24_AK4524_CS },
            cs_none: 0,
            add_flags: 0,
        };

        let err: c_int;
        let ak: *mut snd_akm4xxx;

        /* set the analog DACs */
        (*ice).num_total_dacs = 2;

        /* set the analog ADCs */
        (*ice).num_total_adcs = 2;

        /* analog section */
        ak = kmalloc_obj::<snd_akm4xxx>();
        (*ice).akm = ak;
        if ak.is_null() {
            return -ENOMEM;
        }
        (*ice).akm_codecs = 1;

        err = snd_ice1712_akm4xxx_init(ak, &AKM_STDSP24_MV, &AKM_STDSP24_MV_PRIV, ice);
        if err < 0 {
            return err;
        }

        /* ak4524 controls */
        snd_ice1712_akm4xxx_build_controls(ice)
    }
}

unsafe extern "C" fn snd_ice1712_ez8_init(ice: *mut snd_ice1712) -> c_int {
    unsafe {
        (*ice).gpio.write_mask = (*ice).eeprom.gpiomask;
        (*ice).gpio.direction = (*ice).eeprom.gpiodir;
        snd_ice1712_write(
            ice,
            ICE1712_IREG_GPIO_WRITE_MASK,
            (*ice).eeprom.gpiomask,
        );
        snd_ice1712_write(
            ice,
            ICE1712_IREG_GPIO_DIRECTION,
            (*ice).eeprom.gpiodir,
        );
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, (*ice).eeprom.gpiostate);
        0
    }
}

/* entry point */
#[unsafe(no_mangle)]
pub static mut snd_ice1712_hoontech_cards: [snd_ice1712_card_info; 6] = unsafe {
    [
        snd_ice1712_card_info {
            subvendor: ICE1712_SUBDEVICE_STDSP24,
            name: c"Hoontech SoundTrack Audio DSP24".as_ptr(),
            model: c"dsp24".as_ptr(),
            chip_init: Some(snd_ice1712_hoontech_init),
            mpu401_1_name: c"MIDI-1 Hoontech/STA DSP24".as_ptr(),
            mpu401_2_name: c"MIDI-2 Hoontech/STA DSP24".as_ptr(),
        },
        snd_ice1712_card_info {
            subvendor: ICE1712_SUBDEVICE_STDSP24_VALUE, /* a dummy id */
            name: c"Hoontech SoundTrack Audio DSP24 Value".as_ptr(),
            model: c"dsp24_value".as_ptr(),
            chip_init: Some(snd_ice1712_value_init),
            mpu401_1_name: core::ptr::null(),
            mpu401_2_name: core::ptr::null(),
        },
        snd_ice1712_card_info {
            subvendor: ICE1712_SUBDEVICE_STDSP24_MEDIA7_1,
            name: c"Hoontech STA DSP24 Media 7.1".as_ptr(),
            model: c"dsp24_71".as_ptr(),
            chip_init: Some(snd_ice1712_hoontech_init),
            mpu401_1_name: core::ptr::null(),
            mpu401_2_name: core::ptr::null(),
        },
        snd_ice1712_card_info {
            subvendor: ICE1712_SUBDEVICE_EVENT_EZ8, /* a dummy id */
            name: c"Event Electronics EZ8".as_ptr(),
            model: c"ez8".as_ptr(),
            chip_init: Some(snd_ice1712_ez8_init),
            mpu401_1_name: core::ptr::null(),
            mpu401_2_name: core::ptr::null(),
        },
        snd_ice1712_card_info {
            /* STAudio ADCIII has the same SSID as Hoontech StA DSP24,
             * thus identified only via the explicit model option
             */
            subvendor: ICE1712_SUBDEVICE_STAUDIO_ADCIII, /* a dummy id */
            name: c"STAudio ADCIII".as_ptr(),
            model: c"staudio".as_ptr(),
            chip_init: Some(snd_ice1712_staudio_init),
            mpu401_1_name: core::ptr::null(),
            mpu401_2_name: core::ptr::null(),
        },
        snd_ice1712_card_info {
            subvendor: 0,
            name: core::ptr::null(),
            model: core::ptr::null(),
            chip_init: None,
            mpu401_1_name: core::ptr::null(),
            mpu401_2_name: core::ptr::null(),
        }, /* terminator */
    ]
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
