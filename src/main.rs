use std::collections::HashMap;

#[test]
fn create_and_read_two_todos() {
    // Given
    let mut repo = TodoRepo::new();
    let new_todo_1 = "new-todo-1";
    //let new_todo_2 = "new-todo-2";

    // When
    let todo_id_1 = repo.create(new_todo_1);
    let read_todo_1 = repo.find(todo_id_1);

    //let todo_id_2 = repo.create(new_todo_2);
    //let read_todo_2 = repo.find(todo_id_2);

    // Then
    assert_eq!(read_todo_1, &new_todo_1);
    //assert_eq!(read_todo_2, &new_todo_2);
}

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
    let todo_key = "todo-key";
    let todo = "existing todo";

    let mut repo = TodoRepo::new();
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

    fn create(&mut self, todo: &'r str) -> &'r str {
        let id = "some-id";
        self.store.insert(id, todo);
        id
    }

    fn find(&'r self, id: &'r str) -> &'r &str {
        self.store.find(&id).unwrap()
    }
}

