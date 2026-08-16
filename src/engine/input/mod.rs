pub mod action;

pub use action::{
    Action,
    ActionMap,
    Binding,
};

use std::collections::HashSet;

use winit::{
    event::{ElementState, MouseButton},
    keyboard::KeyCode,
};

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

    action_map: ActionMap,
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

            action_map: ActionMap::new(),
        }
    }

    pub fn is_action_down(
        &self,
        action: Action,
    ) -> bool {
        self.action_map
            .bindings(action)
            .iter()
            .any(|binding| {
                match binding {
                    Binding::Key(key) => {
                        self.is_key_down(*key)
                    }

                    Binding::Mouse(button) => {
                        self.is_mouse_button_down(*button)
                    }
                }
            })
    }

    pub fn is_action_pressed(
        &self,
        action: Action,
    ) -> bool {
        self.action_map
            .bindings(action)
            .iter()
            .any(|binding| {
                match binding {
                    Binding::Key(key) => {
                        self.is_key_pressed(*key)
                    }

                    Binding::Mouse(button) => {
                        self.is_mouse_button_pressed(*button)
                    }
                }
            })
    }

    pub fn is_action_released(
        &self,
        action: Action,
    ) -> bool {
        self.action_map
            .bindings(action)
            .iter()
            .any(|binding| {
                match binding {
                    Binding::Key(key) => {
                        self.is_key_released(*key)
                    }

                    Binding::Mouse(button) => {
                        self.is_mouse_button_released(*button)
                    }
                }
            })
    }

    pub fn set_action_bindings(
        &mut self,
        action: Action,
        bindings: Vec<Binding>,
    ) {
        self.action_map.set_bindings(
            action,
            bindings,
        );
    }

    pub fn set_action_binding(
        &mut self,
        action: Action,
        binding: Binding,
    ) {
        self.action_map.set_binding(
            action,
            binding,
        );
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