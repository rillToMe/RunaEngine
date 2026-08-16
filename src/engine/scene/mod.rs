use crate::engine::{
    math::Transform,
    renderer::{
        Renderer,
        Sprite,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(
        x: f32,
        y: f32,
    ) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

pub type EntityId = u32;

pub struct Entity {
    pub id: EntityId,
    pub sprite: Sprite,
    pub transform: Transform,
    pub velocity: Velocity,
    pub visible: bool,
}

pub struct Scene {
    entities: Vec<Entity>,
    next_id: EntityId,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 0,
        }
    }

    pub fn spawn(
        &mut self,
        sprite: Sprite,
        transform: Transform,
        velocity: Velocity,
    ) -> EntityId {
        let id = self.next_id;

        self.next_id += 1;

        self.entities.push(
            Entity {
                id,
                sprite,
                transform,
                velocity,
                visible: true,
            }
        );

        id
    }

    pub fn get(
        &self,
        id: EntityId,
    ) -> Option<&Entity> {
        self.entities
            .iter()
            .find(|entity| entity.id == id)
    }

    pub fn get_mut(
        &mut self,
        id: EntityId,
    ) -> Option<&mut Entity> {
        self.entities
            .iter_mut()
            .find(|entity| entity.id == id)
    }

    pub fn entities(
        &self,
    ) -> &[Entity] {
        &self.entities
    }

    pub fn entities_mut(
        &mut self,
    ) -> &mut [Entity] {
        &mut self.entities
    }

    pub fn despawn(
        &mut self,
        id: EntityId,
    ) {
        self.entities.retain(
            |entity| entity.id != id
        );
    }

    pub fn update(
        &mut self,
        dt: f32,
    ) {
        for entity in &mut self.entities {
            entity.transform.position[0] +=
                entity.velocity.x * dt;

            entity.transform.position[1] +=
                entity.velocity.y * dt;
        }
    }

    pub fn render(
        &self,
        renderer: &mut Renderer,
    ) {
        for entity in &self.entities {
            if !entity.visible {
                continue;
            }

            renderer.draw_sprite(
                &entity.sprite,
                &entity.transform,
            );
        }
    }

}