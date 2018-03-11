use std::rc::Rc;
use std::cell::RefCell;
use output;
use output::Output;
use rule::Rule;

pub struct Mock<I, O> {
    calls: Rc<RefCell<Vec<I>>>,
    rules: Rc<RefCell<Vec<Rule<I, Output<O>>>>>,
    default: O,
}

impl<I, O: Clone> Clone for Mock<I, O> {
    fn clone(&self) -> Mock<I, O> {
        Mock {
            calls: self.calls.clone(),
            rules: self.rules.clone(),
            default: self.default.clone(),
        }
    }
}

impl<I: PartialEq, O: Clone> Mock<I, O> {
    pub fn new(default: O) -> Mock<I, O> {
        Mock {
            calls: Rc::new(RefCell::new(Vec::new())),
            rules: Rc::new(RefCell::new(Vec::new())),
            default: default,
        }
    }

    pub fn given(&self, input: I) -> Output<O> {
        let return_value = Rc::new(RefCell::new(self.default.clone()));
        self.rules
            .borrow_mut()
            .push(Rule::new(input, Output::new(return_value.clone())));
        Output::new(return_value)
    }

    pub fn called(&self, input: I) -> O {
        for value in &*self.rules.borrow() {
            if &value.input == &input {
                self.calls.borrow_mut().push(input);
                return output::value_of(value.output.clone());
            }
        }

        self.calls.borrow_mut().push(input);
        return self.default.clone();
    }

    pub fn was_called_with(&self, input: I) -> bool {
        for value in &*self.calls.borrow() {
            if value == &input {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    trait ATrait {
        fn int_to_string(&self, input: i64) -> String;
    }

    #[derive(Clone)]
    struct MyMock {
        int_to_string: Mock<i64, String>,
    }

    impl MyMock {
        fn new() -> MyMock {
            MyMock {
                int_to_string: Mock::new(String::new()),
            }
        }
    }

    impl ATrait for MyMock {
        fn int_to_string(&self, input: i64) -> String {
            self.int_to_string.called(input)
        }
    }

    #[test]
    fn given_mock_with_rules_when_call_it_then_it_should_respect_those_rules() {
        let mock = MyMock::new();
        mock.int_to_string.given(65).will_return(String::from("65"));
        mock.int_to_string.given(63).will_return(String::from("63"));
        mock.int_to_string.given(-1).will_return(String::from("-1"));
        let a_trait = Box::new(mock);

        assert_eq!("65", a_trait.int_to_string(65));
        assert_eq!("63", a_trait.int_to_string(63));
        assert_eq!("-1", a_trait.int_to_string(-1));
        assert_eq!("", a_trait.int_to_string(0));
    }

    #[test]
    fn given_mock_called_with_values_when_was_called_with_then_is_true_for_those_values_false_otherwise(
) {
        let mock = MyMock::new();
        let a_trait = Box::new(mock.clone());

        a_trait.int_to_string(65);
        a_trait.int_to_string(63);
        a_trait.int_to_string(-1);

        assert!(mock.int_to_string.was_called_with(65));
        assert!(mock.int_to_string.was_called_with(63));
        assert!(mock.int_to_string.was_called_with(-1));
        assert!(!mock.int_to_string.was_called_with(0));
    }
}
