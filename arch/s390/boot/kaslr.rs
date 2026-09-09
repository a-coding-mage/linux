// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2019
 */

const PRNG_MODE_TDES: i32 = 1;
const PRNG_MODE_SHA512: i32 = 2;
const PRNG_MODE_TRNG: i32 = 3;

#[repr(C)]
struct prno_parm {
    res: u32,
    reseed_counter: u32,
    stream_bytes: u64,
    V: [u8; 112],
    C: [u8; 112],
}

#[repr(C)]
struct prng_parm {
    parm_block: [u8; 32],
    reseed_counter: u32,
    byte_counter: u64,
}

unsafe fn check_prng() -> i32 {
    if cpacf_query_func(CPACF_KMC, CPACF_KMC_PRNG) == 0 {
        boot_warn("KASLR disabled: CPU has no PRNG\n");
        return 0;
    }
    if cpacf_query_func(CPACF_PRNO, CPACF_PRNO_TRNG) != 0 {
        return PRNG_MODE_TRNG;
    }
    if cpacf_query_func(CPACF_PRNO, CPACF_PRNO_SHA512_DRNG_GEN) != 0 {
        return PRNG_MODE_SHA512;
    } else {
        return PRNG_MODE_TDES;
    }
}

unsafe fn get_random(limit: c_ulong, value: *mut c_ulong) -> i32 {
    let mut prng = prng_parm {
        // Initial parameter block for tdes mode, copied from libica.
        parm_block: [
            0x0F, 0x2B, 0x8E, 0x63, 0x8C, 0x8E, 0xD2, 0x52,
            0x64, 0xB7, 0xA0, 0x7B, 0x75, 0x28, 0xB8, 0xF4,
            0x75, 0x5F, 0xD2, 0xA6, 0x8D, 0x97, 0x11, 0xFF,
            0x49, 0xD8, 0x23, 0xF3, 0x7E, 0x21, 0xEC, 0xA0,
        ],
        reseed_counter: 0,
        byte_counter: 0,
    };
    let mut seed: c_ulong;
    let mut random: c_ulong = 0;
    let mut prno: prno_parm = core::mem::zeroed();
    let mut entropy: [u64; 4] = [0; 4];
    let mode = check_prng();
    seed = get_tod_clock_fast();

    match mode {
        PRNG_MODE_TRNG => cpacf_trng(core::ptr::null_mut(), 0, &mut random as *mut _ as *mut u8, core::mem::size_of::<c_ulong>()),
        PRNG_MODE_SHA512 => {
            cpacf_prno(CPACF_PRNO_SHA512_DRNG_SEED, &mut prno, core::ptr::null_mut(), 0, &mut seed as *mut _ as *mut u8, core::mem::size_of::<c_ulong>());
            cpacf_prno(CPACF_PRNO_SHA512_DRNG_GEN, &mut prno, &mut random as *mut _ as *mut u8, core::mem::size_of::<c_ulong>(), core::ptr::null_mut(), 0);
        }
        PRNG_MODE_TDES => {
            *(prng.parm_block.as_mut_ptr() as *mut c_ulong) ^= seed;
            for _ in 0..16 {
                cpacf_kmc(CPACF_KMC_PRNG, prng.parm_block.as_mut_ptr(), entropy.as_mut_ptr() as *mut u8, entropy.as_mut_ptr() as *mut u8, core::mem::size_of_val(&entropy));
                core::ptr::copy_nonoverlapping(entropy.as_ptr() as *const u8, prng.parm_block.as_mut_ptr(), core::mem::size_of_val(&entropy));
            }
            random = seed;
            cpacf_kmc(CPACF_KMC_PRNG, prng.parm_block.as_mut_ptr(), &mut random as *mut _ as *mut u8, &mut random as *mut _ as *mut u8, core::mem::size_of::<c_ulong>());
        }
        _ => return -1,
    }
    *value = random % limit;
    0
}

unsafe fn sort_reserved_ranges(res: *mut reserved_range, size: c_ulong) {
    for i in 1..size {
        let tmp = *res.add(i as usize);
        let mut j = i as isize - 1;
        while j >= 0 && (*res.add(j as usize)).start > tmp.start {
            *res.add((j + 1) as usize) = *res.add(j as usize);
            j -= 1;
        }
        *res.add((j + 1) as usize) = tmp;
    }
}

// The following declarations are supplied by the surrounding decompressor environment.
unsafe fn iterate_valid_positions(size: c_ulong, align: c_ulong, _min: c_ulong, _max: c_ulong, res: *mut reserved_range, res_count: usize, pos_count: bool, find_pos: c_ulong) -> c_ulong {
    let mut start: c_ulong;
    let mut end: c_ulong;
    let mut pos = 0;
    let res_end = res.add(res_count);
    let mut res = res;
    let align = core::cmp::max(align, 8);
    let _min = round_up(_min, align);
    let mut i = 0;
    while for_each_physmem_usable_range(i, &mut start, &mut end) {
        i += 1;
        if _min >= end { continue; }
        start = round_up(start, align);
        if start >= _max { break; }
        start = core::cmp::max(_min, start);
        end = core::cmp::min(_max, end);
        while start + size <= end {
            while !res.is_null() && (*res).end <= start {
                res = res.add(1);
                if res >= res_end { res = core::ptr::null_mut(); }
            }
            let mut tmp_end = end;
            let mut skip_res = core::ptr::null_mut();
            if !res.is_null() && (*res).start < end { skip_res = res; tmp_end = (*res).start; }
            if start + size <= tmp_end {
                let range_pos = (tmp_end - start - size) / align + 1;
                if pos_count { pos += range_pos; } else if range_pos >= find_pos { return start + (find_pos - 1) * align; }
            }
            if skip_res.is_null() { break; }
            start = round_up((*skip_res).end, align);
        }
    }
    if pos_count { pos } else { 0 }
}

unsafe fn randomize_within_range(size: c_ulong, align: c_ulong, min: c_ulong, max: c_ulong) -> c_ulong {
    let mut res: [reserved_range; RR_MAX] = core::mem::zeroed();
    core::ptr::copy_nonoverlapping(physmem_info.reserved.as_ptr(), res.as_mut_ptr(), RR_MAX);
    sort_reserved_ranges(res.as_mut_ptr(), RR_MAX as c_ulong);
    let max = core::cmp::min(max, get_physmem_alloc_pos());
    let max_pos = iterate_valid_positions(size, align, min, max, res.as_mut_ptr(), RR_MAX, true, 0);
    if max_pos == 0 { return 0; }
    let mut pos = 0;
    if get_random(max_pos, &mut pos) != 0 { return 0; }
    iterate_valid_positions(size, align, min, max, res.as_mut_ptr(), RR_MAX, false, pos + 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
