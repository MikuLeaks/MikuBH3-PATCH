use core::iter::once;
use std::ffi::OsStr;

use std::os::windows::ffi::OsStrExt;
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;

use crate::{config, BASE};

fn wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub unsafe fn try_get_module(module_name: &str) -> Option<(usize, usize)> {
    let w_module_name = wide_str(module_name);

    match GetModuleHandleW(PCWSTR::from_raw(w_module_name.as_ptr())) {
        Ok(module) => {
            let mut module_info = MODULEINFO {
                lpBaseOfDll: std::ptr::null_mut(),
                SizeOfImage: 0,
                EntryPoint: std::ptr::null_mut(),
            };

            GetModuleInformation(
                GetCurrentProcess(),
                module,
                &mut module_info,
                std::mem::size_of::<MODULEINFO>() as u32,
            )
            .unwrap();

            Some((module.0 as usize, module_info.SizeOfImage as usize))
        }
        Err(_) => None,
    }
}

pub unsafe fn string_from_csharp(addr: usize) -> String {
    if addr == 0 {
        return String::new();
    }
    let str_length = *(addr.wrapping_add(16) as *const u32);
    let str_ptr = addr.wrapping_add(20) as *const u8;
    let slice = std::slice::from_raw_parts(str_ptr, (str_length * 2) as usize);
    String::from_utf16le(slice).unwrap()
}

pub unsafe fn create_csharp_string(content: &str) -> *const i8 {
    let func: unsafe extern "fastcall" fn(usize, *const u16) -> *const i8 =
        std::mem::transmute(BASE + config::CTOR_CHAR_PTR_PTR);

    let string_vec = content
        .encode_utf16()
        .chain(std::iter::once(0)) // null terminator
        .collect::<Vec<_>>();

    func(0, string_vec.as_ptr())
}
