-- Add migration script here
CREATE TABLE M_USER (
    USER_ID         VARCHAR(36) PRIMARY KEY
    , USER_NAME     VARCHAR(165)
    , EMAIL         VARCHAR(255)
    , PASSWORD      VARCHAR(255)
    , SALT          VARCHAR(255)
    , ROLE          VARCHAR(1)
    , STATUS        VARCHAR(1) DEFAULT '1'
    , CREATED_AT    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    , UPDATED_AT    TIMESTAMP DEFAULT NULL
    , LAST_LOGIN_AT TIMESTAMP DEFAULT NULL
    , DELETE_FLAG   BOOLEAN DEFAULT FALSE
)
;