// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of mconf.c. External project types and functions are
// declared here only as dependencies supplied by the surrounding build.
use std::ffi::{c_char, c_int, c_void};
use std::ptr;

#[repr(C)] pub struct menu { pub sym: *mut symbol, pub prompt: *mut property, pub list: *mut menu, pub next: *mut menu, pub parent: *mut menu, pub data: *mut c_void }
#[repr(C)] pub struct symbol { pub rev_dep: tristate_dep }
#[repr(C)] pub struct property { pub r#type: c_int }
#[repr(C)] pub struct tristate_dep { pub tri: tristate }
#[repr(C)] pub struct subtitle_list { pub next: *mut subtitle_list, pub text: *const c_char }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct subtitle_part { pub entries: list_head, pub text: *const c_char }
#[repr(C)] pub struct gstr { pub max_width: c_int }
#[repr(C)] pub struct search_data { pub head: *mut list_head, pub target: *mut jump_key }
#[repr(C)] pub struct jump_key { pub entries: list_head }
#[repr(C)] pub struct menu_root { pub prompt: *mut property }
#[repr(C)] pub struct prompt_text { pub text: *const c_char }
#[repr(C)] pub union tristate { pub tri: c_int }

const NO: c_int = 0; const MOD: c_int = 1; const YES: c_int = 2;
const S_BOOLEAN: c_int = 0; const S_TRISTATE: c_int = 1; const S_INT: c_int = 2; const S_HEX: c_int = 3; const S_STRING: c_int = 4;
const P_MENU: c_int = 1; const P_COMMENT: c_int = 2; const KEY_ESC: c_int = 27; const ERRDISPLAYTOOSMALL: c_int = 1000;

extern "C" {
    static mut rootmenu: menu_root; static mut dialog_input_result: *mut c_char; static mut jump_key_char: c_int;
    static mut trail: list_head;
    fn snprintf(dst:*mut c_char, n:usize, fmt:*const c_char, ...) -> c_int; fn printf(fmt:*const c_char, ...)->c_int; fn fprintf(f:*mut c_void, fmt:*const c_char,...)->c_int;
    fn getenv(s:*const c_char)->*mut c_char; fn strcmp(a:*const c_char,b:*const c_char)->c_int; fn strcasecmp(a:*const c_char,b:*const c_char)->c_int; fn strlen(s:*const c_char)->usize; fn strncasecmp(a:*const c_char,b:*const c_char,n:usize)->c_int;
    fn malloc(n:usize)->*mut c_void; fn free(p:*mut c_void); fn exit(n:c_int)->!;
    fn set_dialog_backtitle(s:*const c_char); fn set_dialog_subtitles(s:*mut subtitle_list); fn dialog_clear(); fn dialog_textbox(t:*const c_char,x:*const c_char,r:c_int,c:c_int,v:*mut c_int,h:*mut c_int,cb:Option<unsafe extern "C" fn(c_int,usize,usize,*mut c_void)->c_int>,d:*mut c_void)->c_int;
    fn dialog_inputbox(t:*const c_char,x:*const c_char,r:c_int,c:c_int,i:*const c_char)->c_int; fn dialog_checklist(t:*const c_char,i:*const c_char,a:c_int,b:c_int,c:c_int)->c_int; fn dialog_menu(t:*const c_char,i:*const c_char,m:*mut menu,s:*mut c_int)->c_int; fn dialog_yesno(t:*const c_char,x:*const c_char,r:c_int,c:c_int)->c_int; fn end_dialog(x:c_int,y:c_int);
    fn init_dialog(s:*const c_char)->c_int; static mut saved_x:c_int; static mut saved_y:c_int;
    fn conf_parse(a:*mut c_char); fn conf_read(a:*const c_char)->c_int; fn conf_write(a:*const c_char)->c_int; fn conf_write_autoconf(a:c_int); fn conf_get_configname()->*const c_char; fn conf_get_changed()->bool; fn conf_set_changed(a:bool); fn conf_set_message_callback(f:Option<unsafe extern "C" fn(*const c_char)>);
    fn menu_get_prompt(m:*mut menu)->*const c_char; fn menu_get_ext_help(m:*mut menu,g:*mut gstr); fn menu_is_visible(m:*mut menu)->bool; fn menu_has_prompt(m:*mut menu)->bool; fn menu_is_empty(m:*mut menu)->bool;
    fn sym_get_type(s:*mut symbol)->c_int; fn sym_is_choice(s:*mut symbol)->bool; fn sym_calc_choice(m:*mut menu)->*mut symbol; fn sym_get_tristate_value(s:*mut symbol)->tristate; fn sym_is_changeable(s:*mut symbol)->bool; fn sym_get_string_value(s:*mut symbol)->*const c_char; fn sym_has_value(s:*mut symbol)->bool; fn sym_set_string_value(s:*mut symbol,v:*mut c_char)->c_int; fn sym_set_tristate_value(s:*mut symbol,v:tristate)->c_int; fn sym_toggle_tristate_value(s:*mut symbol);
    fn choice_set_value(m:*mut menu,s:*mut symbol); fn sym_re_search(s:*const c_char)->*mut *mut symbol; fn get_relations_str(a:*mut *mut symbol,h:*mut list_head)->gstr; fn str_new()->gstr; fn str_free(g:*mut gstr); fn str_get(g:*mut gstr)->*const c_char; fn str_printf(g:*mut gstr,f:*const c_char,...)->c_int;
    fn item_reset(); fn item_count()->c_int; fn item_make(f:*const c_char,...); fn item_add_str(f:*const c_char,...); fn item_set_tag(c:c_int); fn item_set_data(p:*mut menu); fn item_set_selected(c:c_int); fn item_activate_selected()->bool; fn item_data()->*mut menu; fn item_tag()->c_int; fn item_is_tag(c:c_int)->bool;
    fn handle_search_keys(a:c_int,b:usize,c:usize,d:*mut c_void)->c_int;
}

static mut indent:c_int=0; static mut current_menu:*mut menu=ptr::null_mut(); static mut child_count:c_int=0; static mut single_menu_mode:c_int=0; static mut show_all_options:c_int=0; static mut save_and_exit:c_int=0; static mut silent:c_int=0; static mut filename:[c_char;4097]=[0;4097];
static MCONF_README:&str="Overview\n--------\nThis interface lets you select features and parameters for the build.\nFeatures can either be built-in, modularized, or ignored. Parameters\nmust be entered in as decimal or hexadecimal numbers or text.\n\nSee the original mconf.c documentation for the complete interactive help text.\n";
static MENU_INSTRUCTIONS:&str="Arrow keys navigate the menu.  <Enter> selects submenus ---> (or empty submenus ----).  Highlighted letters are hotkeys.  Pressing <Y> includes, <N> excludes, <M> modularizes features.  Press <Esc><Esc> to exit, <?> for Help, </> for Search.";
static RADIOLIST_INSTRUCTIONS:&str="Use the arrow keys to navigate this window or press the hotkey of the item you wish to select followed by the <SPACE BAR>. Press <?> for additional information about this option.";
static SEARCH_HELP:&str="Search for symbols and display their relations.\nRegular expressions are allowed.\n";
static mut subtitles:*mut subtitle_list=ptr::null_mut();

unsafe fn set_config_filename(s:*const c_char){ snprintf(filename.as_mut_ptr(),filename.len(),c"%s - %s".as_ptr(),s,(*rootmenu.prompt).text); set_dialog_backtitle(filename.as_ptr()); }
unsafe fn show_textbox(t:*const c_char,x:*const c_char,r:c_int,c:c_int){ dialog_clear(); dialog_textbox(t,x,r,c,ptr::null_mut(),ptr::null_mut(),None,ptr::null_mut()); }
unsafe fn show_helptext(t:*const c_char,x:*const c_char){show_textbox(t,x,0,0)}
unsafe fn show_help(m:*mut menu){let mut g=str_new();g.max_width=80;menu_get_ext_help(m,&mut g);show_helptext(menu_get_prompt(m),str_get(&mut g));str_free(&mut g);}

unsafe fn build_conf(m:*mut menu){
    if !menu_is_visible(m) && show_all_options==0{return} if show_all_options!=0&&!menu_has_prompt(m){return}; let s=(*m).sym; let p=(*m).prompt;
    if s.is_null(){if !p||m==current_menu{} else {child_count+=1;item_make(c"---%*c%s".as_ptr(),indent+1,32,menu_get_prompt(m));item_set_tag(58);item_set_data(m);} indent+=1;let mut ch=(*m).list;while !ch.is_null(){build_conf(ch);ch=(*ch).next}indent-=1;return}
    child_count+=1;let v=sym_get_tristate_value(s);match sym_get_type(s){S_BOOLEAN=>item_make(c"[%c] %s".as_ptr(),if v.tri==NO{32}else{42},menu_get_prompt(m)),S_TRISTATE=>item_make(c"<%c> %s".as_ptr(),if v.tri==YES{42}else if v.tri==MOD{77}else{32},menu_get_prompt(m)),_=>{item_make(c"(%s) %s".as_ptr(),sym_get_string_value(s),menu_get_prompt(m));}} item_set_tag(116);item_set_data(m);if sym_get_type(s)!=S_BOOLEAN&&sym_get_type(s)!=S_TRISTATE{let mut ch=(*m).list;while !ch.is_null(){build_conf(ch);ch=(*ch).next}}
}

unsafe fn conf_string(m:*mut menu){loop{let r=dialog_inputbox(menu_get_prompt(m),c"Please enter a value.".as_ptr(),10,75,sym_get_string_value((*m).sym));match r{0=>{if sym_set_string_value((*m).sym,dialog_input_result)!=0{return}},1=>show_help(m),KEY_ESC=>return,_=>{}}}}
unsafe fn conf_choice(m:*mut menu){let mut active=sym_calc_choice(m);loop{item_reset();let mut ch=(*m).list;while !ch.is_null(){if menu_is_visible(ch){item_make(c"%s".as_ptr(),menu_get_prompt(ch));item_set_data(ch);if (*ch).sym==active{item_set_selected(1)}}ch=(*ch).next}let r=dialog_checklist(menu_get_prompt(m),RADIOLIST_INSTRUCTIONS.as_ptr() as *const c_char,8,60,4);if r==0{let x=item_data();if !x.is_null()&& !(*x).sym.is_null(){choice_set_value(m,(*x).sym)}return}if r==1{let x=item_data();if !x.is_null(){show_help(x);active=(*x).sym}}else if r==KEY_ESC{return}}}

unsafe fn conf(m:*mut menu,mut active:*mut menu){loop{item_reset();current_menu=m;child_count=0;build_conf(m);if child_count==0{break}let r=dialog_menu(menu_get_prompt(m),MENU_INSTRUCTIONS.as_ptr() as *const c_char,active,ptr::null_mut());if r==1||r==KEY_ESC{break}if !item_activate_selected(){continue}let sub=item_data();active=sub;let s=if sub.is_null(){ptr::null_mut()}else{(*sub).sym};match r{0=>if !sub.is_null(){if !s.is_null()&&sym_is_choice(s){conf_choice(sub)}else if !s.is_null()&&(*sub).prompt.as_ref().map_or(false,|p|p.r#type==P_MENU){conf(sub,ptr::null_mut())}else if !s.is_null(){conf_string(sub)}},2=>if !sub.is_null(){show_help(sub)}else{show_helptext(c"README".as_ptr(),MCONF_README.as_ptr() as *const c_char)},5=>if !s.is_null(){sym_set_tristate_value(s,tristate{tri:YES});},6=>if !s.is_null(){sym_set_tristate_value(s,tristate{tri:NO});},7=>if !s.is_null(){sym_set_tristate_value(s,tristate{tri:MOD});},8=>if !s.is_null(){sym_toggle_tristate_value(s)},9=>show_helptext(c"Search Configuration".as_ptr(),SEARCH_HELP.as_ptr() as *const c_char),10=>show_all_options^=1,_=>{}}}}

unsafe extern "C" fn sig_handler(_:c_int){exit(handle_exit())}
unsafe fn handle_exit()->c_int{save_and_exit=1;dialog_clear();let r=if conf_get_changed(){dialog_yesno(ptr::null(),c"Do you wish to save your new configuration?".as_ptr(),6,60)}else{-1};end_dialog(saved_x,saved_y);if r==0{if conf_write(filename.as_ptr())!=0{return 1};conf_write_autoconf(0)}if !silent{printf(c"\n*** End of the configuration.\n*** Execute 'make' to start the build or try 'make help'.\n\n".as_ptr())}0}

#[no_mangle] pub unsafe extern "C" fn main(ac:c_int,av:*mut *mut c_char)->c_int{if ac>1&&strcmp(*av.add(1),c"-s".as_ptr())==0{silent=1;conf_set_message_callback(None);av=av.add(1)}conf_parse(*av.add(1));conf_read(ptr::null());let mode=getenv(c"MENUCONFIG_MODE".as_ptr());if !mode.is_null()&&strcasecmp(mode,c"single_menu".as_ptr())==0{single_menu_mode=1}if init_dialog(ptr::null())!=0{return 1}set_config_filename(conf_get_configname());conf_set_message_callback(None);loop{conf(&mut rootmenu as *mut _ as *mut menu,ptr::null_mut());let r=handle_exit();if r!=KEY_ESC{return r}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
