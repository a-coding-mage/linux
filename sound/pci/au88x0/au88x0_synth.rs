// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

/*
 * Someday its supposed to make use of the WT DMA engine
 * for a Wavetable synthesizer.
 */

/* Depends on declarations from "au88x0.h" and "au88x0_wt.h". */

unsafe extern "C" {
    fn vortex_fifo_setwtvalid(vortex: *mut vortex_t, fifo: i32, en: i32);
    fn vortex_connection_adb_mixin(
        vortex: *mut vortex_t,
        en: i32,
        channel: u8,
        source: u8,
        mixin: u8,
    );
    fn vortex_connection_mixin_mix(
        vortex: *mut vortex_t,
        en: i32,
        mixin: u8,
        mix: u8,
        a: i32,
    );
    fn vortex_fifo_wtinitialize(vortex: *mut vortex_t, fifo: i32, j: i32);
}

/* WT */

/* Put 2 WT channels together for one stereo interlaced channel. */
unsafe fn vortex_wt_setstereo(vortex: *mut vortex_t, wt: u32, stereo: u32) {
    let mut temp: i32;

    //temp = hwread(vortex->mmio, 0x80 + ((wt >> 0x5)<< 0xf) + (((wt & 0x1f) >> 1) << 2));
    temp = hwread((*vortex).mmio, WT_STEREO(wt)) as i32;
    temp = (temp & 0xfe) | (stereo & 1) as i32;
    //hwwrite(vortex->mmio, 0x80 + ((wt >> 0x5)<< 0xf) + (((wt & 0x1f) >> 1) << 2), temp);
    hwwrite((*vortex).mmio, WT_STEREO(wt), temp as u32);
}

/* Join to mixdown route. */
unsafe fn vortex_wt_setdsout(vortex: *mut vortex_t, wt: u32, en: i32) {
    let mut temp: i32;

    /* There is one DSREG register for each bank (32 voices each). */
    temp = hwread((*vortex).mmio, WT_DSREG(if wt >= 0x20 { 1 } else { 0 })) as i32;
    if en != 0 {
        temp |= 1 << (wt & 0x1f);
    } else {
        temp &= !(1 << (wt & 0x1f));
    }
    hwwrite((*vortex).mmio, WT_DSREG(if wt >= 0x20 { 1 } else { 0 }), temp as u32);
}

/* Setup WT route. */
unsafe fn vortex_wt_allocroute(vortex: *mut vortex_t, wt: i32, nr_ch: i32) -> i32 {
    let voice: *mut wt_voice_t = &mut (*vortex).wt_voice[wt as usize];
    let mut temp: i32;

    //FIXME: WT audio routing.
    if nr_ch != 0 {
        vortex_fifo_wtinitialize(vortex, wt, 1);
        vortex_fifo_setwtvalid(vortex, wt, 1);
        vortex_wt_setstereo(vortex, wt as u32, (nr_ch - 1) as u32);
    } else {
        vortex_fifo_setwtvalid(vortex, wt, 0);
    }

    /* Set mixdown mode. */
    vortex_wt_setdsout(vortex, wt as u32, 1);
    /* Set other parameter registers. */
    hwwrite((*vortex).mmio, WT_SRAMP(0), 0x880000);
    //hwwrite(vortex->mmio, WT_GMODE(0), 0xffffffff);
    /* CHIP_AU8830: hwwrite(vortex->mmio, WT_SRAMP(1), 0x880000); */
    //hwwrite(vortex->mmio, WT_GMODE(1), 0xffffffff);
    hwwrite((*vortex).mmio, WT_PARM(wt, 0), 0);
    hwwrite((*vortex).mmio, WT_PARM(wt, 1), 0);
    hwwrite((*vortex).mmio, WT_PARM(wt, 2), 0);

    temp = hwread((*vortex).mmio, WT_PARM(wt, 3)) as i32;
    dev_dbg((*(*vortex).card).dev, "WT PARM3: %x\n", temp);
    //hwwrite(vortex->mmio, WT_PARM(wt, 3), temp);

    hwwrite((*vortex).mmio, WT_DELAY(wt, 0), 0);
    hwwrite((*vortex).mmio, WT_DELAY(wt, 1), 0);
    hwwrite((*vortex).mmio, WT_DELAY(wt, 2), 0);
    hwwrite((*vortex).mmio, WT_DELAY(wt, 3), 0);

    dev_dbg(
        (*(*vortex).card).dev,
        "WT GMODE: %x\n",
        hwread((*vortex).mmio, WT_GMODE(wt)),
    );

    hwwrite((*vortex).mmio, WT_PARM(wt, 2), 0xffffffff);
    hwwrite((*vortex).mmio, WT_PARM(wt, 3), 0xcff1c810);

    (*voice).parm1 = 0xcfb23e2f;
    (*voice).parm0 = (*voice).parm1;
    hwwrite((*vortex).mmio, WT_PARM(wt, 0), (*voice).parm0);
    hwwrite((*vortex).mmio, WT_PARM(wt, 1), (*voice).parm1);
    dev_dbg(
        (*(*vortex).card).dev,
        "WT GMODE 2 : %x\n",
        hwread((*vortex).mmio, WT_GMODE(wt)),
    );
    0
}

unsafe fn vortex_wt_connect(vortex: *mut vortex_t, en: i32) {
    let mut i: i32;
    let mut ii: i32;
    let mut mix: i32;

    const NR_WTROUTES: i32 = 6;
    /* C used CHIP_AU8830 to select 2 WT blocks, otherwise 1. */
    #[cfg(CHIP_AU8830)]
    const NR_WTBLOCKS: i32 = 2;
    #[cfg(not(CHIP_AU8830))]
    const NR_WTBLOCKS: i32 = 1;

    i = 0;
    while i < NR_WTBLOCKS {
        ii = 0;
        while ii < NR_WTROUTES {
            mix = vortex_adb_checkinout(vortex, (*vortex).fixed_res, en, VORTEX_RESOURCE_MIXIN);
            (*vortex).mixwt[(i * NR_WTROUTES + ii) as usize] = mix;

            vortex_route(
                vortex,
                en,
                0x11,
                ADB_WTOUT(i, ii + 0x20),
                ADB_MIXIN(mix),
            );

            vortex_connection_mixin_mix(
                vortex,
                en,
                mix as u8,
                (*vortex).mixplayb[(ii % 2) as usize],
                0,
            );
            if VORTEX_IS_QUAD(vortex) != 0 {
                vortex_connection_mixin_mix(
                    vortex,
                    en,
                    mix as u8,
                    (*vortex).mixplayb[(2 + (ii % 2)) as usize],
                    0,
                );
            }
            ii += 1;
        }
        i += 1;
    }
    i = 0;
    while i < NR_WT {
        hwwrite((*vortex).mmio, WT_RUN(i), 1);
        i += 1;
    }
}

/* Read WT Register */
/* C #if 0 block preserved as non-compiled Rust translation. */
#[cfg(any())]
unsafe fn vortex_wt_GetReg(vortex: *mut vortex_t, reg: i8, wt: i32) -> i32 {
    //int eax, esi;

    if reg == 4 {
        return hwread((*vortex).mmio, WT_PARM(wt, 3)) as i32;
    }
    if reg == 7 {
        return hwread((*vortex).mmio, WT_GMODE(wt)) as i32;
    }

    0
}

/* WT hardware abstraction layer generic register interface. */
#[cfg(any())]
unsafe fn vortex_wt_SetReg2(vortex: *mut vortex_t, reg: u8, wt: i32, val: u16) -> i32 {
    /*
       int eax, edx;

       if (wt >= NR_WT)  // 0x40 -> NR_WT
       return 0;

       if ((reg - 0x20) > 0) {
       if ((reg - 0x21) != 0)
       return 0;
       eax = ((((b & 0xff) << 0xb) + (edx & 0xff)) << 4) + 0x208; // param 2
       } else {
       eax = ((((b & 0xff) << 0xb) + (edx & 0xff)) << 4) + 0x20a; // param 3
       }
       hwwrite(vortex->mmio, eax, c);
     */
    1
}

/*public: static void __thiscall CWTHal::SetReg(unsigned char,int,unsigned long) */
unsafe fn vortex_wt_SetReg(vortex: *mut vortex_t, reg: u8, wt: i32, val: u32) -> i32 {
    let ecx: i32;

    if (reg == 5) || ((reg >= 7) && (reg <= 10)) || (reg == 0xc) {
        if wt >= (NR_WT / NR_WT_PB) {
            dev_warn(
                (*(*vortex).card).dev,
                "WT SetReg: bank out of range. reg=0x%x, wt=%d\n",
                reg,
                wt,
            );
            return 0;
        }
    } else if wt >= NR_WT {
        dev_err((*(*vortex).card).dev, "WT SetReg: voice out of range\n");
        return 0;
    }
    if reg > 0xc {
        return 0;
    }

    match reg {
        /* Voice specific parameters */
        0 => {
            /* running */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_RUN(wt), (int)val);
            */
            hwwrite((*vortex).mmio, WT_RUN(wt), val);
            return 0xc;
        }
        1 => {
            /* param 0 */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_PARM(wt,0), (int)val);
            */
            hwwrite((*vortex).mmio, WT_PARM(wt, 0), val);
            return 0xc;
        }
        2 => {
            /* param 1 */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_PARM(wt,1), (int)val);
            */
            hwwrite((*vortex).mmio, WT_PARM(wt, 1), val);
            return 0xc;
        }
        3 => {
            /* param 2 */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_PARM(wt,2), (int)val);
            */
            hwwrite((*vortex).mmio, WT_PARM(wt, 2), val);
            return 0xc;
        }
        4 => {
            /* param 3 */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_PARM(wt,3), (int)val);
            */
            hwwrite((*vortex).mmio, WT_PARM(wt, 3), val);
            return 0xc;
        }
        6 => {
            /* mute */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_MUTE(wt), (int)val);
            */
            hwwrite((*vortex).mmio, WT_MUTE(wt), val);
            return 0xc;
        }
        0xb => {
            /* delay */
            /*
            pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n",
                   WT_DELAY(wt,0), (int)val);
            */
            hwwrite((*vortex).mmio, WT_DELAY(wt, 3), val);
            hwwrite((*vortex).mmio, WT_DELAY(wt, 2), val);
            hwwrite((*vortex).mmio, WT_DELAY(wt, 1), val);
            hwwrite((*vortex).mmio, WT_DELAY(wt, 0), val);
            return 0xc;
        }
        /* Global WT block parameters */
        5 => {
            /* sramp */
            ecx = WT_SRAMP(wt);
        }
        8 => {
            /* aramp */
            ecx = WT_ARAMP(wt);
        }
        9 => {
            /* mramp */
            ecx = WT_MRAMP(wt);
        }
        0xa => {
            /* ctrl */
            ecx = WT_CTRL(wt);
        }
        0xc => {
            /* ds_reg */
            ecx = WT_DSREG(wt);
        }
        _ => {
            return 0;
        }
    }
    /*
    pr_debug( "vortex: WT SetReg(0x%x) = 0x%08x\n", ecx, (int)val);
    */
    hwwrite((*vortex).mmio, ecx, val);
    1
}

unsafe fn vortex_wt_init(vortex: *mut vortex_t) {
    let mut var4: u32;
    let mut var8: u32;
    let mut varc: u32;
    let mut var10: u32 = 0;
    let mut edi: u32;

    var10 &= 0xFFFFFFE3;
    var10 |= 0x22;
    var10 &= 0xFFFFFEBF;
    var10 |= 0x80;
    var10 |= 0x200;
    var10 &= 0xfffffffe;
    var10 &= 0xfffffbff;
    var10 |= 0x1800;
    // var10 = 0x1AA2
    var4 = 0x10000000;
    varc = 0x00830000;
    var8 = 0x00830000;

    /* Init Bank registers. */
    edi = 0;
    while edi < (NR_WT / NR_WT_PB) as u32 {
        vortex_wt_SetReg(vortex, 0xc, edi as i32, 0); /* ds_reg */
        vortex_wt_SetReg(vortex, 0xa, edi as i32, var10); /* ctrl  */
        vortex_wt_SetReg(vortex, 0x9, edi as i32, var4); /* mramp */
        vortex_wt_SetReg(vortex, 0x8, edi as i32, varc); /* aramp */
        vortex_wt_SetReg(vortex, 0x5, edi as i32, var8); /* sramp */
        edi += 1;
    }
    /* Init Voice registers. */
    edi = 0;
    while edi < NR_WT as u32 {
        vortex_wt_SetReg(vortex, 0x4, edi as i32, 0); /* param 3 0x20c */
        vortex_wt_SetReg(vortex, 0x3, edi as i32, 0); /* param 2 0x208 */
        vortex_wt_SetReg(vortex, 0x2, edi as i32, 0); /* param 1 0x204 */
        vortex_wt_SetReg(vortex, 0x1, edi as i32, 0); /* param 0 0x200 */
        vortex_wt_SetReg(vortex, 0xb, edi as i32, 0); /* delay 0x400 - 0x40c */
        edi += 1;
    }
    var10 |= 1;
    edi = 0;
    while edi < (NR_WT / NR_WT_PB) as u32 {
        vortex_wt_SetReg(vortex, 0xa, edi as i32, var10); /* ctrl */
        edi += 1;
    }
}

/* Extract of CAdbTopology::SetVolume(struct _ASPVOLUME *) */
/* C #if 0 block preserved as non-compiled Rust translation. */
#[cfg(any())]
unsafe fn vortex_wt_SetVolume(vortex: *mut vortex_t, wt: i32, vol: *mut i32) {
    let voice: *mut wt_voice_t = &mut (*vortex).wt_voice[wt as usize];
    let mut ecx: i32 = *vol.add(1);
    let mut eax: i32 = *vol.add(0);

    /* This is pure guess */
    (*voice).parm0 &= 0xff00ffff;
    (*voice).parm0 |= ((*vol.add(0) & 0xff) << 0x10) as u32;
    (*voice).parm1 &= 0xff00ffff;
    (*voice).parm1 |= ((*vol.add(1) & 0xff) << 0x10) as u32;

    /* This is real */
    hwwrite(vortex, WT_PARM(wt, 0), (*voice).parm0);
    hwwrite(vortex, WT_PARM(wt, 1), (*voice).parm0);

    if ((*voice).this_1D0 & 4) != 0 {
        eax >>= 8;
        ecx = eax;
        if ecx < 0x80 {
            ecx = 0x7f;
        }
        (*voice).parm3 &= 0xFFFFC07F;
        (*voice).parm3 |= ((ecx & 0x7f) << 7) as u32;
        (*voice).parm3 &= 0xFFFFFF80;
        (*voice).parm3 |= (eax & 0x7f) as u32;
    } else {
        (*voice).parm3 &= 0xFFE03FFF;
        (*voice).parm3 |= ((eax & 0xFE00) << 5) as u32;
    }

    hwwrite(vortex, WT_PARM(wt, 3), (*voice).parm3);
}

/* Extract of CAdbTopology::SetFrequency(unsigned long arg_0) */
#[cfg(any())]
unsafe fn vortex_wt_SetFrequency(vortex: *mut vortex_t, wt: i32, sr: u32) {
    let voice: *mut wt_voice_t = &mut (*vortex).wt_voice[wt as usize];
    let mut eax: u32;
    let mut edx: u32;

    //FIXME: 64 bit operation.
    eax = ((sr << 0xf).wrapping_mul(0x57619F1)) & 0xffffffff;
    edx = ((sr << 0xf).wrapping_mul(0x57619F1)) >> 0x20;

    edx >>= 0xa;
    edx <<= 1;
    if edx != 0 {
        if (edx & 0x0FFF80000) != 0 {
            eax = 0x7fff;
        } else {
            edx <<= 0xd;
            eax = 7;
            while (edx & 0x80000000) == 0 {
                edx <<= 1;
                eax -= 1;
                if eax == 0 {
                    break;
                }
            }
            if eax != 0 {
                edx <<= 1;
            }
            eax <<= 0xc;
            edx >>= 0x14;
            eax |= edx;
        }
    } else {
        eax = 0;
    }
    (*voice).parm0 &= 0xffff0001;
    (*voice).parm0 |= (eax & 0x7fff) << 1;
    (*voice).parm1 = (*voice).parm0 | 1;
    // Wt: this_1D4
    //AuWt::WriteReg((ulong)(this_1DC<<4)+0x200, (ulong)this_1E4);
    //AuWt::WriteReg((ulong)(this_1DC<<4)+0x204, (ulong)this_1E8);
    hwwrite((*vortex).mmio, WT_PARM(wt, 0), (*voice).parm0);
    hwwrite((*vortex).mmio, WT_PARM(wt, 1), (*voice).parm1);
}

/* End of File */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
