-- Add down migration script here
drop trigger update_project_permissions_modtime on project_permissions;

drop table project_permissions;
