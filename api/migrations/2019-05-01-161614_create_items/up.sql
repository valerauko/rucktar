create table items (
  id serial primary key,

  name varchar not null,
  description text not null,

  created_at timestamptz not null default now()
)
