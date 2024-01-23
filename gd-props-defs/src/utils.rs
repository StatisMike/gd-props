use godot::{
    engine::{Engine, RefCounted},
    obj::{bounds::MemRefCounted, cap::GodotDefault, Bounds, Gd, Inherits},
    register::GodotClass,
};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct RefCountedSingletonWrapper {
    obj: Gd<RefCounted>,
}

impl RefCountedSingletonWrapper {
    pub fn new(obj: Gd<RefCounted>) -> Self {
        Self { obj }
    }
    pub fn clone_inner(&self) -> Gd<RefCounted> {
        self.obj.clone()
    }
}

pub trait RefCountedSingleton
where
    Self: Inherits<RefCounted> + Bounds<Memory = MemRefCounted> + GodotDefault,
{
    const SINGLETON_NAME: &'static str;

    fn singleton_refcount() -> Gd<Self> {
        let mut engine = Engine::singleton();
        // Need to check explicitly to not cause Godot error.
        let engine_has_singleton = engine.has_singleton(Self::SINGLETON_NAME.into());

        if engine_has_singleton {
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .unwrap()
                .cast::<RefCountedSingletonWrapper>()
                .bind()
                .clone_inner()
                .cast()
        } else {
            let object = Gd::<Self>::default();
            let wrapper = Gd::from_object(RefCountedSingletonWrapper::new(object.clone().upcast()));
            engine.register_singleton(Self::SINGLETON_NAME.into(), wrapper.upcast());
            object
        }
    }

    fn free_singleton() {
        let mut engine = Engine::singleton();

        let engine_has_singleton = engine.has_singleton(Self::SINGLETON_NAME.into());

        if !engine_has_singleton {
            return;
        }

        let obj = engine.get_singleton(Self::SINGLETON_NAME.into()).unwrap();

        engine.unregister_singleton(Self::SINGLETON_NAME.into());
        obj.free();
    }
}
