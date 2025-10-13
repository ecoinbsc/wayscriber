use anyhow::Result;

pub mod wayland;

// Removed: Backend trait - no longer needed with single backend
// Removed: BackendChoice enum - Wayland is the only backend

/// Run Wayland backend with full event loop
pub fn run_wayland() -> Result<()> {
    let mut backend = wayland::WaylandBackend::new()?;
    backend.init()?;
    backend.show()?; // show() calls run() internally
    backend.hide()?;
    Ok(())
}
