// In order to downcast from a trait object to a concrete implementation, a trait needs an `as_any` method.
// [Source](https://stackoverflow.com/a/33687996/6827589)
// This is an issue, because there's no way to sensibly mock this method with mockiato.
// The solution is to split the `as_any` method into a separate trait.

#[cfg(test)]
use mockiato::mockable;
use std::any::Any;

// `static_references` is required, because the `Any` trait requires `'static`.
#[cfg_attr(test, mockable(static_references))]
trait ObjectBehavior: ObjectBehaviorAsAny {
    fn step(&mut self);
}

trait ObjectBehaviorAsAny {
    fn as_any(&self) -> &'_ dyn Any;
}

// We can then automatically implement the `AsAny` trait:
impl<T> ObjectBehaviorAsAny for T
where
    T: ObjectBehavior + 'static,
{
    fn as_any(&self) -> &'_ dyn Any {
        self
    }
}

#[test]
fn object_behavior_can_be_downcast() {
    let behavior: Box<dyn ObjectBehavior> = Box::new(ObjectBehaviorMock::new());
    let concrete_behavior: &ObjectBehaviorMock = behavior.as_any().downcast_ref().unwrap();
    dbg!(&concrete_behavior);
}
