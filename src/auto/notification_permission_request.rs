// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct NotificationPermissionRequest(Object<ffi::WebKitNotificationPermissionRequest, ffi::WebKitNotificationPermissionRequestClass>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_notification_permission_request_get_type(),
    }
}

impl NotificationPermissionRequest {}
