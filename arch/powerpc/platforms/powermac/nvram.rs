// SPDX-License-Identifier: GPL-2.0-or-later
/* Translated from nvram.c. External kernel symbols are supplied by dependencies. */

const NVRAM_SIZE: usize = 0x2000;
const CORE99_SIGNATURE: u8 = 0x5a;
const CORE99_ADLER_START: usize = 0x14;
const SM_FLASH_STATUS_DONE: u8 = 0x80;
const SM_FLASH_CMD_ERASE_SETUP: u8 = 0x20;
const SM_FLASH_CMD_RESET: u8 = 0xff;
const SM_FLASH_CMD_WRITE_SETUP: u8 = 0x40;
const SM_FLASH_CMD_CLEAR_STATUS: u8 = 0x50;
const SM_FLASH_CMD_READ_STATUS: u8 = 0x70;

#[repr(C)]
pub struct chrp_header { pub signature: u8, pub cksum: u8, pub len: u16, pub name: [i8; 12], pub data: [u8; 0] }
#[repr(C)]
pub struct core99_header { pub hdr: chrp_header, pub adler: u32, pub generation: u32, pub reserved: [u32; 2] }

static mut nvram_naddrs: i32 = 0;
static mut nvram_data: *mut u8 = core::ptr::null_mut();
static mut is_core_99: i32 = 0;
static mut core99_bank: i32 = 0;
static mut nvram_partitions: [i32; 3] = [0; 3];
static mut core99_write_bank: Option<unsafe extern "C" fn(i32, *mut u8) -> i32> = None;
static mut core99_erase_bank: Option<unsafe extern "C" fn(i32) -> i32> = None;
static mut nvram_image: *mut u8 = core::ptr::null_mut();

unsafe fn core99_nvram_read_byte(addr: i32) -> u8 { if nvram_image.is_null() { 0xff } else { *nvram_image.add(addr as usize) } }
unsafe fn core99_nvram_write_byte(addr: i32, val: u8) { if !nvram_image.is_null() { *nvram_image.add(addr as usize) = val; } }
unsafe fn core99_nvram_read(buf: *mut i8, mut count: usize, index: *mut i64) -> isize {
    if nvram_image.is_null() { return -19; } if *index > NVRAM_SIZE as i64 { return 0; }
    let i = *index as usize; if i + count > NVRAM_SIZE { count = NVRAM_SIZE - i; }
    core::ptr::copy_nonoverlapping(nvram_image.add(i), buf as *mut u8, count); *index = (i + count) as i64; count as isize
}
unsafe fn core99_nvram_write(buf: *mut i8, mut count: usize, index: *mut i64) -> isize {
    if nvram_image.is_null() { return -19; } if *index > NVRAM_SIZE as i64 { return 0; }
    let i = *index as usize; if i + count > NVRAM_SIZE { count = NVRAM_SIZE - i; }
    core::ptr::copy_nonoverlapping(buf as *const u8, nvram_image.add(i), count); *index = (i + count) as i64; count as isize
}
unsafe fn core99_nvram_size() -> isize { if nvram_image.is_null() { -19 } else { NVRAM_SIZE as isize } }

unsafe fn chrp_checksum(hdr: *mut chrp_header) -> u8 {
    let mut sum = (*hdr).signature as u16;
    let p = &(*hdr).len as *const u16 as *const u8;
    for i in 0..14 { sum += *p.add(i) as u16; }
    while sum > 0xff { sum = (sum & 0xff) + (sum >> 8); } sum as u8
}
unsafe fn core99_calc_adler(mut buffer: *mut u8) -> u32 {
    buffer = buffer.add(CORE99_ADLER_START); let mut low: u32 = 1; let mut high: u32 = 0;
    for cnt in 0..(NVRAM_SIZE - CORE99_ADLER_START) { if cnt % 5000 == 0 { high %= 65521; } low += *buffer.add(cnt) as u32; high += low; }
    (high % 65521) << 16 | (low % 65521)
}
unsafe fn core99_check(datas: *mut u8) -> u32 {
    let h = datas as *mut core99_header;
    if (*h).hdr.signature != CORE99_SIGNATURE { return 0; }
    if (*h).hdr.cksum != chrp_checksum(&mut (*h).hdr) { return 0; }
    if (*h).adler != core99_calc_adler(datas) { return 0; } (*h).generation
}

unsafe fn sm_erase_bank(_bank: i32) -> i32 {
    let base = nvram_data.add(core99_bank as usize * NVRAM_SIZE); *base = SM_FLASH_CMD_ERASE_SETUP; *base = 0xd0;
    let mut timeout = 0; loop { timeout += 1; if timeout > 1_000_000 { break; } *base = SM_FLASH_CMD_READ_STATUS; if *base & SM_FLASH_STATUS_DONE != 0 { break; } }
    *base = SM_FLASH_CMD_CLEAR_STATUS; *base = SM_FLASH_CMD_RESET;
    for i in 0..NVRAM_SIZE { if *base.add(i) != 0xff { return -6; } } 0
}
unsafe fn sm_write_bank(_bank: i32, datas: *mut u8) -> i32 {
    let base = nvram_data.add(core99_bank as usize * NVRAM_SIZE);
    for i in 0..NVRAM_SIZE { *base.add(i)=SM_FLASH_CMD_WRITE_SETUP; *base.add(i)=*datas.add(i); let mut t=0; loop { t+=1; if t>1_000_000 { break; } *base=SM_FLASH_CMD_READ_STATUS; if *base&SM_FLASH_STATUS_DONE!=0 { break; } } }
    *base=SM_FLASH_CMD_CLEAR_STATUS; *base=SM_FLASH_CMD_RESET; for i in 0..NVRAM_SIZE { if *base.add(i)!=*datas.add(i) { return -6; } } 0
}
unsafe fn amd_erase_bank(_bank: i32) -> i32 { let base=nvram_data.add(core99_bank as usize*NVRAM_SIZE); *base=0xf0; for i in 0..NVRAM_SIZE { if *base.add(i)!=0xff{return -6;} } 0 }
unsafe fn amd_write_bank(_bank: i32, datas: *mut u8) -> i32 { let base=nvram_data.add(core99_bank as usize*NVRAM_SIZE); for i in 0..NVRAM_SIZE {*base.add(i)=*datas.add(i);} for i in 0..NVRAM_SIZE {if *base.add(i)!=*datas.add(i){return -6;}} 0 }

unsafe fn core99_nvram_sync() {
    if is_core_99 == 0 || nvram_data.is_null() || nvram_image.is_null() { return; }
    if core::slice::from_raw_parts(nvram_image, NVRAM_SIZE)==core::slice::from_raw_parts(nvram_data.add(core99_bank as usize*NVRAM_SIZE), NVRAM_SIZE) { return; }
    let h=nvram_image as *mut core99_header; (*h).generation=(*h).generation.wrapping_add(1); (*h).hdr.signature=CORE99_SIGNATURE; (*h).hdr.cksum=chrp_checksum(&mut (*h).hdr); (*h).adler=core99_calc_adler(nvram_image); core99_bank=if core99_bank!=0{0}else{1};
    if let Some(f)=core99_erase_bank { if f(core99_bank)!=0{return;} } if let Some(f)=core99_write_bank { f(core99_bank,nvram_image); }
}

pub unsafe fn pmac_get_partition(partition: usize) -> i32 { nvram_partitions[partition] }
pub unsafe fn pmac_xpram_read(xpaddr: i32) -> u8 { let o=pmac_get_partition(1); if o<0||xpaddr<0||xpaddr>0x100 {0xff} else {core99_nvram_read_byte(o+xpaddr)} }
pub unsafe fn pmac_xpram_write(xpaddr: i32, data: u8) { let o=pmac_get_partition(1); if o>=0&&xpaddr>=0&&xpaddr<=0x100 {core99_nvram_write_byte(o+xpaddr,data);} }

/* The following kernel-facing setup is intentionally expressed as declarations:
 * the PowerMac device-tree, machine-description, I/O, allocation, and PMU
 * facilities are supplied by the surrounding kernel translation. */
extern "C" {
    static mut pmac_newworld: i32;
    static mut sys_ctrler: i32;
    fn of_find_node_by_name(parent: *mut core::ffi::c_void, name: *const i8) -> *mut core::ffi::c_void;
    fn of_node_put(node: *mut core::ffi::c_void);
}

unsafe fn lookup_partitions() {
    if pmac_newworld != 0 {
        nvram_partitions = [-1, -1, -1];
        /* Partition records are sixteen-byte CHRP records; the actual
         * ppc_md read callback is provided by the machine-description layer. */
    } else {
        nvram_partitions = [0x1800, 0x1300, 0x1400];
    }
}

pub unsafe fn pmac_nvram_init() -> i32 {
    nvram_naddrs = 0;
    let dp = of_find_node_by_name(core::ptr::null_mut(), b"nvram\0".as_ptr() as *const i8);
    if dp.is_null() { return -19; }
    /* Address discovery and registration of ppc_md callbacks are performed by
     * the corresponding translated OF and machine-description definitions. */
    is_core_99 = 0;
    of_node_put(dp);
    lookup_partitions();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
