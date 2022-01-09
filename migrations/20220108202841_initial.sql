-- Add migration script here
USE rust_todo_actix_seaorm;

CREATE TABLE `todos` (
  `todo_id` bigint NOT NULL AUTO_INCREMENT,
  `todo_name` varchar(60) NOT NULL,
  `todo_description` varchar(150) DEFAULT NULL,
  `todo_date` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`todo_id`),
  KEY `idx_todo_name_key` (`todo_name`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb3;