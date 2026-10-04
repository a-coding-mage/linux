// SPDX-License-Identifier: GPL-2.0
/*
 * Physical/virtual KASLR and the compressed-boot entropy implementation.
 * Keep the unchanged kaslr.c and ../../lib/kaslr.c as the behavioral reference.
 * Firmware layouts and configuration constants come from the C boot headers.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of, addr_of_mut, read, read_unaligned};

use crate::bindings as b;
use b::{boot_e820_entry, boot_params, mem_vector, setup_data, setup_indirect};

#[inline]
unsafe fn maxmem() -> c_ulong {
    // misc.h selects the early-boot variable predicate, not cpu_feature_enabled.
    // Bindgen evaluates native MAXMEM for each branch without a C runtime body.
    // SAFETY: configure_5level_paging initializes this native unsigned-int flag
    // before KASLR, and compressed boot has exclusive access to its state.
    unsafe {
        if b::__pgtable_l5_enabled != 0 {
            b::LUPOS_BOOT_MAXMEM_L5 as c_ulong
        } else {
            b::LUPOS_BOOT_MAXMEM_L4 as c_ulong
        }
    }
}

#[inline]
#[cfg(CONFIG_X86_VERBOSE_BOOTUP)]
unsafe fn debug_putstr(value: *const c_char) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        b::__putstr(value);
    }
}

#[inline]
#[cfg(not(CONFIG_X86_VERBOSE_BOOTUP))]
unsafe fn debug_putstr(_value: *const c_char) {}

#[inline]
fn align_u64(value: u64, alignment: u64) -> u64 {
    let mask = alignment.wrapping_sub(1);
    value.wrapping_add(mask) & !mask
}

#[inline]
fn align_ulong(value: c_ulong, alignment: c_ulong) -> c_ulong {
    let mask = alignment.wrapping_sub(1);
    value.wrapping_add(mask) & !mask
}

unsafe fn rotate_xor(mut hash: c_ulong, area: *const c_void, size: usize) -> c_ulong {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let words = area.cast::<c_ulong>();
        let mut i = 0;
        while i < size / size_of::<c_ulong>() {
            hash = hash.rotate_right(7);
            // x86 allows an unaligned build string, just as the C load does.
            hash ^= read_unaligned(words.add(i));
            i += 1;
        }
        hash
    }
}

unsafe fn get_boot_seed() -> c_ulong {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        // The generated C string includes its terminating NUL: sizeof(build_str)
        // in C hashes that byte too whenever it completes an unsigned-long word.
        let mut hash = rotate_xor(
            0,
            b::LUPOS_BOOT_BUILD_STR.as_ptr().cast(),
            b::LUPOS_BOOT_BUILD_STR.len(),
        );
        hash = rotate_xor(hash, b::boot_params_ptr.cast(), size_of::<boot_params>());
        hash
    }
}

const I8254_PORT_CONTROL: u16 = 0x43;
const I8254_PORT_COUNTER0: u16 = 0x40;
const I8254_CMD_READBACK: u8 = 0xc0;
const I8254_SELECT_COUNTER0: u8 = 0x02;
const I8254_STATUS_NOTREADY: u8 = 0x40;

#[inline]
unsafe fn outb(value: u8, port: u16) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        // misc.h includes boot/io.h first: the compressed C inclusion uses the
        // initialized callbacks, including TDX overrides, rather than bare OUT.
        (b::pio_ops.f_outb.unwrap_unchecked())(value, port);
    }
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe { (b::pio_ops.f_inb.unwrap_unchecked())(port) }
}

unsafe fn i8254() -> u16 {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        loop {
            outb(
                I8254_CMD_READBACK | I8254_SELECT_COUNTER0,
                I8254_PORT_CONTROL,
            );
            let status = inb(I8254_PORT_COUNTER0);
            let mut timer = inb(I8254_PORT_COUNTER0) as u16;
            timer |= (inb(I8254_PORT_COUNTER0) as u16) << 8;
            if status & I8254_STATUS_NOTREADY == 0 {
                return timer;
            }
        }
    }
}

// The same bounded carry-flag retry loop as asm/archrandom.h:rdrand_long().
unsafe fn rdrand_long(value: *mut c_ulong) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let mut retry = b::RDRAND_RETRY_LOOPS as c_uint;
        loop {
            let raw: c_ulong;
            let ok: u8;
            core::arch::asm!("rdrand {raw}", "setc {ok}", raw = out(reg) raw,
            ok = out(reg_byte) ok, options(nomem, nostack));
            *value = raw;
            if ok != 0 {
                return true;
            }
            retry = retry.wrapping_sub(1);
            if retry == 0 {
                return false;
            }
        }
    }
}

unsafe fn rdtsc() -> u64 {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let low: u32;
        let high: u32;
        core::arch::asm!("rdtsc", out("eax") low, out("edx") high,
        options(nomem, nostack, preserves_flags));
        ((high as u64) << 32) | low as u64
    }
}

/// Compressed-boot version of arch/x86/lib/kaslr.c, including all entropy paths.
///
/// # Safety
/// Boot parameters and I/O callbacks must be initialized and mapped. `purpose`
/// is null or a readable C string. The caller runs in the early-boot CPU context.
#[no_mangle]
pub(crate) unsafe extern "C" fn kaslr_get_random_long(purpose: *const c_char) -> c_ulong {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        #[cfg(CONFIG_X86_64)]
        let mix_const: c_ulong = 0x5d6008cbf3848dd3;
        #[cfg(not(CONFIG_X86_64))]
        let mix_const: c_ulong = 0x3f39e593;
        let mut random = get_boot_seed();
        let mut raw = 0;
        let mut use_i8254 = true;

        if !purpose.is_null() {
            debug_putstr(purpose);
            debug_putstr(c" KASLR using".as_ptr());
        }
        if b::has_cpuflag(b::X86_FEATURE_RDRAND as c_int) {
            if !purpose.is_null() {
                debug_putstr(c" RDRAND".as_ptr());
            }
            if rdrand_long(&mut raw) {
                random ^= raw;
                use_i8254 = false;
            }
        }
        if b::has_cpuflag(b::X86_FEATURE_TSC as c_int) {
            if !purpose.is_null() {
                debug_putstr(c" RDTSC".as_ptr());
            }
            // The C assignment truncates the timestamp on x86_32.
            random ^= rdtsc() as c_ulong;
            use_i8254 = false;
        }
        if use_i8254 {
            if !purpose.is_null() {
                debug_putstr(c" i8254".as_ptr());
            }
            random ^= i8254() as c_ulong;
        }

        // Circular multiply: preserve both native-width halves without requiring
        // an early-boot 128-bit compiler-runtime multiplication helper.
        #[cfg(CONFIG_X86_64)]
        core::arch::asm!("mul {mix}", mix = in(reg) mix_const,
        inlateout("rax") random, lateout("rdx") raw, options(nomem, nostack));
        #[cfg(not(CONFIG_X86_64))]
        core::arch::asm!("mul {mix:e}", mix = in(reg) mix_const,
        inlateout("eax") random, lateout("edx") raw, options(nomem, nostack));
        random = random.wrapping_add(raw);
        if !purpose.is_null() {
            debug_putstr(c"...\n".as_ptr());
        }
        random
    }
}

const MAX_MEMMAP_REGIONS: usize = 4;
const MEM_AVOID_ZO_RANGE: usize = 0;
const MEM_AVOID_INITRD: usize = 1;
const MEM_AVOID_CMDLINE: usize = 2;
const MEM_AVOID_BOOTPARAMS: usize = 3;
const MEM_AVOID_MEMMAP_BEGIN: usize = 4;
const MEM_AVOID_MAX: usize = MEM_AVOID_MEMMAP_BEGIN + MAX_MEMMAP_REGIONS;
static mut MEMMAP_TOO_LARGE: bool = false;
static mut MEM_LIMIT: u64 = 0;
static mut NUM_IMMOVABLE_MEM: c_int = 0;
static mut MEM_AVOID: [mem_vector; MEM_AVOID_MAX] =
    [const { mem_vector { start: 0, size: 0 } }; MEM_AVOID_MAX];

#[inline]
unsafe fn mem_avoid(index: usize) -> *mut mem_vector {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe { addr_of_mut!(MEM_AVOID).cast::<mem_vector>().add(index) }
}

unsafe fn mem_overlaps(one: *const mem_vector, two: *const mem_vector) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if (*one).start.wrapping_add((*one).size) <= (*two).start {
            return false;
        }
        if (*one).start >= (*two).start.wrapping_add((*two).size) {
            return false;
        }
        true
    }
}

/// Skip whitespace in a readable NUL-terminated kernel command-line string.
///
/// # Safety
/// `value` remains readable through the first non-whitespace byte.
#[no_mangle]
pub(crate) unsafe extern "C" fn skip_spaces(mut value: *const c_char) -> *mut c_char {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        while crate::ctype::isspace(*value as c_int) {
            value = value.add(1);
        }
        value as *mut c_char
    }
}

// lib/ctype.rs and lib/cmdline.rs are shared by the freestanding boot crate.
unsafe fn parse_memmap(mut p: *mut c_char, start: *mut u64, size: *mut u64) -> c_int {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if p.is_null() || b::strncmp(p, c"exactmap".as_ptr(), 8) == 0 {
            return -(b::EINVAL as c_int);
        }
        let oldp = p;
        *size = crate::cmdline::memparse(p, &mut p);
        if p == oldp {
            return -(b::EINVAL as c_int);
        }
        match *p as u8 {
            b'#' | b'$' | b'!' => *start = crate::cmdline::memparse(p.add(1), &mut p),
            b'@' => {
                *size = 0;
                *start = 0;
            }
            _ => *start = 0,
        }
        0
    }
}

unsafe fn mem_avoid_memmap(mut value: *mut c_char) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        static mut INDEX: c_int = 0;
        if INDEX >= MAX_MEMMAP_REGIONS as c_int {
            return;
        }
        while !value.is_null() && INDEX < MAX_MEMMAP_REGIONS as c_int {
            let mut start = 0;
            let mut size = 0;
            let mut next = b::strchr(value, b',' as c_int);
            if !next.is_null() {
                *next = 0;
                next = next.add(1);
            }
            if parse_memmap(value, &mut start, &mut size) < 0 {
                break;
            }
            value = next;
            if start == 0 {
                if size > 0 && size < MEM_LIMIT {
                    MEM_LIMIT = size;
                }
                continue;
            }
            *mem_avoid(MEM_AVOID_MEMMAP_BEGIN + INDEX as usize) = mem_vector { start, size };
            INDEX += 1;
        }
        if INDEX >= MAX_MEMMAP_REGIONS as c_int && !value.is_null() {
            MEMMAP_TOO_LARGE = true;
        }
    }
}

static mut MAX_GB_HUGE_PAGES: c_ulong = 0;

unsafe fn parse_gb_huge_pages(param: *mut c_char, val: *mut c_char) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        static mut GBPAGE_SZ: bool = false;
        if b::strcmp(param, c"hugepagesz".as_ptr()) == 0 {
            let mut p = val;
            if crate::cmdline::memparse(p, &mut p) != b::PUD_SIZE as u64 {
                GBPAGE_SZ = false;
                return;
            }
            if GBPAGE_SZ {
                b::warn(c"Repeatedly set hugeTLB page size of 1G!\n".as_ptr());
            }
            GBPAGE_SZ = true;
            return;
        }
        if b::strcmp(param, c"hugepages".as_ptr()) == 0 && GBPAGE_SZ {
            if b::boot_kstrtoul(val, 0, addr_of_mut!(MAX_GB_HUGE_PAGES)) != 0 {
                b::warn(c"Failed to parse hugepages= boot parameter\n".as_ptr());
            }
        }
    }
}

unsafe fn handle_mem_options() {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let mut args = b::get_cmd_line_ptr() as *mut c_char;
        if args.is_null() {
            return;
        }
        let len = b::strnlen(args, b::COMMAND_LINE_SIZE as usize - 1);
        // malloc's C prototype accepts int, not size_t.
        let tmp_cmdline = b::malloc(len.wrapping_add(1) as c_int).cast::<c_char>();
        if tmp_cmdline.is_null() {
            b::error(
                c"Failed to allocate space for tmp_cmdline"
                    .as_ptr()
                    .cast_mut(),
            );
        }
        b::memcpy(tmp_cmdline.cast(), args.cast(), len);
        *tmp_cmdline.add(len) = 0;
        args = skip_spaces(tmp_cmdline);
        while *args != 0 {
            let mut param = core::ptr::null_mut();
            let mut val = core::ptr::null_mut();
            args = crate::cmdline::next_arg(args, &mut param, &mut val);
            if val.is_null() && b::strcmp(param, c"--".as_ptr()) == 0 {
                break;
            }
            if b::strcmp(param, c"memmap".as_ptr()) == 0 {
                mem_avoid_memmap(val);
            } else if cfg!(CONFIG_X86_64) && !b::strstr(param, c"hugepages".as_ptr()).is_null() {
                parse_gb_huge_pages(param, val);
            } else if b::strcmp(param, c"mem".as_ptr()) == 0 {
                let mut p = val;
                if b::strcmp(p, c"nopentium".as_ptr()) == 0 {
                    continue;
                }
                let mem_size = crate::cmdline::memparse(p, &mut p);
                if mem_size == 0 {
                    break;
                }
                if mem_size < MEM_LIMIT {
                    MEM_LIMIT = mem_size;
                }
            }
        }
        b::free(tmp_cmdline.cast());
    }
}

unsafe fn mem_avoid_init(input: c_ulong, _input_size: c_ulong, output: c_ulong) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let init_size = (*b::boot_params_ptr).hdr.init_size as c_ulong;
        *mem_avoid(MEM_AVOID_ZO_RANGE) = mem_vector {
            start: input as u64,
            // This expression is unsigned long before assignment to u64 in C.
            size: output.wrapping_add(init_size).wrapping_sub(input) as u64,
        };
        let initrd_start = ((*b::boot_params_ptr).ext_ramdisk_image as u64) << 32
            | (*b::boot_params_ptr).hdr.ramdisk_image as u64;
        let initrd_size = ((*b::boot_params_ptr).ext_ramdisk_size as u64) << 32
            | (*b::boot_params_ptr).hdr.ramdisk_size as u64;
        *mem_avoid(MEM_AVOID_INITRD) = mem_vector {
            start: initrd_start,
            size: initrd_size,
        };
        let cmd_line = b::get_cmd_line_ptr();
        if cmd_line != 0 {
            let cmd_line_size =
                b::strnlen(cmd_line as *const c_char, b::COMMAND_LINE_SIZE as usize - 1)
                    .wrapping_add(1) as c_ulong;
            *mem_avoid(MEM_AVOID_CMDLINE) = mem_vector {
                start: cmd_line as u64,
                size: cmd_line_size as u64,
            };
        }
        *mem_avoid(MEM_AVOID_BOOTPARAMS) = mem_vector {
            start: b::boot_params_ptr as c_ulong as u64,
            size: size_of::<boot_params>() as u64,
        };
        handle_mem_options();
        #[cfg(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE, CONFIG_ACPI))]
        {
            NUM_IMMOVABLE_MEM = b::count_immovable_mem_regions();
        }
        #[cfg(not(all(CONFIG_RANDOMIZE_BASE, CONFIG_MEMORY_HOTREMOVE, CONFIG_ACPI)))]
        {
            NUM_IMMOVABLE_MEM = 0;
        }
    }
}

unsafe fn mem_avoid_overlap(img: *const mem_vector, overlap: *mut mem_vector) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let mut earliest = (*img).start.wrapping_add((*img).size);
        let mut found = false;
        let mut i = 0;
        while i < MEM_AVOID_MAX {
            let avoid = mem_avoid(i);
            if mem_overlaps(img, avoid) && (*avoid).start < earliest {
                *overlap = read(avoid);
                earliest = (*overlap).start;
                found = true;
            }
            i += 1;
        }
        let mut ptr = (*b::boot_params_ptr).hdr.setup_data as c_ulong as *const setup_data;
        while !ptr.is_null() {
            let mut avoid = mem_vector {
                start: ptr as c_ulong as u64,
                // sizeof() and len are size_t-width before assignment to u64.
                size: size_of::<setup_data>().wrapping_add((*ptr).len as usize) as u64,
            };
            if mem_overlaps(img, &avoid) && avoid.start < earliest {
                *overlap = read(&avoid);
                earliest = avoid.start;
                found = true;
            }
            let indirect = addr_of!((*ptr).data).cast::<setup_indirect>();
            if (*ptr).type_ == b::SETUP_INDIRECT && (*indirect).type_ != b::SETUP_INDIRECT {
                avoid.start = (*indirect).addr;
                avoid.size = (*indirect).len;
                if mem_overlaps(img, &avoid) && avoid.start < earliest {
                    *overlap = read(&avoid);
                    earliest = avoid.start;
                    found = true;
                }
            }
            ptr = (*ptr).next as c_ulong as *const setup_data;
        }
        found
    }
}

struct SlotArea {
    addr: u64,
    num: c_ulong,
}
const MAX_SLOT_AREA: usize = 100;
static mut SLOT_AREAS: [SlotArea; MAX_SLOT_AREA] =
    [const { SlotArea { addr: 0, num: 0 } }; MAX_SLOT_AREA];
static mut SLOT_AREA_INDEX: c_uint = 0;
static mut SLOT_MAX: c_ulong = 0;

#[inline]
unsafe fn slot_area(index: c_uint) -> *mut SlotArea {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        addr_of_mut!(SLOT_AREAS)
            .cast::<SlotArea>()
            .add(index as usize)
    }
}

unsafe fn store_slot_info(region: *const mem_vector, image_size: c_ulong) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if SLOT_AREA_INDEX as usize == MAX_SLOT_AREA {
            return;
        }
        let num = 1u64.wrapping_add(
            (*region).size.wrapping_sub(image_size as u64) / b::CONFIG_PHYSICAL_ALIGN as u64,
        ) as c_ulong;
        *slot_area(SLOT_AREA_INDEX) = SlotArea {
            addr: (*region).start,
            num,
        };
        SLOT_AREA_INDEX += 1;
        SLOT_MAX = SLOT_MAX.wrapping_add(num);
    }
}

unsafe fn process_gb_huge_pages(region: *const mem_vector, image_size: c_ulong) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if !cfg!(CONFIG_X86_64) || MAX_GB_HUGE_PAGES == 0 {
            store_slot_info(region, image_size);
            return;
        }
        let pud_start = align_u64((*region).start, b::PUD_SIZE as u64);
        let mut pud_end =
            (*region).start.wrapping_add((*region).size) & !(b::PUD_SIZE as u64).wrapping_sub(1);
        if pud_start >= pud_end {
            store_slot_info(region, image_size);
            return;
        }
        if pud_start >= (*region).start.wrapping_add(image_size as u64) {
            let tmp = mem_vector {
                start: (*region).start,
                size: pud_start.wrapping_sub((*region).start),
            };
            store_slot_info(&tmp, image_size);
        }
        let gb_huge_pages = (pud_end.wrapping_sub(pud_start) >> b::PUD_SHIFT) as c_ulong;
        if gb_huge_pages > MAX_GB_HUGE_PAGES {
            pud_end =
                pud_start.wrapping_add(MAX_GB_HUGE_PAGES.wrapping_shl(b::PUD_SHIFT as u32) as u64);
            MAX_GB_HUGE_PAGES = 0;
        } else {
            MAX_GB_HUGE_PAGES = MAX_GB_HUGE_PAGES.wrapping_sub(gb_huge_pages);
        }
        if (*region).start.wrapping_add((*region).size) >= pud_end.wrapping_add(image_size as u64) {
            let tmp = mem_vector {
                start: pud_end,
                size: (*region)
                    .start
                    .wrapping_add((*region).size)
                    .wrapping_sub(pud_end),
            };
            store_slot_info(&tmp, image_size);
        }
    }
}

unsafe fn slots_fetch_random() -> u64 {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if SLOT_MAX == 0 {
            return 0;
        }
        let mut slot = kaslr_get_random_long(c"Physical".as_ptr()) % SLOT_MAX;
        let mut i = 0;
        while i < SLOT_AREA_INDEX {
            let area = slot_area(i);
            if slot >= (*area).num {
                slot = slot.wrapping_sub((*area).num);
                i += 1;
                continue;
            }
            return (*area)
                .addr
                .wrapping_add((slot as u64).wrapping_mul(b::CONFIG_PHYSICAL_ALIGN as u64));
        }
        debug_putstr(c"slots_fetch_random() failed!?\n".as_ptr());
        0
    }
}

unsafe fn __process_mem_region(entry: *const mem_vector, minimum: c_ulong, image_size: c_ulong) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let mut region = mem_vector {
            start: core::cmp::max((*entry).start, minimum as u64),
            size: 0,
        };
        let region_end = core::cmp::min((*entry).start.wrapping_add((*entry).size), MEM_LIMIT);
        while (SLOT_AREA_INDEX as usize) < MAX_SLOT_AREA {
            region.start = align_u64(region.start, b::CONFIG_PHYSICAL_ALIGN as u64);
            if region.start > region_end {
                return;
            }
            region.size = region_end.wrapping_sub(region.start);
            if region.size < image_size as u64 {
                return;
            }
            let mut overlap = mem_vector { start: 0, size: 0 };
            if !mem_avoid_overlap(&region, &mut overlap) {
                process_gb_huge_pages(&region, image_size);
                return;
            }
            if overlap.start >= region.start.wrapping_add(image_size as u64) {
                region.size = overlap.start.wrapping_sub(region.start);
                process_gb_huge_pages(&region, image_size);
            }
            region.start = overlap.start.wrapping_add(overlap.size);
        }
    }
}

unsafe fn process_mem_region(
    region: *const mem_vector,
    minimum: c_ulong,
    image_size: c_ulong,
) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if NUM_IMMOVABLE_MEM == 0 {
            __process_mem_region(region, minimum, image_size);
            if SLOT_AREA_INDEX as usize == MAX_SLOT_AREA {
                debug_putstr(c"Aborted e820/efi memmap scan (slot_areas full)!\n".as_ptr());
                return true;
            }
            return false;
        }
        #[cfg(all(CONFIG_MEMORY_HOTREMOVE, CONFIG_ACPI))]
        {
            let mut i = 0;
            while i < NUM_IMMOVABLE_MEM {
                let immovable = addr_of!(b::immovable_mem)
                    .cast::<mem_vector>()
                    .add(i as usize);
                i += 1;
                if !mem_overlaps(region, immovable) {
                    continue;
                }
                let start = (*immovable).start;
                let end = start.wrapping_add((*immovable).size);
                let region_end = (*region).start.wrapping_add((*region).size);
                // linux/minmax.h tests the high limit first. Preserve the exact
                // ternary order even if start + size wraps below the low limit.
                let entry_start = if (*region).start >= end {
                    end
                } else if (*region).start <= start {
                    start
                } else {
                    (*region).start
                };
                let entry_end = if region_end >= end {
                    end
                } else if region_end <= start {
                    start
                } else {
                    region_end
                };
                let entry = mem_vector {
                    start: entry_start,
                    size: entry_end.wrapping_sub(entry_start),
                };
                __process_mem_region(&entry, minimum, image_size);
                if SLOT_AREA_INDEX as usize == MAX_SLOT_AREA {
                    debug_putstr(c"Aborted e820/efi memmap scan when walking immovable regions(slot_areas full)!\n".as_ptr());
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(CONFIG_EFI)]
unsafe fn memory_type_is_free(md: *const b::efi_memory_desc_t) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        (*md).type_ == b::EFI_CONVENTIONAL_MEMORY
            || (cfg!(CONFIG_UNACCEPTED_MEMORY) && (*md).type_ == b::EFI_UNACCEPTED_MEMORY)
    }
}

#[cfg(CONFIG_EFI)]
unsafe fn efi_memdesc(pmap: c_ulong, desc_size: u32, index: c_int) -> *const b::efi_memory_desc_t {
    // efi_early_memdesc_ptr multiplies int by u32 *before* pointer addition.
    let offset = (index as u32).wrapping_mul(desc_size);
    pmap.wrapping_add(offset as c_ulong) as *const b::efi_memory_desc_t
}

#[cfg(CONFIG_EFI)]
unsafe fn process_efi_entries(minimum: c_ulong, image_size: c_ulong) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let e = addr_of!((*b::boot_params_ptr).efi_info);
        let signature = addr_of!((*e).efi_loader_signature).cast::<c_char>();
        if b::strncmp(signature, b::EFI32_LOADER_SIGNATURE.as_ptr().cast(), 4) != 0
            && b::strncmp(signature, b::EFI64_LOADER_SIGNATURE.as_ptr().cast(), 4) != 0
        {
            return false;
        }
        #[cfg(CONFIG_X86_32)]
        let pmap = {
            if (*e).efi_memmap_hi != 0 {
                b::warn(c"EFI memmap is above 4GB, can't be handled now on x86_32. EFI should be disabled.\n".as_ptr());
                return false;
            }
            (*e).efi_memmap as c_ulong
        };
        #[cfg(not(CONFIG_X86_32))]
        let pmap = ((*e).efi_memmap as u64 | ((*e).efi_memmap_hi as u64) << 32) as c_ulong;
        let nr_desc = (*e).efi_memmap_size / (*e).efi_memdesc_size;
        let mut efi_mirror_found = false;
        let mut i: c_int = 0;
        while (i as u32) < nr_desc {
            let md = efi_memdesc(pmap, (*e).efi_memdesc_size, i);
            if (*md).attribute & b::EFI_MEMORY_MORE_RELIABLE as u64 != 0 {
                efi_mirror_found = true;
                break;
            }
            i = i.wrapping_add(1);
        }
        i = 0;
        while (i as u32) < nr_desc {
            let md = efi_memdesc(pmap, (*e).efi_memdesc_size, i);
            i = i.wrapping_add(1);
            if !memory_type_is_free(md) {
                continue;
            }
            #[cfg(CONFIG_EFI_SOFT_RESERVE)]
            if b::__efi_soft_reserve_enabled() && (*md).attribute & b::EFI_MEMORY_SP as u64 != 0 {
                continue;
            }
            if efi_mirror_found && (*md).attribute & b::EFI_MEMORY_MORE_RELIABLE as u64 == 0 {
                continue;
            }
            let region = mem_vector {
                start: (*md).phys_addr,
                size: (*md).num_pages.wrapping_shl(b::EFI_PAGE_SHIFT as u32),
            };
            if process_mem_region(&region, minimum, image_size) {
                break;
            }
        }
        true
    }
}

#[cfg(not(CONFIG_EFI))]
unsafe fn process_efi_entries(_minimum: c_ulong, _image_size: c_ulong) -> bool {
    false
}

unsafe fn process_e820_entries(minimum: c_ulong, image_size: c_ulong) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let mut i = 0;
        while i < (*b::boot_params_ptr).e820_entries as usize {
            let entry = addr_of!((*b::boot_params_ptr).e820_table)
                .cast::<boot_e820_entry>()
                .add(i);
            i += 1;
            if (*entry).type_ != b::E820_TYPE_RAM {
                continue;
            }
            let region = mem_vector {
                start: (*entry).addr,
                size: (*entry).size,
            };
            if process_mem_region(&region, minimum, image_size) {
                break;
            }
        }
    }
}

unsafe fn process_kho_entries(minimum: c_ulong, image_size: c_ulong) -> bool {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if !cfg!(CONFIG_KEXEC_HANDOVER) {
            return false;
        }
        let mut scratch = core::ptr::null::<b::kho_scratch>();
        let mut nr_areas: c_int = 0;
        let mut ptr = (*b::boot_params_ptr).hdr.setup_data as c_ulong as *const setup_data;
        while !ptr.is_null() {
            if (*ptr).type_ == b::SETUP_KEXEC_KHO {
                let kho = addr_of!((*ptr).data).cast::<b::kho_data>();
                scratch = (*kho).scratch_addr as c_ulong as *const b::kho_scratch;
                nr_areas = ((*kho).scratch_size / size_of::<b::kho_scratch>() as u64) as c_int;
                break;
            }
            ptr = (*ptr).next as c_ulong as *const setup_data;
        }
        if nr_areas == 0 {
            return false;
        }
        let mut i = 0;
        while i < nr_areas {
            let area = scratch.add(i as usize);
            let region = mem_vector {
                start: (*area).addr as u64,
                size: (*area).size as u64,
            };
            if process_mem_region(&region, minimum, image_size) {
                break;
            }
            i += 1;
        }
        true
    }
}

unsafe fn find_random_phys_addr(minimum: c_ulong, image_size: c_ulong) -> c_ulong {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        // Addition is unsigned-long width before C's comparison promotes to u64.
        if minimum.wrapping_add(image_size) as u64 > MEM_LIMIT {
            return 0;
        }
        if MEMMAP_TOO_LARGE {
            debug_putstr(c"Aborted memory entries scan (more than 4 memmap= args)!\n".as_ptr());
            return 0;
        }
        if !process_kho_entries(minimum, image_size) && !process_efi_entries(minimum, image_size) {
            process_e820_entries(minimum, image_size);
        }
        let phys_addr = slots_fetch_random();
        if phys_addr < minimum as u64 || phys_addr.wrapping_add(image_size as u64) > MEM_LIMIT {
            b::warn(c"Invalid physical address chosen!\n".as_ptr());
            return 0;
        }
        phys_addr as c_ulong
    }
}

unsafe fn find_random_virt_addr(minimum: c_ulong, image_size: c_ulong) -> c_ulong {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        let slots = (1 as c_ulong).wrapping_add(
            (b::KERNEL_IMAGE_SIZE as c_ulong)
                .wrapping_sub(minimum)
                .wrapping_sub(image_size)
                / b::CONFIG_PHYSICAL_ALIGN as c_ulong,
        );
        let random_addr = kaslr_get_random_long(c"Virtual".as_ptr()) % slots;
        random_addr
            .wrapping_mul(b::CONFIG_PHYSICAL_ALIGN as c_ulong)
            .wrapping_add(minimum)
    }
}

/// Select physical and virtual kernel locations using the early-boot memory map.
///
/// # Safety
/// The boot parameters and their referenced firmware/setup records must be
/// initialized and mapped, the allocator and I/O ready, and both output pointers
/// writable. The caller provides exclusive access to this single-boot state.
#[no_mangle]
pub(crate) unsafe extern "C" fn choose_random_location(
    input: c_ulong,
    input_size: c_ulong,
    output: *mut c_ulong,
    output_size: c_ulong,
    virt_addr: *mut c_ulong,
) {
    // SAFETY: The early-boot caller supplies the original C routine's valid
    // memory, initialized I/O, and exclusive-state preconditions.
    unsafe {
        if b::cmdline_find_option_bool(c"nokaslr".as_ptr()) != 0 {
            b::warn(c"KASLR disabled: 'nokaslr' on cmdline.".as_ptr());
            return;
        }
        (*b::boot_params_ptr).hdr.loadflags |= b::KASLR_FLAG as u8;
        MEM_LIMIT = if cfg!(CONFIG_X86_32) {
            b::KERNEL_IMAGE_SIZE as u64
        } else {
            maxmem() as u64
        };
        mem_avoid_init(input, input_size, *output);
        let min_addr = align_ulong(
            core::cmp::min(*output, (512 as c_ulong) << 20),
            b::CONFIG_PHYSICAL_ALIGN as c_ulong,
        );
        let mut random_addr = find_random_phys_addr(min_addr, output_size);
        if random_addr == 0 {
            b::warn(c"Physical KASLR disabled: no suitable memory region!".as_ptr());
        } else if *output != random_addr {
            *output = random_addr;
        }
        if cfg!(CONFIG_X86_64) {
            random_addr = find_random_virt_addr(b::LOAD_PHYSICAL_ADDR as c_ulong, output_size);
        }
        *virt_addr = random_addr;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
