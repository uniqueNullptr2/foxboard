use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::data::models::{LabelModel, Permissions, ProjectColumnModel, ProjectModel, StateModel};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectMessage {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub public: bool,
    pub labels: Option<Vec<LabelMessage>>,
    pub columns: Option<Vec<ProjectColumnMessage>>,
}

impl From<ProjectModel> for ProjectMessage {
    fn from(value: ProjectModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            owner_id: value.owner_id,
            public: value.public,
            labels: None,
            columns: None,
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
    pub owner_id: Option<Uuid>,
}

impl UpdateProjectMessage {
    pub fn update_project(self, mod_proj: &mut ProjectModel) {
        if let Some(v) = self.name {
            mod_proj.name = v
        };
        if let Some(v) = self.owner_id {
            mod_proj.owner_id = v
        };
        if let Some(v) = self.public {
            mod_proj.public = v
        };
    }
    pub fn get_requirements(&self) -> Permissions {
        if self.public.is_some() || self.owner_id.is_some() {
            Permissions::Owner
        } else {
            Permissions::Editor
        }
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
    pub card_limit: Option<i32>,
    pub index: i32,
}
impl CreateProjectColumnMessage {
    pub fn to_model(self, project_id: Uuid) -> ProjectColumnModel {
        ProjectColumnModel {
            id: Uuid::nil(),
            name: self.name,
            project_id,
            index: self.index,
            card_limit: self.card_limit.unwrap_or(0),
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateProjectColumnMessage {
    pub name: Option<String>,
    pub index: Option<i32>,
    pub card_limit: Option<i32>,
}
impl UpdateProjectColumnMessage {
    pub fn update_model(self, mod_column: &mut ProjectColumnModel) {
        if let Some(name) = self.name {
            mod_column.name = name
        };
        if let Some(index) = self.index {
            mod_column.index = index
        };
        if let Some(card_limit) = self.card_limit {
            mod_column.card_limit = card_limit
        };
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LabelMessage {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
}

impl From<LabelModel> for LabelMessage {
    fn from(value: LabelModel) -> Self {
        LabelMessage {
            id: value.id,
            name: value.name,
            project_id: value.project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateLabelMessage {
    pub name: String,
}
impl CreateLabelMessage {
    pub fn to_model(self, project_id: Uuid) -> LabelModel {
        LabelModel {
            id: Uuid::nil(),
            name: self.name,
            project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateLabelMessage {
    pub name: String,
}

impl UpdateLabelMessage {
    pub fn update_model(self, mod_label: &mut LabelModel) {
        mod_label.name = self.name;
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StateMessage {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
}

impl From<StateModel> for StateMessage {
    fn from(value: StateModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            project_id: value.project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateStateMessage {
    pub name: String,
}

impl CreateStateMessage {
    pub fn to_model(self, project_id: Uuid) -> StateModel {
        StateModel {
            id: Uuid::nil(),
            name: self.name,
            project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateStateMessage {
    pub name: String,
}

impl UpdateStateMessage {
    pub fn update_model(self, model: &mut StateModel) {
        model.name = self.name;
    }
}
