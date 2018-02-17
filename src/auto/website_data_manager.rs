// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
use CookieManager;
use ffi;
use glib;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_10", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct WebsiteDataManager(Object<ffi::WebKitWebsiteDataManager, ffi::WebKitWebsiteDataManagerClass>);

    match fn {
        get_type => || ffi::webkit_website_data_manager_get_type(),
    }
}

impl WebsiteDataManager {
    //#[cfg(any(feature = "v2_10", feature = "dox"))]
    //pub fn new(first_option_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> WebsiteDataManager {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_new() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn new_ephemeral() -> WebsiteDataManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_website_data_manager_new_ephemeral())
        }
    }
}

pub trait WebsiteDataManagerExt {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn clear<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: WebsiteDataTypes, timespan: /*Ignored*/glib::TimeSpan, cancellable: P, callback: Q, user_data: R);

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn fetch<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: WebsiteDataTypes, cancellable: P, callback: Q, user_data: R);

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_base_cache_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_base_data_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_cookie_manager(&self) -> Option<CookieManager>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_disk_cache_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_indexeddb_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_local_storage_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_offline_application_cache_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_websql_directory(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn is_ephemeral(&self) -> bool;

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn remove<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: WebsiteDataTypes, website_data: /*Ignored*/&[&WebsiteData], cancellable: P, callback: Q, user_data: R);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_property_is_ephemeral(&self) -> bool;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_base_cache_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_base_data_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_disk_cache_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_indexeddb_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_is_ephemeral_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_local_storage_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_offline_application_cache_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_websql_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebsiteDataManager> + IsA<glib::object::Object>> WebsiteDataManagerExt for O {
    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn clear<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: WebsiteDataTypes, timespan: /*Ignored*/glib::TimeSpan, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_clear() }
    //}

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn fetch<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: WebsiteDataTypes, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_fetch() }
    //}

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_base_cache_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_cache_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_base_data_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_data_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_cookie_manager(&self) -> Option<CookieManager> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_cookie_manager(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_disk_cache_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_disk_cache_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_indexeddb_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_indexeddb_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_local_storage_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_local_storage_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_offline_application_cache_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_offline_application_cache_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn get_websql_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_websql_directory(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_is_ephemeral(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_16", feature = "dox"))]
    //fn remove<'a, 'b, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: WebsiteDataTypes, website_data: /*Ignored*/&[&WebsiteData], cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_remove() }
    //}

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_property_is_ephemeral(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-ephemeral".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_base_cache_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::base-cache-directory",
                transmute(notify_base_cache_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_base_data_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::base-data-directory",
                transmute(notify_base_data_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_disk_cache_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::disk-cache-directory",
                transmute(notify_disk_cache_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_indexeddb_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::indexeddb-directory",
                transmute(notify_indexeddb_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_is_ephemeral_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-ephemeral",
                transmute(notify_is_ephemeral_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_local_storage_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::local-storage-directory",
                transmute(notify_local_storage_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_offline_application_cache_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::offline-application-cache-directory",
                transmute(notify_offline_application_cache_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_10", feature = "dox"))]
    fn connect_property_websql_directory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::websql-directory",
                transmute(notify_websql_directory_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_base_cache_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_base_data_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_disk_cache_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_indexeddb_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn notify_is_ephemeral_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_local_storage_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_offline_application_cache_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_10", feature = "dox"))]
unsafe extern "C" fn notify_websql_directory_trampoline<P>(this: *mut ffi::WebKitWebsiteDataManager, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<WebsiteDataManager> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&WebsiteDataManager::from_glib_borrow(this).downcast_unchecked())
}
