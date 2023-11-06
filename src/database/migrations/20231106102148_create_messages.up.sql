CREATE TABLE IF NOT EXISTS messages ();

ALTER TABLE messages
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN title VARCHAR NOT NULL,
  ADD COLUMN body VARCHAR,
  ADD COLUMN message_type VARCHAR NOT NULL DEFAULT 'info',
  ADD COLUMN content TEXT[]  NOT NULL DEFAULT '{}',
  ADD COLUMN data JSONB NOT NULL DEFAULT '{}'
  ;

INSERT INTO messages (title, body, message_type, content, data) VALUES
  ('Welcome', 'Welcome to QWebApp!', 'info', '{"Welcome to QWebApp!!","We hope you enjoy it 🖖"}', '{"path": "/home"}'),
  ('Reminder', '', 'alert', '{"Dont forget to complete the pending resoruces.","🙇"}', '{"path": "/modules"}'),
  ('New Content', '', 'info', '{"There is new content waiting for you"}', '{"path": "/home"}'),
  ('Complete', '', 'info', '{"The protocol is complete!", "Thanks a lot for your collaboration"}', '{"path": "/home"}')
  ;
