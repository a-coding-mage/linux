/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ov772x Camera
 *
 * Copyright (C) 2008 Renesas Solutions Corp.
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 */

/* C header guard: __OV772X_H__ */

/* for flags */
pub const OV772X_FLAG_VFLIP: u32 = 1 << 0; /* Vertical flip image */
pub const OV772X_FLAG_HFLIP: u32 = 1 << 1; /* Horizontal flip image */

/*
 * for Edge ctrl
 *
 * strength also control Auto or Manual Edge Control Mode
 * see also OV772X_MANUAL_EDGE_CTRL
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_edge_ctrl {
    pub strength: u8,
    pub threshold: u8,
    pub upper: u8,
    pub lower: u8,
}

pub const OV772X_MANUAL_EDGE_CTRL: u8 = 0x80; /* un-used bit of strength */
pub const OV772X_EDGE_STRENGTH_MASK: u8 = 0x1F;
pub const OV772X_EDGE_THRESHOLD_MASK: u8 = 0x0F;
pub const OV772X_EDGE_UPPER_MASK: u8 = 0xFF;
pub const OV772X_EDGE_LOWER_MASK: u8 = 0xFF;

#[inline]
pub const fn OV772X_AUTO_EDGECTRL(u: u8, l: u8) -> ov772x_edge_ctrl {
    ov772x_edge_ctrl {
        strength: 0,
        threshold: 0,
        upper: u & OV772X_EDGE_UPPER_MASK,
        lower: l & OV772X_EDGE_LOWER_MASK,
    }
}

#[inline]
pub const fn OV772X_MANUAL_EDGECTRL(s: u8, t: u8) -> ov772x_edge_ctrl {
    ov772x_edge_ctrl {
        strength: (s & OV772X_EDGE_STRENGTH_MASK) | OV772X_MANUAL_EDGE_CTRL,
        threshold: t & OV772X_EDGE_THRESHOLD_MASK,
        upper: 0,
        lower: 0,
    }
}

/**
 * struct ov772x_camera_info -\tov772x driver interface structure
 * @flags:\t\tSensor configuration flags
 * @edgectrl:\tSensor edge control
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_camera_info {
    pub flags: ::core::ffi::c_ulong,
    pub edgectrl: ov772x_edge_ctrl,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
