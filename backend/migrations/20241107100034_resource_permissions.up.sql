-- Add up migration script here
Create table if not exists project_permissions (
  perm int UNIQUE NOT NULL default 5,
  user_id uuid NOT NULL,
  project_id uuid NOT NULL,
  created timestamp NOT NULL default current_timestamp,
  modified timestamp NOT NULL default current_timestamp,
  CONSTRAINT u_project_perms UNIQUE (user_id, project_id),
  CONSTRAINT fk_project_permissions_user
      FOREIGN KEY(user_id)
        REFERENCES users(id)
        on delete cascade,
  CONSTRAINT fk_project_permissions_project
      FOREIGN KEY(project_id) 
        REFERENCES projects(id)
        on delete cascade
);

CREATE TRIGGER update_project_permissions_modtime BEFORE UPDATE ON project_permissions FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
