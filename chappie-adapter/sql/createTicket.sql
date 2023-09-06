INSERT INTO T_TICKET (
    TICKET_TITLE
    , DESCRIPTION
    , PRIORITY
    , STATUS_ID
    , PROGRESS
    , START_DATE
    , DUE_DATE
    , CREATED_AT
    , UPDATED_AT
    , PARENT_TICKET_ID
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