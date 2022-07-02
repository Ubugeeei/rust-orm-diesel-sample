-- Your SQL goes hereDROP TABLE IF EXISTS todos;
CREATE TABLE `todos` (
  `id` int(10) NOT NULL PRIMARY KEY,
  `title` varchar(128) NOT NULL,
  `description` varchar(256) NOT NULL,
  `is_done` int(1) DEFAULT '0'
)