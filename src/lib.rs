use parking_lot::{
    lock_api::{ArcMutexGuard, Mutex},
    RawMutex,
};
use std::{
    any::Any,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    panic::Location,
    sync::Arc,
};

use halfbrown::HashMap as Map;

pub struct Wrapper<T> {
    artex_guard: ArcMutexGuard<RawMutex, Box<dyn Any>>,
    _ty: PhantomData<T>,
}
impl<T: 'static> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.artex_guard.deref().downcast_ref().unwrap()
    }
}
impl<T: 'static> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.artex_guard.deref_mut().downcast_mut().unwrap()
    }
}

pub struct State {
    #[allow(clippy::type_complexity)]
    stuff: Map<&'static Location<'static>, Arc<Mutex<RawMutex, Box<dyn Any>>>>,
}
impl State {
    pub fn empty() -> Self {
        Self { stuff: Map::new() }
    }
    #[track_caller]
    pub fn preserve<T: 'static>(&mut self, init: impl FnOnce() -> T) -> Wrapper<T> {
        //dbg!(std::panic::Location::caller());
        let entry = self.stuff.entry(std::panic::Location::caller());

        let val = entry.or_insert_with(|| Arc::new(Mutex::new(Box::new(init()))));
        let r = Mutex::lock_arc(&val.clone());
        Wrapper {
            artex_guard: r,
            _ty: PhantomData::<T>,
        }
    }
}
