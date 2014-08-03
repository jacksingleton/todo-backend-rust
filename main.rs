use std::collections::HashMap;

//#[test]
//#[should_fail]
//fn create_and_read_todo() {
//    // Given
//    let mut repo = TodoRepo::new();
//    let todo = "I am a new todo";
//
//    // When
//    let todo_id = { repo.create(todo) };
//    let read_todo = repo.find(todo_id);
//
//    // Then
//    assert_eq!(read_todo, &todo);
//}


#[test]
fn create_should_add_todo_to_store() {
    // Given
    let mut repo = TodoRepo::new();
    let new_todo = "new todo";

    // When
    repo.create(new_todo);

    // Then
    assert_eq!(repo.store.len(), 1);
}

#[test]
fn find_should_find_an_existing_todo() {
    // Given
    let mut repo = TodoRepo::new();
    let todo_key = "todo-key";
    let todo = "existing todo";

    repo.store.insert(todo_key, todo);

    // When
    let found_todo = repo.find(todo_key);

    // Then
    assert_eq!(found_todo, &todo);
}

struct TodoRepo<'r> {
    store: HashMap<&'r str, &'r str>
}

impl <'r> TodoRepo<'r> {
    fn new<'r>() -> TodoRepo<'r> {
        TodoRepo {
            store: HashMap::new()
        }
    }

    fn create(&mut self, todo: &'r str) -> &str {
        let id = "some-id";
        self.store.insert(id, todo);
        id
    }

    fn find(&self, id: &'r str) -> &&'r str {
        self.store.find(&id).unwrap()
    }
}

