CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  role_id INTEGER NOT NULL DEFAULT 99,
  tanshi INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  avatar TEXT NOT NULL DEFAULT 'https://www.gravatar.com/avatar/3ece7a0e953c642c06083c2b5e0dcb8a?s=128&d=identicon',
  UNIQUE (email, username)
);

INSERT INTO users (id, email, username, password, role_id, tanshi, created_at, avatar) VALUES
(1, 'rustlangcn@163.com',   'admin', '$2y$12$jZDZwadBNutfJeELHEHDhe3KDV1iKc3macGi4Dx6OK4NDNoHm/lIG', 99, 0, '2018-07-08 13:00:26.353041', 'http://www.gravatar.com/avatar/3ece7a0e953c642c06083c2b5e0dcb8a?s=128&d=identicon');
SELECT setval('users_id_seq', 1, true);