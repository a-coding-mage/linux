// SPDX-License-Identifier: GPL-2.0

// Types, constants, and symbols below are supplied by the corresponding
// s390 kernel dependencies.

macro_rules! CCW0 {
    ($cmd:expr, $addr:expr, $cnt:expr, $flg:expr) => {
        ccw0_t { cmd_code: $cmd, cda: $addr, count: $cnt, flags: $flg }
    };
}

const PSW_MASK_DISABLED: _ = PSW_MASK_WAIT | PSW_MASK_EA | PSW_MASK_BA;

#[repr(C)]
struct ipl_lowcore {
    ipl_psw: psw32_t,             // 0x0000
    ccwpgm: [ccw0_t; 2],          // 0x0008
    fill: [u8; 56],                // 0x0018
    ccwpgmcc: [ccw0_t; 20],       // 0x0050
    pad_0xf0: [u8; 0x0140 - 0x00f0], // 0x00f0
    svc_old_psw: psw_t,            // 0x0140
    pad_0x150: [u8; 0x01a0 - 0x0150], // 0x0150
    restart_psw: psw_t,            // 0x01a0
    external_new_psw: psw_t,       // 0x01b0
    svc_new_psw: psw_t,            // 0x01c0
    program_new_psw: psw_t,        // 0x01d0
    mcck_new_psw: psw_t,           // 0x01e0
    io_new_psw: psw_t,              // 0x01f0
}

/*
 * Initial lowcore for IPL: the first 24 bytes are loaded by IPL to
 * addresses 0-23 (a PSW and two CCWs). Bytes 24-79 are discarded.
 * The next 160 bytes are loaded to addresses 0x18-0xb7. They form
 * the continuation of the CCW program started by IPL and load the
 * range 0x0f0-0x730 from the image to the range 0x0f0-0x730 in
 * memory. At the end of the channel program the PSW at location 0 is
 * loaded.
 * Initial processing starts at 0x200 = iplstart.
 *
 * The restart psw points to iplstart which allows to load a kernel
 * image into memory and starting it by a psw restart on any cpu. All
 * other default psw new locations contain a disabled wait psw where
 * the address indicates which psw was loaded.
 *
 * Note that the 'file' utility can detect s390 kernel images. For
 * that to succeed the two initial CCWs, and the 0x40 fill bytes must
 * be present.
 */
#[used]
#[link_section = ".ipldata"]
static mut ipl_lowcore: ipl_lowcore = ipl_lowcore {
    ipl_psw: psw32_t { mask: PSW32_MASK_BASE, addr: PSW32_ADDR_AMODE | IPL_START },
    ccwpgm: [
        CCW0!(CCW_CMD_READ_IPL, 0x018, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x068, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
    ],
    fill: [0x40; 56],
    ccwpgmcc: [
        CCW0!(CCW_CMD_READ_IPL, 0x0f0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x140, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x190, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x1e0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x230, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x280, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x2d0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x320, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x370, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x3c0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x410, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x460, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x4b0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x500, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x550, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x5a0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x5f0, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x640, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x690, 0x50, CCW_FLAG_SLI | CCW_FLAG_CC),
        CCW0!(CCW_CMD_READ_IPL, 0x6e0, 0x50, CCW_FLAG_SLI),
    ],
    svc_old_psw: psw_t { mask: 0, addr: jump_to_kernel as usize },
    restart_psw: psw_t { mask: 0, addr: IPL_START },
    external_new_psw: psw_t { mask: PSW_MASK_DISABLED, addr: __LC_EXT_NEW_PSW },
    svc_new_psw: psw_t { mask: PSW_MASK_DISABLED, addr: __LC_SVC_NEW_PSW },
    program_new_psw: psw_t { mask: PSW_MASK_DISABLED, addr: __LC_PGM_NEW_PSW },
    mcck_new_psw: psw_t { mask: PSW_MASK_DISABLED, addr: __LC_MCK_NEW_PSW },
    io_new_psw: psw_t { mask: PSW_MASK_DISABLED, addr: __LC_IO_NEW_PSW },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
