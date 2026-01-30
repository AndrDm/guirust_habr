#![allow(dead_code)]
//==============================================================================
//
// Title:		UserInt Extender
// Purpose:		Safe wrappers for CVI User Interface functions
//
// Created on:	25.06.2025 at 08:57:26 by AD.
//
//==============================================================================

use std::os::raw::{c_char, c_int, c_void};

use crate::userint::*;

#[inline(always)]
pub fn init_cvi_rte(reserved: c_int, argv: *const *const c_char, reserved2: c_int) -> c_int {
	unsafe { InitCVIRTE(reserved, argv, reserved2) }
}

#[inline(always)]
pub fn quit_user_interface() -> c_int {
	unsafe {
		let ret = QuitUserInterface(0);
		println!("QuitUserInterface returned: {}", ret);
		let mut panel_data: i32 = 0;
		let mut control_data: i32 = 0;
		GetUserEvent(0, &mut panel_data as *mut i32, &mut control_data as *mut i32);
		//HidePanel(PANEL as i32);
		RunUserInterface();
		QuitUserInterface(0);
		return ret;
	}
}

#[inline(always)]
pub fn load_panel_ex(reserved: c_int, uir_file: *const c_char, panel: i32) -> c_int {
	unsafe { LoadPanelAnsi(reserved, uir_file, panel as i32) }
}
//pub fn DisplayPanel(panel_handle: c_int) -> c_int;
//pub fn RunUserInterface() -> c_int;
//pub fn DiscardPanel(panel_handle: c_int) -> c_int;

#[inline(always)]
pub fn set_localized_decimal_symbol(enable: bool) {
	unsafe {
		if enable {
			SetSystemAttributeAnsi(ATTR_USE_LOCALIZED_DECIMAL_SYMBOL as i32, 1);
		} else {
			// SetSystemAttributeAnsi(ATTR_USE_LOCALIZED_DECIMAL_SYMBOL as i32, 1);
			// This is the default behavior, so we can skip setting it to 1
			// unless we want to explicitly disable localized decimal symbols.
			SetSystemAttributeAnsi(ATTR_USE_LOCALIZED_DECIMAL_SYMBOL as i32, 0);
		}
	}
}

#[inline(always)]
pub fn display_panel(panel_handle: c_int) -> c_int {
	unsafe { DisplayPanel(panel_handle) }
}

#[inline(always)]
pub fn run_user_interface() -> c_int {
	unsafe { RunUserInterface() }
}

#[inline(always)]
pub fn discard_panel(panel_handle: c_int) -> c_int {
	unsafe { DiscardPanel(panel_handle) }
}

#[inline(always)]
pub fn close_cvi_rte() {
	unsafe { CloseCVIRTE() }
}

#[inline(always)]
pub fn set_ctrl_val_str(panel: c_int, ctrl_id: u32, value: &str) {
	let c_str = std::ffi::CString::new(value).unwrap();
	unsafe { SetCtrlValUtf8(panel, ctrl_id as i32, c_str.as_ptr()) };
}

#[inline(always)]
pub fn set_ctrl_val_i32(panel: c_int, ctrl_id: u32, value: i32) {
	unsafe { SetCtrlValUtf8(panel, ctrl_id as i32, value) };
}

#[inline(always)]
pub fn set_ctrl_val_f64(panel: c_int, ctrl_id: i32, value: f64) {
	unsafe { SetCtrlValUtf8(panel, ctrl_id as i32, value) };
}

// Helper functions to get control values
pub fn get_ctrl_val_f64(panel: i32, control: i32) -> f64 {
	let mut value: f64 = 0.0;
	unsafe {
		GetCtrlValAnsi(panel, control as i32, &mut value as *mut f64 as *mut c_void);
	}
	value
}

unsafe extern "C" {
	pub fn InitCVIRTE(reserved: c_int, argv: *const *const c_char, reserved2: c_int) -> c_int;
	pub fn LoadPanel(reserved: c_int, uir_file: *const c_char, panel: c_int) -> c_int;
	//pub fn DisplayPanel(panel_handle: c_int) -> c_int;
	//pub fn RunUserInterface() -> c_int;
	//pub fn DiscardPanel(panel_handle: c_int) -> c_int;
}
