// This file was generated by gir (7484d29) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gtk;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FileChooserRequest(Object<ffi::WebKitFileChooserRequest>);

    match fn {
        get_type => || ffi::webkit_file_chooser_request_get_type(),
    }
}

pub trait FileChooserRequestExt {
    fn cancel(&self);

    fn get_mime_types(&self) -> Vec<String>;

    fn get_mime_types_filter(&self) -> Option<gtk::FileFilter>;

    fn get_select_multiple(&self) -> bool;

    fn get_selected_files(&self) -> Vec<String>;

    fn select_files(&self, files: &[&str]);

    fn get_property_filter(&self) -> Option<gtk::FileFilter>;

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mime_types_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserRequest> + IsA<glib::object::Object>> FileChooserRequestExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::webkit_file_chooser_request_cancel(self.to_glib_none().0);
        }
    }

    fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_mime_types(self.to_glib_none().0))
        }
    }

    fn get_mime_types_filter(&self) -> Option<gtk::FileFilter> {
        unsafe {
            from_glib_none(ffi::webkit_file_chooser_request_get_mime_types_filter(self.to_glib_none().0))
        }
    }

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_file_chooser_request_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_selected_files(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_selected_files(self.to_glib_none().0))
        }
    }

    fn select_files(&self, files: &[&str]) {
        unsafe {
            ffi::webkit_file_chooser_request_select_files(self.to_glib_none().0, files.to_glib_none().0);
        }
    }

    fn get_property_filter(&self) -> Option<gtk::FileFilter> {
        let mut value = Value::from(None::<&gtk::FileFilter>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "filter".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::filter",
                transmute(notify_filter_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_mime_types_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::mime-types",
                transmute(notify_mime_types_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_select_multiple_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::select-multiple",
                transmute(notify_select_multiple_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selected_files_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::selected-files",
                transmute(notify_selected_files_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_filter_trampoline<P>(this: *mut ffi::WebKitFileChooserRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserRequest::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_mime_types_trampoline<P>(this: *mut ffi::WebKitFileChooserRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserRequest::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_select_multiple_trampoline<P>(this: *mut ffi::WebKitFileChooserRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserRequest::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_selected_files_trampoline<P>(this: *mut ffi::WebKitFileChooserRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileChooserRequest> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileChooserRequest::from_glib_borrow(this).downcast_unchecked())
}
