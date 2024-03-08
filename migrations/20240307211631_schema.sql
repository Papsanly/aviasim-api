create table "client" (
    id integer primary key generated always as identity,
    phone varchar not null unique,
    email varchar not null unique,
    first_name varchar not null,
    second_name varchar not null,
    provider varchar not null
);

create table promocode (
    code varchar primary key,
    expire_date date not null,
    discount_type varchar not null,
    discount_amount integer not null
);

create table "order_info" (
    id integer primary key generated always as identity,
    client integer references "client" not null,
    duration smallint not null,
    simulator_type varchar not null,
    payment_status varchar not null,
    promocode varchar references "promocode",
    comment text,
    created_at timestamp default current_timestamp,
    modified_at timestamp default current_timestamp
);

create table "direct_order" (
    order_info integer primary key references "order_info",
    time timestamp not null
);

create table "gift_code" (
    code varchar(4) primary key,
    expire_date date not null,
    is_physical boolean not null
);

create table "gift_order" (
    order_info integer primary key references "order_info",
    gift_code varchar(4) references "gift_code" not null unique,
    recipient integer references "client" not null,
    delivery_address varchar,
    is_activated boolean not null
);