fn contains<P: std::cmp::PartialOrd + std::ops::Add<Output = P> + Copy>(
    base: &tauri::PhysicalPosition<P>,
    size: &tauri::PhysicalSize<P>,
    pos: &tauri::PhysicalPosition<P>,
) -> bool {
    base.x <= pos.x
        && pos.x <= base.x + size.width
        && base.y <= pos.y
        && pos.y <= base.y + size.height
}

#[cfg(test)]
mod tests {
    use super::contains;
    use tauri::{PhysicalPosition, PhysicalSize};

    #[test]
    fn test_contains_inside() {
        let base = PhysicalPosition { x: 0.0, y: 0.0 };
        let size = PhysicalSize {
            width: 100.0,
            height: 100.0,
        };
        let pos = PhysicalPosition { x: 50.0, y: 50.0 };
        assert!(contains(&base, &size, &pos));
    }

    #[test]
    fn test_contains_on_edge() {
        let base = PhysicalPosition { x: 0.0, y: 0.0 };
        let size = PhysicalSize {
            width: 100.0,
            height: 100.0,
        };
        let pos = PhysicalPosition { x: 100.0, y: 100.0 };
        assert!(contains(&base, &size, &pos));
    }

    #[test]
    fn test_contains_outside_x() {
        let base = PhysicalPosition { x: 0.0, y: 0.0 };
        let size = PhysicalSize {
            width: 100.0,
            height: 100.0,
        };
        let pos = PhysicalPosition { x: 101.0, y: 50.0 };
        assert!(!contains(&base, &size, &pos));
    }

    #[test]
    fn test_contains_outside_y() {
        let base = PhysicalPosition { x: 0.0, y: 0.0 };
        let size = PhysicalSize {
            width: 100.0,
            height: 100.0,
        };
        let pos = PhysicalPosition { x: 50.0, y: 101.0 };
        assert!(!contains(&base, &size, &pos));
    }

    #[test]
    fn test_contains_negative_position() {
        let base = PhysicalPosition { x: -50.0, y: -50.0 };
        let size = PhysicalSize {
            width: 100.0,
            height: 100.0,
        };
        let pos = PhysicalPosition { x: 0.0, y: 0.0 };
        assert!(contains(&base, &size, &pos));
    }

    #[test]
    fn test_contains_outside_negative() {
        let base = PhysicalPosition { x: -50.0, y: -50.0 };
        let size = PhysicalSize {
            width: 100.0,
            height: 100.0,
        };
        let pos = PhysicalPosition { x: 60.0, y: 0.0 };
        assert!(!contains(&base, &size, &pos));
    }
}

fn get_scale_factor(
    handler: &tauri::AppHandle,
    position: &tauri::PhysicalPosition<f64>,
) -> Option<f64> {
    match &handler.available_monitors() {
        Ok(monitors) => {
            for monitor in monitors {
                if contains(
                    &monitor.position().cast::<f64>(),
                    &monitor.size().cast::<f64>(),
                    &position,
                ) {
                    return Some(monitor.scale_factor());
                }
            }
            eprintln!("Error retrieving available monitors");
        }
        Err(_) => {
            eprintln!("Error retrieving available monitors");
        }
    };
    return None;
}

pub fn set_webview_windows_to_position(
    window: &tauri::webview::WebviewWindow,
    handler: &tauri::AppHandle,
    position: &tauri::PhysicalPosition<f64>,
) {
    match get_scale_factor(handler, position) {
        Some(scale_factor) => window.set_position(position.to_logical::<f64>(scale_factor)),
        None => window.set_position(position.clone()),
    }
    .unwrap_or_else(|e| {
        eprintln!("Error setting position for webview window: {}", e);
    });
}
