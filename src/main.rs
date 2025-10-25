use todo_manager::controller::ApplicationController;
use todo_manager::{run_tui, App, TodoListStorage};
use std::env;

fn main() {
    // Check for --tui flag
    let args: Vec<String> = env::args().collect();
    let use_tui = args.contains(&"--tui".to_string());

    if use_tui {
        // Run TUI mode
        let storage = TodoListStorage::new("tasks.json");
        let todo_list = storage.load().unwrap_or_default();
        let app = App::new(todo_list);
        
        match run_tui(app) {
            Ok(final_app) => {
                // Save the todo list before exiting
                if let Err(e) = storage.save(&final_app.todo_list) {
                    eprintln!("Failed to save tasks: {}", e);
                }
            }
            Err(e) => {
                eprintln!("TUI Error: {}", e);
            }
        }
    } else {
        // Run normal CLI mode
        let mut controller = ApplicationController::with_stdio_default();
        controller.run();
    }
}

