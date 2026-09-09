// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation are intentionally
// left external here.

const BIOS_RAM_SIZE_KB_PTR: usize = 0x413;

const BIOS_START_MIN: u32 = 0x20000; // 128K, less than this is insane
const BIOS_START_MAX: u32 = 0x9f000; // 640K, absolute maximum

extern "C" {
    static x86_platform: X86Platform;
    fn __va(addr: usize) -> *const core::ffi::c_void;
    fn get_bios_ebda() -> u32;
    fn memblock_reserve(start: u32, size: u32);
}

#[repr(C)]
struct X86PlatformLegacy {
    reserve_bios_regions: bool,
}

#[repr(C)]
struct X86Platform {
    legacy: X86PlatformLegacy,
}

/*
 * This function reserves all conventional PC system BIOS related
 * firmware memory areas (some of which are data, some of which
 * are code), that must not be used by the kernel as available
 * RAM.
 *
 * The BIOS places the EBDA/XBDA at the top of conventional
 * memory, and usually decreases the reported amount of
 * conventional memory (int 0x12) too.
 *
 * This means that as a first approximation on most systems we can
 * guess the reserved BIOS area by looking at the low BIOS RAM size
 * value and assume that everything above that value (up to 1MB) is
 * reserved.
 *
 * But life in firmware country is not that simple:
 *
 * - This code also contains a quirk for Dell systems that neglect
 *   to reserve the EBDA area in the 'RAM size' value ...
 *
 * - The same quirk also avoids a problem with the AMD768MPX
 *   chipset: reserve a page before VGA to prevent PCI prefetch
 *   into it (errata #56). (Usually the page is reserved anyways,
 *   unless you have no PS/2 mouse plugged in.)
 *
 * - Plus paravirt systems don't have a reliable value in the
 *   'BIOS RAM size' pointer we can rely on, so we must quirk
 *   them too.
 *
 * Due to those various problems this function is deliberately
 * very conservative and tries to err on the side of reserving
 * too much, to not risk reserving too little.
 *
 * Losing a small amount of memory in the bottom megabyte is
 * rarely a problem, as long as we have enough memory to install
 * the SMP bootup trampoline which *must* be in this area.
 *
 * Using memory that is in use by the BIOS or by some DMA device
 * the BIOS didn't shut down *is* a big problem to the kernel,
 * obviously.
 */
pub unsafe extern "C" fn reserve_bios_regions() {
    let mut bios_start: u32;
    let ebda_start: u32;

    /*
     * NOTE: In a paravirtual environment the BIOS reserved
     * area is absent. We'll just have to assume that the
     * paravirt case can handle memory setup correctly,
     * without our help.
     */
    if !(*core::ptr::addr_of!(x86_platform)).legacy.reserve_bios_regions {
        return;
    }

    /*
     * BIOS RAM size is encoded in kilobytes, convert it
     * to bytes to get a first guess at where the BIOS
     * firmware area starts:
     */
    bios_start = core::ptr::read_unaligned(__va(BIOS_RAM_SIZE_KB_PTR) as *const u16) as u32;
    bios_start <<= 10;

    /*
     * If bios_start is less than 128K, assume it is bogus
     * and bump it up to 640K.  Similarly, if bios_start is above 640K,
     * don't trust it.
     */
    if bios_start < BIOS_START_MIN || bios_start > BIOS_START_MAX {
        bios_start = BIOS_START_MAX;
    }

    /* Get the start address of the EBDA page: */
    ebda_start = get_bios_ebda();

    /*
     * If the EBDA start address is sane and is below the BIOS region,
     * then also reserve everything from the EBDA start address up to
     * the BIOS region.
     */
    if ebda_start >= BIOS_START_MIN && ebda_start < bios_start {
        bios_start = ebda_start;
    }

    /* Reserve all memory between bios_start and the 1MB mark: */
    memblock_reserve(bios_start, 0x100000u32.wrapping_sub(bios_start));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
