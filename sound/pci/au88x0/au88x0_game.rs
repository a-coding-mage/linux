// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Manuel Jander.
 *
 *  Based on the work of:
 *  Vojtech Pavlik
 *  Raymond Ingles
 *
 * Should you need to contact me, the author, you can do so either by
 * e-mail - mail your message to <vojtech@suse.cz>, or by paper mail:
 * Vojtech Pavlik, Ucitelska 1576, Prague 8, 182 00 Czech Republic
 *
 * Based 90% on Vojtech Pavlik pcigame driver.
 * Merged and modified by Manuel Jander, for the OpenVortex
 * driver. (email: mjander@embedded.cl).
 */

/* C dependencies:
 * <linux/time.h>, <linux/delay.h>, <linux/init.h>, <sound/core.h>,
 * "au88x0.h", <linux/gameport.h>, <linux/export.h>
 */

/* Original condition: #if IS_REACHABLE(CONFIG_GAMEPORT) */

const VORTEX_GAME_DWAIT: u32 = 20; /* 20 ms */

unsafe extern "C" {
    fn gameport_get_port_data(gameport: *mut gameport) -> *mut core::ffi::c_void;
    fn hwread(mmio: *mut core::ffi::c_void, offset: u32) -> u32;
    fn hwwrite(mmio: *mut core::ffi::c_void, offset: u32, data: u32);
    fn msleep(msecs: u32);
    fn gameport_allocate_port() -> *mut gameport;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn gameport_set_name(gameport: *mut gameport, name: *const core::ffi::c_char);
    fn gameport_set_phys(gameport: *mut gameport, fmt: *const core::ffi::c_char, ...);
    fn pci_name(pci_dev: *mut pci_dev) -> *const core::ffi::c_char;
    fn gameport_set_dev_parent(gameport: *mut gameport, dev: *mut core::ffi::c_void);
    fn gameport_set_port_data(gameport: *mut gameport, data: *mut core::ffi::c_void);
    fn gameport_register_port(gameport: *mut gameport);
    fn gameport_unregister_port(gameport: *mut gameport);
}

unsafe extern "C" {
    static VORTEX_GAME_LEGACY: u32;
    static VORTEX_GAME_AXIS: u32;
    static AXIS_SIZE: u32;
    static AXIS_RANGE: i32;
    static VORTEX_CTRL2: u32;
    static CTRL2_GAME_ADCMODE: u32;
    static GAMEPORT_MODE_COOKED: i32;
    static GAMEPORT_MODE_RAW: i32;
    static ENOMEM: i32;
    static ENOSYS: i32;
}

#[repr(C)]
pub struct pci_dev {
    pub dev: core::ffi::c_void,
}

#[repr(C)]
pub struct gameport {
    pub read: Option<unsafe extern "C" fn(*mut gameport) -> u8>,
    pub trigger: Option<unsafe extern "C" fn(*mut gameport)>,
    pub cooked_read: Option<unsafe extern "C" fn(*mut gameport, *mut i32, *mut i32) -> i32>,
    pub open: Option<unsafe extern "C" fn(*mut gameport, i32) -> i32>,
    pub fuzz: i32,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct vortex_t {
    pub mmio: *mut core::ffi::c_void,
    pub gameport: *mut gameport,
    pub card: *mut snd_card,
    pub pci_dev: *mut pci_dev,
}

unsafe extern "C" fn vortex_game_read(gameport: *mut gameport) -> u8 {
    let vortex: *mut vortex_t = unsafe { gameport_get_port_data(gameport) as *mut vortex_t };
    unsafe { hwread((*vortex).mmio, VORTEX_GAME_LEGACY) as u8 }
}

unsafe extern "C" fn vortex_game_trigger(gameport: *mut gameport) {
    let vortex: *mut vortex_t = unsafe { gameport_get_port_data(gameport) as *mut vortex_t };
    unsafe {
        hwwrite((*vortex).mmio, VORTEX_GAME_LEGACY, 0xff);
    }
}

unsafe extern "C" fn vortex_game_cooked_read(
    gameport: *mut gameport,
    axes: *mut i32,
    buttons: *mut i32,
) -> i32 {
    let vortex: *mut vortex_t = unsafe { gameport_get_port_data(gameport) as *mut vortex_t };
    let mut i: i32;

    unsafe {
        *buttons = ((!hwread((*vortex).mmio, VORTEX_GAME_LEGACY)) >> 4 & 0xf) as i32;
    }

    i = 0;
    while i < 4 {
        unsafe {
            *axes.offset(i as isize) =
                hwread((*vortex).mmio, VORTEX_GAME_AXIS + (i as u32 * AXIS_SIZE)) as i32;
            if *axes.offset(i as isize) == AXIS_RANGE {
                *axes.offset(i as isize) = -1;
            }
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn vortex_game_open(gameport: *mut gameport, mode: i32) -> i32 {
    let vortex: *mut vortex_t = unsafe { gameport_get_port_data(gameport) as *mut vortex_t };

    unsafe {
        if mode == GAMEPORT_MODE_COOKED {
            hwwrite(
                (*vortex).mmio,
                VORTEX_CTRL2,
                hwread((*vortex).mmio, VORTEX_CTRL2) | CTRL2_GAME_ADCMODE,
            );
            msleep(VORTEX_GAME_DWAIT);
            return 0;
        } else if mode == GAMEPORT_MODE_RAW {
            hwwrite(
                (*vortex).mmio,
                VORTEX_CTRL2,
                hwread((*vortex).mmio, VORTEX_CTRL2) & !CTRL2_GAME_ADCMODE,
            );
            return 0;
        } else {
            return -1;
        }
    }

    #[allow(unreachable_code)]
    0
}

unsafe extern "C" fn vortex_gameport_register(vortex: *mut vortex_t) -> i32 {
    let gp: *mut gameport;

    unsafe {
        gp = gameport_allocate_port();
        (*vortex).gameport = gp;
        if gp.is_null() {
            dev_err(
                (*(*vortex).card).dev,
                c"cannot allocate memory for gameport\n".as_ptr(),
            );
            return -ENOMEM;
        }

        gameport_set_name(gp, c"AU88x0 Gameport".as_ptr());
        gameport_set_phys(gp, c"pci%s/gameport0".as_ptr(), pci_name((*vortex).pci_dev));
        gameport_set_dev_parent(gp, &mut (*(*vortex).pci_dev).dev as *mut core::ffi::c_void);

        (*gp).read = Some(vortex_game_read);
        (*gp).trigger = Some(vortex_game_trigger);
        (*gp).cooked_read = Some(vortex_game_cooked_read);
        (*gp).open = Some(vortex_game_open);

        gameport_set_port_data(gp, vortex as *mut core::ffi::c_void);
        (*gp).fuzz = 64;

        gameport_register_port(gp);
    }

    0
}

unsafe extern "C" fn vortex_gameport_unregister(vortex: *mut vortex_t) {
    unsafe {
        if !(*vortex).gameport.is_null() {
            gameport_unregister_port((*vortex).gameport);
            (*vortex).gameport = core::ptr::null_mut();
        }
    }
}

/* Original #else fallback when !IS_REACHABLE(CONFIG_GAMEPORT):
static inline int vortex_gameport_register(vortex_t * vortex) { return -ENOSYS; }
static inline void vortex_gameport_unregister(vortex_t * vortex) { }
*/

#[allow(dead_code)]
unsafe extern "C" fn vortex_gameport_register_no_gameport(_vortex: *mut vortex_t) -> i32 {
    unsafe { -ENOSYS }
}

#[allow(dead_code)]
unsafe extern "C" fn vortex_gameport_unregister_no_gameport(_vortex: *mut vortex_t) {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
