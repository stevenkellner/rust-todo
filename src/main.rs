use todo_manager::controller::TodoController;

fn main() {
    let mut controller = TodoController::new();
    controller.run();
}
