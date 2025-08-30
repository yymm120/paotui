-- Add up migration script here
-- pending → (sqlx migrate run) → applied → (sqlx migrate revert) → pending


-- begin [ store ]
CREATE SEQUENCE IF NOT EXISTS store_id_seq START WITH 10000;
CREATE TABLE IF NOT EXISTS public.store (
    store_id bigint NOT NULL DEFAULT nextval('store_id_seq'),
    store_name varchar NOT NULL,
    password varchar NOT NULL,
    address bigint NOT NULL,
    telephone varchar NOT NULL,
    money decimal DEFAULT 0 NOT NULL,
    status varchar NOT NULL
        CHECK (status IN ('working', 'resting'))
        DEFAULT 'resting',
    created_at timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT store_pk PRIMARY KEY (store_id),
    CONSTRAINT store_telephone_unique UNIQUE (telephone)
);

ALTER SEQUENCE store_id_seq OWNED BY store.store_id;

-- 创建索引（添加IF NOT EXISTS）
CREATE INDEX IF NOT EXISTS idx_store_status ON store(status);

-- 插入测试数据（使用条件插入避免重复）
INSERT INTO store (store_name, telephone, password, address, money, status)
SELECT '板凳面馆', '13800138001', '123456', 10001, 99.9, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM store WHERE telephone = '13800138001');

INSERT INTO store (store_name, telephone, password, address, money, status)
SELECT '混沌店', '13800138002', '123456', 10002, 89.9, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM store WHERE telephone = '13800138002');

INSERT INTO store (store_name, telephone, password, address, money, status)
SELECT '奶茶店', '13800138003', '123456', 10003, 79.9, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM store WHERE telephone = '13800138003');

INSERT INTO store (store_name, telephone, password, address, money, status)
SELECT '中餐馆', '13800138004', '123456', 10004, 69.9, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM store WHERE telephone = '13800138004');

INSERT INTO store (store_name, telephone, password, address, money, status)
SELECT '早餐店', '13800138005', '123456', 10005, 59.9, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM store WHERE telephone = '13800138005');
-- end [ store ]



-- begin [ user ]
CREATE SEQUENCE IF NOT EXISTS user_id_seq START WITH 10000;

CREATE TABLE IF NOT EXISTS public.user (
    user_id bigint NOT NULL DEFAULT nextval('user_id_seq'),
    username varchar NOT NULL,
    password varchar NOT NULL,
    telephone varchar NOT NULL,
    user_type varchar NOT NULL,
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp DEFAULT now() NOT NULL,
    deleted_at timestamp NULL,
    CONSTRAINT user_pk PRIMARY KEY (user_id, user_type)
--     CONSTRAINT user_telephone_unique UNIQUE (telephone)
);

ALTER SEQUENCE user_id_seq OWNED BY public.user.user_id;
CREATE INDEX IF NOT EXISTS idx_user_telephone ON public.user(telephone);
-- end [ user ]



-- begin [ delivery_person ]
CREATE SEQUENCE IF NOT EXISTS delivery_person_id_seq START WITH 10000;
CREATE TABLE IF NOT EXISTS public.delivery_person (
    delivery_person_id bigint DEFAULT nextval('delivery_person_id_seq') NOT NULL,
    money decimal NOT NULL DEFAULT 0,
    status varchar NOT NULL
        CHECK (status IN ('working', 'resting'))
                              DEFAULT 'resting',
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp DEFAULT now() NOT NULL,
    deleted_at timestamp NULL,
    CONSTRAINT delivery_person_table_pk PRIMARY KEY (delivery_person_id)
);
ALTER SEQUENCE delivery_person_id_seq OWNED BY public.delivery_person.delivery_person_id;
-- end [ delivery_person ]



-- begin [ delivery_app_user ]
CREATE TABLE IF NOT EXISTS public.delivery_app_user (
    delivery_person_id bigint NOT NULL REFERENCES public.delivery_person(delivery_person_id),
    user_id bigint NOT NULL,
    user_type VARCHAR GENERATED ALWAYS AS ('delivery') STORED,
    CONSTRAINT delivery_app_user_table_pk PRIMARY KEY (user_id, user_type),
    CONSTRAINT fk_delivery_app_user_user FOREIGN KEY (user_id, user_type)
            REFERENCES public.user(user_id, user_type),
    CONSTRAINT delivery_app_user_telephone_unique UNIQUE (user_id)
);
-- end [ delivery_user ]


-- begin [ delivery_app_user ]

-- end [ delivery_app_user ]


-- 插入delivery user数据
-- 1.先插入user
INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10000, '罗先生', '123456', '17815349591', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349591');
-- 2.再插入delivery_person
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10000, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10000);
-- 3.最后插入delivery_app_user
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10000, 10000
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10000 AND user_id = 10000);

-- 1.先插入user
INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10001, '罗先生', '123456', '17815349594', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349594');
-- 2.再插入delivery_person
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10001, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10001);
-- 3.最后插入delivery_app_user
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10001, 10001
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10001 AND user_id = 10001);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10002, '张先生', '123456', '17815349595', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349595');
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10002, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10002);
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10002, 10002
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10002 AND user_id = 10002);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10003, '刘先生', '123456', '17815349596', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349596');
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10003, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10003);
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10003, 10003
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10003 AND user_id = 10003);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10004, '白先生', '123456', '17815349597', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349597');
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10004, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10004);
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10004, 10004
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10004 AND user_id = 10004);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10005, '马先生', '123456', '17815349598', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349598');
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10005, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10005);
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10005, 10005
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10005 AND user_id = 10005);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10006, '林先生', '123456', '17815349599', 'delivery'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349599');
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10006, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10006);
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10006, 10006
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10006 AND user_id = 10006);





-- begin [ task_person ]
CREATE SEQUENCE IF NOT EXISTS task_person_id_seq START WITH 10000;

CREATE TABLE IF NOT EXISTS public.task_person (
    task_person_id bigint NOT NULL DEFAULT nextval('task_person_id_seq'),
    created_at timestamp DEFAULT now() NOT NULL,
    updated_at timestamp DEFAULT now() NOT NULL,
    deleted_at timestamp NULL,
    CONSTRAINT task_person_id PRIMARY KEY (task_person_id)
);

ALTER SEQUENCE task_person_id_seq OWNED BY public.task_person.task_person_id;
-- end [ task_person ]


-- begin [ task_app_user ]
CREATE TABLE IF NOT EXISTS public.task_app_user (
    user_id bigint NOT NULL,
    task_person_id bigint NOT NULL REFERENCES public.task_person(task_person_id),
    user_type VARCHAR GENERATED ALWAYS AS ('task') STORED,
    CONSTRAINT task_app_user_table_pk PRIMARY KEY (user_id, user_type),
    CONSTRAINT fk_task_app_user_user FOREIGN KEY (user_id, user_type)
        REFERENCES public.user(user_id, user_type),
    CONSTRAINT task_app_user_telephone_unique UNIQUE (user_id)

);
-- end [ task_app_user ]


-- begin [ address ]
CREATE SEQUENCE IF NOT EXISTS address_id_seq START WITH 10000;
create table if not exists public.address(
    id         bigint default nextval('address_id_seq'::regclass) not null,
    latitude   double precision                                   not null,
    longitude  double precision                                   not null,
    altitude   double precision                                   null,
    accuracy   double precision                                   null,
    created_at date   default CURRENT_TIMESTAMP                   not null,
    updated_at date   default CURRENT_TIMESTAMP                   not null,
    deleted_at date                                               null,
    CONSTRAINT address_pk PRIMARY KEY (id)
);
ALTER SEQUENCE address_id_seq OWNED BY public.address.id;
-- end [ address ]


-- begin [ task_app_user_address ]
create table if not exists public.task_app_user_address(
    user_id    bigint references public.task_app_user(user_id)    not null,
    address_id  double precision                                  not null,
    address_detail varchar                                        null,
    city        varchar                                           null,
    CONSTRAINT task_app_user_address_pk PRIMARY KEY (user_id, address_id)
);
-- end [ task_app_user_address ]


-- begin [ task_address ]
CREATE SEQUENCE IF NOT EXISTS task_address_id_seq START WITH 10000;
create table if not exists public.task_address(
    id     bigint default nextval('task_address_id_seq'::regclass) not null,
    address_id         double precision                            not null,
    address_detail     varchar                                     null,
    city               varchar                                     null,
    CONSTRAINT task_address_pk PRIMARY KEY (id)
);
ALTER SEQUENCE task_address_id_seq OWNED BY public.task_address.id;
-- end [ task_address ]

-- begin [ delivery_task ]
create table IF NOT EXISTS public.delivery_task
(
    id                  varchar                                        not null
        constraint delivery_task_table_pk
            primary key,
    tag                 varchar,
    items               varchar,
    note                varchar,
    status              varchar                                        not null,
    priority            varchar default '“default"'::character varying not null,
    estimated_income    varchar                                        not null,
    username_send       varchar                                        not null,
    username_receive    varchar                                        not null,
    username_delivery   varchar,
    address_start_id    bigint                                         not null
        constraint "delivery_task_start_address_FK"
            references task_address(id),
    address_end_id      bigint                                         not null
        constraint "delivery_task_end_address_FK"
            references task_address(id),
    user_id_created_by  bigint                                         not null
        constraint "delivery_task_create_by_FK"
            references task_app_user(user_id),
    telephone_send      varchar                                        not null,
    telephone_receive   varchar                                        not null,
    telephone_delivery  varchar,
    user_id_delivery_by bigint                                         null
    constraint "delivery_task_delivery_by_FK"
        references public.delivery_app_user(user_id),
    create_time         timestamp                                      not null,
    arrived_time        timestamp

);

CREATE INDEX IF NOT EXISTS idx_delivery_by ON delivery_task(user_id_delivery_by);
CREATE INDEX IF NOT EXISTS idx_created_by ON delivery_task(user_id_created_by);

comment on table public.delivery_task is '配送任务表';

comment on column public.delivery_task.id is '订单id，也是订单编号，应该有一种固定的编号规则';

comment on column public.delivery_task.tag is '物品标签，可以为空';

comment on column public.delivery_task.items is '配送的物品，可以为空。';

comment on column public.delivery_task.note is '注释';

comment on column public.delivery_task.status is '配送的状态，包括：new / idle / pend / done 四个状态';

comment on column public.delivery_task.priority is '优先级，表明是否优先配送';

comment on column public.delivery_task.estimated_income is '预计收入，指的是骑手的收入';

comment on column public.delivery_task.username_send is '谁发的';

comment on column public.delivery_task.username_receive is '谁收的';

comment on column public.delivery_task.username_delivery is '谁送的，未接单时，为空';

comment on column public.delivery_task.address_start_id is '起始地址';

comment on column public.delivery_task.address_end_id is '结束地址';

comment on column public.delivery_task.user_id_created_by is '创建task的人';

-- end [ delivery_task ]

-- begin insert data for delivery_task, task_app_user, address
-- 1.先插入 user
INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10007, '王先生', '123456', '17815349691', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349691');
-- 2.再插入 task_person
INSERT INTO public.task_person (task_person_id)
SELECT 10000
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10000);
-- 3.最后插入 task_app_user
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10007, 10000
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10000 AND user_id = 10007);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10008, '季先生', '123456', '17815349692', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349692');
INSERT INTO public.task_person (task_person_id)
SELECT 10001
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10001);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10008, 10001
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10001 AND user_id = 10008);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10009, '高先生', '123456', '17815349693', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349693');
INSERT INTO public.task_person (task_person_id)
SELECT 10002
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10002);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10009, 10002
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10002 AND user_id = 10009);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10010, '宋先生', '123456', '17815349694', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349694');
INSERT INTO public.task_person (task_person_id)
SELECT 10003
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10003);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10010, 10003
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10003 AND user_id = 10010);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10011, '郭先生', '123456', '17815349695', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349695');
INSERT INTO public.task_person (task_person_id)
SELECT 10004
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10004);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10011, 10004
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10004 AND user_id = 10011);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10012, '董先生', '123456', '17815349696', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349696');
INSERT INTO public.task_person (task_person_id)
SELECT 10005
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10005);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10012, 10005
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10005 AND user_id = 10012);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10013, '马先生', '123456', '17815349697', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349697');
INSERT INTO public.task_person (task_person_id)
SELECT 10006
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10006);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10013, 10006
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10006 AND user_id = 10013);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10014, '孙先生', '123456', '17815349698', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349698');
INSERT INTO public.task_person (task_person_id)
SELECT 10007
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10007);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10014, 10007
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10007 AND user_id = 10014);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10015, '刘先生', '123456', '17815349699', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349699');
INSERT INTO public.task_person (task_person_id)
SELECT 10008
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10008);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10015, 10008
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10008 AND user_id = 10015);

INSERT INTO public.user (user_id, username, password, telephone, user_type)
SELECT 10016, '李先生', '123456', '17815349690', 'task'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '17815349690');
INSERT INTO public.task_person (task_person_id)
SELECT 10009
WHERE NOT EXISTS (SELECT 1 FROM public.task_person WHERE task_person_id = 10009);
INSERT INTO public.task_app_user (user_id, task_person_id)
SELECT 10016, 10009
WHERE NOT EXISTS (SELECT 1 FROM public.task_app_user WHERE task_person_id = 10009 AND user_id = 10016);


-- begin [ create task ]  1. address 2. task_address 3. task

-- task1
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10000, 29.5, 106.5, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10000);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10000, 10000, '重庆市渝中区解放碑1号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10000);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10001, 29.6, 106.6, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10001);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10001, 10001, '重庆市江北区观音桥2号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10001);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000001', 'urgent', '文件', '重要合同', 'new', 'default', 5.0,
       '张先生', '王先生', null, 10000, 10001,
       10007, '17815349593', '15732043188', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000001');

-- task2
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10002, 29.55, 106.55, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10002);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10002, 10002, '重庆市沙坪坝区三峡广场3号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10002);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10003, 29.65, 106.65, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10003);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10003, 10003, '重庆市南岸区南坪4号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10003);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000002', null, '电子产品', '易碎品', 'new', 'default', 8.5,
       '李先生', '赵女士', '配送员1', 10002, 10003,
       10007, '13512345678', '13898765432', '15987654321',
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000002');

-- task3
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10004, 29.52, 106.52, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10004);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10004, 10004, '重庆市九龙坡区杨家坪5号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10004);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10005, 29.62, 106.62, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10005);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10005, 10005, '重庆市渝北区新牌坊6号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10005);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000003', 'fragile', '玻璃制品', '小心轻放', 'new', 'default', 6.0,
       '陈先生', '刘女士', null, 10004, 10005,
       10008, '13612345678', '13998765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000003');


-- task4
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10006, 29.52, 106.52, 980, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10006);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10006, 10002, '重庆市沙坪坝区三峡广场3号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10006);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10007, 29.62, 106.62, 1020, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10007);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10007, 10003, '重庆市南岸区南坪4号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10007);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000004', 'fragile', '玻璃器皿', '易碎品小心搬运', 'new', 'default', 7.5,
       '李先生', '赵女士', null, 10006, 10007,
       10009, '13512345678', '13898765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000004');

-- task5
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10008, 29.55, 106.55, 950, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10008);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10008, 10004, '重庆市九龙坡区杨家坪5号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10008);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10009, 29.65, 106.65, 1050, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10009);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10009, 10005, '重庆市渝北区新牌坊6号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10009);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000005', 'food', '生日蛋糕', '需冷藏保持低温', 'new', 'default', 12.0,
       '王女士', '刘先生', null, 10004, 10005,
       10010, '13612345678', '13998765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000005');

-- task6
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10010, 29.58, 106.58, 1005, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10010);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10010, 10006, '重庆市大渡口区钢花路7号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10010);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10011, 29.68, 106.68, 995, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10011);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10011, 10007, '重庆市巴南区鱼洞8号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10011);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000006', null, '办公用品', '10箱A4打印纸', 'new', 'default', 6.8,
       '陈先生', '林女士', null, 10006, 10007,
       10011, '13712345678', '13198765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000006');

-- task7
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10012, 29.53, 106.53, 1010, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10012);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10012, 10008, '重庆市北碚区城南9号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10012);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10013, 29.63, 106.63, 990, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10013);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10013, 10009, '重庆市渝北区回兴10号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10013);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000007', 'medical', '药品', '需2-8℃冷藏运输', 'new', 'default', 15.5,
       '杨先生', '黄女士', null, 10008, 10009,
       10012, '13812345678', '13298765432', null,
       null, now() - interval '3 hours', null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000007');

-- task8
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10014, 29.57, 106.57, 1025, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10014);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10014, 10010, '重庆市渝中区大坪11号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10014);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10015, 29.67, 106.67, 975, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10015);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10015, 10011, '重庆市江北区五里店12号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10015);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000008', 'electronics', '笔记本电脑', '贵重物品小心轻放', 'new', 'default', 9.9,
       '周先生', '吴女士', null, 10010, 10011,
       10013, '13912345678', '13398765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000008');



-- task9
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10016, 29.54, 106.54, 1015, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10016);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10016, 10016, '重庆市渝中区两路口13号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10016);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10017, 29.64, 106.64, 985, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10017);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10017, 10017, '重庆市江北区石马河14号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10017);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000009', 'documents', '合同文件', '加急送达', 'new', 'default', 8.8,
       '郑先生', '孙女士', null, 10016, 10017,
       10014, '13412345678', '13598765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000009');

-- task10
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10018, 29.56, 106.56, 1030, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10018);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10018, 10018, '重庆市沙坪坝区磁器口15号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10018);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10019, 29.66, 106.66, 970, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10019);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10019, 10019, '重庆市南岸区弹子石16号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10019);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000010', 'flowers', '鲜花礼盒', '保鲜配送', 'new', 'default', 6.5,
       '冯先生', '朱女士', null, 10018, 10019,
       10015, '13612345678', '13798765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000010');

-- task11
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10020, 29.58, 106.58, 1040, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10020);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10020, 10020, '重庆市九龙坡区石桥铺17号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10020);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10021, 29.68, 106.68, 960, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10021);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10021, 10021, '重庆市渝北区龙溪18号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10021);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000011', 'food', '火锅食材', '冷链运输', 'new', 'default', 11.2,
       '韩先生', '曹女士', null, 10020, 10021,
       10016, '13712345678', '13898765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000011');

-- task12
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10022, 29.59, 106.59, 1028, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10022);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10022, 10022, '重庆市大渡口区跃进村19号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10022);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10023, 29.69, 106.69, 972, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10023);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10023, 10023, '重庆市巴南区李家沱20号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10023);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000012', 'equipment', '摄影器材', '防震包装', 'new', 'default', 14.5,
       '谢先生', '董女士', null, 10022, 10023,
       10007, '13812345678', '13998765432', null,
       null, now() - interval '2 hours', null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000012');

-- task13
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10024, 29.51, 106.51, 1035, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10024);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10024, 10024, '重庆市北碚区天生桥21号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10024);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10025, 29.61, 106.61, 965, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10025);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10025, 10025, '重庆市渝北区黄泥塝22号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10025);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000013', 'medical', '医疗设备', '恒温运输', 'new', 'default', 18.0,
       '崔先生', '袁女士', null, 10024, 10025,
       10008, '13912345678', '13198765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000013');

-- task14
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10026, 29.53, 106.53, 1045, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10026);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10026, 10026, '重庆市渝中区朝天门23号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10026);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10027, 29.63, 106.63, 955, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10027);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10027, 10027, '重庆市江北区大石坝24号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10027);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000014', 'electronics', '智能手机', '保价配送', 'new', 'default', 22.5,
       '彭先生', '邓女士', null, 10026, 10027,
       10009, '13112345678', '13298765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000014');

-- task15
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10028, 29.55, 106.55, 1050, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10028);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10028, 10028, '重庆市沙坪坝区小龙坎25号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10028);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10029, 29.65, 106.65, 950, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10029);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10029, 10029, '重庆市南岸区海棠溪26号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10029);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000015', 'documents', '房产证', '重要文件', 'new', 'default', 9.9,
       '曾先生', '于女士', null, 10028, 10029,
       10010, '13212345678', '13398765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000015');

-- task16
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10030, 29.57, 106.57, 1038, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10030);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10030, 10030, '重庆市九龙坡区杨家坪27号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10030);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10031, 29.67, 106.67, 962, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10031);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10031, 10031, '重庆市渝北区冉家坝28号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10031);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000016', 'food', '海鲜礼盒', '冷链运输', 'new', 'default', 15.8,
       '苏先生', '潘女士', null, 10030, 10031,
       10011, '13312345678', '13498765432', null,
       null, now() - interval '4 hours', null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000016');

-- task17
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10032, 29.52, 106.52, 1042, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10032);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10032, 10032, '重庆市大渡口区新山村29号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10032);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10033, 29.62, 106.62, 958, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10033);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10033, 10033, '重庆市巴南区花溪30号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10033);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000017', 'equipment', '实验器材', '防震包装', 'new', 'default', 20.0,
       '董先生', '余女士', null, 10032, 10033,
       10012, '13412345678', '13598765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000017');

-- task18
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10034, 29.54, 106.54, 1036, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10034);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10034, 10034, '重庆市北碚区朝阳31号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10034);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10035, 29.64, 106.64, 964, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10035);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10035, 10035, '重庆市渝北区人和32号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10035);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000018', 'fragile', '玻璃工艺品', '易碎品小心搬运', 'new', 'default', 12.5,
       '叶先生', '杜女士', null, 10034, 10035,
       10013, '13512345678', '13698765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000018');



-- task19
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10036, 29.78, 106.78, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10036);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10036, 10036, '重庆市巴南区鱼洞29号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10036);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10037, 29.88, 106.88, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10037);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10037, 10037, '重庆市北碚区城南30号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10037);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000019', 'food', '生日蛋糕', '需冷藏', 'new', 'default', 12.0,
       '周先生', '吴女士', null, 10036, 10037,
       10014, '13312345678', '13798765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000019');


-- task20
INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10038, 29.78, 106.78, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10038);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10038, 10038, '重庆市巴南区鱼洞29号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10038);

INSERT INTO address(id, latitude, longitude, altitude, accuracy)
SELECT 10039, 29.88, 106.88, 1000, 1
WHERE NOT EXISTS (SELECT 1 FROM public.address WHERE id = 10039);
INSERT INTO task_address(id, address_id, address_detail, city)
SELECT 10039, 10039, '重庆市北碚区城南30号', '重庆'
WHERE NOT EXISTS (SELECT 1 FROM public.task_address WHERE id = 10039);

INSERT INTO public.delivery_task (id, tag, items, note, status, priority, estimated_income, username_send, username_receive,
                                  username_delivery, address_start_id, address_end_id, user_id_created_by, telephone_send,
                                  telephone_receive, telephone_delivery, user_id_delivery_by, create_time, arrived_time)
SELECT 'DO-20250807-00000020', 'food', '生日蛋糕', '需冷藏', 'new', 'default', 12.0,
       '周先生', '吴女士', null, 10038, 10039,
       10015, '13312345678', '13798765432', null,
       null, now(), null
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_task WHERE id = 'DO-20250807-00000020');

-- end [create task]

-- end

-- begin [ task_view ]


-- 插入数据应该避免手动指定id, 否则需要执行以下语句来将序列变更为最大值
SELECT setval('user_id_seq', (SELECT MAX(user_id) FROM public.user));
SELECT setval('store_id_seq', (SELECT MAX(store_id) FROM store));
SELECT setval('address_id_seq', (SELECT MAX(id) FROM address));
SELECT setval('delivery_person_id_seq', (SELECT MAX(delivery_person_id) FROM delivery_person));
SELECT setval('task_address_id_seq', (SELECT MAX(id) FROM task_address));
SELECT setval('task_person_id_seq', (SELECT MAX(task_person_id) FROM task_person));