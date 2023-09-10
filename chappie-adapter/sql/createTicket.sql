INSERT INTO T_TICKET (
    TICKET_TITLE
    , TICKET_TYPE_ID
    , DESCRIPTION
    , PRIORITY
    , STATUS_ID
    , PROGRESS
    , START_DATE
    , DUE_DATE
    , CREATED_AT
    , UPDATED_AT
    , PROJECT_ID
    , ASSIGNEE_ID
) VALUES (
    $1
    , $2
    , $3
    , $4
    , $5
    , $6
    , $7
    , $8
    , $9
    , $10
    , $11
    , $12
)