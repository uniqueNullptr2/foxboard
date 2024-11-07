-- Add down migration script here
ALTER TABLE tasks ALTER COLUMN column_id set not null;
alter table tasks alter column column_id drop default;

alter table tasks drop constraint fk_tasks_state;
alter table tasks drop constraint fk_tasks_parent;

Alter table tasks drop column state_id;
Alter table tasks drop column parent_id;
Alter table tasks drop column task_type;

Alter Table tasks drop constraint fk_tasks_assignee;
alter table tasks add constraint fk_tasks_assignee foreign key(assignee_id) references users(id) on delete cascade;

Alter Table tasks drop constraint fk_tasks_creator;
alter table tasks add constraint fk_tasks_creator foreign key(creator_id) references users(id) on delete cascade;

Alter Table tasks drop constraint fk_tasks_column;
alter table tasks add constraint fk_tasks_column foreign key(column_id) references project_columns(id) on delete cascade;

drop trigger update_states_modtime on states;
drop table states;