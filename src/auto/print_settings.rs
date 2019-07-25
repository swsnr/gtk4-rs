// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::translate::*;
use glib::GString;
use gtk_sys;
use std;
use std::fmt;
use std::mem;
use std::ptr;
use Error;
use NumberUpLayout;
use PageOrientation;
use PageRange;
use PageSet;
use PaperSize;
use PrintDuplex;
use PrintPages;
use PrintQuality;
use Unit;

glib_wrapper! {
    pub struct PrintSettings(Object<gtk_sys::GtkPrintSettings, PrintSettingsClass>);

    match fn {
        get_type => || gtk_sys::gtk_print_settings_get_type(),
    }
}

impl PrintSettings {
    pub fn new() -> PrintSettings {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_print_settings_new()) }
    }

    pub fn new_from_file<P: AsRef<std::path::Path>>(file_name: P) -> Result<PrintSettings, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_print_settings_new_from_file(
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_gvariant(variant: &glib::Variant) -> PrintSettings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_print_settings_new_from_gvariant(
                variant.to_glib_none().0,
            ))
        }
    }

    pub fn new_from_key_file(
        key_file: &glib::KeyFile,
        group_name: Option<&str>,
    ) -> Result<PrintSettings, Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_print_settings_new_from_key_file(
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn copy(&self) -> Option<PrintSettings> {
        unsafe { from_glib_full(gtk_sys::gtk_print_settings_copy(self.to_glib_none().0)) }
    }

    pub fn foreach<P: FnMut(&str, &str)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&str, &str)>(
            key: *const libc::c_char,
            value: *const libc::c_char,
            user_data: glib_sys::gpointer,
        ) {
            let key: GString = from_glib_borrow(key);
            let value: GString = from_glib_borrow(value);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(key.as_str(), value.as_str());
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            gtk_sys::gtk_print_settings_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    pub fn get(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_bool(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    pub fn get_collate(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_collate(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_default_source(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get_default_source(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_dither(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get_dither(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        unsafe {
            gtk_sys::gtk_print_settings_get_double(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn get_double_with_default(&self, key: &str, def: f64) -> f64 {
        unsafe {
            gtk_sys::gtk_print_settings_get_double_with_default(
                self.to_glib_none().0,
                key.to_glib_none().0,
                def,
            )
        }
    }

    pub fn get_duplex(&self) -> PrintDuplex {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_duplex(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_finishings(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get_finishings(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_int(&self, key: &str) -> i32 {
        unsafe { gtk_sys::gtk_print_settings_get_int(self.to_glib_none().0, key.to_glib_none().0) }
    }

    pub fn get_int_with_default(&self, key: &str, def: i32) -> i32 {
        unsafe {
            gtk_sys::gtk_print_settings_get_int_with_default(
                self.to_glib_none().0,
                key.to_glib_none().0,
                def,
            )
        }
    }

    pub fn get_length(&self, key: &str, unit: Unit) -> f64 {
        unsafe {
            gtk_sys::gtk_print_settings_get_length(
                self.to_glib_none().0,
                key.to_glib_none().0,
                unit.to_glib(),
            )
        }
    }

    pub fn get_media_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get_media_type(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_n_copies(&self) -> i32 {
        unsafe { gtk_sys::gtk_print_settings_get_n_copies(self.to_glib_none().0) }
    }

    pub fn get_number_up(&self) -> i32 {
        unsafe { gtk_sys::gtk_print_settings_get_number_up(self.to_glib_none().0) }
    }

    pub fn get_number_up_layout(&self) -> NumberUpLayout {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_number_up_layout(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_orientation(&self) -> PageOrientation {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_orientation(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_output_bin(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get_output_bin(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_page_ranges(&self) -> Vec<PageRange> {
        unsafe {
            let mut num_ranges = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                gtk_sys::gtk_print_settings_get_page_ranges(
                    self.to_glib_none().0,
                    num_ranges.as_mut_ptr(),
                ),
                num_ranges.assume_init() as usize,
            );
            ret
        }
    }

    pub fn get_page_set(&self) -> PageSet {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_page_set(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_paper_height(&self, unit: Unit) -> f64 {
        unsafe {
            gtk_sys::gtk_print_settings_get_paper_height(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_paper_size(&self) -> PaperSize {
        unsafe {
            from_glib_full(gtk_sys::gtk_print_settings_get_paper_size(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_paper_width(&self, unit: Unit) -> f64 {
        unsafe {
            gtk_sys::gtk_print_settings_get_paper_width(self.to_glib_none().0, unit.to_glib())
        }
    }

    pub fn get_print_pages(&self) -> PrintPages {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_print_pages(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_printer(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_get_printer(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_printer_lpi(&self) -> f64 {
        unsafe { gtk_sys::gtk_print_settings_get_printer_lpi(self.to_glib_none().0) }
    }

    pub fn get_quality(&self) -> PrintQuality {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_quality(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_resolution(&self) -> i32 {
        unsafe { gtk_sys::gtk_print_settings_get_resolution(self.to_glib_none().0) }
    }

    pub fn get_resolution_x(&self) -> i32 {
        unsafe { gtk_sys::gtk_print_settings_get_resolution_x(self.to_glib_none().0) }
    }

    pub fn get_resolution_y(&self) -> i32 {
        unsafe { gtk_sys::gtk_print_settings_get_resolution_y(self.to_glib_none().0) }
    }

    pub fn get_reverse(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_reverse(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_scale(&self) -> f64 {
        unsafe { gtk_sys::gtk_print_settings_get_scale(self.to_glib_none().0) }
    }

    pub fn get_use_color(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_get_use_color(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_print_settings_has_key(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    pub fn load_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_print_settings_load_file(
                self.to_glib_none().0,
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load_key_file(
        &self,
        key_file: &glib::KeyFile,
        group_name: Option<&str>,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_print_settings_load_key_file(
                self.to_glib_none().0,
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn set(&self, key: &str, value: Option<&str>) {
        unsafe {
            gtk_sys::gtk_print_settings_set(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    pub fn set_bool(&self, key: &str, value: bool) {
        unsafe {
            gtk_sys::gtk_print_settings_set_bool(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib(),
            );
        }
    }

    pub fn set_collate(&self, collate: bool) {
        unsafe {
            gtk_sys::gtk_print_settings_set_collate(self.to_glib_none().0, collate.to_glib());
        }
    }

    pub fn set_default_source(&self, default_source: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_set_default_source(
                self.to_glib_none().0,
                default_source.to_glib_none().0,
            );
        }
    }

    pub fn set_dither(&self, dither: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_set_dither(self.to_glib_none().0, dither.to_glib_none().0);
        }
    }

    pub fn set_double(&self, key: &str, value: f64) {
        unsafe {
            gtk_sys::gtk_print_settings_set_double(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value,
            );
        }
    }

    pub fn set_duplex(&self, duplex: PrintDuplex) {
        unsafe {
            gtk_sys::gtk_print_settings_set_duplex(self.to_glib_none().0, duplex.to_glib());
        }
    }

    pub fn set_finishings(&self, finishings: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_set_finishings(
                self.to_glib_none().0,
                finishings.to_glib_none().0,
            );
        }
    }

    pub fn set_int(&self, key: &str, value: i32) {
        unsafe {
            gtk_sys::gtk_print_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, value);
        }
    }

    pub fn set_length(&self, key: &str, value: f64, unit: Unit) {
        unsafe {
            gtk_sys::gtk_print_settings_set_length(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value,
                unit.to_glib(),
            );
        }
    }

    pub fn set_media_type(&self, media_type: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_set_media_type(
                self.to_glib_none().0,
                media_type.to_glib_none().0,
            );
        }
    }

    pub fn set_n_copies(&self, num_copies: i32) {
        unsafe {
            gtk_sys::gtk_print_settings_set_n_copies(self.to_glib_none().0, num_copies);
        }
    }

    pub fn set_number_up(&self, number_up: i32) {
        unsafe {
            gtk_sys::gtk_print_settings_set_number_up(self.to_glib_none().0, number_up);
        }
    }

    pub fn set_number_up_layout(&self, number_up_layout: NumberUpLayout) {
        unsafe {
            gtk_sys::gtk_print_settings_set_number_up_layout(
                self.to_glib_none().0,
                number_up_layout.to_glib(),
            );
        }
    }

    pub fn set_orientation(&self, orientation: PageOrientation) {
        unsafe {
            gtk_sys::gtk_print_settings_set_orientation(
                self.to_glib_none().0,
                orientation.to_glib(),
            );
        }
    }

    pub fn set_output_bin(&self, output_bin: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_set_output_bin(
                self.to_glib_none().0,
                output_bin.to_glib_none().0,
            );
        }
    }

    pub fn set_page_set(&self, page_set: PageSet) {
        unsafe {
            gtk_sys::gtk_print_settings_set_page_set(self.to_glib_none().0, page_set.to_glib());
        }
    }

    pub fn set_paper_height(&self, height: f64, unit: Unit) {
        unsafe {
            gtk_sys::gtk_print_settings_set_paper_height(
                self.to_glib_none().0,
                height,
                unit.to_glib(),
            );
        }
    }

    pub fn set_paper_size(&self, paper_size: &PaperSize) {
        unsafe {
            gtk_sys::gtk_print_settings_set_paper_size(
                self.to_glib_none().0,
                mut_override(paper_size.to_glib_none().0),
            );
        }
    }

    pub fn set_paper_width(&self, width: f64, unit: Unit) {
        unsafe {
            gtk_sys::gtk_print_settings_set_paper_width(
                self.to_glib_none().0,
                width,
                unit.to_glib(),
            );
        }
    }

    pub fn set_print_pages(&self, pages: PrintPages) {
        unsafe {
            gtk_sys::gtk_print_settings_set_print_pages(self.to_glib_none().0, pages.to_glib());
        }
    }

    pub fn set_printer(&self, printer: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_set_printer(
                self.to_glib_none().0,
                printer.to_glib_none().0,
            );
        }
    }

    pub fn set_printer_lpi(&self, lpi: f64) {
        unsafe {
            gtk_sys::gtk_print_settings_set_printer_lpi(self.to_glib_none().0, lpi);
        }
    }

    pub fn set_quality(&self, quality: PrintQuality) {
        unsafe {
            gtk_sys::gtk_print_settings_set_quality(self.to_glib_none().0, quality.to_glib());
        }
    }

    pub fn set_resolution(&self, resolution: i32) {
        unsafe {
            gtk_sys::gtk_print_settings_set_resolution(self.to_glib_none().0, resolution);
        }
    }

    pub fn set_resolution_xy(&self, resolution_x: i32, resolution_y: i32) {
        unsafe {
            gtk_sys::gtk_print_settings_set_resolution_xy(
                self.to_glib_none().0,
                resolution_x,
                resolution_y,
            );
        }
    }

    pub fn set_reverse(&self, reverse: bool) {
        unsafe {
            gtk_sys::gtk_print_settings_set_reverse(self.to_glib_none().0, reverse.to_glib());
        }
    }

    pub fn set_scale(&self, scale: f64) {
        unsafe {
            gtk_sys::gtk_print_settings_set_scale(self.to_glib_none().0, scale);
        }
    }

    pub fn set_use_color(&self, use_color: bool) {
        unsafe {
            gtk_sys::gtk_print_settings_set_use_color(self.to_glib_none().0, use_color.to_glib());
        }
    }

    pub fn to_file<P: AsRef<std::path::Path>>(&self, file_name: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gtk_sys::gtk_print_settings_to_file(
                self.to_glib_none().0,
                file_name.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(gtk_sys::gtk_print_settings_to_gvariant(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn to_key_file(&self, key_file: &glib::KeyFile, group_name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_print_settings_to_key_file(
                self.to_glib_none().0,
                key_file.to_glib_none().0,
                group_name.to_glib_none().0,
            );
        }
    }

    pub fn unset(&self, key: &str) {
        unsafe {
            gtk_sys::gtk_print_settings_unset(self.to_glib_none().0, key.to_glib_none().0);
        }
    }
}

impl Default for PrintSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PrintSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PrintSettings")
    }
}
