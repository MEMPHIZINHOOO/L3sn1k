use std::path::PathBuf;

//****
// auxiliary GUI tools for utilization, keep in mind these are not the same as the "infrastructure" liable code that is currently being developed in the library of this crate
//
//
//
//


/// pub fn choose_file() opens a dialog and chooses a file, it returns an option of a pathBuf
/// be careful unwrapping this, make sure it does in fact exist beforehand, due to how iced is implemented
/// which is not idiomatic at all, we are stuck not being able to deal with unwraps utilizing propagation
/// as such we are left with this
pub fn choose_file() -> Option<PathBuf> {
    rfd::FileDialog::new().add_filter("json file", &["json"],).pick_file()
}
