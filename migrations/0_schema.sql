create table promocode (
    code varchar primary key,
    expire_date date not null,
    discount_type varchar not null,
    discount_amount integer not null
);

create table "order_info" (
    id integer primary key generated always as identity,
    duration smallint not null,
    simulator varchar not null,
    payment_status varchar not null,
    promocode varchar references "promocode",
    comment text,
    created_at timestamp default current_timestamp,
    modified_at timestamp default current_timestamp
);

create table "direct_order" (
    order_info integer primary key references "order_info",
    client_phone varchar not null,
    client_email varchar not null,
    client_name varchar not null,
    client_provider varchar not null,
    time timestamp not null
);

-- separate table for gift_code as there are physical pre-generated codes 
create table "gift_code" (
    code varchar(4) primary key,
    -- expire date could always be the end of the year 
    expire_date date not null,
    is_physical boolean not null
);

create table "gift_order" (
    order_info integer primary key references "order_info",
    -- if its a physical certificate use random value from gift_code
    -- if not generate a new one
    gift_code varchar(4) references "gift_code" not null unique,
    recipient_phone varchar not null,
    recipient_email varchar not null,
    recipient_name varchar not null,
    -- if null - certificate is physical
    delivery_address varchar,
    client_phone varchar not null,
    client_email varchar not null,
    client_name varchar not null,
    -- client orders a gift - time is set to null for now
    -- then recipient activates the certificate - time is set to the value
    -- that recipient choose
    time timestamp
);
