//! Simple dependency injection container.
//!
//! The `ServiceContainer` allows you to register and resolve services by type.
//! It uses Rust’s `TypeId` to look up concrete implementations.  This design
//! mirrors the dependency injection patterns described in the
//! [ARCHITECTURE_ENHANCED](https://github.com/christophermanahan/paradiddle/blob/main/docs/architecture/rust-ide-plans.md)
//! document and provides a foundation for more advanced service registries later on【6955392274892†L521-L533】.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A simple dependency injection container.
#[derive(Default)]
pub struct ServiceContainer {
    services: RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceContainer {
    /// Create a new, empty container.
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    /// Register a service of type `T`.
    ///
    /// The service must be `Send` and `Sync` so it can be shared safely across
    /// threads.
    pub fn register<T: Any + Send + Sync>(&self, service: T) {
        let mut services = self.services.write().expect("container lock poisoned");
        services.insert(TypeId::of::<T>(), Arc::new(service));
    }

    /// Resolve a previously registered service of type `T`.
    /// Returns `Some(Arc<T>)` if found, otherwise `None`.
    pub fn resolve<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let services = self.services.read().expect("container lock poisoned");
        services
            .get(&TypeId::of::<T>())
            .and_then(|service| service.clone().downcast::<T>().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestService {
        value: i32,
    }

    #[derive(Debug, PartialEq)]
    struct AnotherService {
        name: String,
    }

    #[test]
    fn test_register_and_resolve() {
        let container = ServiceContainer::new();
        let service = TestService { value: 42 };

        container.register(service);

        let resolved = container.resolve::<TestService>();
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap().value, 42);
    }

    #[test]
    fn test_resolve_unregistered_returns_none() {
        let container = ServiceContainer::new();

        let resolved = container.resolve::<TestService>();
        assert!(resolved.is_none());
    }

    #[test]
    fn test_multiple_services() {
        let container = ServiceContainer::new();

        container.register(TestService { value: 100 });
        container.register(AnotherService {
            name: "hello".to_string(),
        });

        let test_service = container.resolve::<TestService>().unwrap();
        let another_service = container.resolve::<AnotherService>().unwrap();

        assert_eq!(test_service.value, 100);
        assert_eq!(another_service.name, "hello");
    }

    #[test]
    fn test_overwrite_service() {
        let container = ServiceContainer::new();

        container.register(TestService { value: 1 });
        container.register(TestService { value: 2 });

        let resolved = container.resolve::<TestService>().unwrap();
        assert_eq!(resolved.value, 2);
    }

    #[test]
    fn test_default_constructor() {
        let container = ServiceContainer::default();
        container.register(TestService { value: 99 });

        let resolved = container.resolve::<TestService>().unwrap();
        assert_eq!(resolved.value, 99);
    }

    #[test]
    fn test_thread_safety() {
        use std::thread;

        let container = Arc::new(ServiceContainer::new());
        container.register(TestService { value: 42 });

        let container_clone = Arc::clone(&container);
        let handle = thread::spawn(move || {
            let resolved = container_clone.resolve::<TestService>();
            assert!(resolved.is_some());
            assert_eq!(resolved.unwrap().value, 42);
        });

        handle.join().unwrap();
    }
}
