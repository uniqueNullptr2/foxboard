use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::{data::models::{Permissions, ProjectColumnModel, ProjectModel}, util::Requirement};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectMessage {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub public: bool,
}

impl From<ProjectModel> for ProjectMessage {
    fn from(value: ProjectModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            owner_id: value.owner_id,
            public: value.public,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectMessage {
    pub name: String,
    pub public: bool,
}
impl CreateProjectMessage {
    pub fn to_model(self, owner_id: Uuid) -> ProjectModel {
        ProjectModel {
            id: Uuid::nil(),
            name: self.name,
            owner_id,
            public: self.public,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateProjectMessage {
    pub name: Option<String>,
    pub public: Option<bool>,
    pub owner_id: Option<Uuid>
}

impl Requirement for UpdateProjectMessage {
    fn requirements(&self) -> Permissions {
        if self.public.is_some() || self.owner_id.is_some() {
            Permissions::Owner
        } else {
            Permissions::Editor
        }
    }
}

impl UpdateProjectMessage {
    pub fn update_project(self, mod_proj: &mut ProjectModel) {
        self.name.map(|v| mod_proj.name = v);
        self.owner_id.map(|v| mod_proj.owner_id = v);
        self.public.map(|v| mod_proj.public = v);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectColumnMessage {
    pub id: Uuid,
    pub name: String,
    pub card_limit: i32,
    pub project_id: Uuid,
    pub index: i32,
}
impl From<ProjectColumnModel> for ProjectColumnMessage {
    fn from(value: ProjectColumnModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            card_limit: value.card_limit,
            project_id: value.project_id,
            index: value.index,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectColumnMessage {
    pub name: String,
    pub card_limit: i32,
}
impl CreateProjectColumnMessage {
    pub fn to_model(self, project_id: Uuid, index: i32) -> ProjectColumnModel {
        ProjectColumnModel {
            id: Uuid::nil(),
            name: self.name,
            project_id,
            index,
            card_limit: self.card_limit,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateProjectColumnMessage {
    pub name: String,
    pub index: i32,
    pub card_limit: i32,
}
