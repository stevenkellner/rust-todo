use todo_manager::controller::TodoManager;

fn main() {
    let mut manager = TodoManager::new();
    manager.run();
}
