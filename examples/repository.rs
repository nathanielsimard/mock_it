extern crate mock_it;
use mock_it::mock::Mock;

#[derive(PartialEq)]
struct Person {
    name: String,
    age: i64,
}

impl Person {
    fn new(name: &str, age: i64) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

trait Repository {
    fn create(&self, person: Person) -> Option<String>;
}

struct Service {
    repository: Box<Repository>,
}

impl Service {
    fn new(repository: Box<Repository>) -> Service {
        Service {
            repository: repository,
        }
    }

    fn create(&self, name: &str, age: i64) -> Result<(), String> {
        if age <= 0 {
            Err("Age invalid".to_string())
        } else if name == String::from("") {
            Err("Empty name".to_string())
        } else {
            self.repository.create(Person::new(&name, age));
            Ok(())
        }
    }
}

struct RepositoryMock {
    create: Mock<Person, Option<String>>,
}

impl RepositoryMock {
    fn new() -> RepositoryMock {
        RepositoryMock {
            create: Mock::new(),
        }
    }
}

impl Repository for RepositoryMock {
    fn create(&self, person: Person) -> Option<String> {
        self.create
            .called_with(person)
            .return_clonable_with_default(None)
    }
}

fn main() {
    let a_valid_person = Person::new("John", 27);
    let mut repository_mock = RepositoryMock::new();
    repository_mock
        .create
        .given(a_valid_person)
        .return_value_cloned(None);
    let service = Service::new(Box::new(repository_mock));

    let result = service.create("John", 27);

    assert_eq!(Ok(()), result);
}