create table node
(
    id         bigserial      not null,
    public_key varchar(1000)  not null,
    alias      varchar(50)    not null,
    capacity   numeric(16, 8) not null,
    first_seen timestamptz    not null,
    primary key (id),
    constraint unq_node_public_key unique (public_key),
    constraint unq_node_alias unique (alias)
);
