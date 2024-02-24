use super::resource::*;
use gd_props::gd_props_plugin;

#[gd_props_plugin]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleArray)]
pub(crate) struct PropPlugin;
