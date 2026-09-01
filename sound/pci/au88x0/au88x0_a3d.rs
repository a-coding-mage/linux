// SPDX-License-Identifier: GPL-2.0-or-later
/***************************************************************************
 *            au88x0_a3d.c
 *
 *  Fri Jul 18 14:16:22 2003
 *  Copyright  2003  mjander
 *  mjander@users.sourceforge.net
 *
 * A3D. You may think i'm crazy, but this may work someday. Who knows...
 ****************************************************************************/

/*
 */

/* Dependencies from au88x0_a3d.h, au88x0_a3ddata.c, au88x0_xtalk.h, and au88x0.h. */
use crate::*;

unsafe fn a3dsrc_SetTimeConsts(
    a: *mut a3dsrc_t,
    HrtfTrack: i16,
    ItdTrack: i16,
    GTrack: i16,
    CTrack: i16,
) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        a3d_addrA((*a).slice, (*a).source, A3D_A_HrtfTrackTC),
        HrtfTrack as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrA((*a).slice, (*a).source, A3D_A_ITDTrackTC),
        ItdTrack as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrA((*a).slice, (*a).source, A3D_A_GainTrackTC),
        GTrack as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrA((*a).slice, (*a).source, A3D_A_CoeffTrackTC),
        CTrack as _,
    );
}

/*
#if 0
static void
a3dsrc_GetTimeConsts(a3dsrc_t * a, short *HrtfTrack, short *ItdTrack,
		     short *GTrack, short *CTrack)
{
	// stub!
}

#endif
*/
/* Atmospheric absorption. */

unsafe fn a3dsrc_SetAtmosTarget(a: *mut a3dsrc_t, aa: i16, b: i16, c: i16, d: i16, e: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_A21Target),
        (((e as i32) << 0x10) | (d as i32)) as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_B10Target),
        (((b as i32) << 0x10) | (aa as i32)) as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_B2Target),
        c as _,
    );
}

unsafe fn a3dsrc_SetAtmosCurrent(a: *mut a3dsrc_t, aa: i16, b: i16, c: i16, d: i16, e: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_A12Current),
        (((e as i32) << 0x10) | (d as i32)) as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_B01Current),
        (((b as i32) << 0x10) | (aa as i32)) as _,
    );
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_B2Current),
        c as _,
    );
}

unsafe fn a3dsrc_SetAtmosState(a: *mut a3dsrc_t, x1: i16, x2: i16, y1: i16, y2: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite((*vortex).mmio, a3d_addrA((*a).slice, (*a).source, A3D_A_x1), x1 as _);
    hwwrite((*vortex).mmio, a3d_addrA((*a).slice, (*a).source, A3D_A_x2), x2 as _);
    hwwrite((*vortex).mmio, a3d_addrA((*a).slice, (*a).source, A3D_A_y1), y1 as _);
    hwwrite((*vortex).mmio, a3d_addrA((*a).slice, (*a).source, A3D_A_y2), y2 as _);
}

/*
#if 0
static void
a3dsrc_GetAtmosTarget(a3dsrc_t * a, short *aa, short *b, short *c,
		      short *d, short *e)
{
}
static void
a3dsrc_GetAtmosCurrent(a3dsrc_t * a, short *bb01, short *ab01, short *b2,
		       short *aa12, short *ba12)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*aa12 =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_A12Current));
	*ba12 =
	    hwread(vortex->mmio,
		   a3d_addrB(a->slice, a->source, A3D_B_A12Current));
	*ab01 =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_B01Current));
	*bb01 =
	    hwread(vortex->mmio,
		   a3d_addrB(a->slice, a->source, A3D_B_B01Current));
	*b2 =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_B2Current));
}

static void
a3dsrc_GetAtmosState(a3dsrc_t * a, short *x1, short *x2, short *y1, short *y2)
{

}

#endif
*/
/* HRTF */

unsafe fn a3dsrc_SetHrtfTarget(a: *mut a3dsrc_t, aa: a3d_Hrtf_t, b: a3d_Hrtf_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    let mut i: i32 = 0;

    while i < HRTF_SZ {
        hwwrite(
            (*vortex).mmio,
            a3d_addrB((*a).slice, (*a).source, A3D_B_HrtfTarget) + ((i as _) << 2),
            (((*b.as_ptr().offset(i as isize) as i32) << 0x10)
                | (*aa.as_ptr().offset(i as isize) as i32)) as _,
        );
        i += 1;
    }
}

unsafe fn a3dsrc_SetHrtfCurrent(a: *mut a3dsrc_t, aa: a3d_Hrtf_t, b: a3d_Hrtf_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    let mut i: i32 = 0;

    while i < HRTF_SZ {
        hwwrite(
            (*vortex).mmio,
            a3d_addrB((*a).slice, (*a).source, A3D_B_HrtfCurrent) + ((i as _) << 2),
            (((*b.as_ptr().offset(i as isize) as i32) << 0x10)
                | (*aa.as_ptr().offset(i as isize) as i32)) as _,
        );
        i += 1;
    }
}

unsafe fn a3dsrc_SetHrtfState(a: *mut a3dsrc_t, aa: a3d_Hrtf_t, b: a3d_Hrtf_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    let mut i: i32 = 0;

    while i < HRTF_SZ {
        hwwrite(
            (*vortex).mmio,
            a3d_addrB((*a).slice, (*a).source, A3D_B_HrtfDelayLine) + ((i as _) << 2),
            (((*b.as_ptr().offset(i as isize) as i32) << 0x10)
                | (*aa.as_ptr().offset(i as isize) as i32)) as _,
        );
        i += 1;
    }
}

unsafe fn a3dsrc_SetHrtfOutput(a: *mut a3dsrc_t, left: i16, right: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite((*vortex).mmio, a3d_addrA((*a).slice, (*a).source, A3D_A_HrtfOutL), left as _);
    hwwrite((*vortex).mmio, a3d_addrA((*a).slice, (*a).source, A3D_A_HrtfOutR), right as _);
}

/*
#if 0
static void a3dsrc_GetHrtfTarget(a3dsrc_t * a, a3d_Hrtf_t aa, a3d_Hrtf_t b)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	int i;

	for (i = 0; i < HRTF_SZ; i++)
		aa[i] =
		    hwread(vortex->mmio,
			   a3d_addrA(a->slice, a->source,
				     A3D_A_HrtfTarget + (i << 2)));
	for (i = 0; i < HRTF_SZ; i++)
		b[i] =
		    hwread(vortex->mmio,
			   a3d_addrB(a->slice, a->source,
				     A3D_B_HrtfTarget + (i << 2)));
}

static void a3dsrc_GetHrtfCurrent(a3dsrc_t * a, a3d_Hrtf_t aa, a3d_Hrtf_t b)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	int i;

	for (i = 0; i < HRTF_SZ; i++)
		aa[i] =
		    hwread(vortex->mmio,
			   a3d_addrA(a->slice, a->source,
				     A3D_A_HrtfCurrent + (i << 2)));
	for (i = 0; i < HRTF_SZ; i++)
		b[i] =
		    hwread(vortex->mmio,
			   a3d_addrB(a->slice, a->source,
				     A3D_B_HrtfCurrent + (i << 2)));
}

static void a3dsrc_GetHrtfState(a3dsrc_t * a, a3d_Hrtf_t aa, a3d_Hrtf_t b)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	int i;
	// FIXME: verify this!
	for (i = 0; i < HRTF_SZ; i++)
		aa[i] =
		    hwread(vortex->mmio,
			   a3d_addrA(a->slice, a->source,
				     A3D_A_HrtfDelayLine + (i << 2)));
	for (i = 0; i < HRTF_SZ; i++)
		b[i] =
		    hwread(vortex->mmio,
			   a3d_addrB(a->slice, a->source,
				     A3D_B_HrtfDelayLine + (i << 2)));
}

static void a3dsrc_GetHrtfOutput(a3dsrc_t * a, short *left, short *right)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*left =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_HrtfOutL));
	*right =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_HrtfOutR));
}

#endif
*/

/* Interaural Time Difference.
 * "The other main clue that humans use to locate sounds, is called
 * Interaural Time Difference (ITD). The differences in distance from
 * the sound source to a listeners ears means  that the sound will
 * reach one ear slightly before the other....", found somewhere with google.*/

unsafe fn a3dsrc_SetItdTarget(a: *mut a3dsrc_t, mut litd: i16, mut ritd: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;

    if litd < 0 {
        litd = 0;
    }
    if litd > 0x57FF {
        litd = 0x57FF;
    }
    if ritd < 0 {
        ritd = 0;
    }
    if ritd > 0x57FF {
        ritd = 0x57FF;
    }
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_ITDTarget),
        (((ritd as i32) << 0x10) | (litd as i32)) as _,
    );
    //hwwrite(vortex->mmio, addr(0x191DF+5, this04, this08), (ritd<<0x10)|litd);
}

unsafe fn a3dsrc_SetItdCurrent(a: *mut a3dsrc_t, mut litd: i16, mut ritd: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;

    if litd < 0 {
        litd = 0;
    }
    if litd > 0x57FF {
        litd = 0x57FF;
    }
    if ritd < 0 {
        ritd = 0;
    }
    if ritd > 0x57FF {
        ritd = 0x57FF;
    }
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_ITDCurrent),
        (((ritd as i32) << 0x10) | (litd as i32)) as _,
    );
    //hwwrite(vortex->mmio, addr(0x191DF+1, this04, this08), (ritd<<0x10)|litd);
}

unsafe fn a3dsrc_SetItdDline(a: *mut a3dsrc_t, dline: a3d_ItdDline_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    let mut i: i32 = 0;
    /* 45 != 40 -> Check this ! */
    while i < DLINE_SZ {
        hwwrite(
            (*vortex).mmio,
            a3d_addrA((*a).slice, (*a).source, A3D_A_ITDDelayLine) + ((i as _) << 2),
            *dline.as_ptr().offset(i as isize) as _,
        );
        i += 1;
    }
}

/*
#if 0
static void a3dsrc_GetItdTarget(a3dsrc_t * a, short *litd, short *ritd)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*ritd =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_ITDTarget));
	*litd =
	    hwread(vortex->mmio,
		   a3d_addrB(a->slice, a->source, A3D_B_ITDTarget));
}

static void a3dsrc_GetItdCurrent(a3dsrc_t * a, short *litd, short *ritd)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);

	*ritd =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_ITDCurrent));
	*litd =
	    hwread(vortex->mmio,
		   a3d_addrB(a->slice, a->source, A3D_B_ITDCurrent));
}

static void a3dsrc_GetItdDline(a3dsrc_t * a, a3d_ItdDline_t dline)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	int i;

	for (i = 0; i < DLINE_SZ; i++)
		dline[i] =
		    hwread(vortex->mmio,
			   a3d_addrA(a->slice, a->source,
				     A3D_A_ITDDelayLine + (i << 2)));
}

#endif
*/
/* This is may be used for ILD Interaural Level Difference. */

unsafe fn a3dsrc_SetGainTarget(a: *mut a3dsrc_t, left: i16, right: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_GainTarget),
        (((right as i32) << 0x10) | (left as i32)) as _,
    );
}

unsafe fn a3dsrc_SetGainCurrent(a: *mut a3dsrc_t, left: i16, right: i16) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        a3d_addrB((*a).slice, (*a).source, A3D_B_GainCurrent),
        (((right as i32) << 0x10) | (left as i32)) as _,
    );
}

/*
#if 0
static void a3dsrc_GetGainTarget(a3dsrc_t * a, short *left, short *right)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*right =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_GainTarget));
	*left =
	    hwread(vortex->mmio,
		   a3d_addrB(a->slice, a->source, A3D_B_GainTarget));
}

static void a3dsrc_GetGainCurrent(a3dsrc_t * a, short *left, short *right)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*right =
	    hwread(vortex->mmio,
		   a3d_addrA(a->slice, a->source, A3D_A_GainCurrent));
	*left =
	    hwread(vortex->mmio,
		   a3d_addrB(a->slice, a->source, A3D_B_GainCurrent));
}

/* CA3dIO this func seems to be inlined all over this place. */
static void CA3dIO_WriteReg(a3dsrc_t * a, unsigned long addr, short aa, short b)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	hwwrite(vortex->mmio, addr, (aa << 0x10) | b);
}

#endif
*/
/* Generic A3D stuff */

unsafe fn a3dsrc_SetA3DSampleRate(a: *mut a3dsrc_t, sr: i32) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    let mut esp0: i32 = 0;

    esp0 = (((esp0 & 0x7fffffff) | 0xB8000000u32 as i32) & 0x7) | ((sr & 0x1f) << 3);
    hwwrite((*vortex).mmio, A3D_SLICE_Control + (((*a).slice as _) << 0xd), esp0 as _);
    //hwwrite(vortex->mmio, 0x19C38 + (this08<<0xd), esp0);
}

unsafe fn a3dsrc_EnableA3D(a: *mut a3dsrc_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        A3D_SLICE_Control + (((*a).slice as _) << 0xd),
        0xF0000001u32 as _,
    );
    //hwwrite(vortex->mmio, 0x19C38 + (this08<<0xd), 0xF0000001);
}

unsafe fn a3dsrc_DisableA3D(a: *mut a3dsrc_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite(
        (*vortex).mmio,
        A3D_SLICE_Control + (((*a).slice as _) << 0xd),
        0xF0000000u32 as _,
    );
}

unsafe fn a3dsrc_SetA3DControlReg(a: *mut a3dsrc_t, ctrl: c_ulong) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite((*vortex).mmio, A3D_SLICE_Control + (((*a).slice as _) << 0xd), ctrl as _);
}

unsafe fn a3dsrc_SetA3DPointerReg(a: *mut a3dsrc_t, ptr: c_ulong) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    hwwrite((*vortex).mmio, A3D_SLICE_Pointers + (((*a).slice as _) << 0xd), ptr as _);
}

/*
#if 0
static void a3dsrc_GetA3DSampleRate(a3dsrc_t * a, int *sr)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*sr = ((hwread(vortex->mmio, A3D_SLICE_Control + (a->slice << 0xd))
		>> 3) & 0x1f);
	//*sr = ((hwread(vortex->mmio, 0x19C38 + (this08<<0xd))>>3)&0x1f);
}

static void a3dsrc_GetA3DControlReg(a3dsrc_t * a, unsigned long *ctrl)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*ctrl = hwread(vortex->mmio, A3D_SLICE_Control + ((a->slice) << 0xd));
}

static void a3dsrc_GetA3DPointerReg(a3dsrc_t * a, unsigned long *ptr)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);
	*ptr = hwread(vortex->mmio, A3D_SLICE_Pointers + ((a->slice) << 0xd));
}

#endif
*/
unsafe fn a3dsrc_ZeroSliceIO(a: *mut a3dsrc_t) {
    let vortex: *mut vortex_t = (*a).vortex as *mut vortex_t;
    let mut i: i32 = 0;

    while i < 8 {
        hwwrite(
            (*vortex).mmio,
            A3D_SLICE_VDBDest + (((((*a).slice as _) << 0xb) + i as _) << 2),
            0,
        );
        i += 1;
    }
    i = 0;
    while i < 4 {
        hwwrite(
            (*vortex).mmio,
            A3D_SLICE_VDBSource + (((((*a).slice as _) << 0xb) + i as _) << 2),
            0,
        );
        i += 1;
    }
}

/* Reset Single A3D source. */
unsafe fn a3dsrc_ZeroState(a: *mut a3dsrc_t) {
    /*
    pr_debug( "vortex: ZeroState slice: %d, source %d\n",
           a->slice, a->source);
    */
    a3dsrc_SetAtmosState(a, 0, 0, 0, 0);
    a3dsrc_SetHrtfState(a, A3dHrirZeros, A3dHrirZeros);
    a3dsrc_SetItdDline(a, A3dItdDlineZeros);
    a3dsrc_SetHrtfOutput(a, 0, 0);
    a3dsrc_SetTimeConsts(a, 0, 0, 0, 0);

    a3dsrc_SetAtmosCurrent(a, 0, 0, 0, 0, 0);
    a3dsrc_SetAtmosTarget(a, 0, 0, 0, 0, 0);
    a3dsrc_SetItdCurrent(a, 0, 0);
    a3dsrc_SetItdTarget(a, 0, 0);
    a3dsrc_SetGainCurrent(a, 0, 0);
    a3dsrc_SetGainTarget(a, 0, 0);

    a3dsrc_SetHrtfCurrent(a, A3dHrirZeros, A3dHrirZeros);
    a3dsrc_SetHrtfTarget(a, A3dHrirZeros, A3dHrirZeros);
}

/* Reset entire A3D engine */
unsafe fn a3dsrc_ZeroStateA3D(a: *mut a3dsrc_t, v: *mut vortex_t) {
    let mut i: i32;
    let var: i32;
    let var2: i32;

    if (*a).vortex.is_null() {
        dev_err((*(*v).card).dev, c"ZeroStateA3D: ERROR: a->vortex is NULL\n".as_ptr());
        return;
    }

    a3dsrc_SetA3DControlReg(a, 0);
    a3dsrc_SetA3DPointerReg(a, 0);

    var = (*a).slice;
    var2 = (*a).source;
    i = 0;
    while i < 4 {
        (*a).slice = i;
        a3dsrc_ZeroSliceIO(a);
        //a3dsrc_ZeroState(a);
        i += 1;
    }
    (*a).source = var2;
    (*a).slice = var;
}

/* Program A3D block as pass through */
unsafe fn a3dsrc_ProgramPipe(a: *mut a3dsrc_t) {
    a3dsrc_SetTimeConsts(a, 0, 0, 0, 0);
    a3dsrc_SetAtmosCurrent(a, 0, 0x4000, 0, 0, 0);
    a3dsrc_SetAtmosTarget(a, 0x4000, 0, 0, 0, 0);
    a3dsrc_SetItdCurrent(a, 0, 0);
    a3dsrc_SetItdTarget(a, 0, 0);
    a3dsrc_SetGainCurrent(a, 0x7fff, 0x7fff);
    a3dsrc_SetGainTarget(a, 0x7fff, 0x7fff);

    /* SET HRTF HERE */

    /* Single spike leads to identity transfer function. */
    a3dsrc_SetHrtfCurrent(a, A3dHrirImpulse, A3dHrirImpulse);
    a3dsrc_SetHrtfTarget(a, A3dHrirImpulse, A3dHrirImpulse);

    /* Test: Sounds saturated. */
    //a3dsrc_SetHrtfCurrent(a, A3dHrirSatTest, A3dHrirSatTest);
    //a3dsrc_SetHrtfTarget(a, A3dHrirSatTest, A3dHrirSatTest);
}

/* VDB = Vortex audio Dataflow Bus */
/*
#if 0
static void a3dsrc_ClearVDBData(a3dsrc_t * a, unsigned long aa)
{
	vortex_t *vortex = (vortex_t *) (a->vortex);

	// ((aa >> 2) << 8) - (aa >> 2)
	hwwrite(vortex->mmio,
		a3d_addrS(a->slice, A3D_SLICE_VDBDest) + (a->source << 2), 0);
	hwwrite(vortex->mmio,
		a3d_addrS(a->slice,
			  A3D_SLICE_VDBDest + 4) + (a->source << 2), 0);
	/*
	   hwwrite(vortex->mmio, 0x19c00 + (((aa>>2)*255*4)+aa)*8, 0);
	   hwwrite(vortex->mmio, 0x19c04 + (((aa>>2)*255*4)+aa)*8, 0);
	 */
}
#endif
*/

/* A3D HwSource stuff. */

unsafe fn vortex_A3dSourceHw_Initialize(v: *mut vortex_t, source: i32, slice: i32) {
    let a3dsrc: *mut a3dsrc_t = &mut *(*v).a3d.as_mut_ptr().offset((source + (slice * 4)) as isize);
    //a3dsrc_t *a3dsrc = &(v->a3d[source + (slice*4)]);

    (*a3dsrc).vortex = v as *mut _;
    (*a3dsrc).source = source; /* source */
    (*a3dsrc).slice = slice; /* slice */
    a3dsrc_ZeroState(a3dsrc);
    /* Added by me. */
    a3dsrc_SetA3DSampleRate(a3dsrc, 0x11);
}

unsafe fn Vort3DRend_Initialize(v: *mut vortex_t, mode: u16) -> i32 {
    (*v).xt_mode = mode; /* this_14 */

    vortex_XtalkHw_init(v);
    vortex_XtalkHw_SetGainsAllChan(v);
    match (*v).xt_mode as _ {
        XT_SPEAKER0 => {
            vortex_XtalkHw_ProgramXtalkNarrow(v);
        }
        XT_SPEAKER1 => {
            vortex_XtalkHw_ProgramXtalkWide(v);
        }
        XT_HEADPHONE => {
            vortex_XtalkHw_ProgramPipe(v);
        }
        XT_DIAMOND => {
            vortex_XtalkHw_ProgramDiamondXtalk(v);
        }
        _ => {
            vortex_XtalkHw_ProgramPipe(v);
        }
    }
    vortex_XtalkHw_SetSampleRate(v, 0x11);
    vortex_XtalkHw_Enable(v);
    0
}

/* 3D Sound entry points. */

/* A3D base support init/shudown */
unsafe fn vortex_Vort3D_enable(v: *mut vortex_t) {
    let mut i: i32;

    Vort3DRend_Initialize(v, XT_HEADPHONE as _);
    i = 0;
    while i < NR_A3D {
        vortex_A3dSourceHw_Initialize(v, i % 4, i >> 2);
        a3dsrc_ZeroStateA3D(&mut *(*v).a3d.as_mut_ptr().offset(0), v);
        i += 1;
    }
    /* Register ALSA controls */
    vortex_a3d_register_controls(v);
}

unsafe fn vortex_Vort3D_disable(v: *mut vortex_t) {
    vortex_XtalkHw_Disable(v);
    vortex_a3d_unregister_controls(v);
}

/* Make A3D subsystem connections. */
unsafe fn vortex_Vort3D_connect(v: *mut vortex_t, en: i32) {
    let mut i: i32;

    // Disable AU8810 routes, since they seem to be wrong (in au8810.h).
    #[cfg(CHIP_AU8810)]
    {
        return;
    }

    /* Alloc Xtalk mixin resources */
    (*v).mixxtlk[0] = vortex_adb_checkinout(v, (*v).fixed_res, en, VORTEX_RESOURCE_MIXIN);
    if (*v).mixxtlk[0] < 0 {
        dev_warn(
            (*(*v).card).dev,
            c"vortex_Vort3D: ERROR: not enough free mixer resources.\n".as_ptr(),
        );
        return;
    }
    (*v).mixxtlk[1] = vortex_adb_checkinout(v, (*v).fixed_res, en, VORTEX_RESOURCE_MIXIN);
    if (*v).mixxtlk[1] < 0 {
        dev_warn(
            (*(*v).card).dev,
            c"vortex_Vort3D: ERROR: not enough free mixer resources.\n".as_ptr(),
        );
        return;
    }

    /* Connect A3D -> XTALK */
    i = 0;
    while i < 4 {
        // 2 outputs per each A3D slice.
        vortex_route(v, en, 0x11, ADB_A3DOUT(i * 2), ADB_XTALKIN(i));
        vortex_route(v, en, 0x11, ADB_A3DOUT(i * 2) + 1, ADB_XTALKIN(5 + i));
        i += 1;
    }
    /*
    #if 0
	vortex_route(v, en, 0x11, ADB_XTALKOUT(0), ADB_EQIN(2));
	vortex_route(v, en, 0x11, ADB_XTALKOUT(1), ADB_EQIN(3));
    #else
    */
    /* Connect XTalk -> mixer */
    vortex_route(v, en, 0x11, ADB_XTALKOUT(0), ADB_MIXIN((*v).mixxtlk[0]));
    vortex_route(v, en, 0x11, ADB_XTALKOUT(1), ADB_MIXIN((*v).mixxtlk[1]));
    vortex_connection_mixin_mix(v, en, (*v).mixxtlk[0], (*v).mixplayb[0], 0);
    vortex_connection_mixin_mix(v, en, (*v).mixxtlk[1], (*v).mixplayb[1], 0);
    vortex_mix_setinputvolumebyte(
        v,
        (*v).mixplayb[0],
        (*v).mixxtlk[0],
        if en != 0 { MIX_DEFIGAIN } else { VOL_MIN },
    );
    vortex_mix_setinputvolumebyte(
        v,
        (*v).mixplayb[1],
        (*v).mixxtlk[1],
        if en != 0 { MIX_DEFIGAIN } else { VOL_MIN },
    );
    if VORTEX_IS_QUAD(v) != 0 {
        vortex_connection_mixin_mix(v, en, (*v).mixxtlk[0], (*v).mixplayb[2], 0);
        vortex_connection_mixin_mix(v, en, (*v).mixxtlk[1], (*v).mixplayb[3], 0);
        vortex_mix_setinputvolumebyte(
            v,
            (*v).mixplayb[2],
            (*v).mixxtlk[0],
            if en != 0 { MIX_DEFIGAIN } else { VOL_MIN },
        );
        vortex_mix_setinputvolumebyte(
            v,
            (*v).mixplayb[3],
            (*v).mixxtlk[1],
            if en != 0 { MIX_DEFIGAIN } else { VOL_MIN },
        );
    }
    /*
    #endif
    */
}

/* Initialize one single A3D source. */
unsafe fn vortex_Vort3D_InitializeSource(a: *mut a3dsrc_t, en: i32, v: *mut vortex_t) {
    if (*a).vortex.is_null() {
        dev_warn(
            (*(*v).card).dev,
            c"Vort3D_InitializeSource: A3D source not initialized\n".as_ptr(),
        );
        return;
    }
    if en != 0 {
        a3dsrc_ProgramPipe(a);
        a3dsrc_SetA3DSampleRate(a, 0x11);
        a3dsrc_SetTimeConsts(a, HrtfTCDefault, ItdTCDefault, GainTCDefault, CoefTCDefault);
        /* Remark: zero gain is muted. */
        //a3dsrc_SetGainTarget(a,0,0);
        //a3dsrc_SetGainCurrent(a,0,0);
        a3dsrc_EnableA3D(a);
    } else {
        a3dsrc_DisableA3D(a);
        a3dsrc_ZeroState(a);
    }
}

/* Conversion of coordinates into 3D parameters. */

unsafe fn vortex_a3d_coord2hrtf(_hrtf: a3d_Hrtf_t, _coord: *mut i32) {
    /* FIXME: implement this. */

}
unsafe fn vortex_a3d_coord2itd(_itd: a3d_Itd_t, _coord: *mut i32) {
    /* FIXME: implement this. */

}
unsafe fn vortex_a3d_coord2ild(_ild: a3d_LRGains_t, _left: i32, _right: i32) {
    /* FIXME: implement this. */

}
unsafe fn vortex_a3d_translate_filter(_filter: a3d_atmos_t, _params: *mut i32) {
    /* FIXME: implement this. */

}

/* ALSA control interface.  */

unsafe fn snd_vortex_a3d_hrtf_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 6;
    (*uinfo).value.integer.min = 0x00000000;
    (*uinfo).value.integer.max = 0xffffffffu32 as _;
    0
}
unsafe fn snd_vortex_a3d_itd_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0x00000000;
    (*uinfo).value.integer.max = 0xffffffffu32 as _;
    0
}
unsafe fn snd_vortex_a3d_ild_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0x00000000;
    (*uinfo).value.integer.max = 0xffffffffu32 as _;
    0
}
unsafe fn snd_vortex_a3d_filter_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 4;
    (*uinfo).value.integer.min = 0x00000000;
    (*uinfo).value.integer.max = 0xffffffffu32 as _;
    0
}

unsafe fn snd_vortex_a3d_get(
    _kcontrol: *mut snd_kcontrol,
    _ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    //a3dsrc_t *a = snd_kcontrol_chip(kcontrol);
    /* No read yet. Would this be really useable/needed ? */

    0
}

unsafe fn snd_vortex_a3d_hrtf_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let a: *mut a3dsrc_t = snd_kcontrol_chip(kcontrol) as *mut a3dsrc_t;
    let mut i: i32;
    let mut coord: [i32; 6] = [0; 6];
    i = 0;
    while i < 6 {
        coord[i as usize] = (*ucontrol).value.integer.value[i as usize] as _;
        i += 1;
    }
    /* Translate orientation coordinates to a3d params. */
    vortex_a3d_coord2hrtf((*a).hrtf[0], coord.as_mut_ptr());
    vortex_a3d_coord2hrtf((*a).hrtf[1], coord.as_mut_ptr());
    a3dsrc_SetHrtfTarget(a, (*a).hrtf[0], (*a).hrtf[1]);
    a3dsrc_SetHrtfCurrent(a, (*a).hrtf[0], (*a).hrtf[1]);
    1
}

unsafe fn snd_vortex_a3d_itd_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let a: *mut a3dsrc_t = snd_kcontrol_chip(kcontrol) as *mut a3dsrc_t;
    let mut coord: [i32; 6] = [0; 6];
    let mut i: i32;
    i = 0;
    while i < 6 {
        coord[i as usize] = (*ucontrol).value.integer.value[i as usize] as _;
        i += 1;
    }
    /* Translate orientation coordinates to a3d params. */
    vortex_a3d_coord2itd((*a).hrtf[0], coord.as_mut_ptr());
    vortex_a3d_coord2itd((*a).hrtf[1], coord.as_mut_ptr());
    /* Inter aural time difference. */
    a3dsrc_SetItdTarget(a, (*a).itd[0], (*a).itd[1]);
    a3dsrc_SetItdCurrent(a, (*a).itd[0], (*a).itd[1]);
    a3dsrc_SetItdDline(a, (*a).dline);
    1
}

unsafe fn snd_vortex_a3d_ild_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let a: *mut a3dsrc_t = snd_kcontrol_chip(kcontrol) as *mut a3dsrc_t;
    let l: i32;
    let r: i32;
    /* There may be some scale tranlation needed here. */
    l = (*ucontrol).value.integer.value[0] as _;
    r = (*ucontrol).value.integer.value[1] as _;
    vortex_a3d_coord2ild((*a).ild, l, r);
    /* Left Right panning. */
    a3dsrc_SetGainTarget(a, l as i16, r as i16);
    a3dsrc_SetGainCurrent(a, l as i16, r as i16);
    1
}

unsafe fn snd_vortex_a3d_filter_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let a: *mut a3dsrc_t = snd_kcontrol_chip(kcontrol) as *mut a3dsrc_t;
    let mut i: i32;
    let mut params: [i32; 6] = [0; 6];
    i = 0;
    while i < 6 {
        params[i as usize] = (*ucontrol).value.integer.value[i as usize] as _;
        i += 1;
    }
    /* Translate generic filter params to a3d filter params. */
    vortex_a3d_translate_filter((*a).filter, params.as_mut_ptr());
    /* Atmospheric absorption and filtering. */
    a3dsrc_SetAtmosTarget(
        a,
        (*a).filter[0],
        (*a).filter[1],
        (*a).filter[2],
        (*a).filter[3],
        (*a).filter[4],
    );
    a3dsrc_SetAtmosCurrent(
        a,
        (*a).filter[0],
        (*a).filter[1],
        (*a).filter[2],
        (*a).filter[3],
        (*a).filter[4],
    );
    1
}

static mut vortex_a3d_kcontrol: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: c"Playback PCM advanced processing".as_ptr(),
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(snd_vortex_a3d_hrtf_info),
    get: Some(snd_vortex_a3d_get),
    put: Some(snd_vortex_a3d_hrtf_put),
};

/* Control (un)registration. */
unsafe fn vortex_a3d_register_controls(vortex: *mut vortex_t) -> i32 {
    let mut kcontrol: *mut snd_kcontrol;
    let mut err: i32;
    let mut i: i32;
    /* HRTF controls. */
    i = 0;
    while i < NR_A3D {
        kcontrol = snd_ctl_new1(&raw const vortex_a3d_kcontrol, &mut *(*vortex).a3d.as_mut_ptr().offset(i as isize) as *mut _ as *mut _);
        if kcontrol.is_null() {
            return -ENOMEM;
        }
        (*kcontrol).id.numid = CTRLID_HRTF;
        (*kcontrol).info = Some(snd_vortex_a3d_hrtf_info);
        (*kcontrol).put = Some(snd_vortex_a3d_hrtf_put);
        err = snd_ctl_add((*vortex).card, kcontrol);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    /* ITD controls. */
    i = 0;
    while i < NR_A3D {
        kcontrol = snd_ctl_new1(&raw const vortex_a3d_kcontrol, &mut *(*vortex).a3d.as_mut_ptr().offset(i as isize) as *mut _ as *mut _);
        if kcontrol.is_null() {
            return -ENOMEM;
        }
        (*kcontrol).id.numid = CTRLID_ITD;
        (*kcontrol).info = Some(snd_vortex_a3d_itd_info);
        (*kcontrol).put = Some(snd_vortex_a3d_itd_put);
        err = snd_ctl_add((*vortex).card, kcontrol);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    /* ILD (gains) controls. */
    i = 0;
    while i < NR_A3D {
        kcontrol = snd_ctl_new1(&raw const vortex_a3d_kcontrol, &mut *(*vortex).a3d.as_mut_ptr().offset(i as isize) as *mut _ as *mut _);
        if kcontrol.is_null() {
            return -ENOMEM;
        }
        (*kcontrol).id.numid = CTRLID_GAINS;
        (*kcontrol).info = Some(snd_vortex_a3d_ild_info);
        (*kcontrol).put = Some(snd_vortex_a3d_ild_put);
        err = snd_ctl_add((*vortex).card, kcontrol);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    /* Filter controls. */
    i = 0;
    while i < NR_A3D {
        kcontrol = snd_ctl_new1(&raw const vortex_a3d_kcontrol, &mut *(*vortex).a3d.as_mut_ptr().offset(i as isize) as *mut _ as *mut _);
        if kcontrol.is_null() {
            return -ENOMEM;
        }
        (*kcontrol).id.numid = CTRLID_FILTER;
        (*kcontrol).info = Some(snd_vortex_a3d_filter_info);
        (*kcontrol).put = Some(snd_vortex_a3d_filter_put);
        err = snd_ctl_add((*vortex).card, kcontrol);
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

unsafe fn vortex_a3d_unregister_controls(_vortex: *mut vortex_t) {

}

/* End of File*/

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
