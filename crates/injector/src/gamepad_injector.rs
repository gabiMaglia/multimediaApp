use anyhow::{Result, Context};
use log::{info, warn, error, debug};
use vigem_client::{Client, TargetId, Xbox360Wired};
use input_core::InputEvent;

use crate::injector::InputInjector;
use crate::event_translator;

/// Injects gamepad input via ViGEm Bus Driver as a virtual Xbox 360 controller.
/// Steam reads it natively as a real XInput gamepad.
pub struct GamepadInjector {
    target: Option<Xbox360Wired<Client>>,
    gamepad_state: vigem_client::XGamepad,
    connected: bool,
}

impl GamepadInjector {
    pub fn new() -> Self {
        Self {
            target: None,
            gamepad_state: vigem_client::XGamepad::default(),
            connected: false,
        }
    }
}

impl InputInjector for GamepadInjector {
    fn connect(&mut self) -> Result<()> {
        info!("[gamepad_injector] Connecting to ViGEm Bus Driver...");

        let client = Client::connect()
            .context("Failed to connect to ViGEm Bus Driver. Is it installed?")?;
        info!("[gamepad_injector] Connected to ViGEm Bus Driver");

        let id = TargetId::XBOX360_WIRED;
        let mut target = Xbox360Wired::new(client, id);

        target.plugin()
            .context("Failed to plug in virtual Xbox 360 controller")?;
        target.wait_ready()
            .context("Virtual controller not ready")?;

        info!("[gamepad_injector] Virtual Xbox 360 controller plugged in");

        // Store client from target's internal ref won't work; restructure
        self.target = Some(target);
        self.connected = true;
        Ok(())
    }

    fn inject(&mut self, event: &InputEvent) -> Result<()> {
        let target = self.target.as_mut()
            .context("GamepadInjector not connected")?;

        // Translate the InputEvent into gamepad state changes
        event_translator::apply_gamepad_event(&mut self.gamepad_state, event);

        // Submit entire gamepad state to the virtual controller
        let _ = target.update(&self.gamepad_state)
            .map_err(|e| {
                warn!("[gamepad_injector] Failed to update gamepad state: {:?}", e);
                e
            });

        debug!(
            "[gamepad_injector] Updated virtual gamepad: buttons={:#06x}",
            self.gamepad_state.buttons.raw
        );
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn disconnect(&mut self) -> Result<()> {
        if let Some(mut target) = self.target.take() {
            target.unplug()
                .map_err(|e| anyhow::anyhow!("Failed to unplug virtual controller: {:?}", e))?;
            info!("[gamepad_injector] Virtual Xbox 360 controller unplugged");
        }
        self.connected = false;
        Ok(())
    }
}

impl Drop for GamepadInjector {
    fn drop(&mut self) {
        if self.connected {
            if let Err(e) = self.disconnect() {
                error!("[gamepad_injector] Error during cleanup: {}", e);
            }
        }
    }
}

/// Starts a background thread that reads InputEvents from a channel and injects them
/// as a virtual Xbox 360 controller via ViGEm.
pub fn start_gamepad_injector(
    rx: std::sync::mpsc::Receiver<InputEvent>,
) -> Result<()> {
    let mut injector = GamepadInjector::new();
    injector.connect()?;

    std::thread::spawn(move || {
        info!("[gamepad_injector] Injector thread started");

        while let Ok(event) = rx.recv() {
            if !injector.is_connected() {
                warn!("[gamepad_injector] Lost connection, attempting reconnect...");
                match injector.connect() {
                    Ok(_) => info!("[gamepad_injector] Reconnected"),
                    Err(e) => {
                        error!("[gamepad_injector] Reconnection failed: {}", e);
                        continue;
                    }
                }
            }

            if let Err(e) = injector.inject(&event) {
                warn!("[gamepad_injector] Injection error: {}", e);
            }
        }

        warn!("[gamepad_injector] Channel closed, shutting down");
    });

    Ok(())
}
