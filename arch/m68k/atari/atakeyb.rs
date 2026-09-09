/* Atari Keyboard driver for 680x0 Linux, translated from atakeyb.c. */

use core::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum KB_STATE_T { KEYBOARD, AMOUSE, RMOUSE, JOYSTICK, CLOCK, RESYNC }

#[repr(C)]
pub struct KEYBOARD_STATE { pub buf: [u8; 6], pub len: i32, pub state: KB_STATE_T }

extern "C" {
    static mut acia: ACIA;
    static mut st_mfp: MFP;
    static mut atari_switches: u32;
    static mut jiffies: usize;
    static mut atari_MIDI_interrupt_hook: Option<unsafe extern "C" fn()>;
    static mut atari_input_keyboard_interrupt_hook: Option<unsafe extern "C" fn(u8, i8)>;
    static mut atari_input_mouse_interrupt_hook: Option<unsafe extern "C" fn(*mut i8)>;
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32, flags: u32, name: *const i8, dev: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32) -> i32;
    fn atari_turnoff_irq(irq: i32);
    fn atari_turnon_irq(irq: i32);
    fn atari_joystick_interrupt(buf: *mut u8);
    fn atari_joystick_init();
    fn panic(msg: *const i8) -> !;
}

#[repr(C)] pub struct ACIA { pub key_ctrl: u8, pub key_data: u8, pub mid_ctrl: u8, pub mid_data: u8 }
#[repr(C)] pub struct MFP { pub par_dt_reg: u8, pub active_edge: u8 }

pub const ACIA_IRQ: u8 = 0x80; pub const ACIA_OVRN: u8 = 0x20; pub const ACIA_RDRF: u8 = 0x01;
pub const ACIA_TDRE: u8 = 0x02; pub const ACIA_FE: u8 = 0x10; pub const ACIA_PE: u8 = 0x08;
pub const ACIA_RESET: u8 = 3; pub const ACIA_RHTID: u8 = 0x04; pub const ACIA_RLTID: u8 = 0;
pub const ACIA_DIV64: u8 = 0x00; pub const ACIA_DIV16: u8 = 0x40; pub const ACIA_D8N1S: u8 = 0x10; pub const ACIA_RIE: u8 = 0x80;
pub const ATARI_SWITCH_IKBD: u32 = 1; pub const ATARI_SWITCH_MIDI: u32 = 2; pub const IRQ_MFP_ACIA: i32 = 0;
pub const HZ: usize = 100;

static mut ikbd_self_test: i32 = 0;
static mut self_test_last_rcv: usize = 0;
static mut broken_keys: [usize; 4] = [0; 4];
pub static mut kb_state: KEYBOARD_STATE = KEYBOARD_STATE { buf: [0; 6], len: 0, state: KB_STATE_T::KEYBOARD };
static mut atari_keyb_done: i32 = 0;

#[inline] unsafe fn is_sync_code(sc: i32) -> bool { sc >= 0x04 && sc <= 0xfb }
unsafe fn set_bit(bit: i32) { broken_keys[(bit as usize) / (core::mem::size_of::<usize>() * 8)] |= 1usize << ((bit as usize) % (core::mem::size_of::<usize>() * 8)); }
unsafe fn test_bit(bit: i32) -> bool { broken_keys[(bit as usize) / (core::mem::size_of::<usize>() * 8)] & (1usize << ((bit as usize) % (core::mem::size_of::<usize>() * 8))) != 0 }

pub unsafe extern "C" fn atari_keyboard_interrupt(_irq: i32, _dummy: *mut core::ffi::c_void) -> i32 {
    let mut acia_stat: u8; let mut scancode: i32; let mut break_flag: i32;
    loop {
        if acia.mid_ctrl & ACIA_IRQ != 0 { if let Some(h) = atari_MIDI_interrupt_hook { h(); } }
        acia_stat = acia.key_ctrl;
        if ((acia_stat | acia.mid_ctrl) & ACIA_IRQ) == 0 { return 1; }
        if acia_stat & ACIA_OVRN != 0 {
            scancode = acia.key_data as i32;
            if ikbd_self_test != 0 || is_sync_code(scancode) { kb_state.state = KB_STATE_T::KEYBOARD; }
            else { kb_state.state = KB_STATE_T::RESYNC; kb_state.len = 1; continue; }
        } else if acia_stat & ACIA_RDRF != 0 { scancode = acia.key_data as i32; } else { scancode = -1; }
        if scancode >= 0 { 'interpret: { match kb_state.state {
            KB_STATE_T::KEYBOARD => match scancode {
                0xf7 => { kb_state.state = KB_STATE_T::AMOUSE; kb_state.len = 0; },
                0xf8..=0xfb => { kb_state.state = KB_STATE_T::RMOUSE; kb_state.len = 1; kb_state.buf[0] = scancode as u8; },
                0xfc => { kb_state.state = KB_STATE_T::CLOCK; kb_state.len = 0; },
                0xfe..=0xff => { kb_state.state = KB_STATE_T::JOYSTICK; kb_state.len = 1; kb_state.buf[0] = scancode as u8; },
                0xf1 if ikbd_self_test != 0 => { ikbd_self_test += 1; self_test_last_rcv = jiffies; },
                _ => { break_flag = scancode & 0x80; scancode &= !0x80; if ikbd_self_test != 0 { set_bit(scancode); self_test_last_rcv = jiffies; } else if !test_bit(scancode) { if let Some(h) = atari_input_keyboard_interrupt_hook { h(scancode as u8, (break_flag == 0) as i8); } } }
            },
            KB_STATE_T::AMOUSE => { kb_state.buf[kb_state.len as usize] = scancode as u8; kb_state.len += 1; if kb_state.len == 5 { kb_state.state = KB_STATE_T::KEYBOARD; } },
            KB_STATE_T::RMOUSE => { kb_state.buf[kb_state.len as usize] = scancode as u8; kb_state.len += 1; if kb_state.len == 3 { kb_state.state = KB_STATE_T::KEYBOARD; if let Some(h) = atari_input_mouse_interrupt_hook { h(kb_state.buf.as_mut_ptr() as *mut i8); } } },
            KB_STATE_T::JOYSTICK => { kb_state.buf[1] = scancode as u8; kb_state.state = KB_STATE_T::KEYBOARD; },
            KB_STATE_T::CLOCK => { kb_state.buf[kb_state.len as usize] = scancode as u8; kb_state.len += 1; if kb_state.len == 6 { kb_state.state = KB_STATE_T::KEYBOARD; } },
            KB_STATE_T::RESYNC => { if kb_state.len <= 0 || is_sync_code(scancode) { kb_state.state = KB_STATE_T::KEYBOARD; continue; } kb_state.len -= 1; }
        } } }
        if acia_stat & (ACIA_FE | ACIA_PE) != 0 { }
    }
}

pub unsafe extern "C" fn ikbd_write(str_: *const i8, mut len: i32) { if len < 1 || len > 7 { panic(b"ikbd: maximum string length exceeded\0".as_ptr() as *const i8); } while len != 0 { if acia.key_ctrl & ACIA_TDRE != 0 { acia.key_data = *str_ as u8; str_ = str_.add(1); len -= 1; } } }
unsafe fn command(bytes: &[i8]) { ikbd_write(bytes.as_ptr(), bytes.len() as i32); }
pub unsafe extern "C" fn ikbd_reset() { command(&[-128, 1]); }
pub unsafe extern "C" fn ikbd_mouse_button_action(mode: i32) { command(&[7, mode as i8]); }
pub unsafe extern "C" fn ikbd_mouse_rel_pos() { command(&[8]); }
pub unsafe extern "C" fn ikbd_mouse_abs_pos(xmax: i32, ymax: i32) { command(&[9,(xmax>>8) as i8,xmax as i8,(ymax>>8) as i8,ymax as i8]); }
pub unsafe extern "C" fn ikbd_mouse_kbd_mode(dx:i32,dy:i32){command(&[10,dx as i8,dy as i8]);}
pub unsafe extern "C" fn ikbd_mouse_thresh(x:i32,y:i32){command(&[11,x as i8,y as i8]);}
pub unsafe extern "C" fn ikbd_mouse_scale(x:i32,y:i32){command(&[12,x as i8,y as i8]);}
pub unsafe extern "C" fn ikbd_mouse_pos_get(_x:*mut i32,_y:*mut i32){command(&[13]);}
pub unsafe extern "C" fn ikbd_mouse_pos_set(x:i32,y:i32){command(&[14,0,(x>>8) as i8,x as i8,(y>>8) as i8,y as i8]);}
pub unsafe extern "C" fn ikbd_mouse_y0_bot(){command(&[15]);} pub unsafe extern "C" fn ikbd_mouse_y0_top(){command(&[16]);}
pub unsafe extern "C" fn ikbd_mouse_disable(){command(&[18]);}
pub unsafe extern "C" fn ikbd_joystick_event_on(){command(&[20]);} pub unsafe extern "C" fn ikbd_joystick_event_off(){command(&[21]);}
pub unsafe extern "C" fn ikbd_joystick_get_state(){command(&[22]);} pub unsafe extern "C" fn ikbd_joystick_disable(){command(&[26]);}

pub unsafe extern "C" fn atari_keyb_init() -> i32 {
    if atari_keyb_done != 0 { return 0; }
    kb_state.state=KB_STATE_T::KEYBOARD; kb_state.len=0; atari_turnoff_irq(IRQ_MFP_ACIA);
    loop { acia.key_ctrl=ACIA_RESET|if atari_switches&ATARI_SWITCH_IKBD!=0{ACIA_RHTID}else{0}; let _=acia.key_ctrl; let _=acia.key_data; acia.mid_ctrl=ACIA_RESET|if atari_switches&ATARI_SWITCH_MIDI!=0{ACIA_RHTID}else{0}; let _=acia.mid_ctrl; let _=acia.mid_data; acia.key_ctrl=ACIA_DIV64|ACIA_D8N1S|ACIA_RIE|if atari_switches&ATARI_SWITCH_IKBD!=0{ACIA_RHTID}else{ACIA_RLTID}; acia.mid_ctrl=ACIA_DIV16|ACIA_D8N1S|if atari_switches&ATARI_SWITCH_MIDI!=0{ACIA_RHTID}else{0}; if st_mfp.par_dt_reg&0x10!=0{break;} }
    st_mfp.active_edge&=!0x10; atari_turnon_irq(IRQ_MFP_ACIA); ikbd_self_test=1; ikbd_reset(); self_test_last_rcv=jiffies; while jiffies<self_test_last_rcv.wrapping_add(HZ/4){core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);} ikbd_self_test=0; ikbd_mouse_disable(); ikbd_joystick_disable(); atari_keyb_done=1; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
