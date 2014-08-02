use std::collections::HashMap;

#[test]
#[should_fail]
fn create_and_read_todo() {
    // Given
    let repo = TodoRepo::new();
    let todo = "I am a new todo";

    // When
    let todo_id = repo.create(todo);
    let read_todo = repo.find(todo_id);

    // Then
    assert_eq!(todo, read_todo);
}


#[test]
fn create_should_add_todo_to_global() {
    // Given
    let repo = TodoRepo::new();
    let new_todo = "new todo";

    // When
    repo.create(new_todo);

    // Then
    assert_eq!(repo.store.len(), 1);
}

struct TodoRepo<'r> {
    store: HashMap<&'r str, &'r str>
}

impl <'r> TodoRepo<'r> {
    //let mut map = HashMap::new();
    //map.insert("some-id", new_todo);
    //let retrieved_todo = map.find(&"some-id").unwrap();

    fn new<'r>() -> TodoRepo<'r> {
        TodoRepo {
            store: HashMap::new()
        }
    }

    fn create(&self, todo: &str) -> &str {
        "blah"
    }

    fn find(&self, id: &str) -> &str {
        "blah"
    }
}

