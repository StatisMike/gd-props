use std::collections::HashSet;

use gd_props::GdProp;
use godot::{
    obj::Gd,
    prelude::{godot_api, GodotClass},
};
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::singleton::GodotSingleton;

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct InnerThing {
    pub int: i32,
    pub character: char,
}

impl InnerThing {
    pub fn new_random(rng: &mut impl Rng) -> Self {
        let int = rng.gen_range(-256..=256);
        let character = rng.gen_range(b'A'..=b'Z') as char;

        Self { int, character }
    }
}

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(init,base=Resource)]
pub struct TestResource {
    set: HashSet<InnerThing>,
    vec: Vec<InnerThing>,
}

#[godot_api]
impl TestResource {
    pub(crate) fn new_random(hash_count: u32, vec_count: u32) -> Gd<Self> {
        let mut rng = rand::thread_rng();

        let mut set = HashSet::new();
        let mut vec = Vec::new();

        for _ in 0..hash_count {
            set.insert(InnerThing::new_random(&mut rng));
        }

        for _ in 0..vec_count {
            vec.push(InnerThing::new_random(&mut rng));
        }

        Gd::<Self>::from_object(Self { set, vec })
    }

    pub(crate) fn get_set(&self) -> &HashSet<InnerThing> {
        &self.set
    }

    pub(crate) fn get_vec(&self) -> &Vec<InnerThing> {
        &self.vec
    }

    pub(crate) fn check_set_eq(first: &HashSet<InnerThing>, other: &HashSet<InnerThing>) -> bool {
        let mut first_set = first.clone();
        let mut second_set = other.clone();

        for el in second_set.drain() {
            first_set.remove(&el);
        }

        first_set.is_empty()
    }

    pub(crate) fn check_vec_eq(first: &[InnerThing], other: &[InnerThing]) -> bool {
        let mut eq_count = 0;
        let mut second = other.iter().collect::<Vec<_>>();

        while let Some(el_sec) = second.pop() {
            for el_first in first.iter() {
                if el_first.eq(el_sec) {
                    eq_count += 1;
                }
            }
        }

        eq_count == first.len()
    }
}

impl GodotSingleton for TestResource {
    const SINGLETON_NAME: &'static str = "TestResourceSingleton";

    fn singleton_instance() -> Gd<Self> {
        Self::new_random(50, 50)
    }
}
