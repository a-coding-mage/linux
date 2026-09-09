/*
** asm-m68k/amigahw.h -- This header defines some macros and pointers for
**                    the various Amiga custom hardware registers.
**                    The naming conventions used here conform to those
**                    used in the Amiga Hardware Reference Manual, 3rd Edition
**
** Copyright 1992 by Greg Harp
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
** Created: 9/24/92 by Greg Harp
*/

// C dependencies: linux/ioport.h and asm/bootinfo-amiga.h.

extern "C" {
    pub static mut amiga_chipset: libc::c_ulong;
    pub static mut amiga_eclock: libc::c_ulong;
    pub static mut amiga_colorclock: libc::c_ulong;
    pub static mut amiga_chip_size: libc::c_ulong;
    pub static mut amiga_vblank: libc::c_uchar;
}

#[repr(C)]
pub struct amiga_hw_present {
    pub AMI_VIDEO: libc::c_uint, pub AMI_BLITTER: libc::c_uint, pub AMBER_FF: libc::c_uint,
    pub AMI_AUDIO: libc::c_uint,
    pub AMI_FLOPPY: libc::c_uint, pub A3000_SCSI: libc::c_uint, pub A4000_SCSI: libc::c_uint,
    pub A1200_IDE: libc::c_uint, pub A4000_IDE: libc::c_uint, pub CD_ROM: libc::c_uint,
    pub AMI_KEYBOARD: libc::c_uint, pub AMI_MOUSE: libc::c_uint, pub AMI_SERIAL: libc::c_uint,
    pub AMI_PARALLEL: libc::c_uint,
    pub A2000_CLK: libc::c_uint, pub A3000_CLK: libc::c_uint,
    pub CHIP_RAM: libc::c_uint, pub PAULA: libc::c_uint, pub DENISE: libc::c_uint,
    pub DENISE_HR: libc::c_uint, pub LISA: libc::c_uint, pub AGNUS_PAL: libc::c_uint,
    pub AGNUS_NTSC: libc::c_uint, pub AGNUS_HR_PAL: libc::c_uint, pub AGNUS_HR_NTSC: libc::c_uint,
    pub ALICE_PAL: libc::c_uint, pub ALICE_NTSC: libc::c_uint, pub MAGIC_REKICK: libc::c_uint,
    pub PCMCIA: libc::c_uint, pub ZORRO: libc::c_uint, pub ZORRO3: libc::c_uint,
}
extern "C" { pub static mut amiga_hw_present: amiga_hw_present; }

#[repr(C)]
pub struct CUSTOM {
    pub bltddat:u16,pub dmaconr:u16,pub vposr:u16,pub vhposr:u16,pub dskdatr:u16,pub joy0dat:u16,pub joy1dat:u16,pub clxdat:u16,pub adkconr:u16,pub pot0dat:u16,pub pot1dat:u16,pub potgor:u16,pub serdatr:u16,pub dskbytr:u16,pub intenar:u16,pub intreqr:u16,
    pub dskptr:*mut u8,pub dsklen:u16,pub dskdat:u16,pub refptr:u16,pub vposw:u16,pub vhposw:u16,pub copcon:u16,pub serdat:u16,pub serper:u16,pub potgo:u16,pub joytest:u16,pub strequ:u16,pub strvbl:u16,pub strhor:u16,pub strlong:u16,
    pub bltcon0:u16,pub bltcon1:u16,pub bltafwm:u16,pub bltalwm:u16,pub bltcpt:*mut u8,pub bltbpt:*mut u8,pub bltapt:*mut u8,pub bltdpt:*mut u8,pub bltsize:u16,pub pad2d:u8,pub bltcon0l:u8,pub bltsizv:u16,pub bltsizh:u16,pub bltcmod:u16,pub bltbmod:u16,pub bltamod:u16,pub bltdmod:u16,pub spare2:[u16;4],pub bltcdat:u16,pub bltbdat:u16,pub bltadat:u16,pub spare3:[u16;3],pub deniseid:u16,pub dsksync:u16,pub cop1lc:*mut u16,pub cop2lc:*mut u16,pub copjmp1:u16,pub copjmp2:u16,pub copins:u16,pub diwstrt:u16,pub diwstop:u16,pub ddfstrt:u16,pub ddfstop:u16,pub dmacon:u16,pub clxcon:u16,pub intena:u16,pub intreq:u16,pub adkcon:u16,
    pub aud:[Audio;4],pub bplpt:[*mut u8;8],pub bplcon0:u16,pub bplcon1:u16,pub bplcon2:u16,pub bplcon3:u16,pub bpl1mod:u16,pub bpl2mod:u16,pub bplcon4:u16,pub clxcon2:u16,pub bpldat:[u16;8],pub sprpt:[*mut u8;8],pub spr:[Sprite;8],pub color:[u16;32],pub htotal:u16,pub hsstop:u16,pub hbstrt:u16,pub hbstop:u16,pub vtotal:u16,pub vsstop:u16,pub vbstrt:u16,pub vbstop:u16,pub sprhstrt:u16,pub sprhstop:u16,pub bplhstrt:u16,pub bplhstop:u16,pub hhposw:u16,pub hhposr:u16,pub beamcon0:u16,pub hsstrt:u16,pub vsstrt:u16,pub hcenter:u16,pub diwhigh:u16,pub spare4:[u16;11],pub fmode:u16,
}
#[repr(C)] pub struct Audio { pub audlc:*mut u16,pub audlen:u16,pub audper:u16,pub audvol:u16,pub auddat:u16,pub audspare:[u16;2] }
#[repr(C)] pub struct Sprite { pub pos:u16,pub ctl:u16,pub dataa:u16,pub datab:u16 }

pub const DMAF_SETCLR: u16 = 0x8000; pub const DMAF_AUD0: u16 = 0x0001;
pub const DMAF_AUD1: u16 = 0x0002; pub const DMAF_AUD2: u16 = 0x0004;
pub const DMAF_AUD3: u16 = 0x0008; pub const DMAF_DISK: u16 = 0x0010;
pub const DMAF_SPRITE: u16 = 0x0020; pub const DMAF_BLITTER: u16 = 0x0040;
pub const DMAF_COPPER: u16 = 0x0080; pub const DMAF_RASTER: u16 = 0x0100;
pub const DMAF_MASTER: u16 = 0x0200; pub const DMAF_BLITHOG: u16 = 0x0400;
pub const DMAF_BLTNZERO: u16 = 0x2000; pub const DMAF_BLTDONE: u16 = 0x4000;
pub const DMAF_ALL: u16 = 0x01ff;

#[repr(C)]
pub struct CIA { pub data: [libc::c_uchar; 0], }

pub const zTwoBase: usize = 0x80000000;
#[inline] pub fn ZTWO_PADDR(x: usize) -> usize { x.wrapping_sub(zTwoBase) }
#[inline] pub fn ZTWO_VADDR(x: usize) -> *mut libc::c_void { x.wrapping_add(zTwoBase) as *mut libc::c_void }
pub const CUSTOM_PHYSADDR: usize = 0xdff000;
pub const CIAA_PHYSADDR: usize = 0xbfe001;
pub const CIAB_PHYSADDR: usize = 0xbfd000;
pub const CHIP_PHYSADDR: usize = 0;

extern "C" {
    pub fn amiga_chip_init();
    pub fn amiga_chip_alloc(size: libc::c_ulong, name: *const libc::c_char) -> *mut libc::c_void;
    pub fn amiga_chip_alloc_res(size: libc::c_ulong, res: *mut libc::c_void) -> *mut libc::c_void;
    pub fn amiga_chip_free(ptr: *mut libc::c_void);
    pub fn amiga_chip_avail() -> libc::c_ulong;
    pub static mut amiga_audio_min_period: libc::c_ushort;
}

// `CS_ECS` and `CS_AGA` are supplied by asm/bootinfo-amiga.h.
#[inline]
pub unsafe fn amifb_video_off() {
    if amiga_chipset == CS_ECS || amiga_chipset == CS_AGA {
        // program Denise/Lisa for a higher maximum play rate
        // The CUSTOM register layout is supplied by the hardware bindings.
        (*amiga_custom).htotal = 113; (*amiga_custom).vtotal = 223; (*amiga_custom).beamcon0 = 0x4390;
        (*amiga_custom).hsstrt = 116; (*amiga_custom).hsstop = 116;
        (*amiga_custom).vsstrt = 226; (*amiga_custom).vsstop = 226;
        amiga_audio_min_period = 57;
    }
}

#[repr(C)]
pub struct tod3000 {
    pub second2: u32, pub second1: u32, pub minute2: u32, pub minute1: u32,
    pub hour2: u32, pub hour1: u32, pub weekday: u32, pub day2: u32, pub day1: u32,
    pub month2: u32, pub month1: u32, pub year2: u32, pub year1: u32,
    pub cntrl1: u32, pub cntrl2: u32, pub cntrl3: u32,
}
pub const TOD3000_CNTRL1_HOLD: u32 = 0;
pub const TOD3000_CNTRL1_FREE: u32 = 9;

#[repr(C)]
pub struct tod2000 {
    pub second2: u32, pub second1: u32, pub minute2: u32, pub minute1: u32,
    pub hour2: u32, pub hour1: u32, pub day2: u32, pub day1: u32,
    pub month2: u32, pub month1: u32, pub year2: u32, pub year1: u32,
    pub weekday: u32, pub cntrl1: u32, pub cntrl2: u32, pub cntrl3: u32,
}
pub const TOD2000_CNTRL1_HOLD: u32 = 1 << 0;
pub const TOD2000_CNTRL1_BUSY: u32 = 1 << 1;
pub const TOD2000_CNTRL3_24HMODE: u32 = 1 << 2;
pub const TOD2000_HOUR1_PM: u32 = 1 << 2;

pub const amiga_custom: *mut CUSTOM = (zTwoBase + CUSTOM_PHYSADDR) as *mut CUSTOM;
pub const ciaa: *mut CIA = (zTwoBase + CIAA_PHYSADDR) as *mut CIA;
pub const ciab: *mut CIA = (zTwoBase + CIAB_PHYSADDR) as *mut CIA;
pub const tod_3000: *mut tod3000 = (zTwoBase + 0xdc0000) as *mut tod3000;
pub const tod_2000: *mut tod2000 = (zTwoBase + 0xdc0000) as *mut tod2000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
