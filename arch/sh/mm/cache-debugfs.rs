/*
 * debugfs ops for the L1 cache
 *
 *  Copyright (C) 2006  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct SeqFile {
    pub private: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct CacheInfo {
    pub sets: u32,
    pub entry_shift: u32,
    pub ways: u32,
    pub linesz: u32,
    pub way_incr: u64,
}

#[repr(C)]
pub struct CpuData {
    pub dcache: CacheInfo,
    pub icache: CacheInfo,
}

#[repr(C)]
pub struct FileOperations;

#[repr(C)]
pub struct Dentry;

extern "C" {
    static mut current_cpu_data: CpuData;
    static mut arch_debugfs_dir: *mut Dentry;
    static cache_debugfs_fops: FileOperations;

    fn jump_to_uncached();
    fn back_to_cached();
    fn __raw_readl(address: u64) -> u32;
    fn seq_printf(file: *mut SeqFile, format: *const core::ffi::c_char, ...);
    fn debugfs_create_file(
        name: *const core::ffi::c_char,
        mode: u16,
        parent: *mut Dentry,
        data: *mut core::ffi::c_void,
        fops: *const FileOperations,
    ) -> *mut Dentry;
}

// Values supplied by asm/cache.h and related kernel headers.
extern "C" {
    static SH_CCR: u64;
    static CCR_CACHE_ENABLE: u64;
    static CCR_CACHE_ORA: u64;
    static CACHE_OC_ADDRESS_ARRAY: u64;
    static CACHE_IC_ADDRESS_ARRAY: u64;
}

#[repr(C)]
pub enum CacheType {
    CACHE_TYPE_ICACHE,
    CACHE_TYPE_DCACHE,
    CACHE_TYPE_UNIFIED,
}

unsafe fn cache_debugfs_show(file: *mut SeqFile, _iter: *mut core::ffi::c_void) -> i32 {
    let cache_type = (*file).private as usize as u32;
    let cache: *mut CacheInfo;
    let mut waysize: u32;
    let mut way: u32;
    let ccr: u32;
    let mut addrstart: u64 = 0;

    /*
     * Go uncached immediately so we don't skew the results any
     * more than we already are..
     */
    jump_to_uncached();

    ccr = __raw_readl(SH_CCR);
    if (ccr & CCR_CACHE_ENABLE as u32) == 0 {
        back_to_cached();

        seq_printf(file, b"disabled\n\0".as_ptr() as *const i8);
        return 0;
    }

    if cache_type == CacheType::CACHE_TYPE_DCACHE as u32 {
        addrstart = CACHE_OC_ADDRESS_ARRAY;
        cache = &raw mut current_cpu_data.dcache;
    } else {
        addrstart = CACHE_IC_ADDRESS_ARRAY;
        cache = &raw mut current_cpu_data.icache;
    }

    waysize = (*cache).sets;

    /*
     * If the OC is already in RAM mode, we only have
     * half of the entries to consider..
     */
    if (ccr & CCR_CACHE_ORA as u32) != 0 && cache_type == CacheType::CACHE_TYPE_DCACHE as u32 {
        waysize >>= 1;
    }

    waysize <<= (*cache).entry_shift;

    way = 0;
    while way < (*cache).ways {
        let mut addr: u64;
        let mut line: u32;

        seq_printf(file, b"-----------------------------------------\n\0".as_ptr() as *const i8);
        seq_printf(file, b"Way %d\n\0".as_ptr() as *const i8, way);
        seq_printf(file, b"-----------------------------------------\n\0".as_ptr() as *const i8);

        addr = addrstart;
        line = 0;
        while addr < addrstart + waysize as u64 {
            let data = __raw_readl(addr) as u64;

            /* Check the V bit, ignore invalid cachelines */
            if (data & 1) != 0 {
                seq_printf(
                    file,
                    b"%3d: %c 0x%lx\n\0".as_ptr() as *const i8,
                    line,
                    if data & 2 != 0 { b'U' } else { b' ' },
                    data & 0x1ffffc00,
                );
            }

            addr += (*cache).linesz as u64;
            line += 1;
        }

        addrstart += (*cache).way_incr;
        way += 1;
    }

    back_to_cached();

    0
}

// DEFINE_SHOW_ATTRIBUTE(cache_debugfs);

unsafe fn cache_debugfs_init() -> i32 {
    debugfs_create_file(
        b"dcache\0".as_ptr() as *const i8,
        0o400,
        arch_debugfs_dir,
        CacheType::CACHE_TYPE_DCACHE as usize as *mut core::ffi::c_void,
        &cache_debugfs_fops,
    );
    debugfs_create_file(
        b"icache\0".as_ptr() as *const i8,
        0o400,
        arch_debugfs_dir,
        CacheType::CACHE_TYPE_ICACHE as usize as *mut core::ffi::c_void,
        &cache_debugfs_fops,
    );
    0
}

// module_init(cache_debugfs_init);
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
