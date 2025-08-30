
-- 查找所有user
SELECT * FROM public.delivery_app_user du
LEFT JOIN public.user u ON u.user_id = du.user_id
LEFT JOIN public.delivery_person dp ON du.delivery_person_id = dp.delivery_person_id;

SELECT * FROM public.user WHERE telephone = '17815349553';

-- 修改user_name
UPDATE public.user
SET username = '金先生'
WHERE user_id = 10001;

-- 修改status
SELECT delivery_person_id FROM delivery_app_user
    WHERE user_id = 10001;
UPDATE public.delivery_person
    SET status = 'working'
WHERE delivery_person_id = 10001;

-- 创建user
INSERT INTO public.user (user_id, username, password, telephone)
SELECT 10020, '林先生', '123456', '14815349599'
WHERE NOT EXISTS (SELECT 1 FROM public.user WHERE telephone = '14815349599');
INSERT INTO public.delivery_person (delivery_person_id, money, status)
SELECT 10090, 1, 'resting'
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_person WHERE delivery_person_id = 10090);
INSERT INTO public.delivery_app_user (delivery_person_id, user_id)
SELECT 10060, 10020
WHERE NOT EXISTS (SELECT 1 FROM public.delivery_app_user WHERE delivery_person_id = 10090 AND user_id = 10020);


-- 删除user
SELECT delivery_person_id FROM delivery_app_user
WHERE user_id = 10001;

DELETE FROM delivery_person
WHERE delivery_person_id = 10001;

DELETE FROM public.user
WHERE user_id = 10001;

DELETE FROM delivery_app_user
WHERE delivery_person_id = 10001 AND user_id = 10001;


