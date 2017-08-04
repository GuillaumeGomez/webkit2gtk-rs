// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use CookieAcceptPolicy;
use CookiePersistentStorage;
use ffi;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct CookieManager(Object<ffi::WebKitCookieManager>);

    match fn {
        get_type => || ffi::webkit_cookie_manager_get_type(),
    }
}

impl CookieManager {
    pub fn delete_all_cookies(&self) {
        unsafe {
            ffi::webkit_cookie_manager_delete_all_cookies(self.to_glib_none().0);
        }
    }

    pub fn delete_cookies_for_domain(&self, domain: &str) {
        unsafe {
            ffi::webkit_cookie_manager_delete_cookies_for_domain(self.to_glib_none().0, domain.to_glib_none().0);
        }
    }

    //pub fn get_accept_policy<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_accept_policy() }
    //}

    //pub fn get_accept_policy_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<CookieAcceptPolicy, Error> {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_accept_policy_finish() }
    //}

    //pub fn get_domains_with_cookies<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_domains_with_cookies() }
    //}

    //pub fn get_domains_with_cookies_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<Vec<String>, Error> {
    //    unsafe { TODO: call ffi::webkit_cookie_manager_get_domains_with_cookies_finish() }
    //}

    pub fn set_accept_policy(&self, policy: CookieAcceptPolicy) {
        unsafe {
            ffi::webkit_cookie_manager_set_accept_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    pub fn set_persistent_storage(&self, filename: &str, storage: CookiePersistentStorage) {
        unsafe {
            ffi::webkit_cookie_manager_set_persistent_storage(self.to_glib_none().0, filename.to_glib_none().0, storage.to_glib());
        }
    }

    pub fn connect_changed<F: Fn(&CookieManager) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&CookieManager) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline(this: *mut ffi::WebKitCookieManager, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&CookieManager) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
