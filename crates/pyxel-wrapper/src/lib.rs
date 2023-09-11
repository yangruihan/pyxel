#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::borrow_deref_ref,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::new_without_default,
    clippy::redundant_pub_crate,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::unused_self,
    clippy::use_self,
    clippy::used_underscore_binding,
    clippy::wrong_self_convention,
    clippy::zero_ptr
)]

#[macro_use]
mod utils;
mod audio_wrapper;
mod channel_wrapper;
mod constant_wrapper;
mod graphics_wrapper;
mod image_wrapper;
mod input_wrapper;
mod math_wrapper;
mod music_wrapper;
mod pyxel_singleton;
mod resource_wrapper;
mod sound_wrapper;
mod system_wrapper;
mod tilemap_wrapper;
#[allow(non_snake_case)]
mod variable_wrapper;

use pyo3::prelude::*;

#[pymodule]
fn pyxel_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    crate::image_wrapper::add_image_class(m)?;
    crate::tilemap_wrapper::add_tilemap_class(m)?;
    crate::channel_wrapper::add_channel_class(m)?;
    crate::sound_wrapper::add_sound_class(m)?;
    crate::music_wrapper::add_music_class(m)?;

    crate::constant_wrapper::add_module_constants(m)?;
    crate::variable_wrapper::add_module_variables(m)?;

    crate::system_wrapper::add_system_functions(m)?;
    crate::resource_wrapper::add_resource_functions(m)?;
    crate::input_wrapper::add_input_functions(m)?;
    crate::graphics_wrapper::add_graphics_functions(m)?;
    crate::audio_wrapper::add_audio_functions(m)?;
    crate::math_wrapper::add_math_functions(m)?;

    Ok(())
}