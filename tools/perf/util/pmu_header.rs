/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int, c_ulong, c_void};

/*
 * Translated from perf/util/pmu.h.
 *
 * C include dependencies intentionally remain external to this translation:
 * linux/bitmap.h, linux/compiler.h, linux/perf_event.h, linux/list.h,
 * parse-events.h, pmu-events/pmu-events.h, map_symbol.h, and mem-events.h.
 */

pub const PERF_PMU_FORMAT_VALUE_CONFIG: c_int = 0;
pub const PERF_PMU_FORMAT_VALUE_CONFIG1: c_int = 1;
pub const PERF_PMU_FORMAT_VALUE_CONFIG2: c_int = 2;
pub const PERF_PMU_FORMAT_VALUE_CONFIG3: c_int = 3;
pub const PERF_PMU_FORMAT_VALUE_CONFIG4: c_int = 4;
pub const PERF_PMU_FORMAT_VALUE_CONFIG_END: usize = 5;

pub const PERF_PMU_FORMAT_BITS: usize = 64;
pub const MAX_PMU_NAME_LEN: usize = 128;

const ULONG_BITS: usize = std::mem::size_of::<c_ulong>() * 8;
const PERF_PMU_FORMAT_LONGS: usize = (PERF_PMU_FORMAT_BITS + ULONG_BITS - 1) / ULONG_BITS;

pub type __u32 = u32;
pub type __u64 = u64;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_config_term {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct print_callbacks {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmu_events_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_mem_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parse_events_terms {
    _private: [u8; 0],
}

#[repr(C)]
pub struct parse_events_error {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_pmu_caps {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pmu_kind {
    /* A perf event syscall PMU. */
    PERF_PMU_KIND_PE = 0,
    /* A perf tool provided DRM PMU. */
    PERF_PMU_KIND_DRM = 1,
    /* A perf tool provided HWMON PMU. */
    PERF_PMU_KIND_HWMON = 2,
    /* Perf tool provided PMU for tool events like time. */
    PERF_PMU_KIND_TOOL = 3,
    /* A testing PMU kind. */
    PERF_PMU_KIND_FAKE = 4,
}

pub const PERF_PMU_TYPE_PE_START: __u32 = 0;
pub const PERF_PMU_TYPE_PE_END: __u32 = 0xFFFDFFFF;
pub const PERF_PMU_TYPE_DRM_START: __u32 = 0xFFFE0000;
pub const PERF_PMU_TYPE_DRM_END: __u32 = 0xFFFEFFFF;
pub const PERF_PMU_TYPE_HWMON_START: __u32 = 0xFFFF0000;
pub const PERF_PMU_TYPE_HWMON_END: __u32 = 0xFFFFFFFD;
pub const PERF_PMU_TYPE_TOOL: __u32 = 0xFFFFFFFE;
pub const PERF_PMU_TYPE_FAKE: __u32 = 0xFFFFFFFF;

/**
 * struct perf_pmu
 */
#[repr(C)]
pub struct perf_pmu {
    /** @name: The name of the PMU such as "cpu". */
    pub name: *const c_char,
    /**
     * @alias_name: Optional alternate name for the PMU determined in
     * architecture specific code.
     */
    pub alias_name: *mut c_char,
    /**
     * @id: Optional PMU identifier read from
     * <sysfs>/bus/event_source/devices/<name>/identifier.
     */
    pub id: *const c_char,
    /**
     * @type: Perf event attributed type value, read from
     * <sysfs>/bus/event_source/devices/<name>/type.
     */
    pub type_: __u32,
    /**
     * @selectable: Can the PMU name be selected as if it were an event?
     */
    pub selectable: bool,
    /**
     * @is_core: Is the PMU the core CPU PMU? Determined by the name being
     * "cpu" or by the presence of
     * <sysfs>/bus/event_source/devices/<name>/cpus. There may be >1 core
     * PMU on systems like Intel hybrid.
     */
    pub is_core: bool,
    /**
     * @is_uncore: Is the PMU not within the CPU core? Determined by the
     * presence of <sysfs>/bus/event_source/devices/<name>/cpumask.
     */
    pub is_uncore: bool,
    /**
     * @auxtrace: Are events auxiliary events? Determined in architecture
     * specific code.
     */
    pub auxtrace: bool,
    /**
     * @formats_checked: Only check PMU's formats are valid for
     * perf_event_attr once.
     */
    pub formats_checked: bool,
    /** @config_masks_present: Are there config format values? */
    pub config_masks_present: bool,
    /** @config_masks_computed: Set when masks are lazily computed. */
    pub config_masks_computed: bool,
    /**
     * @max_precise: Number of levels of :ppp precision supported by the
     * PMU, read from
     * <sysfs>/bus/event_source/devices/<name>/caps/max_precise.
     */
    pub max_precise: c_int,
    /**
     * @perf_event_attr_init_default: Optional function to default
     * initialize PMU specific parts of the perf_event_attr.
     */
    pub perf_event_attr_init_default:
        Option<unsafe extern "C" fn(pmu: *const perf_pmu, attr: *mut perf_event_attr)>,
    /**
     * @cpus: Empty or the contents of either of:
     * <sysfs>/bus/event_source/devices/<name>/cpumask.
     * <sysfs>/bus/event_source/devices/<cpu>/cpus.
     */
    pub cpus: *mut perf_cpu_map,
    /**
     * @format: Holds the contents of files read from
     * <sysfs>/bus/event_source/devices/<name>/format/. The contents specify
     * which event parameter changes what config, config1 or config2 bits.
     */
    pub format: list_head,
    /**
     * @aliases: List of struct perf_pmu_alias. Each alias corresponds to an
     * event read from <sysfs>/bus/event_source/devices/<name>/events/ or
     * from json events in pmu-events.c.
     */
    pub aliases: *mut hashmap,
    /**
     * @events_table: The events table for json events in pmu-events.c.
     */
    pub events_table: *const pmu_events_table,
    /** @sysfs_aliases: Number of sysfs aliases loaded. */
    pub sysfs_aliases: u32,
    /** @cpu_json_aliases: Number of json event aliases loaded specific to the CPUID. */
    pub cpu_json_aliases: u32,
    /** @sys_json_aliases: Number of json event aliases loaded matching the PMU's identifier. */
    pub sys_json_aliases: u32,
    /**
     * @cpu_common_json_aliases: Number of json events that overlapped with sysfs when
     * loading all sysfs events.
     */
    pub cpu_common_json_aliases: u32,
    /** @sysfs_aliases_loaded: Are sysfs aliases loaded from disk? */
    pub sysfs_aliases_loaded: bool,
    /**
     * @cpu_aliases_added: Have all json events table entries for the PMU
     * been added?
     */
    pub cpu_aliases_added: bool,
    /** @caps_initialized: Has the list caps been initialized? */
    pub caps_initialized: bool,
    /** @nr_caps: The length of the list caps. */
    pub nr_caps: u32,
    /**
     * @caps: Holds the contents of files read from
     * <sysfs>/bus/event_source/devices/<name>/caps/.
     *
     * The contents are pairs of the filename with the value of its
     * contents, for example, max_precise (see above) may have a value of 3.
     */
    pub caps: list_head,
    /** @list: Element on pmus list in pmu.c. */
    pub list: list_head,

    /**
     * @config_masks: Derived from the PMU's format data, bits that are
     * valid within the config value.
     */
    pub config_masks: [__u64; PERF_PMU_FORMAT_VALUE_CONFIG_END],

    /**
     * @missing_features: Features to inhibit when events on this PMU are
     * opened.
     */
    pub missing_features: perf_pmu_missing_features,

    /**
     * @mem_events: List of the supported mem events
     */
    pub mem_events: *mut perf_mem_event,
}

#[repr(C)]
pub struct perf_pmu_missing_features {
    /**
     * @exclude_guest: Disables perf_event_attr exclude_guest and
     * exclude_host.
     */
    pub exclude_guest: bool,
    /**
     * @checked: Are the missing features checked?
     */
    pub checked: bool,
}

#[repr(C)]
pub struct perf_pmu_info {
    pub unit: *const c_char,
    pub scale: f64,
    pub retirement_latency_mean: f64,
    pub retirement_latency_min: f64,
    pub retirement_latency_max: f64,
    pub per_pkg: bool,
    pub snapshot: bool,
}

#[repr(C)]
pub struct pmu_event_info {
    pub pmu: *const perf_pmu,
    pub name: *const c_char,
    pub alias: *const c_char,
    pub scale_unit: *const c_char,
    pub desc: *const c_char,
    pub long_desc: *const c_char,
    pub encoding_desc: *const c_char,
    pub topic: *const c_char,
    pub pmu_name: *const c_char,
    pub event_type_desc: *const c_char,
    pub str_: *const c_char,
    pub deprecated: bool,
}

/**
 * struct perf_pmu_format - Values from a format file read from
 * <sysfs>/devices/cpu/format/ held in struct perf_pmu.
 *
 * For example, the contents of <sysfs>/devices/cpu/format/event may be
 * "config:0-7" and will be represented here as name="event",
 * value=PERF_PMU_FORMAT_VALUE_CONFIG and bits 0 to 7 will be set.
 */
#[repr(C)]
pub struct perf_pmu_format {
    /** @list: Element on list within struct perf_pmu. */
    pub list: list_head,
    /** @bits: Which config bits are set by this format value. */
    pub bits: [c_ulong; PERF_PMU_FORMAT_LONGS],
    /** @name: The modifier/file name. */
    pub name: *mut c_char,
    /**
     * @value : Which config value the format relates to. Supported values
     * are from PERF_PMU_FORMAT_VALUE_CONFIG to
     * PERF_PMU_FORMAT_VALUE_CONFIG_END.
     */
    pub value: u16,
    /** @loaded: Has the contents been loaded/parsed. */
    pub loaded: bool,
}

pub type pmu_event_callback =
    Option<unsafe extern "C" fn(state: *mut c_void, info: *mut pmu_event_info) -> c_int>;
pub type pmu_format_callback = Option<
    unsafe extern "C" fn(
        state: *mut c_void,
        name: *const c_char,
        config: c_int,
        bits: *const c_ulong,
    ) -> c_int,
>;

unsafe extern "C" {
    pub fn pmu_add_sys_aliases(pmu: *mut perf_pmu);
    pub fn perf_pmu__config(
        pmu: *mut perf_pmu,
        attr: *mut perf_event_attr,
        head_terms: *mut parse_events_terms,
        apply_hardcoded: bool,
        error: *mut parse_events_error,
    ) -> c_int;
    pub fn perf_pmu__config_terms(
        pmu: *const perf_pmu,
        attr: *mut perf_event_attr,
        terms: *mut parse_events_terms,
        zero: bool,
        apply_hardcoded: bool,
        error: *mut parse_events_error,
    ) -> c_int;
    pub fn perf_pmu__format_bits(pmu: *const perf_pmu, name: *const c_char) -> __u64;
    pub fn perf_pmu__format_type(pmu: *const perf_pmu, name: *const c_char) -> c_int;
    pub fn perf_pmu__check_alias(
        pmu: *mut perf_pmu,
        head_terms: *mut parse_events_terms,
        info: *mut perf_pmu_info,
        rewrote_terms: *mut bool,
        alternate_hw_config: *mut u64,
        err: *mut parse_events_error,
    ) -> c_int;
    pub fn perf_pmu__find_event(
        pmu: *mut perf_pmu,
        event: *const c_char,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;

    pub fn perf_pmu__format_pack(
        format: *mut c_ulong,
        value: __u64,
        v: *mut __u64,
        zero: bool,
    );
    pub fn pmu_find_format(
        formats: *const list_head,
        name: *const c_char,
    ) -> *mut perf_pmu_format;
    pub fn perf_pmu_format__set_value(format: *mut c_void, config: c_int, bits: *mut c_ulong);
    pub fn perf_pmu__has_format(pmu: *const perf_pmu, name: *const c_char) -> bool;
    pub fn perf_pmu__for_each_format(
        pmu: *mut perf_pmu,
        state: *mut c_void,
        cb: pmu_format_callback,
    ) -> c_int;
    pub fn perf_pmu__format_unpack(format: *mut c_ulong, config_val: u64) -> u64;

    pub fn is_pmu_core(name: *const c_char) -> bool;
    pub fn perf_pmu__supports_legacy_cache(pmu: *const perf_pmu) -> bool;
    pub fn perf_pmu__auto_merge_stats(pmu: *const perf_pmu) -> bool;
    pub fn perf_pmu__have_event(pmu: *mut perf_pmu, name: *const c_char) -> bool;
    pub fn perf_pmu__num_events(pmu: *mut perf_pmu) -> size_t;
    pub fn perf_pmu__for_each_event(
        pmu: *mut perf_pmu,
        skip_duplicate_pmus: bool,
        state: *mut c_void,
        cb: pmu_event_callback,
    ) -> c_int;
    pub fn perf_pmu__name_wildcard_match(
        pmu: *const perf_pmu,
        to_match: *const c_char,
    ) -> bool;
    pub fn perf_pmu__name_no_suffix_match(
        pmu: *const perf_pmu,
        to_match: *const c_char,
    ) -> bool;

    /**
     * perf_pmu_is_software - is the PMU a software PMU as in it uses the
     *                        perf_sw_context in the kernel?
     */
    pub fn perf_pmu__is_software(pmu: *const perf_pmu) -> bool;
    pub fn perf_pmu__benefits_from_affinity(pmu: *mut perf_pmu) -> bool;

    pub fn perf_pmu__open_file(pmu: *const perf_pmu, name: *const c_char) -> *mut FILE;
    pub fn perf_pmu__open_file_at(
        pmu: *const perf_pmu,
        dirfd: c_int,
        name: *const c_char,
    ) -> *mut FILE;

    /* C __scanf format checking attribute preserved as comment. */
    pub fn perf_pmu__scan_file(
        pmu: *const perf_pmu,
        name: *const c_char,
        fmt: *const c_char,
        ...
    ) -> c_int;
    /* C __scanf format checking attribute preserved as comment. */
    pub fn perf_pmu__scan_file_at(
        pmu: *const perf_pmu,
        dirfd: c_int,
        name: *const c_char,
        fmt: *const c_char,
        ...
    ) -> c_int;

    pub fn perf_pmu__file_exists(pmu: *const perf_pmu, name: *const c_char) -> bool;

    pub fn perf_pmu__test() -> c_int;

    pub fn perf_pmu__arch_init(pmu: *mut perf_pmu);
    pub fn pmu_add_cpu_aliases_table(pmu: *mut perf_pmu, table: *const pmu_events_table);

    pub fn pmu_uncore_identifier_match(compat: *const c_char, id: *const c_char) -> bool;

    pub fn perf_pmu__convert_scale(
        scale: *const c_char,
        end: *mut *mut c_char,
        sval: *mut f64,
    ) -> c_int;

    pub fn perf_pmu__get_cap(pmu: *mut perf_pmu, name: *const c_char) -> *mut perf_pmu_caps;

    pub fn perf_pmu__caps_parse(pmu: *mut perf_pmu) -> c_int;

    pub fn perf_pmu__warn_invalid_config(
        pmu: *mut perf_pmu,
        config: __u64,
        name: *const c_char,
        config_num: c_int,
        config_name: *const c_char,
    );
    pub fn perf_pmu__warn_invalid_formats(pmu: *mut perf_pmu);

    pub fn perf_pmu__wildcard_match(
        pmu: *const perf_pmu,
        wildcard_to_match: *const c_char,
    ) -> bool;

    pub fn perf_pmu__event_source_devices_scnprintf(pathname: *mut c_char, size: size_t)
        -> c_int;
    pub fn perf_pmu__pathname_scnprintf(
        buf: *mut c_char,
        size: size_t,
        pmu_name: *const c_char,
        filename: *const c_char,
    ) -> c_int;
    pub fn perf_pmu__event_source_devices_fd() -> c_int;
    pub fn perf_pmu__pathname_fd(
        dirfd: c_int,
        pmu_name: *const c_char,
        filename: *const c_char,
        flags: c_int,
    ) -> c_int;

    pub fn perf_pmu__init(pmu: *mut perf_pmu, type_: __u32, name: *const c_char) -> c_int;
    pub fn perf_pmu__lookup(
        pmus: *mut list_head,
        dirfd: c_int,
        lookup_name: *const c_char,
        eager_load: bool,
    ) -> *mut perf_pmu;
    pub fn perf_pmu__create_placeholder_core_pmu(core_pmus: *mut list_head) -> *mut perf_pmu;
    pub fn perf_pmu__delete(pmu: *mut perf_pmu);

    pub fn perf_pmu__name_from_config(pmu: *mut perf_pmu, config: u64) -> *const c_char;
    pub fn perf_pmu__is_fake(pmu: *const perf_pmu) -> bool;

    pub fn perf_pmu__reads_only_on_cpu_idx0(attr: *const perf_event_attr) -> bool;
}

#[inline]
pub unsafe fn perf_pmu__kind(pmu: *const perf_pmu) -> pmu_kind {
    let type_: __u32;

    if pmu.is_null() {
        return pmu_kind::PERF_PMU_KIND_PE;
    }

    type_ = unsafe { (*pmu).type_ };
    if type_ <= PERF_PMU_TYPE_PE_END {
        return pmu_kind::PERF_PMU_KIND_PE;
    }
    if type_ <= PERF_PMU_TYPE_DRM_END {
        return pmu_kind::PERF_PMU_KIND_DRM;
    }
    if type_ <= PERF_PMU_TYPE_HWMON_END {
        return pmu_kind::PERF_PMU_KIND_HWMON;
    }
    if type_ == PERF_PMU_TYPE_TOOL {
        return pmu_kind::PERF_PMU_KIND_TOOL;
    }
    pmu_kind::PERF_PMU_KIND_FAKE
}
