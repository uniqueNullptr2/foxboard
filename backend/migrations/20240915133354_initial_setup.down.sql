-- Add down migration script here
drop index invite_code_code_index;
drop index sessions_token_index;

drop trigger update_user_modtime on users;
drop trigger update_invite_code_modtime on invite_codes;
drop trigger update_user_sessions_modtime on user_sessions;

drop TRIGGER update_invite_projects ON projects;
drop TRIGGER update_invite_project_columns ON project_columns;
drop TRIGGER update_invite_tasks ON tasks;
drop TRIGGER update_invite_labels ON labels;
drop TRIGGER update_invite_labels_tasks ON labels_tasks;


drop table user_sessions;
drop table invite_codes;
drop table labels_tasks;
drop table labels;
drop table tasks;
drop table project_columns;
drop table projects;
drop table users;

drop function update_modified_column;
drop extension "uuid-ossp";
