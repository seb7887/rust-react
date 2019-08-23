create table books (
  id serial primary key,
  user_id integer not null,
  title varchar(255) not null,
  author varchar(255) not null,
  cover varchar(255),
  page_count integer default 0,
  publisher varchar(255),
  synopsis varchar(255),
  created_at timestamp with time zone not null,
  updated_at timestamp with time zone not null,
  foreign key (user_id) references users(id) on delete cascade
)