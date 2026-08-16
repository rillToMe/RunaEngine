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
    pub velocity: Option<Velocity>,
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
    ) -> EntityId {
        let id = self.next_id;

        self.next_id += 1;

        self.entities.push(
            Entity {
                id,
                sprite,
                transform,
                velocity: None,
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

    
    pub fn set_velocity(
        &mut self,
        id: EntityId,
        velocity: Velocity,
    ) {
        if let Some(entity) = self.get_mut(id) {
            entity.velocity = Some(velocity);
        }
    }

    pub fn velocity(
        &self,
        id: EntityId,
    ) -> Option<Velocity> {
        self.get(id)
            .and_then(|entity| entity.velocity)
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
            if let Some(velocity) = entity.velocity {
                entity.transform.position[0] +=
                    velocity.x * dt;

                entity.transform.position[1] +=
                    velocity.y * dt;
            }
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