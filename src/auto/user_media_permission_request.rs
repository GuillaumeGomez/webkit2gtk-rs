// This file was generated by gir (32b0f11) from gir-files (857b8f5)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::Value;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct UserMediaPermissionRequest(Object<ffi::WebKitUserMediaPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_user_media_permission_request_get_type(),
    }
}

impl UserMediaPermissionRequest {
    pub fn get_property_is_for_audio_device(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-for-audio-device".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn get_property_is_for_video_device(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-for-video-device".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
