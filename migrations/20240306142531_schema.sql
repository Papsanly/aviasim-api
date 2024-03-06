create table "order" (
    id varchar(6) primary key,
    name varchar(32),
    created_at date default current_date
)
