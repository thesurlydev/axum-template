-- migrations/20251229154254_seed_data.sql
INSERT INTO users (id, username, email, created_by, created_at, modified_by, modified_at)
VALUES ('00000000-0000-0000-0000-000000000001', 'admin', 'shane@surly.dev', NULL, NOW(), NULL, NOW());

-- password: test
INSERT INTO user_auth
(user_id, password_hash, created_at, modified_at)
VALUES('00000000-0000-0000-0000-000000000001', '$argon2id$v=19$m=19456,t=2,p=1$tHlCAKvzFeCc+3ncyIa54g$M/ZfaM8ppfbaUgaJpupMxm7PoHxM6obCkQtjBRlLVHk', NOW(), NOW());
