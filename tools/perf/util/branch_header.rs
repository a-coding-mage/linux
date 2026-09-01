/*
 * The linux/stddef.h isn't need here, but is needed for __always_inline used
 * in files included from uapi/linux/perf_event.h such as
 * /usr/include/linux/swab.h and /usr/include/linux/byteorder/little_endian.h,
 * detected in at least musl libc, used in Alpine Linux. -acme
 *
 * C includes translated as external dependencies:
 * <stdio.h>, <linux/perf_event.h>, <linux/types.h>, "util/map_symbol.h",
 * and "util/sample.h".
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub union branch_flags {
    pub value: u64,
    /*
     * Original C anonymous bitfield layout:
     * u64 mispred:1;
     * u64 predicted:1;
     * u64 in_tx:1;
     * u64 abort:1;
     * u64 cycles:16;
     * u64 type:4;
     * u64 spec:2;
     * u64 new_type:4;
     * u64 priv:3;
     * u64 not_taken:1;
     * u64 reserved:30;
     */
    pub bitfields: branch_flags_bitfields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_flags_bitfields {
    pub value: u64,
}

#[repr(C)]
pub struct branch_info {
    pub from: addr_map_symbol,
    pub to: addr_map_symbol,
    pub flags: branch_flags,
    pub branch_stack_cntr: u64,
    pub srcline_from: *mut ::std::os::raw::c_char,
    pub srcline_to: *mut ::std::os::raw::c_char,
}

#[repr(C)]
pub struct branch_entry {
    pub from: u64,
    pub to: u64,
    pub flags: branch_flags,
}

#[repr(C)]
pub struct branch_stack {
    pub nr: u64,
    pub hw_idx: u64,
    /* Original C flexible array member: struct branch_entry entries[]; */
    pub entries: [branch_entry; 0],
}

/*
 * The hw_idx is only available when PERF_SAMPLE_BRANCH_HW_INDEX is applied.
 * Otherwise, the output format of a sample with branch stack is
 * struct branch_stack {
 *	u64			nr;
 *	struct branch_entry	entries[0];
 * }
 * Check whether the hw_idx is available,
 * and return the corresponding pointer of entries[0].
 */
#[inline]
pub unsafe fn perf_sample__branch_entries(sample: *mut perf_sample) -> *mut branch_entry {
    let mut entry: *mut u64 = (*sample).branch_stack as *mut u64;

    if entry.is_null() {
        return ::std::ptr::null_mut();
    }

    entry = entry.add(1);
    if (*sample).no_hw_idx {
        return entry as *mut branch_entry;
    }
    entry = entry.add(1);
    entry as *mut branch_entry
}

#[repr(C)]
pub struct branch_type_stat {
    pub branch_to: bool,
    pub counts: [u64; PERF_BR_MAX],
    pub new_counts: [u64; PERF_BR_NEW_MAX],
    pub cond_fwd: u64,
    pub cond_bwd: u64,
    pub cross_4k: u64,
    pub cross_2m: u64,
}

extern "C" {
    pub fn branch_type_count(
        st: *mut branch_type_stat,
        flags: *mut branch_flags,
        from: u64,
        to: u64,
    );

    pub fn branch_type_name(type_: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn branch_new_type_name(
        new_type: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn get_branch_type(e: *mut branch_entry) -> *const ::std::os::raw::c_char;
    pub fn branch_type_stat_display(fp: *mut FILE, st: *const branch_type_stat);
    pub fn branch_type_str(
        st: *const branch_type_stat,
        bf: *mut ::std::os::raw::c_char,
        bfsize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn branch_spec_desc(spec: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
