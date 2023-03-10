CREATE TABLE sagas (
  id INTEGER UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
  name VARCHAR(255) NOT NULL,
  author VARCHAR(255) NOT NULL,
  music VARCHAR(255) NOT NULL,
  season SMALLINT UNSIGNED NOT NULL,
  creation_date VARCHAR(255) NOT NULL,
  countryID INTEGER UNSIGNED NOT NULL,
  statusID INTEGER UNSIGNED NOT NULL,
  categoryID INTEGER UNSIGNED NOT NULL,
  description TEXT NOT NULL,
  FOREIGN KEY (countryID) REFERENCES countries(id),
  FOREIGN KEY (statusID) REFERENCES status(id),
  FOREIGN KEY (categoryID) REFERENCES categories(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_general_ci';
