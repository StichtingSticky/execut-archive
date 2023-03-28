create table users
  ( id                uuid
                      primary key
                      default gen_random_uuid()
  , name              text
                      not null
                      unique

  -- Information supplied trough our ticketing system
  , linkedin          text
  , study             text
  , degree            text
  , institution       text
  , graduation_year   text

  , company_slug      text

  , created_at        timestamp with time zone
                      not null
                      default now()
  , updated_at        timestamp with time zone
  );

-- insert into users
--   ( name
--   , linkedin
--   , study
--   , degree
--   , institution
--   , graduation_year
--   )
-- values
--   ( 'Sem van Nieuwenhuizen'
--   , 'https://www.linkedin.com/in/leuke-naam/'
--   , 'Computing Science'
--   , 'Masters'
--   , 'Utrecht University'
--   , '2025'
--   );
