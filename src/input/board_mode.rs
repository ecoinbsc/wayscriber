//! Board/canvas mode selection.

use crate::draw::Color;

/// Board rendering mode
///
/// Determines the background and visual style of the drawing canvas.
/// Each mode maintains its own isolated frame of shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoardMode {
    /// Transparent overlay showing underlying screen (default)
    Transparent,
    /// White/light background for drawing (whiteboard)
    Whiteboard,
    /// Dark/black background for drawing (blackboard)
    Blackboard,
}

impl Default for BoardMode {
    fn default() -> Self {
        Self::Transparent
    }
}

impl BoardMode {
    /// Returns the background color for this mode, if any.
    ///
    /// Transparent mode returns None (no background fill).
    /// Whiteboard and Blackboard return their respective colors.
    pub fn background_color(&self) -> Option<Color> {
        match self {
            Self::Transparent => None,
            Self::Whiteboard => Some(Color {
                r: 0.992,
                g: 0.992,
                b: 0.992,
                a: 1.0,
            }), // Off-white #FDFDFD
            Self::Blackboard => Some(Color {
                r: 0.067,
                g: 0.067,
                b: 0.067,
                a: 1.0,
            }), // Near-black #111111
        }
    }

    /// Returns the default pen color for this mode.
    ///
    /// Used for auto-adjusting pen color when entering board modes
    /// to ensure good contrast.
    pub fn default_pen_color(&self) -> Option<Color> {
        match self {
            Self::Transparent => None, // No default change for transparent
            Self::Whiteboard => Some(Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }), // Black
            Self::Blackboard => Some(Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }), // White
        }
    }

    /// Returns the display name for status bar, if any.
    ///
    /// Transparent mode returns None (no badge displayed).
    pub fn display_name(&self) -> Option<&'static str> {
        match self {
            Self::Transparent => None,
            Self::Whiteboard => Some("WHITEBOARD"),
            Self::Blackboard => Some("BLACKBOARD"),
        }
    }

    /// Returns true if this is a board mode (not transparent).
    pub fn is_board_mode(&self) -> bool {
        !matches!(self, Self::Transparent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_transparent() {
        assert_eq!(BoardMode::default(), BoardMode::Transparent);
    }

    #[test]
    fn test_background_color() {
        assert_eq!(BoardMode::Transparent.background_color(), None);
        assert!(BoardMode::Whiteboard.background_color().is_some());
        assert!(BoardMode::Blackboard.background_color().is_some());

        // Verify specific colors
        let white_bg = BoardMode::Whiteboard.background_color().unwrap();
        assert!((white_bg.r - 0.992).abs() < 0.001);
        assert_eq!(white_bg.a, 1.0);

        let black_bg = BoardMode::Blackboard.background_color().unwrap();
        assert!((black_bg.r - 0.067).abs() < 0.001);
        assert_eq!(black_bg.a, 1.0);
    }

    #[test]
    fn test_default_pen_color() {
        assert_eq!(BoardMode::Transparent.default_pen_color(), None);
        assert!(BoardMode::Whiteboard.default_pen_color().is_some());
        assert!(BoardMode::Blackboard.default_pen_color().is_some());

        // Whiteboard should default to black pen
        let white_pen = BoardMode::Whiteboard.default_pen_color().unwrap();
        assert_eq!(white_pen.r, 0.0);
        assert_eq!(white_pen.g, 0.0);
        assert_eq!(white_pen.b, 0.0);

        // Blackboard should default to white pen
        let black_pen = BoardMode::Blackboard.default_pen_color().unwrap();
        assert_eq!(black_pen.r, 1.0);
        assert_eq!(black_pen.g, 1.0);
        assert_eq!(black_pen.b, 1.0);
    }

    #[test]
    fn test_display_name() {
        assert_eq!(BoardMode::Transparent.display_name(), None);
        assert_eq!(BoardMode::Whiteboard.display_name(), Some("WHITEBOARD"));
        assert_eq!(BoardMode::Blackboard.display_name(), Some("BLACKBOARD"));
    }

    #[test]
    fn test_is_board_mode() {
        assert!(!BoardMode::Transparent.is_board_mode());
        assert!(BoardMode::Whiteboard.is_board_mode());
        assert!(BoardMode::Blackboard.is_board_mode());
    }
}
