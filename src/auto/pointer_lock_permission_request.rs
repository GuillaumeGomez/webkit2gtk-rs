// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::PermissionRequest;
use std::fmt;

glib::wrapper! {
    pub struct PointerLockPermissionRequest(Object<ffi::WebKitPointerLockPermissionRequest, ffi::WebKitPointerLockPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_pointer_lock_permission_request_get_type(),
    }
}

impl PointerLockPermissionRequest {}

pub const NONE_POINTER_LOCK_PERMISSION_REQUEST: Option<&PointerLockPermissionRequest> = None;

impl fmt::Display for PointerLockPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PointerLockPermissionRequest")
    }
}
