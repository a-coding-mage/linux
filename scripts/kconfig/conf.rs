// SPDX-License-Identifier: GPL-2.0
/* Faithful Rust translation of conf.c. External kconfig declarations are supplied by other units. */

use std::ffi::{CStr, CString};
use std::io::{self, Read, Write};
use std::os::raw::{c_char, c_int};
use std::ptr;

extern "C" {
    static mut rootmenu: menu;
    fn str_new() -> gstr; fn menu_get_ext_help(m:*mut menu,g:*mut gstr);
    fn str_get(g:*mut gstr)->*const c_char; fn str_free(g:*mut gstr);
    fn menu_is_visible(m:*mut menu)->bool; fn menu_get_prompt(m:*mut menu)->*const c_char;
    fn sym_has_value(s:*mut symbol)->bool; fn sym_is_changeable(s:*mut symbol)->bool;
    fn sym_get_string_value(s:*mut symbol)->*const c_char; fn sym_set_string_value(s:*mut symbol,v:*const c_char)->bool;
    fn sym_get_tristate_value(s:*mut symbol)->tristate; fn sym_set_tristate_value(s:*mut symbol,v:tristate)->bool;
    fn sym_tristate_within_range(s:*mut symbol,v:tristate)->bool; fn sym_calc_choice(m:*mut menu)->*mut symbol;
    fn choice_set_value(m:*mut menu,s:*mut symbol); fn sym_is_choice(s:*mut symbol)->bool;
    fn sym_is_choice_value(s:*mut symbol)->bool; fn sym_clear_all_valid(); fn sym_get_type(s:*mut symbol)->c_int;
    fn print_symbol_for_listconfig(s:*mut symbol); fn menu_get_menu_or_parent_menu(m:*mut menu)->*mut menu;
    fn conf_parse(s:*const c_char); fn conf_read(s:*const c_char)->c_int; fn conf_read_simple(s:*const c_char,d:c_int)->c_int;
    fn conf_errors()->c_int; fn conf_get_changed()->c_int; fn conf_set_message_callback(p:Option<unsafe extern "C" fn()>);
    fn sym_dep_errors()->c_int; fn conf_write_defconfig(s:*const c_char)->c_int; fn conf_write(s:*const c_char)->c_int;
    fn conf_write_autoconf(sync:c_int)->c_int;
}

#[repr(C)] pub struct gstr { _p:[u8;0] }
#[repr(C)] pub struct menu { pub sym:*mut symbol, pub prompt:*mut property, pub list:*mut menu, pub next:*mut menu, pub choice_members:list_head }
#[repr(C)] pub struct property { pub typ:c_int, pub text:*const c_char }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct symbol { pub name:*const c_char, pub typ:c_int, pub def:[symbol_value;8], pub flags:c_int, pub choice_link:list_head }
#[repr(C)] pub struct symbol_value { pub tri:tristate }
#[repr(C)] #[derive(Clone,Copy,PartialEq)] pub enum tristate { no=0, mod_=1, yes=2 }
const S_DEF_USER:c_int=0; const SYMBOL_DEF_USER:c_int=1; const S_BOOLEAN:c_int=1; const S_TRISTATE:c_int=2; const S_INT:c_int=3; const S_HEX:c_int=4; const S_STRING:c_int=5; const P_MENU:c_int=1; const P_COMMENT:c_int=2;

#[derive(Clone,Copy,PartialEq)] enum input_mode { oldaskconfig,syncconfig,oldconfig,allnoconfig,allyesconfig,allmodconfig,alldefconfig,randconfig,defconfig,savedefconfig,listnewconfig,helpnewconfig,olddefconfig,yes2modconfig,mod2yesconfig,mod2noconfig }
#[derive(Clone,Copy,PartialEq)] enum conf_def_mode { def_default,def_yes,def_mod,def_no,def_random }
static mut input_mode_:input_mode=input_mode::oldaskconfig; static mut input_mode_opt:c_int=0; static mut indent:c_int=1; static mut tty_stdio:bool=false; static mut sync_kconfig:c_int=0; static mut conf_cnt:c_int=0; static mut line:[u8;4096]=[0;4096]; static mut rootEntry:*mut menu=ptr::null_mut();

unsafe fn cstr(p:*const c_char)->String { if p.is_null(){String::new()}else{CStr::from_ptr(p).to_string_lossy().into_owned()} }
unsafe fn print_help(m:*mut menu){let mut h=str_new();menu_get_ext_help(m,&mut h);println!("\n{}",cstr(str_get(&mut h)));str_free(&mut h);}
unsafe fn strip(){let s=String::from_utf8_lossy(&line).trim().as_bytes().to_vec();line=[0;4096];line[..s.len().min(4095)].copy_from_slice(&s[..s.len().min(4095)]);}
unsafe fn xfgets(){let mut s=String::new();io::stdin().read_line(&mut s).ok();let b=s.as_bytes();line=[0;4096];line[..b.len().min(4095)].copy_from_slice(&b[..b.len().min(4095)]);if !tty_stdio{print!("{}",s);}}
unsafe fn conf_askvalue(s:*mut symbol,def:*const c_char)->c_int{if !sym_has_value(s){print!("(NEW) ");}if !sym_is_changeable(s){println!("{}",cstr(def));return 0;}xfgets();1}

unsafe fn conf_string(m:*mut menu)->c_int{let s=(*m).sym;loop{print!("{}{} "," ".repeat((indent-1) as usize),cstr(menu_get_prompt(m)));print!("({}) ",cstr((*s).name));let d=sym_get_string_value(s);if !d.is_null(){print!("[{}] ",cstr(d));}if conf_askvalue(s,d)==0{return 0;}strip();let t=CString::new(cstr(line.as_ptr() as *const c_char)).unwrap();if sym_set_string_value(s,t.as_ptr()){return 0;}}}
unsafe fn conf_sym(m:*mut menu)->c_int{let s=(*m).sym;loop{print!("{}{} "," ".repeat((indent-1) as usize),cstr(menu_get_prompt(m)));let old=sym_get_tristate_value(s);print!("[{}] ",match old{tristate::no=>"N",tristate::mod_=>"M",tristate::yes=>"Y"});if conf_askvalue(s,sym_get_string_value(s))==0{return 0;}strip();let c=line[0] as char;let n=match c{'n'|'N'=>tristate::no,'m'|'M'=>tristate::mod_,'y'|'Y'=>tristate::yes,_=>{if c=='?'{print_help(m);}continue}};if sym_set_tristate_value(s,n){return 0;}}}

unsafe fn conf_choice(m:*mut menu){let mut child;loop{println!("{}{}"," ".repeat((indent-1) as usize),cstr(menu_get_prompt(m)));let def=sym_calc_choice(m);let mut n=0;child=(*m).list;while !child.is_null(){if menu_is_visible(child)&&!(*child).sym.is_null(){n+=1;println!("{} {}. {} ({})"," ".repeat(indent as usize),n,cstr(menu_get_prompt(child)),cstr((*(*child).sym).name));}child=(*child).next;}if n==1{line[0]=b'1';}else{print!("{}choice[1-{}?]: "," ".repeat((indent-1) as usize),n);xfgets();strip();}let k=if line[0]==0{1}else{atoi_line()};child=(*m).list;let mut i=0;while !child.is_null(){if menu_is_visible(child)&&!(*child).sym.is_null(){i+=1;if i==k{choice_set_value(m,(*child).sym);return;}}child=(*child).next;}let _=def;}}
unsafe fn atoi_line()->c_int{cstr(line.as_ptr() as *const c_char).parse().unwrap_or(0)}

unsafe fn conf(m:*mut menu){if !menu_is_visible(m){return;}let s=(*m).sym;if !s.is_null(){if sym_is_choice(s){conf_choice(m);return;}match (*s).typ{S_INT|S_HEX|S_STRING=>{conf_string(m);},_=>{conf_sym(m);}}}let mut c=(*m).list;while !c.is_null(){conf(c);c=(*c).next;}}
unsafe fn check_conf(m:*mut menu){if !menu_is_visible(m){return;}let s=(*m).sym;if !s.is_null()&&!sym_has_value(s)&&sym_is_changeable(s){if input_mode_==input_mode::listnewconfig{print_symbol_for_listconfig(s)}else if input_mode_==input_mode::helpnewconfig{print_help(m)}else{conf_cnt+=1;rootEntry=menu_get_menu_or_parent_menu(m);conf(rootEntry);}}let mut c=(*m).list;while !c.is_null(){check_conf(c);c=(*c).next;}}

pub unsafe fn main_c()->c_int{conf_parse(ptr::null());if conf_errors()!=0{return 1;}match input_mode_{input_mode::oldaskconfig=>{rootEntry=&mut rootmenu;conf(&mut rootmenu);input_mode_=input_mode::oldconfig;},input_mode::oldconfig|input_mode::syncconfig|input_mode::listnewconfig|input_mode::helpnewconfig=>{loop{conf_cnt=0;check_conf(&mut rootmenu);if conf_cnt==0{break;}}},_=>{}}if sym_dep_errors()!=0{return 1;}if conf_write(ptr::null())!=0{return 1;}conf_write_autoconf(sync_kconfig);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
