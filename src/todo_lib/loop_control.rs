/// Represents the control flow decision for the main event loop.
///
/// This enum is used to control whether the application should continue
/// processing events or exit the main loop.
///
/// # Examples
///
/// ```
/// use todo_manager::loop_control::LoopControl;
///
/// let control = LoopControl::Continue;
/// assert_eq!(control, LoopControl::Continue);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum LoopControl {
    /// Continue processing events in the main loop
    Continue,
    /// Exit the event loop and terminate the application
    Exit,
}
