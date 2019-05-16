use mockiato::mockable;

#[mockable]
trait Greeter: GreeterClone {
    fn greet(&self, name: &str) -> String;
}

trait GreeterClone {
    fn clone_box<'a>(&self) -> Box<dyn Greeter + 'a>
    where
        Self: 'a;
}

impl<T> GreeterClone for T
where
    T: Greeter + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn Greeter + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

#[test]
fn cloneable_mocks_work() {
    let name = String::from("Tom");
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(|arg| arg.partial_eq(&name))
        .times(2)
        .returns(String::from("Hello Tom"));

    assert_eq!("Hello Tom", greeter.greet("Tom"));

    {
        let greeter_clone = greeter.clone_box();

        assert_eq!("Hello Tom", greeter_clone.greet("Tom"));
    }

    assert_eq!("Hello Tom", greeter.greet("Tom"));
}
