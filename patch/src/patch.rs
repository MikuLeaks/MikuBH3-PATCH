use crate::{
    config::{self, SERVER_DISPATCH_PORT},
    util::{self},
    BASE, INTERCEPTOR,
};
use ilhook::x64::Registers;

pub unsafe fn hook_unity_webrequtils_makeiniturl() {
    INTERCEPTOR
        .attach(
            BASE + config::UNITY_WEBREQUTILS_MAKEINITURL_PTR,
            on_make_initial_url,
        )
        .unwrap();

    println!("[HTTP] Proxy enabled!")
}

unsafe extern "win64" fn on_make_initial_url(reg: *mut Registers, _: usize) {
    let url = util::string_from_csharp((*reg).rcx as usize);

    let mut new_url = format!("http://127.0.0.1:{}", SERVER_DISPATCH_PORT);
    url.split('/').skip(3).for_each(|s| {
        new_url.push('/');
        new_url.push_str(s);
    });

    if url.contains("autopatch") || url.contains("bundle") || url.contains("statics") {
        println!("[HTTP] {url}");
        return;
    }

    println!("[HTTP REDIRECTED] {url}");
    (*reg).rcx = util::create_csharp_string(&new_url) as u64;
}
