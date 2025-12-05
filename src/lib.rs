#![no_main]

use core::ffi::c_void;
use std::ptr::null_mut;
use windows::{Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS}, core::BOOL};
use windows::Win32::Foundation::HMODULE;

mod utils;
use crate::utils::Logger;

const DLL_PROCESS_ATTACH: u32 = 1;

#[unsafe(no_mangle)]
unsafe extern "system" fn main_thread(_: *mut c_void) -> u32 {
    let mut logger = Logger::new();
    logger.print("DLL injected!");

    0
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    module: HMODULE,
    reason: u32,
    _: *mut c_void
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            // This error should be covered
            let _ = CreateThread(
                Some(null_mut()), 
                0, 
                Some(main_thread), 
                Some(std::mem::transmute(module)), 
                THREAD_CREATION_FLAGS(0),
                Some(null_mut())
            );
        }
    }

    BOOL(1)
}