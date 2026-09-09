/*
 * MTD partitioning layer definitions
 *
 * (C) 2000 Nicolas Pitre <nico@fluxnic.net>
 *
 * This code is GPL
 */

// Dependency intent: names supplied by linux/types.h and other kernel headers.

/*
 * Partition definition structure:
 *
 * An array of struct partition is passed along with a MTD object to
 * mtd_device_register() to create them.
 *
 * For each partition, these fields are available:
 * name: string that will be used to label the partition's MTD device.
 * types: some partitions can be containers using specific format to describe
 * embedded subpartitions / volumes. E.g. many home routers use "firmware"
 * partition that contains at least kernel and rootfs. In such case an
 * extra parser is needed that will detect these dynamic partitions and
 * report them to the MTD subsystem. If set this property stores an array
 * of parser names to use when looking for subpartitions.
 * size: the partition size; if defined as MTDPART_SIZ_FULL, the partition
 * will extend to the end of the master MTD device.
 * offset: absolute starting position within the master MTD device; if
 * defined as MTDPART_OFS_APPEND, the partition will start where the
 * previous one ended; if MTDPART_OFS_NXTBLK, at the next erase block;
 * if MTDPART_OFS_RETAIN, consume as much as possible, leaving size
 * after the end of partition.
 * mask_flags: contains flags that have to be masked (removed) from the
 * master MTD flag set for the corresponding MTD partition.
 * For example, to force a read-only partition, simply adding
 * MTD_WRITEABLE to the mask_flags will do the trick.
 * add_flags: contains flags to add to the parent flags
 *
 * Note: writeable partitions require their size and offset be
 * erasesize aligned (e.g. use MTDPART_OFS_NEXTBLK).
 */
#[repr(C)]
pub struct mtd_partition {
    pub name: *const core::ffi::c_char,
    pub types: *const *const core::ffi::c_char,
    pub size: u64,
    pub offset: u64,
    pub mask_flags: u32,
    pub add_flags: u32,
    pub of_node: *mut device_node,
}

pub const MTDPART_OFS_RETAIN: i32 = -3;
pub const MTDPART_OFS_NXTBLK: i32 = -2;
pub const MTDPART_OFS_APPEND: i32 = -1;
pub const MTDPART_SIZ_FULL: i32 = 0;

pub struct mtd_info;
pub struct device_node;
pub struct list_head;
pub struct module;
pub struct of_device_id;

/**
 * struct mtd_part_parser_data - used to pass data to MTD partition parsers.
 * @origin: for RedBoot, start address of MTD device
 */
#[repr(C)]
pub struct mtd_part_parser_data {
    pub origin: core::ffi::c_ulong,
}

/* Functions dealing with the various ways of partitioning the space */
#[repr(C)]
pub struct mtd_part_parser {
    pub list: list_head,
    pub owner: *mut module,
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
    pub parse_fn: Option<unsafe extern "C" fn(
        *mut mtd_info,
        *mut *const mtd_partition,
        *mut mtd_part_parser_data,
    ) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*const mtd_partition, i32)>,
}

/* Container for passing around a set of parsed partitions */
#[repr(C)]
pub struct mtd_partitions {
    pub parts: *const mtd_partition,
    pub nr_parts: i32,
    pub parser: *const mtd_part_parser,
}

unsafe extern "C" {
    pub fn __register_mtd_parser(parser: *mut mtd_part_parser, owner: *mut module) -> i32;
    pub fn deregister_mtd_parser(parser: *mut mtd_part_parser);
    pub fn mtd_add_partition(
        master: *mut mtd_info,
        name: *const core::ffi::c_char,
        offset: i64,
        length: i64,
    ) -> i32;
    pub fn mtd_del_partition(master: *mut mtd_info, partno: i32) -> i32;
    pub fn mtd_get_device_size(mtd: *const mtd_info) -> u64;
}

// C macro: register_mtd_parser(parser) expands to __register_mtd_parser(parser, THIS_MODULE).
// The THIS_MODULE dependency is supplied by the surrounding kernel build.

// C macro module_mtd_part_parser(__mtd_part_parser) invokes module_driver with
// register_mtd_parser and deregister_mtd_parser; its module-build expansion is
// preserved here as conditional build intent rather than invented Rust code.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
