-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION update_modified_column()   
RETURNS TRIGGER AS $$
BEGIN
    NEW.modified = now();
    RETURN NEW;   
END;
$$ language 'plpgsql';

CREATE TABLE IF NOT EXISTS users (
  username varchar(45) UNIQUE NOT NULL,
  password_hash varchar(450) NOT NULL,
  enabled boolean NOT NULL DEFAULT true,
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  is_admin boolean NOT NULL DEFAULT false,
  created timestamp NOT NULL default current_timestamp,
  modified timestamp NOT NULL default current_timestamp
);

CREATE TABLE IF NOT EXISTS invite_codes (
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  code varchar(64) NOT NULL,
  used boolean NOT NULL DEFAULT false,
  id BIGSERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS user_sessions (
  token varchar(90) NOT NULL,
  user_agent varchar(200) NOT NULL,
  ip_addr varchar(50) NOT NULL,
  user_id uuid NOT NULL,
  created timestamp NOT NULL default current_timestamp,
  modified timestamp NOT NULL default current_timestamp,
  CONSTRAINT fk_user_sessions_users
      FOREIGN KEY(user_id) 
        REFERENCES users(id)
        on delete cascade
);


CREATE TABLE IF NOT EXISTS projects(
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  name varchar(64) NOT NULL,
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  public boolean default false,
  owner_id uuid not null,
  CONSTRAINT fk_projects_owner
      FOREIGN KEY(owner_id) 
        REFERENCES users(id)
        on delete cascade
);

CREATE TABLE IF NOT EXISTS project_columns (
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name varchar(64) not null,
  card_limit int default null,
  project_id uuid not null,
  index int not null,
  CONSTRAINT fk_project_columns_project
      FOREIGN KEY(project_id) 
        REFERENCES projects(id)
        on delete cascade
);

CREATE TABLE IF NOT EXISTS tasks (
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  title varchar(128) NOT NULL,
  descriptsion varchar(4096),
  assignee_id uuid default null,
  creator_id uuid not null,
  column_id uuid not null,
  project_id uuid not null,
  deadline timestamptz,
  estimation int,
  CONSTRAINT fk_tasks_project
      FOREIGN KEY(project_id) 
        REFERENCES projects(id)
        on delete cascade,
  CONSTRAINT fk_tasks_assignee
      FOREIGN KEY(assignee_id) 
        REFERENCES users(id)
        on delete cascade, -- just set null
  CONSTRAINT fk_tasks_creator
      FOREIGN KEY(creator_id) 
        REFERENCES users(id)
        on delete cascade, -- set null
  CONSTRAINT fk_tasks_column
      FOREIGN KEY(column_id) 
        REFERENCES project_columns(id)
        on delete cascade --just set null
);

CREATE TABLE IF NOT EXISTS labels (
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name varchar(64) not null,
  project_id uuid not null,
  CONSTRAINT fk_labels_project
      FOREIGN KEY(project_id) 
        REFERENCES projects(id)
        on delete cascade
);

CREATE TABLE IF NOT EXISTS labels_tasks (
  created timestamp NOT NULL default current_timestamp,
  modified timestamp default current_timestamp,
  label_id uuid not null,
  task_id uuid not null,
  CONSTRAINT u_labels UNIQUE (label_id, task_id),
  CONSTRAINT fk_labels_tasks_label
      FOREIGN KEY(label_id) 
        REFERENCES labels(id)
        on delete cascade,
  CONSTRAINT fk_labels_tasks_task
      FOREIGN KEY(task_id) 
        REFERENCES tasks(id)
        on delete cascade
);

CREATE UNIQUE INDEX IF NOT EXISTS sessions_token_index on user_sessions (token);
CREATE TRIGGER update_user_sessions_modtime BEFORE UPDATE ON user_sessions FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
CREATE TRIGGER update_user_modtime BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
CREATE TRIGGER update_invite_code_modtime BEFORE UPDATE ON invite_codes FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();

CREATE TRIGGER update_invite_projects BEFORE UPDATE ON projects FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
CREATE TRIGGER update_invite_project_columns BEFORE UPDATE ON project_columns FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
CREATE TRIGGER update_invite_tasks BEFORE UPDATE ON tasks FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
CREATE TRIGGER update_invite_labels BEFORE UPDATE ON labels FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();
CREATE TRIGGER update_invite_labels_tasks BEFORE UPDATE ON labels_tasks FOR EACH ROW EXECUTE PROCEDURE  update_modified_column();

CREATE UNIQUE INDEX IF NOT EXISTS invite_code_code_index on invite_codes (code);