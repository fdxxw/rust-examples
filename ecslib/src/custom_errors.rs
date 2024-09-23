use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error(
        "Attemping to access add component to an entity without calling create component first!"
    )]
    CreateComponentNeverCalled,
    #[error("attemping to reference a component that wasn't registered")]
    ComponentNotRegistered,
    #[error("attemping to reference an error that doesn't exist")]
    EntityDoesNotExist,

}
