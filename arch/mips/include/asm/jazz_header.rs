/*
 * Rust translation of asm/jazz.h.
 * Original file is subject to the GNU General Public License.
 */

pub const JAZZ_LOCAL_IO_SPACE: u32 = 0xe0000000;
pub const PICA_ASIC_REVISION: u32 = 0xe0000008;
pub const PICA_LED: u32 = 0xe000f000;

pub const LED_DOT: u32 = 0x01;
pub const LED_SPACE: u32 = 0x00;
pub const LED_0: u32 = 0xfc;
pub const LED_1: u32 = 0x60;
pub const LED_2: u32 = 0xda;
pub const LED_3: u32 = 0xf2;
pub const LED_4: u32 = 0x66;
pub const LED_5: u32 = 0xb6;
pub const LED_6: u32 = 0xbe;
pub const LED_7: u32 = 0xe0;
pub const LED_8: u32 = 0xfe;
pub const LED_9: u32 = 0xf6;
pub const LED_A: u32 = 0xee;
pub const LED_b: u32 = 0x3e;
pub const LED_C: u32 = 0x9c;
pub const LED_d: u32 = 0x7a;
pub const LED_E: u32 = 0x9e;
pub const LED_F: u32 = 0x8e;

#[inline]
pub unsafe fn pica_set_led(bits: u32) {
    core::ptr::write_volatile(PICA_LED as *mut u32, bits);
}

pub const JAZZ_ETHERNET_BASE: u32 = 0xe0001000;
pub const JAZZ_SCSI_BASE: u32 = 0xe0002000;
pub const JAZZ_KEYBOARD_ADDRESS: u32 = 0xe0005000;
pub const JAZZ_KEYBOARD_DATA: u32 = 0xe0005000;
pub const JAZZ_KEYBOARD_COMMAND: u32 = 0xe0005001;

#[repr(C)]
pub struct jazz_keyboard_hardware { pub data: u8, pub command: u8 }
pub const jazz_kh: *mut jazz_keyboard_hardware = JAZZ_KEYBOARD_ADDRESS as *mut jazz_keyboard_hardware;

#[repr(C)]
pub struct mips_keyboard_hardware {
    pub pad0: [u8; 3], pub data: u8, pub pad1: [u8; 3], pub command: u8,
}
pub type keyboard_hardware = jazz_keyboard_hardware;

pub const MIPS_KEYBOARD_ADDRESS: u32 = 0xb9005000;
pub const MIPS_KEYBOARD_DATA: u32 = 0xb9005003;
pub const MIPS_KEYBOARD_COMMAND: u32 = 0xb9005007;
pub const JAZZ_SERIAL1_BASE: u32 = 0xe0006000;
pub const JAZZ_SERIAL2_BASE: u32 = 0xe0007000;
pub const JAZZ_PARALLEL_BASE: u32 = 0xe0008000;
pub const JAZZ_DUMMY_DEVICE: u32 = 0xe000d000;
pub const JAZZ_TIMER_INTERVAL: u32 = 0xe0000228;
pub const JAZZ_TIMER_REGISTER: u32 = 0xe0000230;

// C bit-field layout is endian-dependent; the register is represented as its raw word.
#[repr(transparent)]
pub struct dram_configuration(pub u32);

pub const PICA_DRAM_CONFIG: u32 = 0xe00fffe0;
pub const JAZZ_IO_IRQ_SOURCE: u32 = 0xe0010000;
pub const JAZZ_IO_IRQ_ENABLE: u32 = 0xe0010002;
pub const JAZZ_IRQ_START: u32 = 24;
pub const JAZZ_IRQ_END: u32 = JAZZ_IRQ_START + 9;
pub const JAZZ_PARALLEL_IRQ: u32 = JAZZ_IRQ_START + 0;
pub const JAZZ_FLOPPY_IRQ: u32 = JAZZ_IRQ_START + 1;
pub const JAZZ_SOUND_IRQ: u32 = JAZZ_IRQ_START + 2;
pub const JAZZ_VIDEO_IRQ: u32 = JAZZ_IRQ_START + 3;
pub const JAZZ_ETHERNET_IRQ: u32 = JAZZ_IRQ_START + 4;
pub const JAZZ_SCSI_IRQ: u32 = JAZZ_IRQ_START + 5;
pub const JAZZ_KEYBOARD_IRQ: u32 = JAZZ_IRQ_START + 6;
pub const JAZZ_MOUSE_IRQ: u32 = JAZZ_IRQ_START + 7;
pub const JAZZ_SERIAL1_IRQ: u32 = JAZZ_IRQ_START + 8;
pub const JAZZ_SERIAL2_IRQ: u32 = JAZZ_IRQ_START + 9;
pub const JAZZ_TIMER_IRQ: u32 = MIPS_CPU_IRQ_BASE + 6;

pub const JAZZ_SCSI_DMA: u32 = 0;
pub const JAZZ_FLOPPY_DMA: u32 = 1;
pub const JAZZ_AUDIOL_DMA: u32 = 2;
pub const JAZZ_AUDIOR_DMA: u32 = 3;

pub const JAZZ_R4030_CONFIG: u32 = 0xE0000000;
pub const JAZZ_R4030_REVISION: u32 = 0xE0000008;
pub const JAZZ_R4030_INV_ADDR: u32 = 0xE0000010;
pub const JAZZ_R4030_TRSTBL_BASE: u32 = 0xE0000018;
pub const JAZZ_R4030_TRSTBL_LIM: u32 = 0xE0000020;
pub const JAZZ_R4030_TRSTBL_INV: u32 = 0xE0000028;
pub const JAZZ_R4030_CACHE_MTNC: u32 = 0xE0000030;
pub const JAZZ_R4030_R_FAIL_ADDR: u32 = 0xE0000038;
pub const JAZZ_R4030_M_FAIL_ADDR: u32 = 0xE0000040;
pub const JAZZ_R4030_CACHE_PTAG: u32 = 0xE0000048;
pub const JAZZ_R4030_CACHE_LTAG: u32 = 0xE0000050;
pub const JAZZ_R4030_CACHE_BMASK: u32 = 0xE0000058;
pub const JAZZ_R4030_CACHE_BWIN: u32 = 0xE0000060;
pub const JAZZ_R4030_REM_SPEED: u32 = 0xE0000070;
pub const JAZZ_R4030_IRQ_ENABLE: u32 = 0xE00000E8;
pub const JAZZ_R4030_INVAL_ADDR: u32 = 0xE0000010;
pub const JAZZ_R4030_IRQ_SOURCE: u32 = 0xE0000200;
pub const JAZZ_R4030_I386_ERROR: u32 = 0xE0000208;
pub const JAZZ_EISA_IRQ_ACK: u32 = 0xE0000238;

#[inline]
pub unsafe fn r4030_delay() {
    // The C implementation emits four ordered MIPS nops.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}
#[inline]
pub unsafe fn r4030_read_reg16(addr: u64) -> u16 { let ret = core::ptr::read_volatile(addr as *const u16); r4030_delay(); ret }
#[inline]
pub unsafe fn r4030_read_reg32(addr: u64) -> u32 { let ret = core::ptr::read_volatile(addr as *const u32); r4030_delay(); ret }
#[inline]
pub unsafe fn r4030_write_reg16(addr: u64, val: u32) { core::ptr::write_volatile(addr as *mut u16, val as u16); r4030_delay(); }
#[inline]
pub unsafe fn r4030_write_reg32(addr: u64, val: u32) { core::ptr::write_volatile(addr as *mut u32, val); r4030_delay(); }

pub const JAZZ_FDC_BASE: u32 = 0xe0003000;
pub const JAZZ_RTC_BASE: u32 = 0xe0004000;
pub const JAZZ_PORT_BASE: u32 = 0xe2000000;
pub const JAZZ_EISA_BASE: u32 = 0xe3000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
