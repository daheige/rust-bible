use test;
CREATE TABLE `student` (
                           `id` int(11) NOT NULL AUTO_INCREMENT,
                           `name` varchar(128) NOT NULL,
                           `age` int(11) NOT NULL,
                           `id_card` varchar(128) NOT NULL,
                           `last_update` date NOT NULL,
                           PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- 插入测试数据
insert into student (name, age, id_card, last_update) values ('张三', 23, '123456789X', CURRENT_DATE());
insert into student (name, age, id_card, last_update) values ('xiaoming', 23, '123456789X', CURRENT_DATE());
insert into student (name, age, id_card, last_update) values ('李四', 24, '8382353902', CURRENT_DATE());
