CREATE TABLE IF NOT EXISTS tokens ();

ALTER TABLE tokens
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id SERIAL NOT NULL UNIQUE,
  ADD COLUMN fcm_token VARCHAR DEFAULT NULL,
  ADD COLUMN web_token JSONB DEFAULT NULL
  ;

-- At the beginning, the table is empty
-- ALTER TABLE tokens
--   ADD CONSTRAINT check_tokens_fcm_token_or_web_token
--   CHECK (fcm_token IS NOT NULL OR web_token IS NOT NULL);

INSERT INTO tokens VALUES
  (default, 1, 'fcm_token', NULL),
  (default, 2, NULL, '{"token": "web_token"}');
