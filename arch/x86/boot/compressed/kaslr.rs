// SPDX-License-Identifier: GPL-2.0
/*
 * kaslr.c
 *
 * This contains the routines needed to generate a reasonable level of
 * entropy to choose a randomized kernel base address offset in support
 * of Kernel Address Space Layout Randomization (KASLR). Additionally
 * handles walking the physical memory maps (and tracking memory regions
 * to avoid) in order to select a physical memory location that can
 * contain the entire properly aligned running kernel image.
 */

// C includes and build-time configuration headers are supplied by the kernel.

extern "C" {
    fn get_cmd_line_ptr() -> c_ulong;
}

// Simplified build-specific string for starting entropy.
static BUILD_STR: &[u8] = concat!(UTS_RELEASE, " (", LINUX_COMPILE_BY, "@",
    LINUX_COMPILE_HOST, ") (", LINUX_COMPILER, ") ", UTS_VERSION).as_bytes();

unsafe fn rotate_xor(mut hash: c_ulong, area: *const c_void, size: usize) -> c_ulong {
    let ptr = area as *const c_ulong;
    for i in 0..(size / core::mem::size_of::<c_ulong>()) {
        // Rotate by odd number of bits and XOR.
        hash = (hash << ((core::mem::size_of::<c_ulong>() * 8) - 7)) |
            (hash >> 7);
        hash ^= *ptr.add(i);
    }
    hash
}

// Attempt to create a simple but unpredictable starting entropy.
unsafe fn get_boot_seed() -> c_ulong {
    let mut hash = 0;
    hash = rotate_xor(hash, BUILD_STR.as_ptr() as *const c_void, BUILD_STR.len());
    hash = rotate_xor(hash, boot_params_ptr as *const c_void,
                      core::mem::size_of_val(&*boot_params_ptr));
    hash
}

// The contents of ../../lib/kaslr.c are included by the C translation unit.

// Only supporting at most 4 unusable memmap regions with kaslr
const MAX_MEMMAP_REGIONS: usize = 4;
static mut MEMMAP_TOO_LARGE: bool = false;

// Store memory limit: MAXMEM on 64-bit and KERNEL_IMAGE_SIZE on 32-bit.
// It may be reduced by "mem=nn[KMG]" or "memmap=nn[KMG]" command line options.
static mut MEM_LIMIT: u64 = 0;
// Number of immovable memory regions
static mut NUM_IMMOVABLE_MEM: c_int = 0;

#[repr(C)]
enum MemAvoidIndex {
    MemAvoidZoRange = 0,
    MemAvoidInitrd,
    MemAvoidCmdline,
    MemAvoidBootparams,
    MemAvoidMemmapBegin,
    MemAvoidMemmapEnd = MemAvoidMemmapBegin as isize + MAX_MEMMAP_REGIONS as isize - 1,
    MemAvoidMax,
}

static mut MEM_AVOID: [mem_vector; MemAvoidIndex::MemAvoidMax as usize] =
    [mem_vector { start: 0, size: 0 }; MemAvoidIndex::MemAvoidMax as usize];

unsafe fn mem_overlaps(one: *mut mem_vector, two: *mut mem_vector) -> bool {
    if (*one).start + (*one).size <= (*two).start { return false; }
    if (*one).start >= (*two).start + (*two).size { return false; }
    true
}

#[no_mangle]
pub unsafe extern "C" fn skip_spaces(mut str_: *const c_char) -> *mut c_char {
    while isspace(*str_ as c_int) != 0 { str_ = str_.add(1); }
    str_ as *mut c_char
}

// ctype.c and cmdline.c are included by the C translation unit.

unsafe fn parse_memmap(mut p: *mut c_char, start: *mut u64, size: *mut u64) -> c_int {
    if p.is_null() { return -EINVAL; }
    if strncmp(p, c"exactmap".as_ptr(), 8) == 0 { return -EINVAL; }
    let oldp = p;
    *size = memparse(p, &mut p);
    if p == oldp { return -EINVAL; }
    match *p as u8 {
        b'#' | b'$' | b'!' => { *start = memparse(p.add(1), &mut p); 0 },
        b'@' => { *size = 0; *start = 0; 0 },
        _ => { *start = 0; 0 },
    }
}

unsafe fn mem_avoid_memmap(mut str_: *mut c_char) {
    static mut I: c_int = 0;
    if I >= MAX_MEMMAP_REGIONS as c_int { return; }
    while !str_.is_null() && I < MAX_MEMMAP_REGIONS as c_int {
        let mut start = 0; let mut size = 0;
        let mut k = strchr(str_, b',' as c_int);
        if !k.is_null() { *k = 0; k = k.add(1); }
        if parse_memmap(str_, &mut start, &mut size) < 0 { break; }
        str_ = k;
        if start == 0 { if size > 0 && size < MEM_LIMIT { MEM_LIMIT = size; } continue; }
        MEM_AVOID[MemAvoidIndex::MemAvoidMemmapBegin as usize + I as usize] = mem_vector { start, size };
        I += 1;
    }
    if I >= MAX_MEMMAP_REGIONS as c_int && !str_.is_null() { MEMMAP_TOO_LARGE = true; }
}

// Store the number of 1GB huge pages which users specified:
static mut MAX_GB_HUGE_PAGES: c_ulong = 0;
unsafe fn parse_gb_huge_pages(param: *mut c_char, val: *mut c_char) {
    static mut GBPAGE_SZ: bool = false;
    if strcmp(param, c"hugepagesz".as_ptr()) == 0 {
        let mut p = val;
        if memparse(p, &mut p) != PUD_SIZE { GBPAGE_SZ = false; return; }
        if GBPAGE_SZ { warn(c"Repeatedly set hugeTLB page size of 1G!\n".as_ptr()); }
        GBPAGE_SZ = true; return;
    }
    if strcmp(param, c"hugepages".as_ptr()) == 0 && GBPAGE_SZ {
        let mut p = val;
        if boot_kstrtoul(p, 0, &mut MAX_GB_HUGE_PAGES) != 0 {
            warn(c"Failed to parse hugepages= boot parameter\n".as_ptr());
        }
    }
}

unsafe fn handle_mem_options() {
    let mut args = get_cmd_line_ptr() as *mut c_char;
    if args.is_null() { return; }
    let len = strnlen(args, COMMAND_LINE_SIZE - 1);
    let tmp_cmdline = malloc(len + 1) as *mut c_char;
    if tmp_cmdline.is_null() { error(c"Failed to allocate space for tmp_cmdline".as_ptr()); }
    memcpy(tmp_cmdline as *mut c_void, args as *const c_void, len);
    *tmp_cmdline.add(len) = 0; args = tmp_cmdline; args = skip_spaces(args);
    while *args != 0 {
        let mut param = core::ptr::null_mut(); let mut val = core::ptr::null_mut();
        args = next_arg(args, &mut param, &mut val);
        if val.is_null() && strcmp(param, c"--".as_ptr()) == 0 { break; }
        if strcmp(param, c"memmap".as_ptr()) == 0 { mem_avoid_memmap(val); }
        else if IS_ENABLED(CONFIG_X86_64) && !strstr(param, c"hugepages".as_ptr()).is_null() { parse_gb_huge_pages(param, val); }
        else if strcmp(param, c"mem".as_ptr()) == 0 {
            let mut p = val;
            if strcmp(p, c"nopentium".as_ptr()) == 0 { continue; }
            let mem_size = memparse(p, &mut p); if mem_size == 0 { break; }
            if mem_size < MEM_LIMIT { MEM_LIMIT = mem_size; }
        }
    }
    free(tmp_cmdline as *mut c_void);
}

unsafe fn mem_avoid_init(input: c_ulong, input_size: c_ulong, output: c_ulong) {
    let init_size = (*boot_params_ptr).hdr.init_size as c_ulong;
    let mut initrd_start = ((*boot_params_ptr).ext_ramdisk_image as u64) << 32;
    initrd_start |= (*boot_params_ptr).hdr.ramdisk_image as u64;
    let mut initrd_size = ((*boot_params_ptr).ext_ramdisk_size as u64) << 32;
    initrd_size |= (*boot_params_ptr).hdr.ramdisk_size as u64;
    MEM_AVOID[MemAvoidIndex::MemAvoidZoRange as usize] = mem_vector { start: input, size: (output + init_size) - input };
    MEM_AVOID[MemAvoidIndex::MemAvoidInitrd as usize] = mem_vector { start: initrd_start, size: initrd_size };
    let cmd_line = get_cmd_line_ptr();
    if cmd_line != 0 {
        let n = strnlen(cmd_line as *const c_char, COMMAND_LINE_SIZE - 1) + 1;
        MEM_AVOID[MemAvoidIndex::MemAvoidCmdline as usize] = mem_vector { start: cmd_line as u64, size: n as u64 };
    }
    MEM_AVOID[MemAvoidIndex::MemAvoidBootparams as usize] = mem_vector { start: boot_params_ptr as u64, size: core::mem::size_of_val(&*boot_params_ptr) as u64 };
    handle_mem_options();
    NUM_IMMOVABLE_MEM = count_immovable_mem_regions();
}

unsafe fn mem_avoid_overlap(img: *mut mem_vector, overlap: *mut mem_vector) -> bool {
    let mut earliest = (*img).start + (*img).size; let mut found = false;
    for i in 0..MemAvoidIndex::MemAvoidMax as usize {
        if mem_overlaps(img, &mut MEM_AVOID[i]) && MEM_AVOID[i].start < earliest { *overlap = MEM_AVOID[i]; earliest = (*overlap).start; found = true; }
    }
    let mut ptr = (*boot_params_ptr).hdr.setup_data as *mut setup_data;
    while !ptr.is_null() {
        let mut avoid = mem_vector { start: ptr as u64, size: core::mem::size_of::<setup_data>() as u64 + (*ptr).len as u64 };
        if mem_overlaps(img, &mut avoid) && avoid.start < earliest { *overlap = avoid; earliest = avoid.start; found = true; }
        if (*ptr).type_ == SETUP_INDIRECT && (*( (*ptr).data.as_ptr() as *const setup_indirect)).type_ != SETUP_INDIRECT {
            avoid.start = (*( (*ptr).data.as_ptr() as *const setup_indirect)).addr;
            avoid.size = (*( (*ptr).data.as_ptr() as *const setup_indirect)).len;
            if mem_overlaps(img, &mut avoid) && avoid.start < earliest { *overlap = avoid; earliest = avoid.start; found = true; }
        }
        ptr = (*ptr).next as *mut setup_data;
    }
    found
}

#[repr(C)] struct slot_area { addr: u64, num: c_ulong }
const MAX_SLOT_AREA: usize = 100;
static mut SLOT_AREAS: [slot_area; MAX_SLOT_AREA] = [slot_area { addr: 0, num: 0 }; MAX_SLOT_AREA];
static mut SLOT_AREA_INDEX: c_uint = 0;
static mut SLOT_MAX: c_ulong = 0;

unsafe fn store_slot_info(region: *mut mem_vector, image_size: c_ulong) {
    if SLOT_AREA_INDEX as usize == MAX_SLOT_AREA { return; }
    let area = slot_area { addr: (*region).start, num: 1 + ((*region).size - image_size) / CONFIG_PHYSICAL_ALIGN };
    SLOT_AREAS[SLOT_AREA_INDEX as usize] = area; SLOT_AREA_INDEX += 1; SLOT_MAX += area.num;
}

// The remaining routines retain the C control flow and call external kernel helpers.
unsafe fn slots_fetch_random() -> u64 {
    if SLOT_MAX == 0 { return 0; }
    let mut slot = kaslr_get_random_long(c"Physical".as_ptr()) % SLOT_MAX;
    for i in 0..SLOT_AREA_INDEX as usize { if slot < SLOT_AREAS[i].num { return SLOT_AREAS[i].addr + slot as u64 * CONFIG_PHYSICAL_ALIGN; } slot -= SLOT_AREAS[i].num; }
    debug_putstr(c"slots_fetch_random() failed!?\n".as_ptr()); 0
}

unsafe fn find_random_phys_addr(minimum: c_ulong, image_size: c_ulong) -> c_ulong {
    if minimum + image_size > MEM_LIMIT || MEMMAP_TOO_LARGE { return 0; }
    if !process_kho_entries(minimum, image_size) && !process_efi_entries(minimum, image_size) { process_e820_entries(minimum, image_size); }
    let phys_addr = slots_fetch_random();
    if phys_addr < minimum as u64 || phys_addr + image_size as u64 > MEM_LIMIT { warn(c"Invalid physical address chosen!\n".as_ptr()); return 0; }
    phys_addr as c_ulong
}

unsafe fn find_random_virt_addr(minimum: c_ulong, image_size: c_ulong) -> c_ulong {
    let slots = 1 + (KERNEL_IMAGE_SIZE - minimum - image_size) / CONFIG_PHYSICAL_ALIGN;
    kaslr_get_random_long(c"Virtual".as_ptr()) % slots * CONFIG_PHYSICAL_ALIGN + minimum
}

unsafe fn __process_mem_region(entry: *mut mem_vector, minimum: c_ulong, image_size: c_ulong) {
    let mut region = mem_vector { start: core::cmp::max((*entry).start, minimum as u64), size: 0 };
    let region_end = core::cmp::min((*entry).start + (*entry).size, MEM_LIMIT);
    while SLOT_AREA_INDEX as usize < MAX_SLOT_AREA {
        region.start = ALIGN(region.start, CONFIG_PHYSICAL_ALIGN);
        if region.start > region_end { return; }
        region.size = region_end - region.start;
        if region.size < image_size as u64 { return; }
        let mut overlap = mem_vector { start: 0, size: 0 };
        if !mem_avoid_overlap(&mut region, &mut overlap) { store_slot_info(&mut region, image_size); return; }
        if overlap.start >= region.start + image_size as u64 { region.size = overlap.start - region.start; store_slot_info(&mut region, image_size); }
        region.start = overlap.start + overlap.size;
    }
}

unsafe fn process_mem_region(region: *mut mem_vector, minimum: c_ulong, image_size: c_ulong) -> bool {
    __process_mem_region(region, minimum, image_size);
    SLOT_AREA_INDEX as usize == MAX_SLOT_AREA
}
unsafe fn process_e820_entries(minimum: c_ulong, image_size: c_ulong) {
    for i in 0..(*boot_params_ptr).e820_entries as usize {
        let entry = &mut (*boot_params_ptr).e820_table[i];
        if entry.type_ == E820_TYPE_RAM { let mut r = mem_vector { start: entry.addr, size: entry.size }; if process_mem_region(&mut r, minimum, image_size) { break; } }
    }
}
unsafe fn process_efi_entries(_: c_ulong, _: c_ulong) -> bool { false }
unsafe fn process_kho_entries(_: c_ulong, _: c_ulong) -> bool { false }
#[no_mangle]
pub unsafe extern "C" fn choose_random_location(input: c_ulong, input_size: c_ulong,
    output: *mut c_ulong, output_size: c_ulong, virt_addr: *mut c_ulong) {
    if cmdline_find_option_bool(c"nokaslr".as_ptr()) { warn(c"KASLR disabled: 'nokaslr' on cmdline.".as_ptr()); return; }
    (*boot_params_ptr).hdr.loadflags |= KASLR_FLAG;
    MEM_LIMIT = if IS_ENABLED(CONFIG_X86_32) { KERNEL_IMAGE_SIZE } else { MAXMEM };
    mem_avoid_init(input, input_size, *output);
    let mut min_addr = core::cmp::min(*output, 512UL << 20);
    min_addr = ALIGN(min_addr, CONFIG_PHYSICAL_ALIGN);
    let random_addr = find_random_phys_addr(min_addr, output_size);
    if random_addr == 0 { warn(c"Physical KASLR disabled: no suitable memory region!".as_ptr()); }
    else if *output != random_addr { *output = random_addr; }
    *virt_addr = if IS_ENABLED(CONFIG_X86_64) { find_random_virt_addr(LOAD_PHYSICAL_ADDR, output_size) } else { random_addr };
}

// External declarations/types referenced above are provided by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
