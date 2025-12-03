#![no_main]

use core::ffi::c_void;

use windows::{Win32::{self, Foundation::HMODULE, System::{Console::AllocConsole, Threading::CreateThread}}, core::BOOL};

const DLL_PROCESS_ATTACH: u32 = 1;

unsafe extern "system" fn main_thread(_: *mut c_void) -> u32 {
    unsafe {
        
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    module: HMODULE,
    reason: u32,
    lp_reserved: *mut c_void
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            CreateThread(
                None, 
                0, 
                main_thread, 
                0, 
                0, 
                None
            );
        }
    }

    BOOL(1);
}