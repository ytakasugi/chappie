# chappie

## Execute

```text
curl -X POST -H "Content-Type: application/json" -d @./json/${JSON_FILE_NAME} http://localhost:8080/${ROUTE}
```

## SQL

```sql
TRUNCATE TABLE M_USER;
TRUNCATE TABLE M_PROJECT;
TRUNCATE TABLE M_USER_PROJECT;
TRUNCATE TABLE T_TICKET;

-- check
SELECT * FROM M_USER;
SELECT * FROM M_PROJECT;
SELECT * FROM M_USER_PROJECT;
SELECT * FROM T_TICKET;
```
