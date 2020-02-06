CREATE TABLE users
(
    id          bigserial    not null
        constraint users_pkey
            primary key,
    team_number integer,
    team_type   varchar(255),
    login       varchar(255),
    password    varchar(255),
    inserted_at timestamp(0) not null,
    updated_at  timestamp(0) not null
);

INSERT INTO public.users (id, team_number, team_type, login, password, inserted_at, updated_at) VALUES (1, 1, 'supplier', 'supplier1', '$argon2id$v=19$m=65536,t=10,p=8$uMGy1regLkduJBjv6hHzPw$VfcE6iLYob/gjfOuljXHLIYUptFgQmRJZQiL3ZE4dkM', '2020-01-19 18:29:39', '2020-01-19 18:29:39');
INSERT INTO public.users (id, team_number, team_type, login, password, inserted_at, updated_at) VALUES (2, 2, 'supplier', 'supplier2', '$argon2id$v=19$m=65536,t=10,p=8$TrvXf1BiX0M047a2q+YwTg$W46VJ/gW5AsJuDlQh2BFINZJ/XX0NOtmuYQexWsAZos', '2020-01-19 18:29:39', '2020-01-19 18:29:39');
INSERT INTO public.users (id, team_number, team_type, login, password, inserted_at, updated_at) VALUES (3, 1, 'consumer', 'consumer1', '$argon2id$v=19$m=65536,t=10,p=8$j6bSuUFzVFPlI21MIdUvKw$/MYzNqtOIiUhuXYJN7Xxki3WJpqscoqWklDEOew9gNI', '2020-01-19 18:29:39', '2020-01-19 18:29:39');
INSERT INTO public.users (id, team_number, team_type, login, password, inserted_at, updated_at) VALUES (4, 2, 'consumer', 'consumer2', '$argon2id$v=19$m=65536,t=10,p=8$MavBd7sSgR/1H5NFkkkx+w$euqLCoOU3b7qF8Ojg83C5DG6ni0cdqvU/INkU6m1ew0', '2020-01-19 18:29:39', '2020-01-19 18:29:39');
INSERT INTO public.users (id, team_number, team_type, login, password, inserted_at, updated_at) VALUES (5, 2, 'consumer', 'consumer2', 'foo', '2020-01-19 18:29:39', '2020-01-19 18:29:39');