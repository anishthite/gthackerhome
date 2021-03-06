create table users (
`email` TEXT NOT NULL,
`username`  VARCHAR(200) PRIMARY KEY NOT NULL,
`password` TEXT NOT NULL,
`about` TEXT,
`admin` INT(11),
`timecreated` BIGINT NOT NULL,
`parent` VARCHAR(200) NOT NULL
);

create table items (
`id` VARCHAR(200) PRIMARY KEY NOT NULL,
`author` VARCHAR(200) NOT NULL,
`time` BIGINT NOT NULL,
`itemtype` VARCHAR(200) NOT NULL,
`title` TEXT,
`url` TEXT ,
`text` TEXT,
`parentid` VARCHAR(200),
`descendents` INT,
`score` INT

);

create table invite_tokens(
`token` VARCHAR(200) PRIMARY KEY NOT NULL,
`creator` VARCHAR(200) NOT NULL
);

-- Your SQL goes here
