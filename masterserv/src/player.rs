use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Player {
    pub id:Uuid,
    pub name:String
}