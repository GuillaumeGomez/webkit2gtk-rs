// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_26", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_26", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_26", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_26", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v2_26", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_26", feature = "dox"))]
use std::mem::transmute;
use webkit2_sys;
#[cfg(any(feature = "v2_26", feature = "dox"))]
use GeolocationPosition;

glib_wrapper! {
    pub struct GeolocationManager(Object<webkit2_sys::WebKitGeolocationManager, webkit2_sys::WebKitGeolocationManagerClass, GeolocationManagerClass>);

    match fn {
        get_type => || webkit2_sys::webkit_geolocation_manager_get_type(),
    }
}

pub const NONE_GEOLOCATION_MANAGER: Option<&GeolocationManager> = None;

pub trait GeolocationManagerExt: 'static {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn failed(&self, error_message: &str);

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn get_enable_high_accuracy(&self) -> bool;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn update_position(&self, position: &mut GeolocationPosition);

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn connect_start<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn connect_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn connect_property_enable_high_accuracy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<GeolocationManager>> GeolocationManagerExt for O {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn failed(&self, error_message: &str) {
        unsafe {
            webkit2_sys::webkit_geolocation_manager_failed(
                self.as_ref().to_glib_none().0,
                error_message.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn get_enable_high_accuracy(&self) -> bool {
        unsafe {
            from_glib(
                webkit2_sys::webkit_geolocation_manager_get_enable_high_accuracy(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn update_position(&self, position: &mut GeolocationPosition) {
        unsafe {
            webkit2_sys::webkit_geolocation_manager_update_position(
                self.as_ref().to_glib_none().0,
                position.to_glib_none_mut().0,
            );
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn connect_start<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn start_trampoline<P, F: Fn(&P) -> bool + 'static>(
            this: *mut webkit2_sys::WebKitGeolocationManager,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<GeolocationManager>,
        {
            let f: &F = &*(f as *const F);
            f(&GeolocationManager::from_glib_borrow(this).unsafe_cast_ref()).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn connect_stop<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitGeolocationManager,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GeolocationManager>,
        {
            let f: &F = &*(f as *const F);
            f(&GeolocationManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v2_26", feature = "dox"))]
    fn connect_property_enable_high_accuracy_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_high_accuracy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut webkit2_sys::WebKitGeolocationManager,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GeolocationManager>,
        {
            let f: &F = &*(f as *const F);
            f(&GeolocationManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-high-accuracy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_high_accuracy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GeolocationManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GeolocationManager")
    }
}
