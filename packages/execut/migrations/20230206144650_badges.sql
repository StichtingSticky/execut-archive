create table badges
  ( id                uuid
                      primary key
                      default gen_random_uuid()
  , user_id           uuid
                      not null
  , badge_code        text
                      not null

  ,       foreign key (user_id)
           references users (id)
            on delete cascade
            on update cascade
  );

create table scans
  ( id                uuid
                      primary key
                      default gen_random_uuid()
  , initiator_id      uuid
                      not null
  , subject_id        uuid
                      not null
  , created_at        timestamp with time zone
                      not null
                      default now()
  , updated_at        timestamp with time zone

  -- ,            unique (initiator_id, subject_id)

  ,       foreign key (initiator_id)
           references users (id)
            on delete cascade
            on update cascade

  ,       foreign key (subject_id)
           references users (id)
            on delete cascade
            on update cascade
  );

-- insert into badges
--   ( user_id
--   , badge_code
--   )
-- values
--   ( (select id from users where name = 'Sem van Nieuwenhuizen' )
--   , 'slqtjubiqz'
--   );
