#![feature(str_from_utf16_endian)]
use std::{thread, time::Duration};

use interceptor::Interceptor;
use patch::hook_unity_webrequtils_makeiniturl;
use util::try_get_module;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{Console, SystemServices::DLL_PROCESS_ATTACH},
};

mod config;
mod interceptor;
mod patch;
mod util;

pub static mut BASE: usize = 0;
pub static mut BASE_SIZE: usize = 0;
pub static mut INTERCEPTOR: Interceptor = Interceptor::default();

unsafe fn main() -> anyhow::Result<()> {
    loop {
        if let Some((addr, size)) = try_get_module("UserAssembly.dll") {
            BASE = addr;
            BASE_SIZE = size;
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Console::AllocConsole()?;

    println!("UA: 0x{:X}", BASE);
    println!("BH3 MikuMiku. discord.gg/MdHC4AJvec");

    hook_unity_webrequtils_makeiniturl();

    println!("Initialization done!");

    Ok(())
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| main());
    }

    true
}
