use todo_manager::controller::TodoManager;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let input_stream = todo_manager::ui::input::FileInputStream::new(std::io::stdin());
    let output_writer = todo_manager::ui::output::FileOutputWriter::new(std::io::stdout());
    let mut manager = TodoManager::new(
        Rc::new(RefCell::new(input_stream)),
        Rc::new(RefCell::new(output_writer))
    );
    manager.run();
}
