// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_sys;
use PermissionRequest;

glib_wrapper! {
    pub struct UserMediaPermissionRequest(Object<webkit2_sys::WebKitUserMediaPermissionRequest, webkit2_sys::WebKitUserMediaPermissionRequestClass, UserMediaPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        get_type => || webkit2_sys::webkit_user_media_permission_request_get_type(),
    }
}

pub const NONE_USER_MEDIA_PERMISSION_REQUEST: Option<&UserMediaPermissionRequest> = None;

pub trait UserMediaPermissionRequestExt: 'static {
    fn get_property_is_for_audio_device(&self) -> bool;

    fn get_property_is_for_video_device(&self) -> bool;

    fn connect_property_is_for_audio_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_for_video_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UserMediaPermissionRequest>> UserMediaPermissionRequestExt for O {
    fn get_property_is_for_audio_device(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"is-for-audio-device\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `is-for-audio-device` getter").unwrap()
        }
    }

    fn get_property_is_for_video_device(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"is-for-video-device\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `is-for-video-device` getter").unwrap()
        }
    }

    fn connect_property_is_for_audio_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_for_audio_device_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitUserMediaPermissionRequest, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<UserMediaPermissionRequest>
        {
            let f: &F = &*(f as *const F);
            f(&UserMediaPermissionRequest::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-for-audio-device\0".as_ptr() as *const _,
                Some(transmute(notify_is_for_audio_device_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_is_for_video_device_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_for_video_device_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_sys::WebKitUserMediaPermissionRequest, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<UserMediaPermissionRequest>
        {
            let f: &F = &*(f as *const F);
            f(&UserMediaPermissionRequest::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-for-video-device\0".as_ptr() as *const _,
                Some(transmute(notify_is_for_video_device_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for UserMediaPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserMediaPermissionRequest")
    }
}
