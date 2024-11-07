-- Add up migration script here
  
CREATE TABLE IF NOT EXISTS states (
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name varchar(64) not null,
  project_id uuid not null,
  CONSTRAINT fk_states_project
      FOREIGN KEY(project_id) 
        REFERENCES projects(id)
        on delete cascade
);


ALTER TABLE tasks ALTER COLUMN column_id drop not null;
alter table tasks alter column column_id set default null;
Alter table tasks add column state_id uuid default null;
alter table tasks add constraint fk_tasks_state foreign key(state_id) references states(id) on delete set null;
Alter table tasks add column parent_id uuid default null;
alter table tasks add constraint fk_tasks_parent foreign key(parent_id) references tasks(id) on delete cascade;
Alter table tasks add column task_type int default 0;

Alter Table tasks drop constraint fk_tasks_assignee;
alter table tasks add constraint fk_tasks_assignee foreign key(assignee_id) references users(id) on delete set null;

Alter Table tasks drop constraint fk_tasks_creator;
alter table tasks add constraint fk_tasks_creator foreign key(creator_id) references users(id) on delete set null;

Alter Table tasks drop constraint fk_tasks_column;
alter table tasks add constraint fk_tasks_column foreign key(column_id) references project_columns(id) on delete set null;




CREATE TRIGGER update_states_modtime BEFORE UPDATE ON states FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
