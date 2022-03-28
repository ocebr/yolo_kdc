-- Add migration script here

create table yolo_bundle (
    id serial primary key,
    name_ varchar not null,
    identity_key varchar not null,
    signed_pre_key varchar not null,
    signature_ varchar not null,
    one_time_pre_key varchar not null
);