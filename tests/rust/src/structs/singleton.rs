use godot::engine::{Engine, Object};
use godot::obj::cap::GodotDefault;
use godot::obj::dom::UserDomain;
use godot::obj::mem::StaticRefCount;
use godot::obj::{Gd, Inherits};
use godot::prelude::GodotClass;

pub(crate) trait GodotSingleton
where
    Self: GodotClass<Declarer = UserDomain> + Inherits<Object> + GodotDefault<Mem = StaticRefCount>,
{
    const SINGLETON_NAME: &'static str;

    fn singleton_instance() -> Gd<Self>;

    fn singleton() -> Gd<Self> {
        let mut engine = Engine::singleton();
        if engine.has_singleton(Self::SINGLETON_NAME.into()) {
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .expect("no singleton found")
                .cast()
        } else {
            let object = Self::singleton_instance();
            engine.register_singleton(Self::SINGLETON_NAME.into(), object.clone().upcast());
            // TODO: Remove after https://github.com/godot-rust/gdext/issues/522 is fixed.
            std::mem::forget(object);
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .expect("no singleton found")
                .cast()
        }
    }
}
