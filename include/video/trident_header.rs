/* SPDX-License-Identifier: GPL-2.0 */

/* C preprocessor configuration: TRIDENTFB_DEBUG defaults to disabled. */
pub const TRIDENTFB_DEBUG: i32 = 0;

/* Kernel logging macros; the referenced logging facilities are supplied externally. */
#[cfg(feature = "tridentfb_debug")]
macro_rules! debug {
    ($f:expr $(, $a:expr)*) => {{
        printk!(concat!("{}:", $f), module_path!() $(, $a)*);
    }};
}
#[cfg(not(feature = "tridentfb_debug"))]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

macro_rules! output {
    ($f:expr $(, $a:expr)*) => {{
        pr_info!(concat!("tridentfb: ", $f) $(, $a)*);
    }};
}

pub const Kb: i32 = 1024;
pub const Mb: i32 = Kb * Kb;

/* PCI IDS of supported cards temporarily here */
pub const CYBER9320: u32 = 0x9320;
pub const CYBER9388: u32 = 0x9388;
pub const CYBER9382: u32 = 0x9382; /* the real PCI id for this is 9660 */
pub const CYBER9385: u32 = 0x9385; /* ditto */
pub const CYBER9397: u32 = 0x9397;
pub const CYBER9397DVD: u32 = 0x939A;
pub const CYBER9520: u32 = 0x9520;
pub const CYBER9525DVD: u32 = 0x9525;
pub const TGUI9440: u32 = 0x9440;
pub const TGUI9660: u32 = 0x9660;
pub const PROVIDIA9685: u32 = 0x9685;
pub const IMAGE975: u32 = 0x9750;
pub const IMAGE985: u32 = 0x9850;
pub const BLADE3D: u32 = 0x9880;
pub const CYBERBLADEE4: u32 = 0x9540;
pub const CYBERBLADEi7: u32 = 0x8400;
pub const CYBERBLADEi7D: u32 = 0x8420;
pub const CYBERBLADEi1: u32 = 0x8500;
pub const CYBERBLADEi1D: u32 = 0x8520;
pub const CYBERBLADEAi1: u32 = 0x8600;
pub const CYBERBLADEAi1D: u32 = 0x8620;
pub const CYBERBLADEXPAi1: u32 = 0x8820;
pub const CYBERBLADEXPm8: u32 = 0x9910;
pub const CYBERBLADEXPm16: u32 = 0x9930;

/* these defines are for 'lcd' variable */
pub const LCD_STRETCH: i32 = 0;
pub const LCD_CENTER: i32 = 1;
pub const LCD_BIOS: i32 = 2;

/* General Registers */
pub const SPR: u32 = 0x1F; /* Software Programming Register (videoram) */

/* 3C4 */
pub const RevisionID: u32 = 0x09;
pub const OldOrNew: u32 = 0x0B;
pub const ConfPort1: u32 = 0x0C;
pub const ConfPort2: u32 = 0x0C;
pub const NewMode2: u32 = 0x0D;
pub const NewMode1: u32 = 0x0E;
pub const Protection: u32 = 0x11;
pub const MCLKLow: u32 = 0x16;
pub const MCLKHigh: u32 = 0x17;
pub const ClockLow: u32 = 0x18;
pub const ClockHigh: u32 = 0x19;
pub const SSetup: u32 = 0x20;
pub const SKey: u32 = 0x37;
pub const SPKey: u32 = 0x57;

/* 3x4 */
pub const CRTCModuleTest: u32 = 0x1E;
pub const FIFOControl: u32 = 0x20;
pub const LinearAddReg: u32 = 0x21;
pub const DRAMTiming: u32 = 0x23;
pub const New32: u32 = 0x23;
pub const RAMDACTiming: u32 = 0x25;
pub const CRTHiOrd: u32 = 0x27;
pub const AddColReg: u32 = 0x29;
pub const InterfaceSel: u32 = 0x2A;
pub const HorizOverflow: u32 = 0x2B;
pub const GETest: u32 = 0x2D;
pub const Performance: u32 = 0x2F;
pub const GraphEngReg: u32 = 0x36;
pub const I2C: u32 = 0x37;
pub const PixelBusReg: u32 = 0x38;
pub const PCIReg: u32 = 0x39;
pub const DRAMControl: u32 = 0x3A;
pub const MiscContReg: u32 = 0x3C;
pub const CursorXLow: u32 = 0x40;
pub const CursorXHigh: u32 = 0x41;
pub const CursorYLow: u32 = 0x42;
pub const CursorYHigh: u32 = 0x43;
pub const CursorLocLow: u32 = 0x44;
pub const CursorLocHigh: u32 = 0x45;
pub const CursorXOffset: u32 = 0x46;
pub const CursorYOffset: u32 = 0x47;
pub const CursorFG1: u32 = 0x48;
pub const CursorFG2: u32 = 0x49;
pub const CursorFG3: u32 = 0x4A;
pub const CursorFG4: u32 = 0x4B;
pub const CursorBG1: u32 = 0x4C;
pub const CursorBG2: u32 = 0x4D;
pub const CursorBG3: u32 = 0x4E;
pub const CursorBG4: u32 = 0x4F;
pub const CursorControl: u32 = 0x50;
pub const PCIRetry: u32 = 0x55;
pub const PreEndControl: u32 = 0x56;
pub const PreEndFetch: u32 = 0x57;
pub const PCIMaster: u32 = 0x60;
pub const Enhancement0: u32 = 0x62;
pub const NewEDO: u32 = 0x64;
pub const TVinterface: u32 = 0xC0;
pub const TVMode: u32 = 0xC1;
pub const ClockControl: u32 = 0xCF;

/* 3CE */
pub const MiscExtFunc: u32 = 0x0F;
pub const PowerStatus: u32 = 0x23;
pub const MiscIntContReg: u32 = 0x2F;
pub const CyberControl: u32 = 0x30;
pub const CyberEnhance: u32 = 0x31;
pub const FPConfig: u32 = 0x33;
pub const VertStretch: u32 = 0x52;
pub const HorStretch: u32 = 0x53;
pub const BiosMode: u32 = 0x5c;
pub const BiosReg: u32 = 0x5d;

/* Graphics Engine */
pub const STATUS: u32 = 0x2120;
pub const OLDCMD: u32 = 0x2124;
pub const DRAWFL: u32 = 0x2128;
pub const OLDCLR: u32 = 0x212C;
pub const OLDDST: u32 = 0x2138;
pub const OLDSRC: u32 = 0x213C;
pub const OLDDIM: u32 = 0x2140;
pub const CMD: u32 = 0x2144;
pub const ROP: u32 = 0x2148;
pub const COLOR: u32 = 0x2160;
pub const BGCOLOR: u32 = 0x2164;
pub const SRC1: u32 = 0x2100;
pub const SRC2: u32 = 0x2104;
pub const DST1: u32 = 0x2108;
pub const DST2: u32 = 0x210C;

pub const ROP_S: u32 = 0xCC;
pub const ROP_P: u32 = 0xF0;
pub const ROP_X: u32 = 0x66;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
