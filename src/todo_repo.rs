extern crate uuid;

use std::collections::HashMap;
use std::rand;

use self::uuid::Uuid;

#[test]
fn create_and_read_multiple_todos() {
    // Given
    let mut repo = TodoRepo::new();
    let new_todo_1 = "new-todo-1";
    let new_todo_2 = "new-todo-2";

    // When
    let todo_id_1 = repo.create(new_todo_1);
    let todo_id_2 = repo.create(new_todo_2);

    let read_todo_1 = repo.find(todo_id_1.as_slice());
    let read_todo_2 = repo.find(todo_id_1.as_slice());

    // ??? Why does this not compile?
    // error: cannot borrow `repo` as mutable because it is also borrowed as immutable
    //repo.create("blah");

    // Then
    assert_eq!(read_todo_1.as_slice(), new_todo_1);
    assert_eq!(read_todo_2.as_slice(), new_todo_2);
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
    repo.store.insert(String::from_str(todo_key), String::from_str(todo));

    // When
    let found_todo = repo.find(todo_key);

    // Then
    assert_eq!(found_todo, String::from_str(todo));
}


pub struct TodoRepo<'r> {
    store: HashMap<String, String>
}

impl <'r> TodoRepo<'r> {
    fn new<'r>() -> TodoRepo<'r> {
        TodoRepo {
            store: HashMap::new()
        }
    }

    fn create(&mut self, todo: &str) -> String {
        //let id = Uuid::new_v4().to_string().as_slice();
        let id = "dummy-id";
        self.store.insert(String::from_str(id), String::from_str(todo));
        String::from_str(id)
    }

    fn find(&'r self, id: &str) -> String {
        self.store.find(&String::from_str(id)).unwrap()
    }
}

