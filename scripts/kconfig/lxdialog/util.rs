// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of util.c. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct WINDOW { _private: [u8; 0] }
#[repr(C)] pub struct dialog_color { pub fg: c_int, pub bg: c_int, pub hl: bool, pub atr: u32 }
#[repr(C)] pub struct subtitle_list { pub text: *const c_char, pub next: *mut subtitle_list }
#[repr(C)] pub struct dialog_node { pub str_: [c_char; 1024], pub tag: c_char, pub data: *mut c_void, pub selected: c_int }
#[repr(C)] pub struct dialog_list { pub node: dialog_node, pub next: *mut dialog_list }
#[repr(C)] pub struct dialog_info {
    pub screen: dialog_color, pub shadow: dialog_color, pub dialog: dialog_color,
    pub title: dialog_color, pub border: dialog_color, pub button_active: dialog_color,
    pub button_inactive: dialog_color, pub button_key_active: dialog_color,
    pub button_key_inactive: dialog_color, pub button_label_active: dialog_color,
    pub button_label_inactive: dialog_color, pub inputbox: dialog_color,
    pub position_indicator: dialog_color, pub menubox: dialog_color,
    pub menubox_border: dialog_color, pub item: dialog_color, pub item_selected: dialog_color,
    pub tag: dialog_color, pub tag_selected: dialog_color, pub tag_key: dialog_color,
    pub tag_key_selected: dialog_color, pub check: dialog_color, pub check_selected: dialog_color,
    pub uarrow: dialog_color, pub darrow: dialog_color,
    pub backtitle: *const c_char, pub subtitles: *mut subtitle_list,
}

pub static mut saved_x: c_int = 0;
pub static mut saved_y: c_int = 0;
pub static mut dlg: dialog_info = unsafe { core::mem::zeroed() };
pub static mut item_cur: *mut dialog_list = core::ptr::null_mut();
pub static mut item_nil: dialog_list = unsafe { core::mem::zeroed() };
pub static mut item_head: *mut dialog_list = core::ptr::null_mut();

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn strspn(s: *const c_char, accept: *const c_char) -> usize;
    fn tolower(c: c_int) -> c_int; fn isalpha(c: c_int) -> c_int;
    fn malloc(n: usize) -> *mut c_void; fn free(p: *mut c_void); fn memset(p: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn getenv(s: *const c_char) -> *const c_char;
    fn initscr(); fn endwin(); fn getmaxy(w: *mut WINDOW) -> c_int; fn getmaxx(w: *mut WINDOW) -> c_int;
    fn getyx(w: *mut WINDOW, y: *mut c_int, x: *mut c_int); fn getmaxyx(w: *mut WINDOW, y: *mut c_int, x: *mut c_int);
    fn wattrset(w: *mut WINDOW, a: u32); fn wmove(w: *mut WINDOW,y:c_int,x:c_int)->c_int; fn waddch(w:*mut WINDOW,c:u32)->c_int;
    fn waddstr(w:*mut WINDOW,s:*const c_char)->c_int; fn waddnstr(w:*mut WINDOW,s:*const c_char,n:c_int)->c_int;
    fn mvwaddstr(w:*mut WINDOW,y:c_int,x:c_int,s:*const c_char)->c_int; fn mvwaddch(w:*mut WINDOW,y:c_int,x:c_int,c:u32)->c_int;
    fn wnoutrefresh(w:*mut WINDOW)->c_int; fn touchwin(w:*mut WINDOW)->c_int; fn move_(y:c_int,x:c_int)->c_int; fn refresh()->c_int;
    fn has_colors()->c_int; fn start_color(); fn init_pair(p:c_int,f:c_int,b:c_int); fn keypad(w:*mut WINDOW,b:c_int)->c_int;
    fn cbreak(); fn noecho(); fn nodelay(w:*mut WINDOW,b:c_int)->c_int; fn wgetch(w:*mut WINDOW)->c_int; fn ungetch(c:c_int)->c_int;
    fn winch(w:*mut WINDOW)->u32;
    static mut stdscr: *mut WINDOW;
    fn snprintf(dst:*mut c_char,n:usize,fmt:*const c_char,...)->c_int;
    fn vsnprintf(dst:*mut c_char,n:usize,fmt:*const c_char,ap:*mut c_void)->c_int;
}

const A_BOLD:u32=1; const A_REVERSE:u32=2; const A_DIM:u32=4; const COLOR_PAIR_MASK:u32=0xffff;
const COLOR_BLACK:c_int=0; const COLOR_RED:c_int=1; const COLOR_GREEN:c_int=2; const COLOR_YELLOW:c_int=3; const COLOR_BLUE:c_int=4; const COLOR_WHITE:c_int=7; const COLOR_CYAN:c_int=6;
const TRUE:c_int=1; const ERR:c_int=-1; const KEY_ESC:c_int=27; const KEY_RESIZE:c_int=0x101; const WINDOW_HEIGHT_MIN:c_int=19; const WINDOW_WIDTH_MIN:c_int=80; const ERRDISPLAYTOOSMALL:c_int=1;
const ACS_RARROW:u32=0; const ACS_HLINE:u32=0; const ACS_ULCORNER:u32=0; const ACS_LLCORNER:u32=0; const ACS_URCORNER:u32=0; const ACS_LRCORNER:u32=0; const ACS_VLINE:u32=0; const A_CHARTEXT:u32=0xff;

unsafe fn color(c:&mut dialog_color,f:c_int,b:c_int,h:bool){c.fg=f;c.bg=b;c.hl=h}
unsafe fn set_mono_theme(){ for c in [&mut dlg.title,&mut dlg.button_active,&mut dlg.button_key_active,&mut dlg.button_label_active,&mut dlg.item_selected,&mut dlg.tag,&mut dlg.tag_selected,&mut dlg.tag_key,&mut dlg.tag_key_selected,&mut dlg.check,&mut dlg.check_selected,&mut dlg.uarrow,&mut dlg.darrow]{c.atr=A_BOLD}; dlg.button_inactive.atr=A_DIM; }
unsafe fn set_classic_theme(){ let p=[(&mut dlg.screen,COLOR_CYAN,COLOR_BLUE,true),(&mut dlg.shadow,COLOR_BLACK,COLOR_BLACK,true),(&mut dlg.dialog,COLOR_BLACK,COLOR_WHITE,false),(&mut dlg.title,COLOR_YELLOW,COLOR_WHITE,true),(&mut dlg.border,COLOR_WHITE,COLOR_WHITE,true),(&mut dlg.button_active,COLOR_WHITE,COLOR_BLUE,true),(&mut dlg.button_inactive,COLOR_BLACK,COLOR_WHITE,false),(&mut dlg.button_key_active,COLOR_WHITE,COLOR_BLUE,true),(&mut dlg.button_key_inactive,COLOR_RED,COLOR_WHITE,false),(&mut dlg.button_label_active,COLOR_YELLOW,COLOR_BLUE,true),(&mut dlg.button_label_inactive,COLOR_BLACK,COLOR_WHITE,true),(&mut dlg.inputbox,COLOR_BLACK,COLOR_WHITE,false),(&mut dlg.position_indicator,COLOR_YELLOW,COLOR_WHITE,true),(&mut dlg.menubox,COLOR_BLACK,COLOR_WHITE,false),(&mut dlg.menubox_border,COLOR_WHITE,COLOR_WHITE,true),(&mut dlg.item,COLOR_BLACK,COLOR_WHITE,false),(&mut dlg.item_selected,COLOR_WHITE,COLOR_BLUE,true),(&mut dlg.tag,COLOR_YELLOW,COLOR_WHITE,true),(&mut dlg.tag_selected,COLOR_YELLOW,COLOR_BLUE,true),(&mut dlg.tag_key,COLOR_YELLOW,COLOR_WHITE,true),(&mut dlg.tag_key_selected,COLOR_YELLOW,COLOR_BLUE,true),(&mut dlg.check,COLOR_BLACK,COLOR_WHITE,false),(&mut dlg.check_selected,COLOR_WHITE,COLOR_BLUE,true),(&mut dlg.uarrow,COLOR_GREEN,COLOR_WHITE,true),(&mut dlg.darrow,COLOR_GREEN,COLOR_WHITE,true)]; for (c,f,b,h) in p {color(c,f,b,h)} }
unsafe fn set_blackbg_theme(){set_classic_theme()}
unsafe fn set_bluetitle_theme(){set_classic_theme(); color(&mut dlg.title,COLOR_BLUE,COLOR_WHITE,true); color(&mut dlg.button_key_active,COLOR_YELLOW,COLOR_BLUE,true); color(&mut dlg.button_label_active,COLOR_WHITE,COLOR_BLUE,true); color(&mut dlg.position_indicator,COLOR_BLUE,COLOR_WHITE,true); color(&mut dlg.tag,COLOR_BLUE,COLOR_WHITE,true); color(&mut dlg.tag_key,COLOR_BLUE,COLOR_WHITE,true)}
unsafe fn set_theme(t:*const c_char)->c_int{if t.is_null(){set_bluetitle_theme()}else if strcmp(t,b"classic\0".as_ptr() as _)==0{set_classic_theme()}else if strcmp(t,b"bluetitle\0".as_ptr() as _)==0{set_bluetitle_theme()}else if strcmp(t,b"blackbg\0".as_ptr() as _)==0{set_blackbg_theme()}else if strcmp(t,b"mono\0".as_ptr() as _)==0{return 0} 1}
unsafe fn init_one_color(c:&mut dialog_color){static mut pair:c_int=0;pair+=1;init_pair(pair,c.fg,c.bg);c.atr=if c.hl{A_BOLD|(pair as u32)}else{pair as u32}}
unsafe fn init_dialog_colors(){for c in [&mut dlg.screen,&mut dlg.shadow,&mut dlg.dialog,&mut dlg.title,&mut dlg.border,&mut dlg.button_active,&mut dlg.button_inactive,&mut dlg.button_key_active,&mut dlg.button_key_inactive,&mut dlg.button_label_active,&mut dlg.button_label_inactive,&mut dlg.inputbox,&mut dlg.position_indicator,&mut dlg.menubox,&mut dlg.menubox_border,&mut dlg.item,&mut dlg.item_selected,&mut dlg.tag,&mut dlg.tag_selected,&mut dlg.tag_key,&mut dlg.tag_key_selected,&mut dlg.check,&mut dlg.check_selected,&mut dlg.uarrow,&mut dlg.darrow]{init_one_color(c)}}
unsafe fn color_setup(t:*const c_char){if set_theme(t)!=0&&has_colors()!=0{start_color();init_dialog_colors()}else{set_mono_theme()}}

pub unsafe fn attr_clear(w:*mut WINDOW,h:c_int,width:c_int,a:u32){wattrset(w,a);for i in 0..h{wmove(w,i,0);for _ in 0..width{waddch(w,b' ' as u32)}}touchwin(w)}
pub unsafe fn dialog_clear(){let l=getmaxy(stdscr);let c=getmaxx(stdscr);attr_clear(stdscr,l,c,dlg.screen.atr);wnoutrefresh(stdscr)}
pub unsafe fn init_dialog(back:*const c_char)->c_int{initscr();getyx(stdscr,&mut saved_y,&mut saved_x);let mut h=0;let mut w=0;getmaxyx(stdscr,&mut h,&mut w);if h<WINDOW_HEIGHT_MIN||w<WINDOW_WIDTH_MIN{endwin();return -ERRDISPLAYTOOSMALL}dlg.backtitle=back;color_setup(getenv(b"MENUCONFIG_COLOR\0".as_ptr() as _));keypad(stdscr,TRUE);cbreak();noecho();dialog_clear();0}
pub unsafe fn set_dialog_backtitle(b:*const c_char){dlg.backtitle=b} pub unsafe fn set_dialog_subtitles(s:*mut subtitle_list){dlg.subtitles=s} pub unsafe fn end_dialog(x:c_int,y:c_int){move_(y,x);refresh();endwin()}
pub unsafe fn first_alpha(s:*const c_char,e:*const c_char)->c_int{for i in 0..strlen(s){let c=tolower(*s.add(i) as c_int);if !strchr(b"<[(\0".as_ptr() as _,c).is_null(){} if isalpha(c)!=0&&strchr(e,c).is_null(){return i as c_int}}0}
pub unsafe fn item_reset(){let mut p=item_head;while !p.is_null(){let n=(*p).next;free(p as _);p=n}item_head=core::ptr::null_mut();item_cur=&mut item_nil}
pub unsafe fn item_set_tag(t:c_char){(*item_cur).node.tag=t} pub unsafe fn item_set_data(p:*mut c_void){(*item_cur).node.data=p} pub unsafe fn item_set_selected(v:c_int){(*item_cur).node.selected=v}
pub unsafe fn item_make(_fmt:*const c_char,...){let p=malloc(core::mem::size_of::<dialog_list>()) as *mut dialog_list;memset(p as _,0,core::mem::size_of::<dialog_list>());if !item_head.is_null(){(*item_cur).next=p}else{item_head=p}item_cur=p}
pub unsafe fn item_add_str(_fmt:*const c_char,...){(*item_cur).node.str_[1023]=0}
pub unsafe fn item_data()->*mut c_void{(*item_cur).node.data} pub unsafe fn item_tag()->c_char{(*item_cur).node.tag} pub unsafe fn item_count()->c_int{let mut n=0;let mut p=item_head;while !p.is_null(){n+=1;p=(*p).next}n} pub unsafe fn item_is_selected()->c_int{((*item_cur).node.selected!=0) as c_int} pub unsafe fn item_is_tag(t:c_char)->c_int{((*item_cur).node.tag==t) as c_int}

pub unsafe fn on_key_esc(w:*mut WINDOW)->c_int{nodelay(w,TRUE);keypad(w,0);let k=wgetch(w);let k2=wgetch(w);let mut k3;loop{k3=wgetch(w);if k3==ERR{break}}nodelay(w,0);keypad(w,TRUE);if k==KEY_ESC&&k2==ERR{KEY_ESC}else if k!=ERR&&k!=KEY_ESC&&k2==ERR{ungetch(k);-1}else{-1}}
pub unsafe fn on_key_resize()->c_int{dialog_clear();KEY_RESIZE}
pub unsafe fn item_activate_selected()->c_int{let mut p=item_head;while !p.is_null(){item_cur=p;if item_is_selected()!=0{return 1}p=(*p).next}0}
pub unsafe fn item_set(n:c_int){let mut i=0;let mut p=item_head;while !p.is_null(){item_cur=p;if i==n{return}i+=1;p=(*p).next}}
pub unsafe fn item_n()->c_int{let mut n=0;let mut p=item_head;while !p.is_null(){if p==item_cur{return n}n+=1;p=(*p).next}0}
pub unsafe fn item_str()->*const c_char{(*item_cur).node.str_.as_ptr()}

pub unsafe fn draw_box(w:*mut WINDOW,y:c_int,x:c_int,h:c_int,width:c_int,box_:u32,border:u32){wattrset(w,0);for i in 0..h{wmove(w,y+i,x);for j in 0..width{let ch=if i==0&&j==0{border|ACS_ULCORNER}else if i==h-1&&j==0{border|ACS_LLCORNER}else if i==0&&j==width-1{box_|ACS_URCORNER}else if i==h-1&&j==width-1{box_|ACS_LRCORNER}else if i==0||i==h-1{box_|ACS_HLINE}else if j==0||j==width-1{box_|ACS_VLINE}else{box_|b' ' as u32};waddch(w,ch)}}}
pub unsafe fn draw_shadow(w:*mut WINDOW,y:c_int,x:c_int,h:c_int,width:c_int){if has_colors()!=0{wattrset(w,dlg.shadow.atr);wmove(w,y+h,x+2);for _ in 0..width{waddch(w,winch(w)&A_CHARTEXT)}for i in y+1..y+h+1{wmove(w,i,x+width);waddch(w,winch(w)&A_CHARTEXT);waddch(w,winch(w)&A_CHARTEXT)}wnoutrefresh(w)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
