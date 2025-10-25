use todo_manager::controller::TodoManager;

fn main() {
    let mut manager = TodoManager::with_stdio_default();
    manager.run();
}
