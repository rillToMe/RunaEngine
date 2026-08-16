use std::collections::HashMap;

use winit::{
    event::MouseButton,
    keyboard::KeyCode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    ZoomIn,
    ZoomOut,

    RotateLeft,
    RotateRight,

    PanCamera,

    Jump,
    Interact,
    Attack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Binding {
    Key(KeyCode),
    Mouse(MouseButton),
}

pub struct ActionMap {
    bindings: HashMap<Action, Vec<Binding>>,
}

impl ActionMap {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();

        bindings.insert(
            Action::MoveUp,
            vec![
                Binding::Key(KeyCode::KeyW),
                Binding::Key(KeyCode::ArrowUp),
            ],
        );

        bindings.insert(
            Action::MoveDown,
            vec![
                Binding::Key(KeyCode::KeyS),
                Binding::Key(KeyCode::ArrowDown),
            ],
        );

        bindings.insert(
            Action::MoveLeft,
            vec![
                Binding::Key(KeyCode::KeyA),
                Binding::Key(KeyCode::ArrowLeft),
            ],
        );

        bindings.insert(
            Action::MoveRight,
            vec![
                Binding::Key(KeyCode::KeyD),
                Binding::Key(KeyCode::ArrowRight),
            ],
        );

        bindings.insert(
            Action::Jump,
            vec![
                Binding::Key(KeyCode::Space),
            ],
        );

        bindings.insert(
            Action::Interact,
            vec![
                Binding::Key(KeyCode::KeyE),
            ],
        );

        bindings.insert(
            Action::Attack,
            vec![
                Binding::Mouse(MouseButton::Left),
            ],
        );

        bindings.insert(
            Action::ZoomIn,
            vec![
                Binding::Key(KeyCode::KeyE),
            ],
        );

        bindings.insert(
            Action::ZoomOut,
            vec![
                Binding::Key(KeyCode::KeyQ),
            ],
        );

        bindings.insert(
            Action::RotateLeft,
            vec![
                Binding::Key(KeyCode::KeyZ),
            ],
        );

        bindings.insert(
            Action::RotateRight,
            vec![
                Binding::Key(KeyCode::KeyX),
            ],
        );

        bindings.insert(
            Action::PanCamera, 
            vec![
                Binding::Mouse(MouseButton::Middle),
            ],
        );
            
        Self {
            bindings,
        }
    }

    pub fn bindings(
        &self,
        action: Action,
    ) -> &[Binding] {
        self.bindings
            .get(&action)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn set_bindings(
        &mut self,
        action: Action,
        bindings: Vec<Binding>,
    ) {
        self.bindings.insert(
            action,
            bindings,
        );
    }

    pub fn set_binding(
        &mut self,
        action: Action,
        binding: Binding,
    ) {
        self.bindings.insert(
            action,
            vec![binding],
        );
    }

    pub fn clear_bindings(
        &mut self,
        action: Action,
    ) {
        self.bindings.remove(&action);
    }
}