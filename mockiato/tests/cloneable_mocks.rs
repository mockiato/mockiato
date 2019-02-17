use mockiato::{mockable, partial_eq};

#[mockable(static)]
trait Greeter: GreeterClone {
    fn greet(&self, name: &str) -> String;
}

trait GreeterClone {
    fn clone_box(&self) -> Box<dyn Greeter>;
}

impl<T> GreeterClone for T
where
    T: Greeter + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn Greeter> {
        Box::new(self.clone())
    }
}

#[test]
fn cloneable_mocks_work() {
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(partial_eq("Tom"))
        .times(2)
        .returns(String::from("Hello Tom"));

    assert_eq!("Hello Tom", greeter.greet("Tom"));

    {
        let greeter_clone = greeter.clone_box();

        assert_eq!("Hello Tom", greeter_clone.greet("Tom"));
    }

    assert_eq!("Hello Tom", greeter.greet("Tom"));
}
