//! Input handling and tool state machine.
//!
//! This module translates backend keyboard and mouse events into drawing actions.
//! It maintains the current tool state, drawing parameters (color, thickness),
//! and manages the state machine for different drawing modes (idle, drawing, text input).

pub mod events;
pub mod modifiers;
pub mod state;
pub mod tool;

// Re-export commonly used types at module level
pub use events::{Key, MouseButton};
pub use state::{DrawingState, InputState};
pub use tool::Tool;

// Re-export for public API (unused internally but part of public interface)
#[allow(unused_imports)]
pub use modifiers::Modifiers;
