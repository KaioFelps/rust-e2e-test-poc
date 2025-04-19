-- Add migration script here
CREATE TABLE todos (
  id                SERIAL        NOT NULL,
  title             VARCHAR(150)  NOT NULL,
  description       TEXT          NOT NULL,
  completed         BOOLEAN       NOT NULL DEFAULT false,
  created_at        TIMESTAMP     NOT NULL DEFAULT now(),
  
CONSTRAINT pk_todos PRIMARY KEY (id));

