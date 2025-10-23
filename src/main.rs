use todo_manager::todo_controller::TodoController;

fn main() {
    let mut controller = TodoController::new();
    controller.run();
}
