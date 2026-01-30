// CVI Rust UI Example
use std::{
	ffi::{CString, NulError},
	os::raw::{c_char, c_int, c_void},
};

mod ui;
mod userint;
mod userint_ex;

use crate::{ui::*, userint::*, userint_ex::*};

fn main() {
	println!("Hello, GUI world!");

	let c_argv = build_c_argv();

	if !init_runtime(&c_argv) {
		eprintln!("Failed to initialize CVIRTE.");
		return;
	}

	let panel_handle =
		load_panel(0, "bin/ui.uir", PANEL).expect("UIR path contained an interior NUL");

	display_panel(panel_handle);

	run_user_interface(); // <- staying here

	discard_panel(panel_handle);
	close_cvi_rte(); // requered for external compiler
}

#[unsafe(no_mangle)]
pub extern "C" fn OnChange(
	panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT {
		println!("Callback triggered");
		let a = get_ctrl_val_f64(panel, PANEL_NUMERIC_1);
		let b = get_ctrl_val_f64(panel, PANEL_NUMERIC_2);
		set_ctrl_val_f64(panel, PANEL_RESULT, a + b);
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn QuitCallback(
	_panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT {
		quit_user_interface();
	}
	0
}

#[unsafe(no_mangle)]
pub extern "C" fn panelCB(
	_panel: c_int,
	_control: c_int,
	event: c_int,
	_callback_data: *mut c_void,
	_event_data1: c_int,
	_event_data2: c_int,
) -> c_int {
	if event == EVENT_COMMIT as i32 {
		quit_user_interface();
	}
	0
}

fn build_c_argv() -> Vec<*const c_char> {
	std::env::args()
		.map(|arg| CString::new(arg).unwrap())
		.map(|cstr| cstr.into_raw() as *const c_char)
		.collect()
}

#[inline(always)]
fn init_runtime(c_argv: &[*const c_char]) -> bool {
	init_cvi_rte(0, c_argv.as_ptr(), 0) != 0
}

#[inline(always)]
pub fn load_panel(reserved: c_int, uir_file: &str, panel: i32) -> Result<c_int, NulError> {
	let uir = CString::new(uir_file)?; // fails if string contains '\0'
	let rc = unsafe { LoadPanelAnsi(reserved, uir.as_ptr() as *const c_char, panel) };
	Ok(rc)
}
