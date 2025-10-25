use todo_manager::controller::ApplicationController;

fn main() {
    let mut controller = ApplicationController::with_stdio_default();
    controller.run();
}
