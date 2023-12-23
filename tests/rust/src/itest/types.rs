use gd_props::types::GdResVec;
use gd_rehearse::itest::gditest;
use godot::{
    builtin::{meta::ToGodot, Array, VariantArray},
    engine::Resource,
    obj::Gd,
};

#[gditest]
fn gdresvec_construction() {
    let mut resvec = GdResVec::default();
    resvec.push(Gd::<Resource>::default());
    resvec.push(Gd::<Resource>::default());
    assert_eq!(resvec.len(), 2);

    // Create from existing vector
    let vector = vec![Gd::<Resource>::default(), Gd::<Resource>::default()];
    let from_vec = GdResVec::from_vec(vector);
    assert_eq!(from_vec.len(), 2);

    // Create from typed Godot array
    let mut typed_arr: Array<Gd<Resource>> = Array::new();
    typed_arr.push(Gd::<Resource>::default());
    typed_arr.push(Gd::<Resource>::default());
    let from_typed_arr = GdResVec::from_array(typed_arr);
    assert_eq!(from_typed_arr.len(), 2);

    // Create from variant array
    let mut var_arr = VariantArray::new();
    var_arr.push(Gd::<Resource>::default().to_variant());
    var_arr.push(Gd::<Resource>::default().to_variant());
    let from_var_arr = GdResVec::<Resource>::from_variant_array(var_arr);
    assert_eq!(from_var_arr.len(), 2);
}
