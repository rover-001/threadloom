use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static CONTEXT_STACK: RefCell<Vec<HashMap<TypeId, Rc<dyn Any>>>> = RefCell::new(vec![HashMap::new()]);
}
pub fn provide_context<T: 'static>(value: T) {
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if let Some(frame) = stack.last_mut() {
            frame.insert(TypeId::of::<T>(), Rc::new(value));
        }
    });
}

pub fn use_context<T: Clone + 'static>() -> Option<T> {
    CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for frame in stack.iter().rev() {
            if let Some(val) = frame.get(&TypeId::of::<T>()) {
                if let Some(typed_val) = val.downcast_ref::<T>() {
                    return Some(typed_val.clone());
                }
            }
        }
        None
    })
}

pub fn with_context_frame<R>(f: impl FnOnce() -> R) -> R {
    CONTEXT_STACK.with(|stack| stack.borrow_mut().push(HashMap::new()));
    let result = f();
    CONTEXT_STACK.with(|stack| stack.borrow_mut().pop());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Debug)]
    struct Theme(String);

    #[derive(Clone, PartialEq, Debug)]
    struct UserId(u64);

    #[test]
    fn test_provide_and_use_context() {
        provide_context(Theme("dark".to_string()));
        provide_context(UserId(42));

        assert_eq!(use_context::<Theme>(), Some(Theme("dark".to_string())));
        assert_eq!(use_context::<UserId>(), Some(UserId(42)));
        assert_eq!(use_context::<String>(), None);
    }

    #[test]
    fn test_context_scoping_and_shadowing() {
        provide_context(Theme("light".to_string()));

        with_context_frame(|| {
            assert_eq!(use_context::<Theme>(), Some(Theme("light".to_string())));
            // Shadow with new value in nested frame
            provide_context(Theme("solarized".to_string()));
            assert_eq!(use_context::<Theme>(), Some(Theme("solarized".to_string())));
        });

        // Outer scope is restored
        assert_eq!(use_context::<Theme>(), Some(Theme("light".to_string())));
    }
}
