CREATE TABLE IF NOT EXISTS tokens ();

ALTER TABLE tokens
  ADD COLUMN id SERIAL PRIMARY KEY,
  ADD COLUMN user_id SERIAL NOT NULL UNIQUE,
  ADD COLUMN fcm_token VARCHAR DEFAULT NULL,
  ADD COLUMN web_token JSONB DEFAULT NULL
  ;

INSERT INTO tokens VALUES
  (default, 1, NULL, '{ "endpoint": "https://updates.push.services.mozilla.com/wpush/v2/gAAAAABks_33tHmTlwkBDO7YrhGQveg2-djKwa0ElCGGK4WRnHmPkAjbptJTfnt5yA8wujasgL7LMhmAC2zmkRLi2aaFmNvKH29cKZNKaDWyAfLMwslmT7d9ndXCMTBpldJ3gNyrNom3O_EsOZPsweE_0fd2Q5BCCofh7eTdyHlMYR3fzphiNkk", "expirationTime": null, "keys": { "auth": "ZF2TXINVg_HatgUrq-vMlQ", "p256dh": "BCDSOurLX5C0skbIx0Z4nrlnB1OBFqYOhMMT-jT_Rs0-n2yHHmKG6IMhh8anhSDhAuQeysgOsEbbmQc8pJaTHNA" } }')
  ;
