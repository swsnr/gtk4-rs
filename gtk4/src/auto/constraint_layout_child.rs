// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::LayoutChild;
use std::fmt;

glib::wrapper! {
    pub struct ConstraintLayoutChild(Object<ffi::GtkConstraintLayoutChild, ffi::GtkConstraintLayoutChildClass>) @extends LayoutChild;

    match fn {
        get_type => || ffi::gtk_constraint_layout_child_get_type(),
    }
}

impl ConstraintLayoutChild {}

impl fmt::Display for ConstraintLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConstraintLayoutChild")
    }
}
