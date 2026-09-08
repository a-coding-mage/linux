/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org> */

// Qt and expr.h declarations are supplied by external dependencies.

#[repr(C)]
pub struct ConfigSettings {
    pub base: QSettings,
}

#[repr(C)]
pub struct ConfigList {
    pub base: QTreeWidget,
    pub update_all: bool,
    pub show_name: bool,
    pub mode: listMode,
    pub opt_mode: optionMode,
    pub root_entry: *mut menu,
    pub disabled_color_group: QPalette,
    pub inactived_color_group: QPalette,
    pub header_popup: *mut QMenu,
}

#[repr(C)]
pub struct ConfigItem {
    pub base: QTreeWidgetItem,
    pub next_item: *mut ConfigItem,
    pub menu: *mut menu,
    pub go_parent: bool,
}

#[repr(C)]
pub struct ConfigItemDelegate {
    pub base: QStyledItemDelegate,
    pub menu: *mut menu,
}

#[repr(C)]
pub struct ConfigInfoView {
    pub base: QTextBrowser,
    pub context_menu: *mut QMenu,
    pub sym: *mut symbol,
    pub menu: *mut menu,
    pub show_debug: bool,
}

#[repr(C)]
pub struct ConfigSearchWindow {
    pub base: QDialog,
    pub edit_field: *mut QLineEdit,
    pub search_button: *mut QPushButton,
    pub split: *mut QSplitter,
    pub list: *mut ConfigList,
    pub info: *mut ConfigInfoView,
    pub result: *mut *mut symbol,
}

#[repr(C)]
pub struct ConfigMainWindow {
    pub base: QMainWindow,
    pub configname: QString,
    pub search_window: *mut ConfigSearchWindow,
    pub menu_list: *mut ConfigList,
    pub config_list: *mut ConfigList,
    pub help_text: *mut ConfigInfoView,
    pub back_action: *mut QAction,
    pub single_view_action: *mut QAction,
    pub split_view_action: *mut QAction,
    pub full_view_action: *mut QAction,
    pub split1: *mut QSplitter,
    pub split2: *mut QSplitter,
}

#[repr(C)]
pub enum colIdx {
    promptColIdx,
    nameColIdx,
    dataColIdx,
}

#[repr(C)]
pub enum listMode {
    singleMode,
    menuMode,
    symbolMode,
    fullMode,
    listMode,
}

#[repr(C)]
pub enum optionMode {
    normalOpt = 0,
    allOpt,
    promptOpt,
}

extern "C" {
    pub fn ConfigSettings_new() -> *mut ConfigSettings;
    pub fn ConfigSettings_delete(this: *mut ConfigSettings);
    pub fn ConfigSettings_readSizes(this: *mut ConfigSettings, key: *const QString, ok: *mut bool) -> QList_int;
    pub fn ConfigSettings_writeSizes(this: *mut ConfigSettings, key: *const QString, value: *const QList_int) -> bool;

    pub fn ConfigList_new(this: *mut ConfigList, parent: *mut QWidget, name: *const c_char);
    pub fn ConfigList_delete(this: *mut ConfigList);
    pub fn ConfigList_reinit(this: *mut ConfigList);
    pub fn ConfigList_findConfigItem(this: *mut ConfigList, menu: *mut menu) -> *mut ConfigItem;
    pub fn ConfigList_keyPressEvent(this: *mut ConfigList, e: *mut QKeyEvent);
    pub fn ConfigList_mouseReleaseEvent(this: *mut ConfigList, e: *mut QMouseEvent);
    pub fn ConfigList_mouseDoubleClickEvent(this: *mut ConfigList, e: *mut QMouseEvent);
    pub fn ConfigList_focusInEvent(this: *mut ConfigList, e: *mut QFocusEvent);
    pub fn ConfigList_contextMenuEvent(this: *mut ConfigList, e: *mut QContextMenuEvent);
    pub fn ConfigList_setRootMenu(this: *mut ConfigList, menu: *mut menu);
    pub fn ConfigList_updateList(this: *mut ConfigList);
    pub fn ConfigList_setValue(this: *mut ConfigList, item: *mut ConfigItem, val: tristate);
    pub fn ConfigList_changeValue(this: *mut ConfigList, item: *mut ConfigItem);
    pub fn ConfigList_updateSelection(this: *mut ConfigList);
    pub fn ConfigList_saveSettings(this: *mut ConfigList);
    pub fn ConfigList_setOptionMode(this: *mut ConfigList, action: *mut QAction);
    pub fn ConfigList_setShowName(this: *mut ConfigList, on: bool);
    pub fn ConfigList_setAllOpen(this: *mut ConfigList, open: bool);
    pub fn ConfigList_setParentMenu(this: *mut ConfigList);
    pub fn ConfigList_menuSkip(this: *mut ConfigList, menu: *mut menu) -> bool;
    pub fn ConfigList_updateMenuList(this: *mut ConfigList, parent: *mut ConfigItem, menu: *mut menu);
    pub fn ConfigList_updateMenuList_menu(this: *mut ConfigList, menu: *mut menu);
    pub fn ConfigList_updateListForAll();
    pub fn ConfigList_updateListAllForAll();

    pub fn ConfigItem_delete(this: *mut ConfigItem);
    pub fn ConfigItem_init(this: *mut ConfigItem);
    pub fn ConfigItem_updateMenu(this: *mut ConfigItem);
    pub fn ConfigItem_testUpdateMenu(this: *mut ConfigItem);
    pub fn ConfigItemDelegate_createEditor(this: *const ConfigItemDelegate, parent: *mut QWidget, option: *const QStyleOptionViewItem, index: *const QModelIndex) -> *mut QWidget;
    pub fn ConfigItemDelegate_setModelData(this: *const ConfigItemDelegate, editor: *mut QWidget, model: *mut QAbstractItemModel, index: *const QModelIndex);

    pub fn ConfigInfoView_new(this: *mut ConfigInfoView, parent: *mut QWidget, name: *const c_char);
    pub fn ConfigInfoView_setInfo(this: *mut ConfigInfoView, menu: *mut menu);
    pub fn ConfigInfoView_saveSettings(this: *mut ConfigInfoView);
    pub fn ConfigInfoView_setShowDebug(this: *mut ConfigInfoView, value: bool);
    pub fn ConfigInfoView_clicked(this: *mut ConfigInfoView, url: *const QUrl);
    pub fn ConfigInfoView_symbolInfo(this: *mut ConfigInfoView);
    pub fn ConfigInfoView_menuInfo(this: *mut ConfigInfoView);
    pub fn ConfigInfoView_debug_info(this: *mut ConfigInfoView, sym: *mut symbol) -> QString;
    pub fn ConfigInfoView_print_filter(str_: *const QString) -> QString;
    pub fn ConfigInfoView_expr_print_help(data: *mut c_void, sym: *mut symbol, str_: *const c_char);
    pub fn ConfigInfoView_contextMenuEvent(this: *mut ConfigInfoView, event: *mut QContextMenuEvent);

    pub fn ConfigSearchWindow_new(this: *mut ConfigSearchWindow, parent: *mut ConfigMainWindow);
    pub fn ConfigSearchWindow_saveSettings(this: *mut ConfigSearchWindow);
    pub fn ConfigSearchWindow_search(this: *mut ConfigSearchWindow);
    pub fn ConfigMainWindow_new(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_changeMenu(this: *mut ConfigMainWindow, menu: *mut menu);
    pub fn ConfigMainWindow_changeItens(this: *mut ConfigMainWindow, menu: *mut menu);
    pub fn ConfigMainWindow_setMenuLink(this: *mut ConfigMainWindow, menu: *mut menu);
    pub fn ConfigMainWindow_listFocusChanged(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_goBack(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_loadConfig(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_saveConfig(this: *mut ConfigMainWindow) -> bool;
    pub fn ConfigMainWindow_saveConfigAs(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_searchConfig(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_showSingleView(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_showSplitView(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_showFullView(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_showIntro(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_showAbout(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_saveSettings(this: *mut ConfigMainWindow);
    pub fn ConfigMainWindow_closeEvent(this: *mut ConfigMainWindow, e: *mut QCloseEvent);
    pub fn ConfigMainWindow_conf_changed(value: bool);
}

// External Qt and expression-language types.
use core::ffi::{c_char, c_void};
pub enum QSettings {}
pub enum QList_int {}
pub enum QString {}
pub enum QWidget {}
pub enum QTreeWidget {}
pub enum QTreeWidgetItem {}
pub enum QPalette {}
pub enum QMenu {}
pub enum QAction {}
pub enum QIcon {}
pub enum QStyledItemDelegate {}
pub enum QTextBrowser {}
pub enum QDialog {}
pub enum QLineEdit {}
pub enum QPushButton {}
pub enum QSplitter {}
pub enum QMainWindow {}
pub enum QKeyEvent {}
pub enum QMouseEvent {}
pub enum QFocusEvent {}
pub enum QContextMenuEvent {}
pub enum QStyleOptionViewItem {}
pub enum QModelIndex {}
pub enum QAbstractItemModel {}
pub enum QUrl {}
pub enum menu {}
pub enum symbol {}
pub type tristate = i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
