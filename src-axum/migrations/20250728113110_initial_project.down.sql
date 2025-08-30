-- Add down migration script here

-- 删除全部数据
CREATE OR REPLACE FUNCTION truncate_if_exists(
    p_table_name text,
    p_schema_name text DEFAULT 'public'
) RETURNS void AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.tables
               WHERE table_schema = p_schema_name
                 AND table_name = p_table_name) THEN
        EXECUTE format('TRUNCATE TABLE %I.%I', p_schema_name, p_table_name);
        RAISE NOTICE '表 %.% 已清空', p_schema_name, p_table_name;
    ELSE
        RAISE NOTICE '表 %.% 不存在，跳过操作', p_schema_name, p_table_name;
    END IF;
END;
$$ LANGUAGE plpgsql;



-- 删除部分数据
-- DELETE FROM public.store WHERE telephone = '13800138000';


-- task_view
DROP VIEW IF EXISTS task_view;


-- delivery_task
-- TRUNCATE TABLE public.delivery_task;

SELECT truncate_if_exists('public', 'delivery_task');

DROP INDEX IF EXISTS idx_delivery_by;
DROP INDEX IF EXISTS idx_created_by;

DROP TABLE IF EXISTS public.delivery_task;


-- task_address
-- TRUNCATE TABLE public.task_address;
SELECT truncate_if_exists('public', 'task_address');

DROP TABLE IF EXISTS public.task_address;

DROP SEQUENCE IF EXISTS task_address_id_seq;


-- user_address
-- TRUNCATE TABLE public.user_address;
SELECT truncate_if_exists('public', 'task_app_user_address');
DROP TABLE IF EXISTS public.task_app_user_address;


-- address
-- TRUNCATE TABLE public.address;
SELECT truncate_if_exists('public', 'address');
DROP TABLE IF EXISTS public.address;

DROP SEQUENCE IF EXISTS address_id_seq;

-- task_app_user
-- TRUNCATE TABLE public.task_app_user;
SELECT truncate_if_exists('public', 'task_app_user');

DROP TABLE IF EXISTS public.task_app_user;


-- task_person
-- TRUNCATE TABLE public.task_person;
SELECT truncate_if_exists('public', 'task_person');

DROP TABLE IF EXISTS public.task_person;

DROP SEQUENCE IF EXISTS task_person_id_seq;


-- delivery_app_user
-- TRUNCATE TABLE public.delivery_app_user;
SELECT truncate_if_exists('public', 'delivery_app_user');
DROP TABLE IF EXISTS public.delivery_app_user;


-- delivery_person table
-- TRUNCATE TABLE public.delivery_person;
SELECT truncate_if_exists('public', 'delivery_person');

DROP TABLE IF EXISTS public.delivery_person;

DROP SEQUENCE IF EXISTS delivery_person_id_seq;

-- user table
-- TRUNCATE TABLE public.user;
SELECT truncate_if_exists('public', 'user');

DROP INDEX IF EXISTS idx_user_telephone;

DROP TABLE IF EXISTS public.user;

DROP SEQUENCE IF EXISTS user_id_seq;




-- store table
-- TRUNCATE TABLE public.store;
SELECT truncate_if_exists('public', 'store');
DROP INDEX IF EXISTS idx_store_status;
DROP INDEX IF EXISTS idx_store_telephone;

DROP TABLE IF EXISTS store;

DROP SEQUENCE IF EXISTS store_id_seq;
