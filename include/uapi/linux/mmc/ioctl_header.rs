/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent from the C header: linux/types.h and linux/major.h.

#[repr(C)]
pub struct mmc_ioc_cmd {
    /*
     * Direction of data: nonzero = write, zero = read.
     * Bit 31 selects 'Reliable Write' for RPMB.
     */
    pub write_flag: ::core::ffi::c_int,

    /* Application-specific command.  true = precede with CMD55 */
    pub is_acmd: ::core::ffi::c_int,

    pub opcode: __u32,
    pub arg: __u32,
    pub response: [__u32; 4], /* CMD response */
    pub flags: ::core::ffi::c_uint,
    pub blksz: ::core::ffi::c_uint,
    pub blocks: ::core::ffi::c_uint,

    /*
     * Sleep at least postsleep_min_us useconds, and at most
     * postsleep_max_us useconds *after* issuing command.  Needed for
     * some read commands for which cards have no other way of indicating
     * they're ready for the next command (i.e. there is no equivalent of a
     * "busy" indicator for read operations).
     */
    pub postsleep_min_us: ::core::ffi::c_uint,
    pub postsleep_max_us: ::core::ffi::c_uint,

    /* Override driver-computed timeouts.  Note the difference in units! */
    pub data_timeout_ns: ::core::ffi::c_uint,
    pub cmd_timeout_ms: ::core::ffi::c_uint,

    /*
     * For 64-bit machines, the next member, ``__u64 data_ptr``, wants to
     * be 8-byte aligned.  Make sure this struct is the same size when
     * built for 32-bit.
     */
    pub __pad: __u32,

    /* DAT buffer */
    pub data_ptr: __u64,
}

#[macro_export]
macro_rules! mmc_ioc_cmd_set_data {
    ($ic:expr, $ptr:expr) => {
        $ic.data_ptr = (__u64)($ptr as ::core::ffi::c_ulong);
    };
}

/**
 * struct mmc_ioc_multi_cmd - multi command information
 * @num_of_cmds: Number of commands to send. Must be equal to or less than
 *\tMMC_IOC_MAX_CMDS.
 * @cmds: Array of commands with length equal to 'num_of_cmds'
 */
#[repr(C)]
pub struct mmc_ioc_multi_cmd {
    pub num_of_cmds: __u64,
    pub cmds: [mmc_ioc_cmd; 0],
}

#[macro_export]
macro_rules! MMC_IOC_CMD {
    () => {
        _IOWR!(MMC_BLOCK_MAJOR, 0, mmc_ioc_cmd)
    };
}

/*
 * MMC_IOC_MULTI_CMD: Used to send an array of MMC commands described by
 *\tthe structure mmc_ioc_multi_cmd. The MMC driver will issue all
 *\tcommands in array in sequence to card.
 */
#[macro_export]
macro_rules! MMC_IOC_MULTI_CMD {
    () => {
        _IOWR!(MMC_BLOCK_MAJOR, 1, mmc_ioc_multi_cmd)
    };
}

/*
 * Since this ioctl is only meant to enhance (and not replace) normal access
 * to the mmc bus device, an upper data transfer limit of MMC_IOC_MAX_BYTES
 * is enforced per ioctl call.  For larger data transfers, use the normal
 * block device operations.
 */
pub const MMC_IOC_MAX_BYTES: ::core::ffi::c_long = 512 * 1024;
pub const MMC_IOC_MAX_CMDS: ::core::ffi::c_int = 255;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
