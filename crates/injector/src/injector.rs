use anyhow::Result;
use input_core::InputEvent;

/// Common trait for all input injectors.
pub trait InputInjector: Send {
    /// Initialize the injector (connect to driver, find window, etc.)
    fn connect(&mut self) -> Result<()>;

    /// Inject a single input event.
    fn inject(&mut self, event: &InputEvent) -> Result<()>;

    /// Check if the injector is still connected / functional.
    fn is_connected(&self) -> bool;

    /// Disconnect and clean up resources.
    fn disconnect(&mut self) -> Result<()>;
}
