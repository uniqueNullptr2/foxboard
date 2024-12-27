use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::models::TaskModel;

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskMessage {
    pub id: Uuid,
    pub title: String,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub assignee_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
    pub deadline: Option<i64>,
    pub estimation: Option<i32>,
    pub state_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub task_type: i32,
    pub labels: Option<Vec<Uuid>>,
}

impl From<TaskModel> for TaskMessage {
    fn from(value: TaskModel) -> Self {
        Self {
            id: value.id,
            title: value.title,
            project_id: value.project_id,
            column_id: value.column_id,
            assignee_id: value.assignee_id,
            creator_id: value.creator_id,
            deadline: value.deadline.map(|x| x.timestamp_millis()),
            estimation: value.estimation,
            state_id: value.state_id,
            parent_id: value.parent_id,
            task_type: value.task_type,
            labels: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateOrUpdateTaskMessage {
    pub title: String,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub assignee_id: Option<Uuid>,
    pub creator_id: Option<Uuid>,
    pub deadline: Option<i64>,
    pub estimation: Option<i32>,
    pub state_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub task_type: i32,
    pub labels: Option<Vec<Uuid>>,
}

impl CreateOrUpdateTaskMessage {
    pub fn to_model(self) -> TaskModel {
        TaskModel {
            id: Uuid::nil(),
            title: self.title,
            project_id: self.project_id,
            column_id: self.column_id,
            assignee_id: self.assignee_id,
            creator_id: self.creator_id,
            deadline: self
                .deadline
                .and_then(chrono::DateTime::<chrono::Utc>::from_timestamp_millis),
            estimation: self.estimation,
            state_id: self.state_id,
            parent_id: self.parent_id,
            task_type: self.task_type,
        }
    }

    pub fn update_model(self, model: &mut TaskModel) {
        model.title = self.title;
        model.column_id = self.column_id;
        model.assignee_id = self.assignee_id;
        model.creator_id = self.creator_id;
        model.deadline = self
            .deadline
            .and_then(chrono::DateTime::<chrono::Utc>::from_timestamp_millis);
        model.estimation = self.estimation;
        model.state_id = self.state_id;
        model.task_type = self.task_type;
    }
}
