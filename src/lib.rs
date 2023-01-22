use std::ops::{Deref, DerefMut};

/// Usage of lateinit:
///```
/// let lateinit = Lateinit::<u8>::new();
/// lateinit.init(0);
/// println!("Lateinit containts: {}", lateinit);
/// ```
/// This panics(usage without initializing):
/// ```should_panic
/// let lateinit = Lateinit::<u8>::new();
/// println!("Lateinit contains: {}", lateinit);
/// ```
pub struct Lateinit<T>(Option<T>);

impl<T> Lateinit<T> {
    /// Creates new instance of [Lateinit]
    pub fn new() -> Self {
        Lateinit(None)
    }
    /// Initializes [Lateinit]
    pub fn init(&mut self, value: T) {
        self.0 = Some(value)
    }
    /// Checks if [Lateinit] initialized
    pub fn is_initialized(&self) -> bool {
        self.0.is_some()
    } 
    /// Returs value inside [Lateinit]
    pub fn get_value(self) -> T {
        self.0.unwrap()
    }
}

impl<T> Deref for Lateinit<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<T> DerefMut for Lateinit<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}


/// Lateinit without initialization checking, so dereferencing it without initialization leads to UB.
/// 
/// Undefined behavior code example:
/// ```no_run
/// let lateinit = Lateinit_Unchecked::<u8>();
/// println!("Lateinit value: {}", lateinit);
/// ```
pub struct LateinitUnchecked<T>(Option<T>);

impl<T> LateinitUnchecked<T> {
    /// Creates new instance of [LateinitUnchecked]
    pub fn new() -> Self {
        LateinitUnchecked(None)
    }
    /// Initializes [LateinitUnchecked]
    pub fn init(&mut self, value: T) {
        self.0 = Some(value)
    }
    /// Checks if [LateinitUnchecked] initialized
    pub fn is_initialized(&self) -> bool {
        self.0.is_some()
    }
    /// Returns value inside [LateinitUnchecked]
    pub fn get_value(self) -> T {
        unsafe {
            self.0.unwrap_unchecked()
        }
    }
}

impl<T> Deref for LateinitUnchecked<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        #[warn(unused_unsafe)]
        unsafe {
            self.0.as_ref().unwrap_unchecked()
        }
    }
}

impl<T> DerefMut for LateinitUnchecked<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        #[warn(unused_unsafe)]
        unsafe {
            self.0.as_mut().unwrap_unchecked()
        }
    }
}
