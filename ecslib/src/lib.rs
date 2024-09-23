use std::any::Any;

use entities::{query::Query, Entities};
pub mod custom_errors;
mod entities;
mod resource;
pub use custom_errors::*;
use eyre::{Ok, Result};
#[derive(Default, Debug)]
pub struct World {
    resources: resource::Resource,
    entities: entities::Entities,
}
impl World {
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data)
    }

    pub fn get_resource<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a mutable reference to it. The type of the resource must be added in that we can find it
    /// ```
    /// use ecslib::World;
    /// let mut world = World::default();
    /// world.add_resource(10_u32);
    /// {
    ///     let resource = world.get_resource_mut::<u32>().unwrap();
    ///     *resource += 1;
    /// }
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource, 11);
    /// ```
    pub fn get_resource_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any,
    {
        self.resources.get_mut::<T>()
    }

    /// Delete a resource from the world.
    pub fn delete_resource<T>(&mut self)
    where
        T: Any,
    {
        self.resources.remove::<T>();
    }

    pub fn register_component<T>(&mut self)
    where
        T: Any,
    {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }
    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }

    pub fn delete_component_by_entity_id<T>(&mut self, index: usize) -> Result<()>
    where
        T: Any,
    {
        self.entities.delete_component_by_entity_id::<T>(index)
    }

    pub fn add_component_to_entity_by_id(&mut self, data: impl Any, entity_id: usize) -> Result<()> {
        self.entities.add_component_by_entity_id(data, entity_id)
    }

    pub fn delete_entity_by_id(&mut self, entity_id: usize) -> Result<()> {
        self.entities.delete_entity_by_id(entity_id)
    }
}
#[cfg(test)]
mod tests {}
