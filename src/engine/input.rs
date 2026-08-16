use std::collections::HashSet;

use winit::{
    event::{ElementState, MouseButton},
    keyboard::KeyCode,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Interact,
    Attack,
}

pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_released: HashSet<MouseButton>,

    mouse_position: [f32; 2],
    mouse_delta: [f32; 2],
    last_mouse_position: [f32; 2],
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),

            mouse_buttons_down: HashSet::new(),
            mouse_position: [0.0, 0.0],

            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(),

            mouse_delta: [0.0, 0.0],
            last_mouse_position: [0.0, 0.0],
        }
    }

    pub fn is_action_down(
        &self,
        action: Action,
    ) -> bool {
        match action {
            Action::MoveUp => {
                self.is_key_down(KeyCode::KeyW)
                    || self.is_key_down(KeyCode::ArrowUp)
            }

            Action::MoveDown => {
                self.is_key_down(KeyCode::KeyS)
                    || self.is_key_down(KeyCode::ArrowDown)
            }

            Action::MoveLeft => {
                self.is_key_down(KeyCode::KeyA)
                    || self.is_key_down(KeyCode::ArrowLeft)
            }

            Action::MoveRight => {
                self.is_key_down(KeyCode::KeyD)
                    || self.is_key_down(KeyCode::ArrowRight)
            }

            Action::Jump => {
                self.is_key_down(KeyCode::Space)
            }

            Action::Interact => {
                self.is_key_down(KeyCode::KeyE)
            }

            Action::Attack => {
                self.is_mouse_button_down(MouseButton::Left)
            }
        }
    }

    pub fn is_action_pressed(
        &self,
        action: Action,
    ) -> bool {
        match action {
            Action::MoveUp => {
                self.is_key_pressed(KeyCode::KeyW)
                    || self.is_key_pressed(KeyCode::ArrowUp)
            }

            Action::MoveDown => {
                self.is_key_pressed(KeyCode::KeyS)
                    || self.is_key_pressed(KeyCode::ArrowDown)
            }

            Action::MoveLeft => {
                self.is_key_pressed(KeyCode::KeyA)
                    || self.is_key_pressed(KeyCode::ArrowLeft)
            }

            Action::MoveRight => {
                self.is_key_pressed(KeyCode::KeyD)
                    || self.is_key_pressed(KeyCode::ArrowRight)
            }

            Action::Jump => {
                self.is_key_pressed(KeyCode::Space)
            }

            Action::Interact => {
                self.is_key_pressed(KeyCode::KeyE)
            }

            Action::Attack => {
                self.is_mouse_button_pressed(
                    MouseButton::Left
                )
            }
        }
    }

    pub fn is_action_released(
        &self,
        action: Action,
    ) -> bool {
        match action {
            Action::MoveUp => {
                self.is_key_released(KeyCode::KeyW)
                    || self.is_key_released(KeyCode::ArrowUp)
            }

            Action::MoveDown => {
                self.is_key_released(KeyCode::KeyS)
                    || self.is_key_released(KeyCode::ArrowDown)
            }

            Action::MoveLeft => {
                self.is_key_released(KeyCode::KeyA)
                    || self.is_key_released(KeyCode::ArrowLeft)
            }

            Action::MoveRight => {
                self.is_key_released(KeyCode::KeyD)
                    || self.is_key_released(KeyCode::ArrowRight)
            }

            Action::Jump => {
                self.is_key_released(KeyCode::Space)
            }

            Action::Interact => {
                self.is_key_released(KeyCode::KeyE)
            }

            Action::Attack => {
                self.is_mouse_button_released(
                    MouseButton::Left
                )
            }
        }
    }


    pub fn update_key(
        &mut self,
        key: KeyCode,
        state: ElementState,
    ) {
        match state {
            ElementState::Pressed => {
                if !self.keys_down.contains(&key) {
                    self.keys_pressed.insert(key);
                }

                self.keys_down.insert(key);
            }

            ElementState::Released => {
                self.keys_down.remove(&key);
                self.keys_released.insert(key);
            }
        }
    }

    pub fn is_key_down(
        &self,
        key: KeyCode,
    ) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(
        &self,
        key: KeyCode,
    ) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(
        &self,
        key: KeyCode,
    ) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn update_mouse_button(
        &mut self,
        button: MouseButton,
        state: ElementState,
    ) {
        match state {
            ElementState::Pressed => {
                if !self.mouse_buttons_down.contains(&button) {
                    self.mouse_buttons_pressed.insert(button);
                }

                self.mouse_buttons_down.insert(button);
            }

            ElementState::Released => {
                self.mouse_buttons_down.remove(&button);
                self.mouse_buttons_released.insert(button);
            }
        }
    }

    pub fn is_mouse_button_down(
        &self,
        button: MouseButton,
    ) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    pub fn set_mouse_position(
        &mut self,
        position: [f32; 2],
    ) {
        self.mouse_delta = [
            position[0] - self.mouse_position[0],
            position[1] - self.mouse_position[1],
        ];

        self.last_mouse_position =
            self.mouse_position;

        self.mouse_position = position;
    }

    pub fn is_mouse_button_pressed(
        &self,
        button: MouseButton,
    ) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn is_mouse_button_released(
        &self,
        button: MouseButton,
    ) -> bool {
        self.mouse_buttons_released.contains(&button)
    }

    pub fn mouse_delta(&self) -> [f32; 2] {
        self.mouse_delta
    }

    pub fn mouse_position(&self) -> [f32; 2] {
        self.mouse_position
    }


    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();

        self.mouse_delta = [0.0, 0.0];
    }
}