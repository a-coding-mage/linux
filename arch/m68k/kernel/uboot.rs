/*
 * uboot.c -- uboot arguments support
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Linux and architecture header dependencies are supplied by other files. */

extern "C" {
    static mut _init_sp: ::core::ffi::c_ulong;
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    static mut initrd_start: ::core::ffi::c_ulong;
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    static mut initrd_end: ::core::ffi::c_ulong;
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    static mut ROOT_DEV: ::core::ffi::c_ulong;

    fn strscpy(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char, count: usize) -> isize;
    fn strnlen(s: *const ::core::ffi::c_char, maxlen: usize) -> usize;
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    fn pr_info(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

/*
 * parse_uboot_commandline
 *
 * Copies u-boot commandline arguments and store them in the proper linux
 * variables.
 *
 * Assumes:
 *     _init_sp global contains the address in the stack pointer when the
 *     kernel starts (see head.S::_start)
 *
 *     U-Boot calling convention:
 *     (*kernel) (kbd, initrd_start, initrd_end, cmd_start, cmd_end);
 *
 *     _init_sp can be parsed as such
 *
 *     _init_sp+00 = u-boot cmd after jsr into kernel (skip)
 *     _init_sp+04 = &kernel board_info (residual data)
 *     _init_sp+08 = &initrd_start
 *     _init_sp+12 = &initrd_end
 *     _init_sp+16 = &cmd_start
 *     _init_sp+20 = &cmd_end
 *
 * This also assumes that the memory locations pointed to are still
 * unmodified. U-boot places them near the end of external SDRAM.
 *
 * Argument(s):
 * commandp = the linux commandline arg container to fill.
 * size     = the sizeof commandp.
 *
 * Returns:
 */
unsafe fn parse_uboot_commandline(commandp: *mut ::core::ffi::c_char, size: ::core::ffi::c_int) {
    let sp = _init_sp as *mut ::core::ffi::c_ulong;
    let uboot_cmd_start = *sp.add(4);
    let uboot_cmd_end = *sp.add(5);

    if uboot_cmd_start != 0 && uboot_cmd_end != 0 {
        strscpy(commandp, uboot_cmd_start as *const ::core::ffi::c_char, size as usize);
    }

    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    {
        let uboot_initrd_start = *sp.add(2);
        let uboot_initrd_end = *sp.add(3);

        if uboot_initrd_start != 0
            && uboot_initrd_end != 0
            && uboot_initrd_end > uboot_initrd_start
        {
            initrd_start = uboot_initrd_start;
            initrd_end = uboot_initrd_end;
            ROOT_DEV = Root_RAM0;
            pr_info(
                b"initrd at 0x%lx:0x%lx\0".as_ptr() as *const ::core::ffi::c_char,
                initrd_start,
                initrd_end,
            );
        }
    }
}

/* __init */
pub unsafe extern "C" fn process_uboot_commandline(
    mut commandp: *mut ::core::ffi::c_char,
    size: ::core::ffi::c_int,
) {
    let n = strnlen(commandp, size as usize);
    commandp = commandp.add(n);
    let mut len = size - n as ::core::ffi::c_int;
    if len != 0 {
        /* Add the whitespace separator */
        *commandp = b' ' as ::core::ffi::c_char;
        commandp = commandp.add(1);
        len -= 1;
    }

    parse_uboot_commandline(commandp, len);
    *commandp.add((len - 1) as usize) = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
