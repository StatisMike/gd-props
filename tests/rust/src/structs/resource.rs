use std::collections::{HashMap, HashSet};

use gd_props::GdProp;

use gd_props::types::GdResVec;
use godot::builtin::GString;
use godot::engine::{IResource, ResourceSaver};
use godot::obj::Gd;
use godot::prelude::{godot_api, GodotClass};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::random_string;

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

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource)]
pub(crate) struct WithBundledGd {
    #[export]
    #[serde(with = "gd_props::serde_gd::gd")]
    pub first: Gd<TestResource>,
    #[export]
    #[serde(with = "gd_props::serde_gd::gd_option")]
    pub second: Option<Gd<TestResource>>,
}

impl WithBundledGd {
    pub(crate) fn new() -> Self {
        Self {
            first: TestResource::new_random(3, 4),
            second: Some(TestResource::new_random(5, 2)),
        }
    }
}

#[godot_api]
impl IResource for WithBundledGd {
    fn init(_base: godot::obj::Base<Self::Base>) -> Self {
        Self::new()
    }
}

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource, init)]
pub(crate) struct WithBundleHashMap {
    #[serde(with = "gd_props::serde_gd::gd_hashmap")]
    pub map: HashMap<i32, Gd<TestResource>>,
}

impl WithBundleHashMap {
    pub fn new(res_n: usize) -> Self {
        let mut map = HashMap::new();
        let mut rng = rand::thread_rng();
        let mut set = HashSet::new();
        for _ in 0..res_n {
            set.insert(rng.gen_range(-256..=256));
        }
        for key in set {
            let vec_n = rng.gen_range(1..10);
            let set_n = rng.gen_range(1..10);
            let res = TestResource::new_random(set_n, vec_n);
            map.insert(key, res);
        }
        Self { map }
    }
}

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource, init)]
pub(crate) struct WithBundleResVec {
    #[export]
    #[serde(with = "gd_props::serde_gd::gd_resvec")]
    pub vec: GdResVec<TestResource>,
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub(crate) struct TestGodotResource {
    #[export]
    pub(crate) int: i32,
    #[export]
    pub(crate) str: GString,
}

impl TestGodotResource {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let int = rng.gen_range(-1000..1000);
        let str = random_string(&mut rng, 10).into();
        Self { int, str }
    }

    pub fn new_saved_multiple(path: &str, n: usize) -> HashMap<String, Gd<Self>> {
        let mut obj = HashMap::new();
        let mut saver = ResourceSaver::singleton();
        for _ in 0..n {
            let mut rng = rand::thread_rng();
            let mut file = random_string(&mut rng, 10);
            file.push_str(".tres");
            let mut res = Gd::<TestGodotResource>::default();
            res.set_path(format!("{path}{file}").into());
            saver.save(res.clone().upcast());
            obj.insert(file, res);
        }
        obj
    }
}

#[godot_api]
impl IResource for TestGodotResource {
    fn init(_base: godot::obj::Base<Self::Base>) -> Self {
        Self::new()
    }
}

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource, init)]
pub(crate) struct WithExtGd {
    #[export]
    #[serde(with = "gd_props::serde_gd::ext")]
    pub first: Gd<TestResource>,
    #[export]
    #[serde(with = "gd_props::serde_gd::ext_option")]
    pub second: Option<Gd<TestGodotResource>>,
}

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource, init)]
pub(crate) struct WithExtResVec {
    #[serde(with = "gd_props::serde_gd::ext_resvec")]
    pub vec: GdResVec<TestGodotResource>,
}

#[derive(GodotClass, Serialize, Deserialize, GdProp)]
#[class(base=Resource, init)]
pub(crate) struct WithExtHashMap {
    #[serde(with = "gd_props::serde_gd::ext_hashmap")]
    pub map: HashMap<String, Gd<TestGodotResource>>,
}
