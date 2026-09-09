/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: this header includes linux/ioctl.h and linux/types.h.

/*
 * These constants are also useful for user-level apps (e.g., VC
 * resizing).
 */
pub const MIN_NR_CONSOLES: u32 = 1; // must be at least 1
pub const MAX_NR_CONSOLES: u32 = 63; // serial lines start at 64
// Note: the ioctl VT_GETSTATE does not work for consoles 16 and higher
// (since it returns a short)

// 0x56 is 'V', to avoid collision with termios and kd

pub const VT_OPENQRY: u32 = 0x5600; // find available vt

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_mode {
    pub mode: u8, // vt mode
    pub waitv: u8, // if set, hang on writes if not active
    pub relsig: i16, // signal to raise on release req
    pub acqsig: i16, // signal to raise on acquisition
    pub frsig: i16, // unused (set to 0)
}
pub const VT_GETMODE: u32 = 0x5601; // get mode of active vt
pub const VT_SETMODE: u32 = 0x5602; // set mode of active vt
pub const VT_AUTO: u32 = 0x00; // auto vt switching
pub const VT_PROCESS: u32 = 0x01; // process controls switching
pub const VT_ACKACQ: u32 = 0x02; // acknowledge switch

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_stat {
    pub v_active: u16, // active vt
    pub v_signal: u16, // signal to send
    pub v_state: u16, // vt bitmask
}
pub const VT_GETSTATE: u32 = 0x5603; // get global vt state info
pub const VT_SENDSIG: u32 = 0x5604; // signal to send to bitmask of vts

pub const VT_RELDISP: u32 = 0x5605; // release display
pub const VT_ACTIVATE: u32 = 0x5606; // make vt active
pub const VT_WAITACTIVE: u32 = 0x5607; // wait for vt active
pub const VT_DISALLOCATE: u32 = 0x5608; // free memory associated to vt

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_sizes {
    pub v_rows: u16, // number of rows
    pub v_cols: u16, // number of columns
    pub v_scrollsize: u16, // number of lines of scrollback
}
pub const VT_RESIZE: u32 = 0x5609; // set kernel's idea of screensize

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_consize {
    pub v_rows: u16, // number of rows
    pub v_cols: u16, // number of columns
    pub v_vlin: u16, // number of pixel rows on screen
    pub v_clin: u16, // number of pixel rows per character
    pub v_vcol: u16, // number of pixel columns on screen
    pub v_ccol: u16, // number of pixel columns per character
}
pub const VT_RESIZEX: u32 = 0x560A; // set kernel's idea of screensize + more
pub const VT_LOCKSWITCH: u32 = 0x560B; // disallow vt switching
pub const VT_UNLOCKSWITCH: u32 = 0x560C; // allow vt switching
pub const VT_GETHIFONTMASK: u32 = 0x560D; // return hi font mask

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_event {
    pub event: u32,
    pub oldev: u32, // Old console
    pub newev: u32, // New console (if changing)
    pub pad: [u32; 4], // Padding for expansion
}
pub const VT_EVENT_SWITCH: u32 = 0x0001; // Console switch
pub const VT_EVENT_BLANK: u32 = 0x0002; // Screen blank
pub const VT_EVENT_UNBLANK: u32 = 0x0004; // Screen unblank
pub const VT_EVENT_RESIZE: u32 = 0x0008; // Resize display
pub const VT_MAX_EVENT: u32 = 0x000F;

pub const VT_WAITEVENT: u32 = 0x560E; // Wait for an event

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_setactivate {
    pub console: u32,
    pub mode: vt_mode,
}

pub const VT_SETACTIVATE: u32 = 0x560F; // Activate and set the mode of a console

/* get console size and cursor position */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vt_consizecsrpos {
    pub con_rows: u16, // number of console rows
    pub con_cols: u16, // number of console columns
    pub csr_row: u16, // current cursor's row
    pub csr_col: u16, // current cursor's column
}

// _IOR('V', 0x10, struct vt_consizecsrpos), supplied by linux/ioctl.h.
pub const VT_GETCONSIZECSRPOS: u32 = _IOR('V', 0x10, vt_consizecsrpos);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
