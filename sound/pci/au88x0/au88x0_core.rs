// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

/*
    Vortex core low level functions.

 Author: Manuel Jander (mjander@users.sourceforge.cl)
 These functions are mainly the result of translations made
 from the original disassembly of the au88x0 binary drivers,
 written by Aureal before they went down.
 Many thanks to the Jeff Muizelaar, Kester Maddock, and whoever
 contributed to the OpenVortex project.
 The author of this file, put the few available pieces together
 and translated the rest of the riddle (Mix, Src and connection stuff).
 Some things are still to be discovered, and their meanings are unclear.

 Some of these functions aren't intended to be really used, rather
 to help to understand how does the AU88X0 chips work. Keep them in, because
 they could be used somewhere in the future.

 This code hasn't been tested or proof read thoroughly. If you wanna help,
 take a look at the AU88X0 assembly and check if this matches.
 Functions tested ok so far are (they show the desired effect
 at least):
   vortex_routes(); (1 bug fixed).
   vortex_adb_addroute();
   vortex_adb_addroutes();
   vortex_connect_codecplay();
   vortex_src_flushbuffers();
   vortex_adbdma_setmode();  note: still some unknown arguments!
   vortex_adbdma_startfifo();
   vortex_adbdma_stopfifo();
   vortex_fifo_setadbctrl(); note: still some unknown arguments!
   vortex_mix_setinputvolumebyte();
   vortex_mix_enableinput();
   vortex_mixer_addWTD(); (fixed)
   vortex_connection_adbdma_src_src();
   vortex_connection_adbdma_src();
   vortex_src_change_convratio();
   vortex_src_addWTD(); (fixed)

 History:

 01-03-2003 First revision.
 01-21-2003 Some bug fixes.
 17-02-2003 many bugfixes after a big versioning mess.
 18-02-2003 JAAAAAHHHUUUUUU!!!! The mixer works !! I'm just so happy !
             (2 hours later...) I cant believe it! Im really lucky today.
             Now the SRC is working too! Yeah! XMMS works !
 20-02-2003 First steps into the ALSA world.
 28-02-2003 As my birthday present, i discovered how the DMA buffer pages really
            work :-). It was all wrong.
 12-03-2003 ALSA driver starts working (2 channels).
 16-03-2003 More srcblock_setupchannel discoveries.
 12-04-2003 AU8830 playback support. Recording in the works.
 17-04-2003 vortex_route() and vortex_routes() bug fixes. AU8830 recording
            works now, but chipn' dale effect is still there.
 16-05-2003 SrcSetupChannel cleanup. Moved the Src setup stuff entirely
            into au88x0_pcm.c .
 06-06-2003 Buffer shifter bugfix. Mixer volume fix.
 07-12-2003 A3D routing finally fixed. Believed to be OK.
 25-03-2004 Many thanks to Claudia, for such valuable bug reports.

*/

// Dependencies from au88x0.h, au88x0_a3d.h, and Linux/ALSA headers are expected
// to be supplied by the surrounding translation unit.

type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type c_ushort = u16;
type u32 = u32;
type u16 = u16;
type irqreturn_t = c_int;
type snd_pcm_format_t = c_int;
type ADBRamLink = c_int;

#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut core::ffi::c_void,
    pub num: c_int,
}

#[repr(C)]
pub struct pcm_vol {
    pub dma: c_int,
    pub mixin: [c_int; 4],
    pub vol: [c_int; 4],
}

#[repr(C)]
pub struct stream_t {
    pub dma_ctrl: c_int,
    pub period_real: c_int,
    pub period_virt: c_int,
    pub period_bytes: c_int,
    pub nr_periods: c_int,
    pub cfg0: c_int,
    pub cfg1: c_int,
    pub substream: *mut core::ffi::c_void,
    pub dma_unknown: c_int,
    pub fifo_status: c_int,
    pub fifo_enabled: c_int,
    pub resources: [c_int; VORTEX_RESOURCE_LAST as usize],
    pub dma: c_int,
    pub dir: c_int,
    pub type_: c_int,
}

#[repr(C)]
pub struct card_t {
    pub dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct rmidi_t {
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct vortex_t {
    pub mmio: *mut core::ffi::c_void,
    pub card: *mut card_t,
    pub dma_adb: [stream_t; NR_ADB as usize],
    pub dma_wt: [stream_t; NR_WT as usize],
    pub fixed_res: [c_int; VORTEX_RESOURCE_LAST as usize],
    pub mixplayb: [c_uchar; 4],
    pub mixcapt: [c_uchar; 2],
    pub mixspdif: [c_uchar; 2],
    pub pcm_vol: [pcm_vol; 4],
    pub a3d: [core::ffi::c_void; NR_A3D as usize],
    pub lock: core::ffi::c_void,
    pub rmidi: *mut rmidi_t,
    pub irq: c_int,
}

extern "C" {
    fn hwread(mmio: *mut core::ffi::c_void, addr: u32) -> c_int;
    fn hwwrite(mmio: *mut core::ffi::c_void, addr: u32, val: c_int);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_warn(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_info(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn msleep(ms: c_uint);
    fn udelay(us: c_uint);
    fn snd_pcm_sgbuf_get_addr(substream: *mut core::ffi::c_void, offset: c_int) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut core::ffi::c_void);
    fn snd_mpu401_uart_interrupt(irq: c_int, private_data: *mut core::ffi::c_void);
    fn spin_lock(lock: *mut core::ffi::c_void);
    fn spin_unlock(lock: *mut core::ffi::c_void);
    fn spin_lock_init(lock: *mut core::ffi::c_void);
    fn vortex_wt_connect(vortex: *mut vortex_t, en: c_int);
    fn vortex_wt_init(vortex: *mut vortex_t);
    fn vortex_eq_init(vortex: *mut vortex_t);
    fn vortex_eq_free(vortex: *mut vortex_t);
    fn vortex_Vort3D_connect(vortex: *mut vortex_t, en: c_int);
    fn vortex_Vort3D_enable(vortex: *mut vortex_t);
    fn vortex_Vort3D_disable(vortex: *mut vortex_t);
    fn vortex_Vort3D_InitializeSource(a3d: *mut core::ffi::c_void, en: c_int, vortex: *mut vortex_t);
    fn VORTEX_IS_QUAD(vortex: *mut vortex_t) -> c_int;
    fn IRQ_RETVAL(handled: c_int) -> irqreturn_t;
    fn unlikely(x: c_int) -> c_int;
}

extern "C" {
    static NR_MIXIN: c_int;
    static NR_MIXOUT: c_int;
    static NR_ADB: c_int;
    static NR_SRC: c_int;
    static NR_WT: c_int;
    static NR_A3D: c_int;
    static VORTEX_RESOURCE_LAST: c_int;
    static VORTEX_RESOURCE_DMA: c_int;
    static VORTEX_RESOURCE_SRC: c_int;
    static VORTEX_RESOURCE_MIXIN: c_int;
    static VORTEX_RESOURCE_MIXOUT: c_int;
    static VORTEX_RESOURCE_A3D: c_int;
}

// Register constants and macros are external to this isolated source file.
// Names below are used exactly as in the C implementation.

static mut mchannels: [c_int; NR_MIXIN as usize] = [0; NR_MIXIN as usize];
static mut rampchs: [c_int; NR_MIXIN as usize] = [0; NR_MIXIN as usize];

unsafe fn vortex_mixer_en_sr(vortex: *mut vortex_t, channel: c_int) {
    hwwrite((*vortex).mmio, VORTEX_MIXER_SR, hwread((*vortex).mmio, VORTEX_MIXER_SR) | (0x1 << channel));
}

unsafe fn vortex_mixer_dis_sr(vortex: *mut vortex_t, channel: c_int) {
    hwwrite((*vortex).mmio, VORTEX_MIXER_SR, hwread((*vortex).mmio, VORTEX_MIXER_SR) & !(0x1 << channel));
}

/*
#if 0
Disabled mixer helper translations from the C source are intentionally preserved
as comments: vortex_mix_muteinputgain, vortex_mix_getvolume,
vortex_mix_getinputvolume, vortex_mix_boost6db, vortex_mix_rampvolume, and
vortex_mix_getenablebit.
#endif
*/

unsafe fn vortex_mix_setvolumebyte(vortex: *mut vortex_t, mix: c_uchar, vol: c_uchar) {
    let mut temp: c_int;
    hwwrite((*vortex).mmio, VORTEX_MIX_VOL_A + ((mix as c_int) << 2), vol as c_int);
    if true {
        temp = hwread((*vortex).mmio, VORTEX_MIX_VOL_B + ((mix as c_int) << 2));
        if temp != 0x80 || vol as c_int == 0x80 {
            return;
        }
    }
    hwwrite((*vortex).mmio, VORTEX_MIX_VOL_B + ((mix as c_int) << 2), vol as c_int);
}

unsafe fn vortex_mix_setinputvolumebyte(vortex: *mut vortex_t, mix: c_uchar, mixin: c_int, vol: c_uchar) {
    let mut temp: c_int;
    hwwrite((*vortex).mmio, VORTEX_MIX_INVOL_A + ((((mix as c_int) << 5) + mixin) << 2), vol as c_int);
    if true {
        temp = hwread((*vortex).mmio, VORTEX_MIX_INVOL_B + ((((mix as c_int) << 5) + mixin) << 2));
        if temp != 0x80 || vol as c_int == 0x80 {
            return;
        }
    }
    hwwrite((*vortex).mmio, VORTEX_MIX_INVOL_B + ((((mix as c_int) << 5) + mixin) << 2), vol as c_int);
}

unsafe fn vortex_mix_setenablebit(vortex: *mut vortex_t, mix: c_uchar, mixin: c_int, en: c_int) {
    let mut addr: c_int = if mixin < 0 { mixin + 3 } else { mixin };
    addr = (((mix as c_int) << 3) + (addr >> 2)) << 2;
    let mut temp = hwread((*vortex).mmio, VORTEX_MIX_ENIN + addr);
    if en != 0 {
        temp |= 1 << (mixin & 3);
    } else {
        temp &= !(1 << (mixin & 3));
    }
    hwwrite((*vortex).mmio, VORTEX_MIX_INVOL_B + ((((mix as c_int) << 5) + mixin) << 2), 0x80);
    hwwrite((*vortex).mmio, VORTEX_MIX_SMP + (mixin << 2), 0);
    hwwrite((*vortex).mmio, VORTEX_MIX_SMP + 4 + (mixin << 2), 0);
    hwwrite((*vortex).mmio, VORTEX_MIX_ENIN + addr, temp);
}

unsafe fn vortex_mix_killinput(vortex: *mut vortex_t, mix: c_uchar, mixin: c_int) {
    rampchs[mix as usize] &= !(1 << mixin);
    vortex_mix_setinputvolumebyte(vortex, mix, mixin, 0x80);
    mchannels[mix as usize] &= !(1 << mixin);
    vortex_mix_setenablebit(vortex, mix, mixin, 0);
}

unsafe fn vortex_mix_enableinput(vortex: *mut vortex_t, mix: c_uchar, mixin: c_int) {
    vortex_mix_killinput(vortex, mix, mixin);
    if (mchannels[mix as usize] & (1 << mixin)) == 0 {
        vortex_mix_setinputvolumebyte(vortex, mix, mixin, 0x80);
        mchannels[mix as usize] |= 1 << mixin;
    }
    vortex_mix_setenablebit(vortex, mix, mixin, 1);
}

unsafe fn vortex_mix_disableinput(vortex: *mut vortex_t, mix: c_uchar, channel: c_int, ramp: c_int) {
    if ramp != 0 {
        rampchs[mix as usize] |= 1 << channel;
        vortex_mix_killinput(vortex, mix, channel);
    } else {
        vortex_mix_killinput(vortex, mix, channel);
    }
}

unsafe fn vortex_mixer_addWTD(vortex: *mut vortex_t, mix: c_uchar, ch: c_uchar) -> c_int {
    let mut lifeboat = 0;
    let mut temp = hwread((*vortex).mmio, VORTEX_MIXER_SR);
    if (temp & (1 << ch)) == 0 {
        hwwrite((*vortex).mmio, VORTEX_MIXER_CHNBASE + ((ch as c_int) << 2), mix as c_int);
        vortex_mixer_en_sr(vortex, ch as c_int);
        return 1;
    }
    let mut prev = VORTEX_MIXER_CHNBASE + ((ch as c_int) << 2);
    temp = hwread((*vortex).mmio, prev);
    while (temp & 0x10) != 0 {
        prev = VORTEX_MIXER_RTBASE + ((temp & 0xf) << 2);
        temp = hwread((*vortex).mmio, prev);
        lifeboat += 1;
        if lifeboat > 0xf {
            dev_err((*(*vortex).card).dev, b"vortex_mixer_addWTD: lifeboat overflow\n\0".as_ptr());
            return 0;
        }
    }
    hwwrite((*vortex).mmio, VORTEX_MIXER_RTBASE + ((temp & 0xf) << 2), mix as c_int);
    hwwrite((*vortex).mmio, prev, (temp & 0xf) | 0x10);
    1
}

unsafe fn vortex_mixer_delWTD(vortex: *mut vortex_t, mix: c_uchar, ch: c_uchar) -> c_int {
    let mut esp14 = -1;
    let mut esi = 0;
    let eax = hwread((*vortex).mmio, VORTEX_MIXER_SR);
    if ((1 << ch) & eax) == 0 {
        dev_err((*(*vortex).card).dev, b"mix ALARM %x\n\0".as_ptr(), eax);
        return 0;
    }
    let mut ebp = VORTEX_MIXER_CHNBASE + ((ch as c_int) << 2);
    let esp18 = hwread((*vortex).mmio, ebp);
    if (esp18 & 0x10) != 0 {
        let mut ebx = esp18 & 0xf;
        if mix as c_int == ebx {
            ebx = VORTEX_MIXER_RTBASE + ((mix as c_int) << 2);
            let edx = hwread((*vortex).mmio, ebx);
            hwwrite((*vortex).mmio, ebp, edx);
            hwwrite((*vortex).mmio, ebx, 0);
        } else {
            let mut edx = hwread((*vortex).mmio, VORTEX_MIXER_RTBASE + (ebx << 2));
            while (edx & 0xf) != mix as c_int {
                if esi > 0xf {
                    dev_err((*(*vortex).card).dev, b"mixdelWTD: error lifeboat overflow\n\0".as_ptr());
                    return 0;
                }
                esp14 = ebx;
                ebx = edx & 0xf;
                ebp = ebx << 2;
                edx = hwread((*vortex).mmio, VORTEX_MIXER_RTBASE + ebp);
                esi += 1;
            }
            ebp = ebx << 2;
            if (edx & 0x10) != 0 {
                ebx = VORTEX_MIXER_RTBASE + ((edx & 0xf) << 2);
                edx = hwread((*vortex).mmio, ebx);
                hwwrite((*vortex).mmio, VORTEX_MIXER_RTBASE + ebp, edx);
                hwwrite((*vortex).mmio, ebx, 0);
            } else {
                if esp14 == -1 {
                    hwwrite((*vortex).mmio, VORTEX_MIXER_CHNBASE + ((ch as c_int) << 2), esp18 & 0xef);
                } else {
                    ebx = (0xffffffe0u32 as c_int & edx) | (0xf & ebx);
                    hwwrite((*vortex).mmio, VORTEX_MIXER_RTBASE + (esp14 << 2), ebx);
                }
                hwwrite((*vortex).mmio, VORTEX_MIXER_RTBASE + ebp, 0);
                return 1;
            }
        }
    } else {
        vortex_mixer_dis_sr(vortex, ch as c_int);
        hwwrite((*vortex).mmio, ebp, 0);
    }
    1
}

unsafe fn vortex_mixer_init(vortex: *mut vortex_t) {
    mchannels.fill(0);
    rampchs.fill(0);
    let mut addr = VORTEX_MIX_SMP + 0x17c;
    let mut x = 0x5f;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIX_ENIN + 0x1fc;
    x = 0x7f;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIX_SMP + 0x17c;
    x = 0x5f;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIX_INVOL_A + 0x7fc;
    x = 0x1ff;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0x80);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIX_VOL_A + 0x3c;
    x = 0xf;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0x80);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIX_INVOL_B + 0x7fc;
    x = 0x1ff;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0x80);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIX_VOL_B + 0x3c;
    x = 0xf;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0x80);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_MIXER_RTBASE + (MIXER_RTBASE_SIZE - 1) * 4;
    x = MIXER_RTBASE_SIZE - 1;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0);
        addr -= 4;
        x -= 1;
    }
    hwwrite((*vortex).mmio, VORTEX_MIXER_SR, 0);
}

unsafe fn vortex_src_en_sr(vortex: *mut vortex_t, channel: c_int) {
    hwwrite((*vortex).mmio, VORTEX_SRCBLOCK_SR, hwread((*vortex).mmio, VORTEX_SRCBLOCK_SR) | (0x1 << channel));
}

unsafe fn vortex_src_dis_sr(vortex: *mut vortex_t, channel: c_int) {
    hwwrite((*vortex).mmio, VORTEX_SRCBLOCK_SR, hwread((*vortex).mmio, VORTEX_SRCBLOCK_SR) & !(0x1 << channel));
}

unsafe fn vortex_src_flushbuffers(vortex: *mut vortex_t, src: c_uchar) {
    let mut i = 0x1f;
    while i >= 0 {
        hwwrite((*vortex).mmio, VORTEX_SRC_DATA0 + ((src as c_int) << 7) + (i << 2), 0);
        i -= 1;
    }
    hwwrite((*vortex).mmio, VORTEX_SRC_DATA + ((src as c_int) << 3), 0);
    hwwrite((*vortex).mmio, VORTEX_SRC_DATA + ((src as c_int) << 3) + 4, 0);
}

unsafe fn vortex_src_cleardrift(vortex: *mut vortex_t, src: c_uchar) {
    hwwrite((*vortex).mmio, VORTEX_SRC_DRIFT0 + ((src as c_int) << 2), 0);
    hwwrite((*vortex).mmio, VORTEX_SRC_DRIFT1 + ((src as c_int) << 2), 0);
    hwwrite((*vortex).mmio, VORTEX_SRC_DRIFT2 + ((src as c_int) << 2), 1);
}

unsafe fn vortex_src_set_throttlesource(vortex: *mut vortex_t, src: c_uchar, en: c_int) {
    let mut temp = hwread((*vortex).mmio, VORTEX_SRC_SOURCE);
    if en != 0 {
        temp |= 1 << src;
    } else {
        temp &= !(1 << src);
    }
    hwwrite((*vortex).mmio, VORTEX_SRC_SOURCE, temp);
}

unsafe fn vortex_src_persist_convratio(vortex: *mut vortex_t, src: c_uchar, ratio: c_int) -> c_int {
    let mut lifeboat = 0;
    let mut temp;
    loop {
        hwwrite((*vortex).mmio, VORTEX_SRC_CONVRATIO + ((src as c_int) << 2), ratio);
        temp = hwread((*vortex).mmio, VORTEX_SRC_CONVRATIO + ((src as c_int) << 2));
        lifeboat += 1;
        if lifeboat > 0x9 {
            dev_err((*(*vortex).card).dev, b"Src cvr fail\n\0".as_ptr());
            break;
        }
        if temp == ratio {
            break;
        }
    }
    temp
}

/*
#if 0
Disabled SRC helper translations from the C source are intentionally preserved
as comments: vortex_src_slowlock, vortex_src_change_convratio, and
vortex_src_checkratio.
#endif
*/

unsafe fn vortex_src_setupchannel(
    card: *mut vortex_t,
    src: c_uchar,
    cr: c_uint,
    b: c_uint,
    sweep: c_int,
    d: c_int,
    dirplay: c_int,
    sl: c_int,
    mut tr: c_uint,
    thsource: c_int,
) {
    let mut ebp = 0;
    let esp10: c_int;
    let esi: c_int;

    vortex_src_flushbuffers(card, src);
    if sweep != 0 {
        if (tr & 0x10000) != 0 && tr != 0x10000 {
            tr = 0;
            esi = 0x7;
        } else if ((tr as i16) < 0) && tr != 0x8000 {
            tr = 0;
            esi = 0x8;
        } else {
            tr = 1;
            esi = 0xc;
        }
    } else if (cr & 0x10000) != 0 && cr != 0x10000 {
        tr = 0;
        esi = 0x11 - ((cr >> 0xe) & 7) as c_int - if (cr & 0x3fff) != 0 { 1 } else { 2 };
    } else {
        tr = 1;
        esi = 0xc;
    }
    vortex_src_cleardrift(card, src);
    vortex_src_set_throttlesource(card, src, thsource);
    if dirplay == 0 && sweep == 0 {
        esp10 = if tr != 0 { 0xf } else { 0xc };
        ebp = 0;
    } else {
        ebp = if tr != 0 { 0xf } else { 0xc };
        esp10 = 0;
    }
    hwwrite((*card).mmio, VORTEX_SRC_U0 + ((src as c_int) << 2), (sl << 0x9) | (sweep << 0x8) | ((esi & 0xf) << 4) | d);
    vortex_src_persist_convratio(card, src, cr as c_int);
    hwwrite((*card).mmio, VORTEX_SRC_U1 + ((src as c_int) << 2), (b & 0xffff) as c_int);
    hwwrite((*card).mmio, VORTEX_SRC_U2 + ((src as c_int) << 2), ((tr as c_int) << 0x11) | (dirplay << 0x10) | (ebp << 0x8) | esp10);
}

unsafe fn vortex_srcblock_init(vortex: *mut vortex_t) {
    hwwrite((*vortex).mmio, VORTEX_SRC_SOURCESIZE, 0x1ff);
    let mut addr = VORTEX_SRC_RTBASE + 0x3c;
    let mut x = 0xf;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0);
        addr -= 4;
        x -= 1;
    }
    addr = VORTEX_SRC_CHNBASE + 0x54;
    x = 0x15;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, 0);
        addr -= 4;
        x -= 1;
    }
}

unsafe fn vortex_src_addWTD(vortex: *mut vortex_t, src: c_uchar, ch: c_uchar) -> c_int {
    let mut lifeboat = 0;
    let mut temp = hwread((*vortex).mmio, VORTEX_SRCBLOCK_SR);
    if (temp & (1 << ch)) == 0 {
        hwwrite((*vortex).mmio, VORTEX_SRC_CHNBASE + ((ch as c_int) << 2), src as c_int);
        vortex_src_en_sr(vortex, ch as c_int);
        return 1;
    }
    let mut prev = VORTEX_SRC_CHNBASE + ((ch as c_int) << 2);
    temp = hwread((*vortex).mmio, prev);
    while (temp & 0x10) != 0 {
        prev = VORTEX_SRC_RTBASE + ((temp & 0xf) << 2);
        temp = hwread((*vortex).mmio, prev);
        lifeboat += 1;
        if lifeboat > 0xf {
            dev_err((*(*vortex).card).dev, b"vortex_src_addWTD: lifeboat overflow\n\0".as_ptr());
            return 0;
        }
    }
    hwwrite((*vortex).mmio, VORTEX_SRC_RTBASE + ((temp & 0xf) << 2), src as c_int);
    hwwrite((*vortex).mmio, prev, (temp & 0xf) | 0x10);
    1
}

unsafe fn vortex_src_delWTD(vortex: *mut vortex_t, src: c_uchar, ch: c_uchar) -> c_int {
    let mut esp14 = -1;
    let mut esi = 0;
    let eax = hwread((*vortex).mmio, VORTEX_SRCBLOCK_SR);
    if ((1 << ch) & eax) == 0 {
        dev_err((*(*vortex).card).dev, b"src alarm\n\0".as_ptr());
        return 0;
    }
    let mut ebp = VORTEX_SRC_CHNBASE + ((ch as c_int) << 2);
    let esp18 = hwread((*vortex).mmio, ebp);
    if (esp18 & 0x10) != 0 {
        let mut ebx = esp18 & 0xf;
        if src as c_int == ebx {
            ebx = VORTEX_SRC_RTBASE + ((src as c_int) << 2);
            let edx = hwread((*vortex).mmio, ebx);
            hwwrite((*vortex).mmio, ebp, edx);
            hwwrite((*vortex).mmio, ebx, 0);
        } else {
            let mut edx = hwread((*vortex).mmio, VORTEX_SRC_RTBASE + (ebx << 2));
            while (edx & 0xf) != src as c_int {
                if esi > 0xf {
                    dev_warn((*(*vortex).card).dev, b"srcdelWTD: error, lifeboat overflow\n\0".as_ptr());
                    return 0;
                }
                esp14 = ebx;
                ebx = edx & 0xf;
                ebp = ebx << 2;
                edx = hwread((*vortex).mmio, VORTEX_SRC_RTBASE + ebp);
                esi += 1;
            }
            ebp = ebx << 2;
            if (edx & 0x10) != 0 {
                ebx = VORTEX_SRC_RTBASE + ((edx & 0xf) << 2);
                edx = hwread((*vortex).mmio, ebx);
                hwwrite((*vortex).mmio, VORTEX_SRC_RTBASE + ebp, edx);
                hwwrite((*vortex).mmio, ebx, 0);
            } else {
                if esp14 == -1 {
                    hwwrite((*vortex).mmio, VORTEX_SRC_CHNBASE + ((ch as c_int) << 2), esp18 & 0xef);
                } else {
                    ebx = (0xffffffe0u32 as c_int & edx) | (0xf & ebx);
                    hwwrite((*vortex).mmio, VORTEX_SRC_RTBASE + (esp14 << 2), ebx);
                }
                hwwrite((*vortex).mmio, VORTEX_SRC_RTBASE + ebp, 0);
                return 1;
            }
        }
    } else {
        vortex_src_dis_sr(vortex, ch as c_int);
        hwwrite((*vortex).mmio, ebp, 0);
    }
    1
}

unsafe fn vortex_fifo_clearadbdata(vortex: *mut vortex_t, fifo: c_int, mut x: c_int) {
    x -= 1;
    while x >= 0 {
        hwwrite((*vortex).mmio, VORTEX_FIFO_ADBDATA + (((fifo << FIFO_SIZE_BITS) + x) << 2), 0);
        x -= 1;
    }
}

unsafe fn vortex_fifo_setadbvalid(vortex: *mut vortex_t, fifo: c_int, en: c_int) {
    hwwrite((*vortex).mmio, VORTEX_FIFO_ADBCTRL + (fifo << 2), (hwread((*vortex).mmio, VORTEX_FIFO_ADBCTRL + (fifo << 2)) & 0xffffffefu32 as c_int) | ((1 & en) << 4) | FIFO_U1);
}

unsafe fn vortex_fifo_setadbctrl(vortex: *mut vortex_t, fifo: c_int, stereo: c_int, priority: c_int, empty: c_int, valid: c_int, f: c_int) {
    let mut lifeboat = 0;
    let this_4 = 0x2;
    let mut temp;
    loop {
        temp = hwread((*vortex).mmio, VORTEX_FIFO_ADBCTRL + (fifo << 2));
        if lifeboat > 0xbb8 {
            dev_err((*(*vortex).card).dev, b"vortex_fifo_setadbctrl fail\n\0".as_ptr());
            break;
        }
        lifeboat += 1;
        if (temp & FIFO_RDONLY) == 0 {
            break;
        }
    }
    if valid != 0 {
        if (temp & FIFO_VALID) == 0 {
            vortex_fifo_clearadbdata(vortex, fifo, FIFO_SIZE);
            // CHIP_AU8820 uses (this_4 & 0x1f) << 0xb; other chips use the form below.
            temp = (this_4 & 0x3f) << 0xc;
            temp = (temp & 0xfffffffd_u32 as c_int) | ((stereo & 1) << 1);
            temp = (temp & 0xfffffff3_u32 as c_int) | ((priority & 3) << 2);
            temp = (temp & 0xffffffef_u32 as c_int) | ((valid & 1) << 4);
            temp |= FIFO_U1;
            temp = (temp & 0xffffffdf_u32 as c_int) | ((empty & 1) << 5);
            // CHIP_* conditional f-bit programming is supplied by the build configuration.
        }
    } else if (temp & FIFO_VALID) != 0 {
        // CHIP_* conditional invalidation bits are supplied by the build configuration.
    } else {
        vortex_fifo_clearadbdata(vortex, fifo, FIFO_SIZE);
    }
    hwwrite((*vortex).mmio, VORTEX_FIFO_ADBCTRL + (fifo << 2), temp);
    hwread((*vortex).mmio, VORTEX_FIFO_ADBCTRL + (fifo << 2));
}

// #ifndef CHIP_AU8810
unsafe fn vortex_fifo_clearwtdata(vortex: *mut vortex_t, fifo: c_int, mut x: c_int) {
    if x < 1 {
        return;
    }
    x -= 1;
    while x >= 0 {
        hwwrite((*vortex).mmio, VORTEX_FIFO_WTDATA + (((fifo << FIFO_SIZE_BITS) + x) << 2), 0);
        x -= 1;
    }
}

unsafe fn vortex_fifo_wtinitialize(vortex: *mut vortex_t, fifo: c_int, j: c_int) {
    vortex_fifo_clearwtdata(vortex, fifo, FIFO_SIZE);
    // CHIP_AU8820 uses shift 0xb; other chips use shift 0xc.
    hwwrite((*vortex).mmio, VORTEX_FIFO_WTCTRL + (fifo << 2), FIFO_U1 | ((j & FIFO_MASK) << 0xc));
}

unsafe fn vortex_fifo_setwtvalid(vortex: *mut vortex_t, fifo: c_int, en: c_int) {
    hwwrite((*vortex).mmio, VORTEX_FIFO_WTCTRL + (fifo << 2), (hwread((*vortex).mmio, VORTEX_FIFO_WTCTRL + (fifo << 2)) & 0xffffffefu32 as c_int) | ((en & 1) << 4) | FIFO_U1);
}

unsafe fn vortex_fifo_setwtctrl(vortex: *mut vortex_t, fifo: c_int, ctrl: c_int, priority: c_int, empty: c_int, valid: c_int, f: c_int) {
    let mut temp = 0;
    let mut lifeboat = 0;
    let this_4 = 2;
    loop {
        temp = hwread((*vortex).mmio, VORTEX_FIFO_WTCTRL + (fifo << 2));
        if lifeboat > 0xbb8 {
            dev_err((*(*vortex).card).dev, b"vortex_fifo_setwtctrl fail\n\0".as_ptr());
            break;
        }
        lifeboat += 1;
        if (temp & FIFO_RDONLY) == 0 {
            break;
        }
    }
    if valid != 0 {
        if (temp & FIFO_VALID) == 0 {
            vortex_fifo_clearwtdata(vortex, fifo, FIFO_SIZE);
            // CHIP_AU8820 uses (this_4 & 0x1f) << 0xb; other chips use the form below.
            temp = (this_4 & 0x3f) << 0xc;
            temp = (temp & 0xfffffffd_u32 as c_int) | ((ctrl & 1) << 1);
            temp = (temp & 0xfffffff3_u32 as c_int) | ((priority & 3) << 2);
            temp = (temp & 0xffffffef_u32 as c_int) | ((valid & 1) << 4);
            temp |= FIFO_U1;
            temp = (temp & 0xffffffdf_u32 as c_int) | ((empty & 1) << 5);
            // CHIP_* conditional f-bit programming is supplied by the build configuration.
        }
    } else if (temp & FIFO_VALID) != 0 {
        // CHIP_* conditional invalidation bits are supplied by the build configuration.
    } else {
        vortex_fifo_clearwtdata(vortex, fifo, FIFO_SIZE);
    }
    hwwrite((*vortex).mmio, VORTEX_FIFO_WTCTRL + (fifo << 2), temp);
    hwread((*vortex).mmio, VORTEX_FIFO_WTCTRL + (fifo << 2));
}
// #endif

unsafe fn vortex_fifo_init(vortex: *mut vortex_t) {
    let mut addr = VORTEX_FIFO_ADBCTRL + ((NR_ADB - 1) * 4);
    let mut x = NR_ADB - 1;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, FIFO_U0 | FIFO_U1);
        if hwread((*vortex).mmio, addr) != (FIFO_U0 | FIFO_U1) {
            dev_err((*(*vortex).card).dev, b"bad adb fifo reset!\n\0".as_ptr());
        }
        vortex_fifo_clearadbdata(vortex, x, FIFO_SIZE);
        addr -= 4;
        x -= 1;
    }
    // #ifndef CHIP_AU8810
    addr = VORTEX_FIFO_WTCTRL + ((NR_WT - 1) * 4);
    x = NR_WT - 1;
    while x >= 0 {
        hwwrite((*vortex).mmio, addr, FIFO_U0);
        if hwread((*vortex).mmio, addr) != FIFO_U0 {
            dev_err((*(*vortex).card).dev, b"bad wt fifo reset (0x%08x, 0x%08x)!\n\0".as_ptr(), addr, hwread((*vortex).mmio, addr));
        }
        vortex_fifo_clearwtdata(vortex, x, FIFO_SIZE);
        addr -= 4;
        x -= 1;
    }
    // #endif
    // CHIP_AU8820 writes 0xf8c0; CHIP_AU8830 also triggers WT A/B. The common ADB trigger is translated.
    hwwrite((*vortex).mmio, 0x17008, 0x61);
}

unsafe fn vortex_adbdma_init(_vortex: *mut vortex_t) {}

unsafe fn vortex_adbdma_setfirstbuffer(vortex: *mut vortex_t, adbdma: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    hwwrite((*vortex).mmio, VORTEX_ADBDMA_CTRL + (adbdma << 2), (*dma).dma_ctrl);
}

unsafe fn vortex_adbdma_setstartbuffer(vortex: *mut vortex_t, adbdma: c_int, sb: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    hwwrite((*vortex).mmio, VORTEX_ADBDMA_START + (adbdma << 2), sb << ((0xf - (adbdma & 0xf)) * 2));
    (*dma).period_real = sb;
    (*dma).period_virt = sb;
}

unsafe fn vortex_adbdma_setbuffers(vortex: *mut vortex_t, adbdma: c_int, psize: c_int, count: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    (*dma).period_bytes = psize;
    (*dma).nr_periods = count;
    (*dma).cfg0 = 0;
    (*dma).cfg1 = 0;
    if count >= 4 || count != 1 && count != 2 && count != 3 {
        (*dma).cfg1 |= 0x88000000u32 as c_int | 0x44000000 | 0x30000000 | (psize - 1);
        hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (adbdma << 4) + 0xc, snd_pcm_sgbuf_get_addr((*dma).substream, psize * 3));
    }
    if count >= 3 || count > 4 || count != 1 && count != 2 {
        (*dma).cfg0 |= 0x12000000;
        (*dma).cfg1 |= 0x80000000u32 as c_int | 0x40000000 | ((psize - 1) << 0xc);
        hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (adbdma << 4) + 0x8, snd_pcm_sgbuf_get_addr((*dma).substream, psize * 2));
    }
    if count >= 2 || count > 4 || count != 1 {
        (*dma).cfg0 |= 0x88000000u32 as c_int | 0x44000000 | 0x10000000 | (psize - 1);
        hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (adbdma << 4) + 0x4, snd_pcm_sgbuf_get_addr((*dma).substream, psize));
    }
    (*dma).cfg0 |= 0x80000000u32 as c_int | 0x40000000 | ((psize - 1) << 0xc);
    hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (adbdma << 4), snd_pcm_sgbuf_get_addr((*dma).substream, 0));
    hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFCFG0 + (adbdma << 3), (*dma).cfg0);
    hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFCFG1 + (adbdma << 3), (*dma).cfg1);
    vortex_adbdma_setfirstbuffer(vortex, adbdma);
    vortex_adbdma_setstartbuffer(vortex, adbdma, 0);
}

unsafe fn vortex_adbdma_setmode(vortex: *mut vortex_t, adbdma: c_int, ie: c_int, dir: c_int, fmt: c_int, stereo: c_int, offset: u32) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    (*dma).dma_unknown = stereo;
    (*dma).dma_ctrl = ((offset as c_int & OFFSET_MASK) | ((*dma).dma_ctrl & !OFFSET_MASK));
    (*dma).dma_ctrl = ((*dma).dma_ctrl & !IE_MASK) | ((ie << IE_SHIFT) & IE_MASK);
    (*dma).dma_ctrl = ((*dma).dma_ctrl & !DIR_MASK) | ((dir << DIR_SHIFT) & DIR_MASK);
    (*dma).dma_ctrl = ((*dma).dma_ctrl & !FMT_MASK) | ((fmt << FMT_SHIFT) & FMT_MASK);
    hwwrite((*vortex).mmio, VORTEX_ADBDMA_CTRL + (adbdma << 2), (*dma).dma_ctrl);
    hwread((*vortex).mmio, VORTEX_ADBDMA_CTRL + (adbdma << 2));
}

unsafe fn vortex_adbdma_bufshift(vortex: *mut vortex_t, adbdma: c_int) -> c_int {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let page = (hwread((*vortex).mmio, VORTEX_ADBDMA_STAT + (adbdma << 2)) & ADB_SUBBUF_MASK) >> ADB_SUBBUF_SHIFT;
    let mut delta = if (*dma).nr_periods >= 4 { (page - (*dma).period_real) & 3 } else { page - (*dma).period_real };
    if (*dma).nr_periods < 4 && delta < 0 {
        delta += (*dma).nr_periods;
    }
    if delta == 0 {
        return 0;
    }
    if (*dma).nr_periods > 4 {
        let mut i = 0;
        while i < delta {
            let mut p = (*dma).period_virt + i + 4;
            if p >= (*dma).nr_periods {
                p -= (*dma).nr_periods;
            }
            let mut pp = (*dma).period_real + i;
            if pp >= 4 {
                pp -= 4;
            }
            hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (((adbdma << 2) + pp) << 2), snd_pcm_sgbuf_get_addr((*dma).substream, (*dma).period_bytes * p));
            hwread((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (((adbdma << 2) + pp) << 2));
            i += 1;
        }
    }
    (*dma).period_virt += delta;
    (*dma).period_real = page;
    if (*dma).period_virt >= (*dma).nr_periods {
        (*dma).period_virt -= (*dma).nr_periods;
    }
    if delta != 1 {
        dev_info((*(*vortex).card).dev, b"%d virt=%d, real=%d, delta=%d\n\0".as_ptr(), adbdma, (*dma).period_virt, (*dma).period_real, delta);
    }
    delta
}

unsafe fn vortex_adbdma_resetup(vortex: *mut vortex_t, adbdma: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let mut i = 0;
    while i < 4 && i < (*dma).nr_periods {
        let mut p = (*dma).period_virt + i;
        if p >= (*dma).nr_periods {
            p -= (*dma).nr_periods;
        }
        let mut pp = (*dma).period_real + i;
        if (*dma).nr_periods < 4 {
            if pp >= (*dma).nr_periods {
                pp -= (*dma).nr_periods;
            }
        } else if pp >= 4 {
            pp -= 4;
        }
        hwwrite((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (((adbdma << 2) + pp) << 2), snd_pcm_sgbuf_get_addr((*dma).substream, (*dma).period_bytes * p));
        hwread((*vortex).mmio, VORTEX_ADBDMA_BUFBASE + (((adbdma << 2) + pp) << 2));
        i += 1;
    }
}

unsafe fn vortex_adbdma_getlinearpos(vortex: *mut vortex_t, adbdma: c_int) -> c_int {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let temp = hwread((*vortex).mmio, VORTEX_ADBDMA_STAT + (adbdma << 2));
    let page = (temp & ADB_SUBBUF_MASK) >> ADB_SUBBUF_SHIFT;
    let mut delta = if (*dma).nr_periods >= 4 { (page - (*dma).period_real) & 3 } else { page - (*dma).period_real };
    if (*dma).nr_periods < 4 && delta < 0 {
        delta += (*dma).nr_periods;
    }
    ((*dma).period_virt + delta) * (*dma).period_bytes + (temp & ((*dma).period_bytes - 1))
}

unsafe fn vortex_adbdma_startfifo(vortex: *mut vortex_t, adbdma: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let mut this_8 = 0;
    let this_4 = 0;
    match (*dma).fifo_status {
        FIFO_START => vortex_fifo_setadbvalid(vortex, adbdma, if (*dma).fifo_enabled != 0 { 1 } else { 0 }),
        FIFO_STOP => {
            this_8 = 1;
            hwwrite((*vortex).mmio, VORTEX_ADBDMA_CTRL + (adbdma << 2), (*dma).dma_ctrl);
            vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0);
        }
        FIFO_PAUSE => vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0),
        _ => {}
    }
    (*dma).fifo_status = FIFO_START;
}

unsafe fn vortex_adbdma_resumefifo(vortex: *mut vortex_t, adbdma: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let this_8 = 1;
    let this_4 = 0;
    match (*dma).fifo_status {
        FIFO_STOP => {
            hwwrite((*vortex).mmio, VORTEX_ADBDMA_CTRL + (adbdma << 2), (*dma).dma_ctrl);
            vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0);
        }
        FIFO_PAUSE => vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0),
        _ => {}
    }
    (*dma).fifo_status = FIFO_START;
}

unsafe fn vortex_adbdma_pausefifo(vortex: *mut vortex_t, adbdma: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let this_8 = 0;
    let this_4 = 0;
    match (*dma).fifo_status {
        FIFO_START => vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, 0, 0),
        FIFO_STOP => {
            hwwrite((*vortex).mmio, VORTEX_ADBDMA_CTRL + (adbdma << 2), (*dma).dma_ctrl);
            vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, 0, 0);
        }
        _ => {}
    }
    (*dma).fifo_status = FIFO_PAUSE;
}

unsafe fn vortex_adbdma_stopfifo(vortex: *mut vortex_t, adbdma: c_int) {
    let dma = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let this_4 = 0;
    let this_8 = 0;
    if (*dma).fifo_status == FIFO_START {
        vortex_fifo_setadbctrl(vortex, adbdma, (*dma).dma_unknown, this_4, this_8, 0, 0);
    } else if (*dma).fifo_status == FIFO_STOP {
        return;
    }
    (*dma).fifo_status = FIFO_STOP;
    (*dma).fifo_enabled = 0;
}

// WTDMA functions are translated under the original #ifndef CHIP_AU8810 intent.
unsafe fn vortex_wtdma_setfirstbuffer(vortex: *mut vortex_t, wtdma: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    hwwrite((*vortex).mmio, VORTEX_WTDMA_CTRL + (wtdma << 2), (*dma).dma_ctrl);
}

unsafe fn vortex_wtdma_setstartbuffer(vortex: *mut vortex_t, wtdma: c_int, sb: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    hwwrite((*vortex).mmio, VORTEX_WTDMA_START + (wtdma << 2), sb << ((0xf - (wtdma & 0xf)) * 2));
    (*dma).period_real = sb;
    (*dma).period_virt = sb;
}

unsafe fn vortex_wtdma_setbuffers(vortex: *mut vortex_t, wtdma: c_int, psize: c_int, count: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    (*dma).period_bytes = psize;
    (*dma).nr_periods = count;
    (*dma).cfg0 = 0;
    (*dma).cfg1 = 0;
    if count >= 4 || count != 1 && count != 2 && count != 3 {
        (*dma).cfg1 |= 0x88000000u32 as c_int | 0x44000000 | 0x30000000 | (psize - 1);
        hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFBASE + (wtdma << 4) + 0xc, snd_pcm_sgbuf_get_addr((*dma).substream, psize * 3));
    }
    if count >= 3 || count > 4 || count != 1 && count != 2 {
        (*dma).cfg0 |= 0x12000000;
        (*dma).cfg1 |= 0x80000000u32 as c_int | 0x40000000 | ((psize - 1) << 0xc);
        hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFBASE + (wtdma << 4) + 0x8, snd_pcm_sgbuf_get_addr((*dma).substream, psize * 2));
    }
    if count >= 2 || count > 4 || count != 1 {
        (*dma).cfg0 |= 0x88000000u32 as c_int | 0x44000000 | 0x10000000 | (psize - 1);
        hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFBASE + (wtdma << 4) + 0x4, snd_pcm_sgbuf_get_addr((*dma).substream, psize));
    }
    (*dma).cfg0 |= 0x80000000u32 as c_int | 0x40000000 | ((psize - 1) << 0xc);
    hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFBASE + (wtdma << 4), snd_pcm_sgbuf_get_addr((*dma).substream, 0));
    hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFCFG0 + (wtdma << 3), (*dma).cfg0);
    hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFCFG1 + (wtdma << 3), (*dma).cfg1);
    vortex_wtdma_setfirstbuffer(vortex, wtdma);
    vortex_wtdma_setstartbuffer(vortex, wtdma, 0);
}

unsafe fn vortex_wtdma_setmode(vortex: *mut vortex_t, wtdma: c_int, ie: c_int, fmt: c_int, d: c_int, offset: u32) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    (*dma).dma_unknown = d;
    (*dma).dma_ctrl = 0;
    (*dma).dma_ctrl = (offset as c_int & OFFSET_MASK) | ((*dma).dma_ctrl & !OFFSET_MASK);
    (*dma).dma_ctrl = ((*dma).dma_ctrl & !IE_MASK) | ((ie << IE_SHIFT) & IE_MASK);
    (*dma).dma_ctrl |= 1 << DIR_SHIFT;
    (*dma).dma_ctrl = ((*dma).dma_ctrl & FMT_MASK) | ((fmt << FMT_SHIFT) & FMT_MASK);
    hwwrite((*vortex).mmio, VORTEX_WTDMA_CTRL + (wtdma << 2), (*dma).dma_ctrl);
}

unsafe fn vortex_wtdma_bufshift(vortex: *mut vortex_t, wtdma: c_int) -> c_int {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    let page = (hwread((*vortex).mmio, VORTEX_WTDMA_STAT + (wtdma << 2)) >> WT_SUBBUF_SHIFT) & WT_SUBBUF_MASK;
    let mut delta = if (*dma).nr_periods >= 4 { (page - (*dma).period_real) & 3 } else { page - (*dma).period_real };
    if (*dma).nr_periods < 4 && delta < 0 {
        delta += (*dma).nr_periods;
    }
    if delta == 0 {
        return 0;
    }
    if (*dma).nr_periods > 4 {
        let mut i = 0;
        while i < delta {
            let mut p = (*dma).period_virt + i + 4;
            if p >= (*dma).nr_periods {
                p -= (*dma).nr_periods;
            }
            let mut pp = (*dma).period_real + i;
            if pp >= 4 {
                pp -= 4;
            }
            hwwrite((*vortex).mmio, VORTEX_WTDMA_BUFBASE + (((wtdma << 2) + pp) << 2), snd_pcm_sgbuf_get_addr((*dma).substream, (*dma).period_bytes * p));
            hwread((*vortex).mmio, VORTEX_WTDMA_BUFBASE + (((wtdma << 2) + pp) << 2));
            i += 1;
        }
    }
    (*dma).period_virt += delta;
    if (*dma).period_virt >= (*dma).nr_periods {
        (*dma).period_virt -= (*dma).nr_periods;
    }
    (*dma).period_real = page;
    if delta != 1 {
        dev_warn((*(*vortex).card).dev, b"wt virt = %d, delta = %d\n\0".as_ptr(), (*dma).period_virt, delta);
    }
    delta
}

unsafe fn vortex_wtdma_getlinearpos(vortex: *mut vortex_t, wtdma: c_int) -> c_int {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    let mut temp = hwread((*vortex).mmio, VORTEX_WTDMA_STAT + (wtdma << 2));
    temp = ((*dma).period_virt * (*dma).period_bytes) + (temp & ((*dma).period_bytes - 1));
    temp
}

unsafe fn vortex_wtdma_startfifo(vortex: *mut vortex_t, wtdma: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    let mut this_8 = 0;
    let this_4 = 0;
    match (*dma).fifo_status {
        FIFO_START => vortex_fifo_setwtvalid(vortex, wtdma, if (*dma).fifo_enabled != 0 { 1 } else { 0 }),
        FIFO_STOP => {
            this_8 = 1;
            hwwrite((*vortex).mmio, VORTEX_WTDMA_CTRL + (wtdma << 2), (*dma).dma_ctrl);
            vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0);
        }
        FIFO_PAUSE => vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0),
        _ => {}
    }
    (*dma).fifo_status = FIFO_START;
}

unsafe fn vortex_wtdma_resumefifo(vortex: *mut vortex_t, wtdma: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    let this_8 = 0;
    let this_4 = 0;
    match (*dma).fifo_status {
        FIFO_STOP => {
            hwwrite((*vortex).mmio, VORTEX_WTDMA_CTRL + (wtdma << 2), (*dma).dma_ctrl);
            vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0);
        }
        FIFO_PAUSE => vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, if (*dma).fifo_enabled != 0 { 1 } else { 0 }, 0),
        _ => {}
    }
    (*dma).fifo_status = FIFO_START;
}

unsafe fn vortex_wtdma_pausefifo(vortex: *mut vortex_t, wtdma: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    let this_8 = 0;
    let this_4 = 0;
    match (*dma).fifo_status {
        FIFO_START => vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, 0, 0),
        FIFO_STOP => {
            hwwrite((*vortex).mmio, VORTEX_WTDMA_CTRL + (wtdma << 2), (*dma).dma_ctrl);
            vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, 0, 0);
        }
        _ => {}
    }
    (*dma).fifo_status = FIFO_PAUSE;
}

unsafe fn vortex_wtdma_stopfifo(vortex: *mut vortex_t, wtdma: c_int) {
    let dma = &mut (*vortex).dma_wt[wtdma as usize] as *mut stream_t;
    let this_4 = 0;
    let this_8 = 0;
    if (*dma).fifo_status == FIFO_START {
        vortex_fifo_setwtctrl(vortex, wtdma, (*dma).dma_unknown, this_4, this_8, 0, 0);
    } else if (*dma).fifo_status == FIFO_STOP {
        return;
    }
    (*dma).fifo_status = FIFO_STOP;
    (*dma).fifo_enabled = 0;
}

unsafe fn vortex_adb_init(vortex: *mut vortex_t) {
    hwwrite((*vortex).mmio, VORTEX_ADB_SR, 0);
    let mut i = 0;
    while i < VORTEX_ADB_RTBASE_COUNT {
        hwwrite((*vortex).mmio, VORTEX_ADB_RTBASE + (i << 2), hwread((*vortex).mmio, VORTEX_ADB_RTBASE + (i << 2)) | ROUTE_MASK);
        i += 1;
    }
    i = 0;
    while i < VORTEX_ADB_CHNBASE_COUNT {
        hwwrite((*vortex).mmio, VORTEX_ADB_CHNBASE + (i << 2), hwread((*vortex).mmio, VORTEX_ADB_CHNBASE + (i << 2)) | ROUTE_MASK);
        i += 1;
    }
}

unsafe fn vortex_adb_en_sr(vortex: *mut vortex_t, channel: c_int) {
    hwwrite((*vortex).mmio, VORTEX_ADB_SR, hwread((*vortex).mmio, VORTEX_ADB_SR) | (0x1 << channel));
}

unsafe fn vortex_adb_dis_sr(vortex: *mut vortex_t, channel: c_int) {
    hwwrite((*vortex).mmio, VORTEX_ADB_SR, hwread((*vortex).mmio, VORTEX_ADB_SR) & !(0x1 << channel));
}

unsafe fn vortex_adb_addroutes(vortex: *mut vortex_t, channel: c_uchar, route: *mut ADBRamLink, mut rnum: c_int) {
    if rnum <= 0 || route.is_null() {
        return;
    }
    rnum -= 1;
    hwwrite((*vortex).mmio, VORTEX_ADB_RTBASE + ((*route.add(rnum as usize) & ADB_MASK) << 2), ROUTE_MASK);
    while rnum > 0 {
        hwwrite((*vortex).mmio, VORTEX_ADB_RTBASE + ((*route.add((rnum - 1) as usize) & ADB_MASK) << 2), *route.add(rnum as usize));
        rnum -= 1;
    }
    let mut temp = hwread((*vortex).mmio, VORTEX_ADB_CHNBASE + ((channel as c_int) << 2)) & ADB_MASK;
    if temp == ADB_MASK {
        hwwrite((*vortex).mmio, VORTEX_ADB_CHNBASE + ((channel as c_int) << 2), *route);
        vortex_adb_en_sr(vortex, channel as c_int);
        return;
    }
    let mut lifeboat = 0;
    loop {
        let prev = temp;
        temp = hwread((*vortex).mmio, VORTEX_ADB_RTBASE + (temp << 2)) & ADB_MASK;
        if lifeboat > ADB_MASK {
            dev_err((*(*vortex).card).dev, b"vortex_adb_addroutes: unending route! 0x%x\n\0".as_ptr(), *route);
            return;
        }
        lifeboat += 1;
        if temp == ADB_MASK {
            hwwrite((*vortex).mmio, VORTEX_ADB_RTBASE + (prev << 2), *route);
            break;
        }
    }
}

unsafe fn vortex_adb_delroutes(vortex: *mut vortex_t, channel: c_uchar, route0: ADBRamLink, route1: ADBRamLink) {
    let mut lifeboat = 0;
    let mut temp = hwread((*vortex).mmio, VORTEX_ADB_CHNBASE + ((channel as c_int) << 2)) & ADB_MASK;
    if temp == (route0 & ADB_MASK) {
        temp = hwread((*vortex).mmio, VORTEX_ADB_RTBASE + ((route1 & ADB_MASK) << 2));
        if (temp & ADB_MASK) == ADB_MASK {
            vortex_adb_dis_sr(vortex, channel as c_int);
        }
        hwwrite((*vortex).mmio, VORTEX_ADB_CHNBASE + ((channel as c_int) << 2), temp);
        return;
    }
    let mut prev;
    loop {
        prev = temp;
        temp = hwread((*vortex).mmio, VORTEX_ADB_RTBASE + (prev << 2)) & ADB_MASK;
        if lifeboat > ADB_MASK || temp == ADB_MASK {
            dev_err((*(*vortex).card).dev, b"vortex_adb_delroutes: route not found! 0x%x\n\0".as_ptr(), route0);
            return;
        }
        lifeboat += 1;
        if temp == (route0 & ADB_MASK) {
            break;
        }
    }
    temp = hwread((*vortex).mmio, VORTEX_ADB_RTBASE + (temp << 2));
    if (temp & ADB_MASK) == route1 {
        temp = hwread((*vortex).mmio, VORTEX_ADB_RTBASE + (temp << 2));
    }
    hwwrite((*vortex).mmio, VORTEX_ADB_RTBASE + (prev << 2), temp);
}

unsafe fn vortex_route(vortex: *mut vortex_t, en: c_int, channel: c_uchar, source: c_uchar, dest: c_uchar) {
    let mut route: ADBRamLink = (((source as c_int) & ADB_MASK) << ADB_SHIFT) | ((dest as c_int) & ADB_MASK);
    if en != 0 {
        vortex_adb_addroutes(vortex, channel, &mut route, 1);
        if source as c_int < OFFSET_SRCOUT + NR_SRC && source as c_int >= OFFSET_SRCOUT {
            vortex_src_addWTD(vortex, (source as c_int - OFFSET_SRCOUT) as c_uchar, channel);
        } else if source as c_int < OFFSET_MIXOUT + NR_MIXOUT && source as c_int >= OFFSET_MIXOUT {
            vortex_mixer_addWTD(vortex, (source as c_int - OFFSET_MIXOUT) as c_uchar, channel);
        }
    } else {
        vortex_adb_delroutes(vortex, channel, route, route);
        if source as c_int < OFFSET_SRCOUT + NR_SRC && source as c_int >= OFFSET_SRCOUT {
            vortex_src_delWTD(vortex, (source as c_int - OFFSET_SRCOUT) as c_uchar, channel);
        } else if source as c_int < OFFSET_MIXOUT + NR_MIXOUT && source as c_int >= OFFSET_MIXOUT {
            vortex_mixer_delWTD(vortex, (source as c_int - OFFSET_MIXOUT) as c_uchar, channel);
        }
    }
}

unsafe fn vortex_routeLRT(vortex: *mut vortex_t, en: c_int, ch: c_uchar, source0: c_uchar, source1: c_uchar, dest: c_uchar) {
    let mut route = [0 as ADBRamLink; 2];
    route[0] = (((source0 as c_int) & ADB_MASK) << ADB_SHIFT) | ((dest as c_int) & ADB_MASK);
    route[1] = (((source1 as c_int) & ADB_MASK) << ADB_SHIFT) | ((dest as c_int) & ADB_MASK);
    if dest < 0x10 {
        route[1] = (route[1] & !ADB_MASK) | (dest as c_int + 0x20);
    }
    if en != 0 {
        vortex_adb_addroutes(vortex, ch, route.as_mut_ptr(), 2);
        if source0 as c_int < OFFSET_SRCOUT + NR_SRC && source0 as c_int >= OFFSET_SRCOUT {
            vortex_src_addWTD(vortex, (source0 as c_int - OFFSET_SRCOUT) as c_uchar, ch);
            vortex_src_addWTD(vortex, (source1 as c_int - OFFSET_SRCOUT) as c_uchar, ch);
        } else if source0 as c_int < OFFSET_MIXOUT + NR_MIXOUT && source0 as c_int >= OFFSET_MIXOUT {
            vortex_mixer_addWTD(vortex, (source0 as c_int - OFFSET_MIXOUT) as c_uchar, ch);
            vortex_mixer_addWTD(vortex, (source1 as c_int - OFFSET_MIXOUT) as c_uchar, ch);
        }
    } else {
        vortex_adb_delroutes(vortex, ch, route[0], route[1]);
        if source0 as c_int < OFFSET_SRCOUT + NR_SRC && source0 as c_int >= OFFSET_SRCOUT {
            vortex_src_delWTD(vortex, (source0 as c_int - OFFSET_SRCOUT) as c_uchar, ch);
            vortex_src_delWTD(vortex, (source1 as c_int - OFFSET_SRCOUT) as c_uchar, ch);
        } else if source0 as c_int < OFFSET_MIXOUT + NR_MIXOUT && source0 as c_int >= OFFSET_MIXOUT {
            vortex_mixer_delWTD(vortex, (source0 as c_int - OFFSET_MIXOUT) as c_uchar, ch);
            vortex_mixer_delWTD(vortex, (source1 as c_int - OFFSET_MIXOUT) as c_uchar, ch);
        }
    }
}

unsafe fn vortex_connection_adbdma_src(vortex: *mut vortex_t, en: c_int, ch: c_uchar, adbdma: c_uchar, src: c_uchar) {
    vortex_route(vortex, en, ch, ADB_DMA(adbdma), ADB_SRCIN(src));
}

unsafe fn vortex_connection_src_mixin(vortex: *mut vortex_t, en: c_int, channel: c_uchar, src: c_uchar, mixin: c_uchar) {
    vortex_route(vortex, en, channel, ADB_SRCOUT(src), ADB_MIXIN(mixin));
}

unsafe fn vortex_connection_mixin_mix(vortex: *mut vortex_t, en: c_int, mixin: c_uchar, mix: c_uchar, a: c_int) {
    if en != 0 {
        vortex_mix_enableinput(vortex, mix, mixin as c_int);
        vortex_mix_setinputvolumebyte(vortex, mix, mixin as c_int, MIX_DEFIGAIN);
    } else {
        vortex_mix_disableinput(vortex, mix, mixin as c_int, a);
    }
}

unsafe fn vortex_connection_adb_mixin(vortex: *mut vortex_t, en: c_int, channel: c_uchar, source: c_uchar, mixin: c_uchar) {
    vortex_route(vortex, en, channel, source, ADB_MIXIN(mixin));
}

unsafe fn vortex_connection_src_adbdma(vortex: *mut vortex_t, en: c_int, ch: c_uchar, src: c_uchar, adbdma: c_uchar) {
    vortex_route(vortex, en, ch, ADB_SRCOUT(src), ADB_DMA(adbdma));
}

unsafe fn vortex_connection_src_src_adbdma(vortex: *mut vortex_t, en: c_int, ch: c_uchar, src0: c_uchar, src1: c_uchar, adbdma: c_uchar) {
    vortex_routeLRT(vortex, en, ch, ADB_SRCOUT(src0), ADB_SRCOUT(src1), ADB_DMA(adbdma));
}

unsafe fn vortex_connection_mix_adb(vortex: *mut vortex_t, en: c_int, ch: c_uchar, mix: c_uchar, dest: c_uchar) {
    vortex_route(vortex, en, ch, ADB_MIXOUT(mix), dest);
    vortex_mix_setvolumebyte(vortex, mix, MIX_DEFOGAIN);
}

unsafe fn vortex_connection_mix_src(vortex: *mut vortex_t, en: c_int, ch: c_uchar, mix: c_uchar, src: c_uchar) {
    vortex_route(vortex, en, ch, ADB_MIXOUT(mix), ADB_SRCIN(src));
    vortex_mix_setvolumebyte(vortex, mix, MIX_DEFOGAIN);
}

unsafe fn vortex_connect_codecplay(vortex: *mut vortex_t, en: c_int, mixers: *mut c_uchar) {
    // CHIP_AU8820 routes directly to codec; other chips route front through EQ.
    vortex_connection_mix_adb(vortex, en, 0x11, *mixers.add(0), ADB_EQIN(0));
    vortex_connection_mix_adb(vortex, en, 0x11, *mixers.add(1), ADB_EQIN(1));
    vortex_mix_setvolumebyte(vortex, *mixers.add(0), 0);
    vortex_mix_setvolumebyte(vortex, *mixers.add(1), 0);
    vortex_route(vortex, en, 0x11, ADB_EQOUT(0), ADB_CODECOUT(0));
    vortex_route(vortex, en, 0x11, ADB_EQOUT(1), ADB_CODECOUT(1));
    if VORTEX_IS_QUAD(vortex) != 0 {
        vortex_connection_mix_adb(vortex, en, 0x11, *mixers.add(2), ADB_CODECOUT(4));
        vortex_connection_mix_adb(vortex, en, 0x11, *mixers.add(3), ADB_CODECOUT(5));
    }
}

unsafe fn vortex_connect_codecrec(vortex: *mut vortex_t, en: c_int, mixin0: c_uchar, mixin1: c_uchar) {
    vortex_connection_adb_mixin(vortex, en, 0x11, ADB_CODECIN(0), mixin0);
    vortex_connection_adb_mixin(vortex, en, 0x11, ADB_CODECIN(1), mixin1);
}

static resnum: [c_int; VORTEX_RESOURCE_LAST as usize] = [NR_ADB, NR_SRC, NR_MIXIN, NR_MIXOUT, NR_A3D];

unsafe fn vortex_adb_checkinout(vortex: *mut vortex_t, resmap: *mut c_int, out: c_int, restype: c_int) -> c_int {
    let qty = resnum[restype as usize];
    let mut resinuse = 0;
    if out != 0 {
        let mut i = 0;
        while i < NR_ADB {
            resinuse |= (*vortex).dma_adb[i as usize].resources[restype as usize];
            i += 1;
        }
        resinuse |= (*vortex).fixed_res[restype as usize];
        i = 0;
        while i < qty {
            if (resinuse & (1 << i)) == 0 {
                if !resmap.is_null() {
                    *resmap.add(restype as usize) |= 1 << i;
                } else {
                    (*vortex).dma_adb[i as usize].resources[restype as usize] |= 1 << i;
                }
                return i;
            }
            i += 1;
        }
    } else {
        if resmap.is_null() {
            return -EINVAL;
        }
        let mut i = 0;
        while i < qty {
            if (*resmap.add(restype as usize) & (1 << i)) != 0 {
                *resmap.add(restype as usize) &= !(1 << i);
                return i;
            }
            i += 1;
        }
    }
    dev_err((*(*vortex).card).dev, b"FATAL: ResManager: resource type %d exhausted.\n\0".as_ptr(), restype);
    -ENOMEM
}

unsafe fn vortex_connect_default(vortex: *mut vortex_t, en: c_int) {
    (*vortex).mixplayb[0] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT) as c_uchar;
    (*vortex).mixplayb[1] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT) as c_uchar;
    if VORTEX_IS_QUAD(vortex) != 0 {
        (*vortex).mixplayb[2] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT) as c_uchar;
        (*vortex).mixplayb[3] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT) as c_uchar;
    }
    vortex_connect_codecplay(vortex, en, (*vortex).mixplayb.as_mut_ptr());
    (*vortex).mixcapt[0] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXIN) as c_uchar;
    (*vortex).mixcapt[1] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXIN) as c_uchar;
    vortex_connect_codecrec(vortex, en, MIX_CAPT(0), MIX_CAPT(1));
    // #ifndef CHIP_AU8820
    (*vortex).mixspdif[0] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT) as c_uchar;
    (*vortex).mixspdif[1] = vortex_adb_checkinout(vortex, (*vortex).fixed_res.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT) as c_uchar;
    vortex_connection_mix_adb(vortex, en, 0x14, (*vortex).mixspdif[0], ADB_SPDIFOUT(0));
    vortex_connection_mix_adb(vortex, en, 0x14, (*vortex).mixspdif[1], ADB_SPDIFOUT(1));
    // #endif
    // #ifndef CHIP_AU8810
    vortex_wt_connect(vortex, en);
    // #endif
    // #ifndef CHIP_AU8820
    vortex_Vort3D_connect(vortex, en);
    // #endif
}

unsafe fn vortex_adb_allocroute(vortex: *mut vortex_t, mut dma: c_int, nr_ch: c_int, dir: c_int, type_: c_int, subdev: c_int) -> c_int {
    let en: c_int;
    if dma >= 0 {
        en = 0;
        vortex_adb_checkinout(vortex, (*vortex).dma_adb[dma as usize].resources.as_mut_ptr(), en, VORTEX_RESOURCE_DMA);
    } else {
        en = 1;
        dma = vortex_adb_checkinout(vortex, core::ptr::null_mut(), en, VORTEX_RESOURCE_DMA);
        if dma < 0 {
            return -EBUSY;
        }
    }
    let stream = &mut (*vortex).dma_adb[dma as usize] as *mut stream_t;
    (*stream).dma = dma;
    (*stream).dir = dir;
    (*stream).type_ = type_;
    if dir == SNDRV_PCM_STREAM_PLAYBACK {
        let mut src = [0; 4];
        let mut mix = [0; 4];
        let mut i = 0;
        if (*stream).type_ != VORTEX_PCM_SPDIF {
            while i < nr_ch {
                src[i as usize] = vortex_adb_checkinout(vortex, (*stream).resources.as_mut_ptr(), en, VORTEX_RESOURCE_SRC);
                if src[i as usize] < 0 {
                    (*stream).resources.fill(0);
                    return -EBUSY;
                }
                if (*stream).type_ != VORTEX_PCM_A3D {
                    mix[i as usize] = vortex_adb_checkinout(vortex, (*stream).resources.as_mut_ptr(), en, VORTEX_RESOURCE_MIXIN);
                    if mix[i as usize] < 0 {
                        (*stream).resources.fill(0);
                        return -EBUSY;
                    }
                }
                i += 1;
            }
        }
        // #ifndef CHIP_AU8820
        if (*stream).type_ == VORTEX_PCM_A3D {
            let a3d = vortex_adb_checkinout(vortex, (*stream).resources.as_mut_ptr(), en, VORTEX_RESOURCE_A3D);
            if a3d < 0 {
                (*stream).resources.fill(0);
                dev_err((*(*vortex).card).dev, b"out of A3D sources. Sorry\n\0".as_ptr());
                return -EBUSY;
            }
            vortex_Vort3D_InitializeSource(&mut (*vortex).a3d[a3d as usize], en, vortex);
        }
        if (*stream).type_ == VORTEX_PCM_SPDIF && en != 0 {
            vortex_route(vortex, 0, 0x14, ADB_MIXOUT((*vortex).mixspdif[0]), ADB_SPDIFOUT(0));
            vortex_route(vortex, 0, 0x14, ADB_MIXOUT((*vortex).mixspdif[1]), ADB_SPDIFOUT(1));
        }
        // #endif
        i = 0;
        while i < nr_ch {
            if (*stream).type_ == VORTEX_PCM_ADB {
                vortex_connection_adbdma_src(vortex, en, src[(nr_ch - 1) as usize] as c_uchar, dma as c_uchar, src[i as usize] as c_uchar);
                vortex_connection_src_mixin(vortex, en, 0x11, src[i as usize] as c_uchar, mix[i as usize] as c_uchar);
                vortex_connection_mixin_mix(vortex, en, mix[i as usize] as c_uchar, MIX_PLAYB(i), 0);
                vortex_connection_mixin_mix(vortex, en, mix[i as usize] as c_uchar, MIX_SPDIF(i % 2), 0);
                vortex_mix_setinputvolumebyte(vortex, MIX_SPDIF(i % 2), mix[i as usize], MIX_DEFIGAIN);
            }
            if (*stream).type_ == VORTEX_PCM_A3D {
                vortex_connection_adbdma_src(vortex, en, src[(nr_ch - 1) as usize] as c_uchar, dma as c_uchar, src[i as usize] as c_uchar);
                vortex_route(vortex, en, 0x11, ADB_SRCOUT(src[i as usize] as c_uchar), ADB_A3DIN(0));
            }
            if (*stream).type_ == VORTEX_PCM_SPDIF {
                vortex_route(vortex, en, 0x14, ADB_DMA((*stream).dma as c_uchar), ADB_SPDIFOUT(i as c_uchar));
            }
            i += 1;
        }
        if (*stream).type_ != VORTEX_PCM_SPDIF && (*stream).type_ != VORTEX_PCM_A3D {
            let ch_top = if VORTEX_IS_QUAD(vortex) != 0 { 4 } else { 2 };
            i = nr_ch;
            while i < ch_top {
                vortex_connection_mixin_mix(vortex, en, mix[(i % nr_ch) as usize] as c_uchar, MIX_PLAYB(i), 0);
                vortex_connection_mixin_mix(vortex, en, mix[(i % nr_ch) as usize] as c_uchar, MIX_SPDIF(i % 2), 0);
                vortex_mix_setinputvolumebyte(vortex, MIX_SPDIF(i % 2), mix[(i % nr_ch) as usize], MIX_DEFIGAIN);
                i += 1;
            }
            if (*stream).type_ == VORTEX_PCM_ADB && en != 0 {
                let p = &mut (*vortex).pcm_vol[subdev as usize] as *mut pcm_vol;
                (*p).dma = dma;
                i = 0;
                while i < nr_ch {
                    (*p).mixin[i as usize] = mix[i as usize];
                    i += 1;
                }
                i = 0;
                while i < ch_top {
                    (*p).vol[i as usize] = 0;
                    i += 1;
                }
            }
        } else {
            if nr_ch == 1 && (*stream).type_ == VORTEX_PCM_SPDIF {
                vortex_route(vortex, en, 0x14, ADB_DMA((*stream).dma as c_uchar), ADB_SPDIFOUT(1));
            }
        }
        if (*stream).type_ == VORTEX_PCM_SPDIF && en == 0 {
            vortex_route(vortex, 1, 0x14, ADB_MIXOUT((*vortex).mixspdif[0]), ADB_SPDIFOUT(0));
            vortex_route(vortex, 1, 0x14, ADB_MIXOUT((*vortex).mixspdif[1]), ADB_SPDIFOUT(1));
        }
    } else {
        let mut src = [0; 2];
        let mut mix = [0; 2];
        if nr_ch < 1 {
            return -EINVAL;
        }
        let mut i = 0;
        while i < nr_ch {
            mix[i as usize] = vortex_adb_checkinout(vortex, (*stream).resources.as_mut_ptr(), en, VORTEX_RESOURCE_MIXOUT);
            if mix[i as usize] < 0 {
                (*stream).resources.fill(0);
                return -EBUSY;
            }
            src[i as usize] = vortex_adb_checkinout(vortex, (*stream).resources.as_mut_ptr(), en, VORTEX_RESOURCE_SRC);
            if src[i as usize] < 0 {
                (*stream).resources.fill(0);
                return -EBUSY;
            }
            i += 1;
        }
        vortex_connection_mixin_mix(vortex, en, MIX_CAPT(0), mix[0] as c_uchar, 0);
        vortex_connection_mix_src(vortex, en, 0x11, mix[0] as c_uchar, src[0] as c_uchar);
        if nr_ch == 1 {
            vortex_connection_mixin_mix(vortex, en, MIX_CAPT(1), mix[0] as c_uchar, 0);
            vortex_connection_src_adbdma(vortex, en, src[0] as c_uchar, src[0] as c_uchar, dma as c_uchar);
        } else {
            vortex_connection_mixin_mix(vortex, en, MIX_CAPT(1), mix[1] as c_uchar, 0);
            vortex_connection_mix_src(vortex, en, 0x11, mix[1] as c_uchar, src[1] as c_uchar);
            vortex_connection_src_src_adbdma(vortex, en, src[1] as c_uchar, src[0] as c_uchar, src[1] as c_uchar, dma as c_uchar);
        }
    }
    (*vortex).dma_adb[dma as usize].nr_ch = nr_ch;
    dma
}

unsafe fn vortex_adb_setsrc(vortex: *mut vortex_t, adbdma: c_int, rate: c_uint, dir: c_int) {
    let stream = &mut (*vortex).dma_adb[adbdma as usize] as *mut stream_t;
    let cvrt = if dir != 0 { SRC_RATIO(rate, 48000) } else { SRC_RATIO(48000, rate) };
    let mut i = 0;
    while i < NR_SRC {
        if ((*stream).resources[VORTEX_RESOURCE_SRC as usize] & (1 << i)) != 0 {
            vortex_src_setupchannel(vortex, i as c_uchar, cvrt as c_uint, 0, 0, i, dir, 1, cvrt as c_uint, dir);
        }
        i += 1;
    }
}

unsafe fn vortex_settimer(vortex: *mut vortex_t, period: c_int) {
    hwwrite((*vortex).mmio, VORTEX_IRQ_STAT, period);
}

unsafe fn vortex_enable_int(card: *mut vortex_t) {
    hwwrite((*card).mmio, VORTEX_CTRL, hwread((*card).mmio, VORTEX_CTRL) | CTRL_IRQ_ENABLE);
    hwwrite((*card).mmio, VORTEX_IRQ_CTRL, (hwread((*card).mmio, VORTEX_IRQ_CTRL) & 0xffffefc0u32 as c_int) | 0x24);
}

unsafe fn vortex_disable_int(card: *mut vortex_t) {
    hwwrite((*card).mmio, VORTEX_CTRL, hwread((*card).mmio, VORTEX_CTRL) & !CTRL_IRQ_ENABLE);
}

unsafe fn vortex_interrupt(_irq: c_int, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let vortex = dev_id as *mut vortex_t;
    if (hwread((*vortex).mmio, VORTEX_STAT) & 0x1) == 0 {
        return IRQ_NONE;
    }
    if (hwread((*vortex).mmio, VORTEX_CTRL) & CTRL_IRQ_ENABLE) == 0 {
        return IRQ_NONE;
    }
    let source = hwread((*vortex).mmio, VORTEX_IRQ_SOURCE);
    hwwrite((*vortex).mmio, VORTEX_IRQ_SOURCE, source);
    hwread((*vortex).mmio, VORTEX_IRQ_SOURCE);
    if source == 0 {
        dev_err((*(*vortex).card).dev, b"missing irq source\n\0".as_ptr());
        return IRQ_NONE;
    }
    let mut handled = 0;
    if unlikely(source & IRQ_ERR_MASK) != 0 {
        if (source & IRQ_FATAL) != 0 { dev_err((*(*vortex).card).dev, b"IRQ fatal error\n\0".as_ptr()); }
        if (source & IRQ_PARITY) != 0 { dev_err((*(*vortex).card).dev, b"IRQ parity error\n\0".as_ptr()); }
        if (source & IRQ_REG) != 0 { dev_err((*(*vortex).card).dev, b"IRQ reg error\n\0".as_ptr()); }
        if (source & IRQ_FIFO) != 0 { dev_err((*(*vortex).card).dev, b"IRQ fifo error\n\0".as_ptr()); }
        if (source & IRQ_DMA) != 0 { dev_err((*(*vortex).card).dev, b"IRQ dma error\n\0".as_ptr()); }
        handled = 1;
    }
    if (source & IRQ_PCMOUT) != 0 {
        spin_lock(&mut (*vortex).lock);
        let mut i = 0;
        while i < NR_ADB {
            if (*vortex).dma_adb[i as usize].fifo_status == FIFO_START {
                if vortex_adbdma_bufshift(vortex, i) != 0 {
                    spin_unlock(&mut (*vortex).lock);
                    snd_pcm_period_elapsed((*vortex).dma_adb[i as usize].substream);
                    spin_lock(&mut (*vortex).lock);
                }
            }
            i += 1;
        }
        // #ifndef CHIP_AU8810
        i = 0;
        while i < NR_WT {
            if (*vortex).dma_wt[i as usize].fifo_status == FIFO_START {
                vortex_wtdma_bufshift(vortex, i);
                spin_unlock(&mut (*vortex).lock);
                snd_pcm_period_elapsed((*vortex).dma_wt[i as usize].substream);
                spin_lock(&mut (*vortex).lock);
            }
            i += 1;
        }
        // #endif
        spin_unlock(&mut (*vortex).lock);
        handled = 1;
    }
    if (source & IRQ_TIMER) != 0 {
        hwread((*vortex).mmio, VORTEX_IRQ_STAT);
        handled = 1;
    }
    if (source & IRQ_MIDI) != 0 && !(*vortex).rmidi.is_null() {
        snd_mpu401_uart_interrupt((*vortex).irq, (*(*vortex).rmidi).private_data);
        handled = 1;
    }
    if handled == 0 {
        dev_err((*(*vortex).card).dev, b"unknown irq source %x\n\0".as_ptr(), source);
    }
    IRQ_RETVAL(handled)
}

const POLL_COUNT: c_uint = 1000;

unsafe fn vortex_codec_init(vortex: *mut vortex_t) {
    let mut i = 0;
    while i < 32 {
        hwwrite((*vortex).mmio, VORTEX_CODEC_CHN + (i << 2), -i);
        msleep(2);
        i += 1;
    }
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0x00a8);
    msleep(2);
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0x80a8);
    msleep(2);
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0x80e8);
    msleep(2);
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0x80a8);
    msleep(2);
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0x00a8);
    msleep(2);
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0x00e8);
    i = 0;
    while i < 32 {
        hwwrite((*vortex).mmio, VORTEX_CODEC_CHN + (i << 2), -i);
        msleep(5);
        i += 1;
    }
    hwwrite((*vortex).mmio, VORTEX_CODEC_CTRL, 0xe8);
    msleep(1);
    hwwrite((*vortex).mmio, VORTEX_CODEC_EN, hwread((*vortex).mmio, VORTEX_CODEC_EN) | EN_CODEC);
}

unsafe fn vortex_codec_write(codec: *mut snd_ac97, addr: c_ushort, data: c_ushort) {
    let card = (*codec).private_data as *mut vortex_t;
    let mut lifeboat: c_uint = 0;
    while (hwread((*card).mmio, VORTEX_CODEC_CTRL) & 0x100) == 0 {
        udelay(100);
        if lifeboat > POLL_COUNT {
            dev_err((*(*card).card).dev, b"ac97 codec stuck busy\n\0".as_ptr());
            return;
        }
        lifeboat += 1;
    }
    hwwrite((*card).mmio, VORTEX_CODEC_IO,
        (((addr as c_int) << VORTEX_CODEC_ADDSHIFT) & VORTEX_CODEC_ADDMASK)
        | (((data as c_int) << VORTEX_CODEC_DATSHIFT) & VORTEX_CODEC_DATMASK)
        | VORTEX_CODEC_WRITE
        | ((*codec).num << VORTEX_CODEC_ID_SHIFT));
    hwread((*card).mmio, VORTEX_CODEC_IO);
}

unsafe fn vortex_codec_read(codec: *mut snd_ac97, addr: c_ushort) -> c_ushort {
    let card = (*codec).private_data as *mut vortex_t;
    let mut lifeboat: c_uint = 0;
    while (hwread((*card).mmio, VORTEX_CODEC_CTRL) & 0x100) == 0 {
        udelay(100);
        if lifeboat > POLL_COUNT {
            dev_err((*(*card).card).dev, b"ac97 codec stuck busy\n\0".as_ptr());
            return 0xffff;
        }
        lifeboat += 1;
    }
    let read_addr = (((addr as c_int) << VORTEX_CODEC_ADDSHIFT) & VORTEX_CODEC_ADDMASK) | ((*codec).num << VORTEX_CODEC_ID_SHIFT);
    hwwrite((*card).mmio, VORTEX_CODEC_IO, read_addr);
    let mut data;
    loop {
        udelay(100);
        data = hwread((*card).mmio, VORTEX_CODEC_IO);
        if lifeboat > POLL_COUNT {
            dev_err((*(*card).card).dev, b"ac97 address never arrived\n\0".as_ptr());
            return 0xffff;
        }
        lifeboat += 1;
        if (data & VORTEX_CODEC_ADDMASK) == ((addr as c_int) << VORTEX_CODEC_ADDSHIFT) {
            break;
        }
    }
    (data & VORTEX_CODEC_DATMASK) as c_ushort
}

unsafe fn vortex_spdif_init(vortex: *mut vortex_t, mut spdif_sr: c_int, spdif_mode: c_int) {
    let mut this_38 = 0;
    let this_04 = 0;
    let this_08 = 0;
    let this_0c = 0;
    hwwrite((*vortex).mmio, VORTEX_SPDIF_FLAGS, hwread((*vortex).mmio, VORTEX_SPDIF_FLAGS) & 0xfff3fffdu32 as c_int);
    let mut i = 0;
    while i < 11 {
        hwwrite((*vortex).mmio, VORTEX_SPDIF_CFG1 + (i << 2), 0);
        i += 1;
    }
    hwwrite((*vortex).mmio, VORTEX_CODEC_EN, hwread((*vortex).mmio, VORTEX_CODEC_EN) | EN_SPDIF);
    if this_04 != 0 && this_08 != 0 {
        let t = (((0x5DC00000 / spdif_sr) + 1) >> 1);
        let edi = if t > 0x800 { if t < 0x1ffff { t >> 1 } else { 0x1ffff } } else { 0x800 };
        vortex_src_setupchannel(vortex, this_04 as c_uchar, edi as c_uint, 0, 1, this_0c, 1, 0, edi as c_uint, 1);
        vortex_src_setupchannel(vortex, this_08 as c_uchar, edi as c_uint, 0, 1, this_0c, 1, 0, edi as c_uint, 1);
    }
    i = spdif_sr;
    spdif_sr |= 0x8c;
    match i {
        32000 => {
            this_38 &= 0xFFFFFFFEu32 as c_int;
            this_38 &= 0xFFFFFFFDu32 as c_int;
            this_38 &= 0xF3FFFFFFu32 as c_int;
            this_38 |= 0x03000000;
            this_38 &= 0xFFFFFF3Fu32 as c_int;
            spdif_sr &= 0xFFFFFFFDu32 as c_int;
            spdif_sr |= 1;
        }
        44100 => {
            this_38 &= 0xFFFFFFFEu32 as c_int;
            this_38 &= 0xFFFFFFFDu32 as c_int;
            this_38 &= 0xF0FFFFFFu32 as c_int;
            this_38 |= 0x03000000;
            this_38 &= 0xFFFFFF3Fu32 as c_int;
            spdif_sr &= 0xFFFFFFFCu32 as c_int;
        }
        48000 => {
            if spdif_mode == 1 {
                this_38 &= 0xFFFFFFFEu32 as c_int;
                this_38 &= 0xFFFFFFFDu32 as c_int;
                this_38 &= 0xF2FFFFFFu32 as c_int;
                this_38 |= 0x02000000;
                this_38 &= 0xFFFFFF3Fu32 as c_int;
            } else {
                this_38 |= 0x00000003;
                this_38 &= 0xFFFFFFBFu32 as c_int;
                this_38 |= 0x80;
            }
            spdif_sr |= 2;
            spdif_sr &= 0xFFFFFFFEu32 as c_int;
        }
        _ => {}
    }
    hwwrite((*vortex).mmio, VORTEX_SPDIF_CFG0, this_38 & 0xffff);
    hwwrite((*vortex).mmio, VORTEX_SPDIF_CFG1, this_38 >> 0x10);
    hwwrite((*vortex).mmio, VORTEX_SPDIF_SMPRATE, spdif_sr);
}

unsafe fn vortex_core_init(vortex: *mut vortex_t) -> c_int {
    dev_info((*(*vortex).card).dev, b"init started\n\0".as_ptr());
    hwwrite((*vortex).mmio, VORTEX_CTRL, 0xffffffffu32 as c_int);
    msleep(5);
    hwwrite((*vortex).mmio, VORTEX_CTRL, hwread((*vortex).mmio, VORTEX_CTRL) & 0xffdfffffu32 as c_int);
    msleep(5);
    hwwrite((*vortex).mmio, VORTEX_IRQ_SOURCE, 0xffffffffu32 as c_int);
    hwread((*vortex).mmio, VORTEX_IRQ_STAT);
    vortex_codec_init(vortex);
    // #ifdef CHIP_AU8830
    hwwrite((*vortex).mmio, VORTEX_CTRL, hwread((*vortex).mmio, VORTEX_CTRL) | 0x1000000);
    // #endif
    vortex_adbdma_init(vortex);
    hwwrite((*vortex).mmio, VORTEX_ENGINE_CTRL, 0);
    vortex_adb_init(vortex);
    vortex_fifo_init(vortex);
    vortex_mixer_init(vortex);
    vortex_srcblock_init(vortex);
    // #ifndef CHIP_AU8820
    vortex_eq_init(vortex);
    vortex_spdif_init(vortex, 48000, 1);
    vortex_Vort3D_enable(vortex);
    // #endif
    // #ifndef CHIP_AU8810
    vortex_wt_init(vortex);
    // #endif
    vortex_settimer(vortex, 0x90);
    dev_info((*(*vortex).card).dev, b"init.... done.\n\0".as_ptr());
    spin_lock_init(&mut (*vortex).lock);
    0
}

unsafe fn vortex_core_shutdown(vortex: *mut vortex_t) -> c_int {
    dev_info((*(*vortex).card).dev, b"shutdown started\n\0".as_ptr());
    // #ifndef CHIP_AU8820
    vortex_eq_free(vortex);
    vortex_Vort3D_disable(vortex);
    // #endif
    vortex_disable_int(vortex);
    vortex_connect_default(vortex, 0);
    vortex_fifo_init(vortex);
    vortex_adb_init(vortex);
    hwwrite((*vortex).mmio, VORTEX_IRQ_CTRL, 0);
    hwwrite((*vortex).mmio, VORTEX_CTRL, 0);
    msleep(5);
    hwwrite((*vortex).mmio, VORTEX_IRQ_SOURCE, 0xffff);
    dev_info((*(*vortex).card).dev, b"shutdown.... done.\n\0".as_ptr());
    0
}

unsafe fn vortex_alsafmt_aspfmt(alsafmt: snd_pcm_format_t, v: *mut vortex_t) -> c_int {
    let fmt;
    match alsafmt {
        SNDRV_PCM_FORMAT_U8 => fmt = 0x1,
        SNDRV_PCM_FORMAT_MU_LAW => fmt = 0x2,
        SNDRV_PCM_FORMAT_A_LAW => fmt = 0x3,
        SNDRV_PCM_FORMAT_SPECIAL => fmt = 0x4,
        SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE => fmt = 0x5,
        SNDRV_PCM_FORMAT_S16_LE => fmt = 0x8,
        SNDRV_PCM_FORMAT_S16_BE => fmt = 0x9,
        _ => {
            fmt = 0x8;
            dev_err((*(*v).card).dev, b"format unsupported %d\n\0".as_ptr(), alsafmt);
        }
    }
    fmt
}

/*
Some not yet useful translations from the final #if 0 block are preserved in
comments: ASPENCODING, vortex_translateformat, and vortex_cdmacore_setformat.
*/

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
