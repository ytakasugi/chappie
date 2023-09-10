# chappie

## ユーザー作成

```text
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "admin", "email": "admin@email.com", "password": "PasswordAdmin", "role": "0"}' http://localhost:8080/users
curl -X POST -H "Content-Type: application/json" -d '{"project_name": "51_Integration_Test3", "description": "Integration_Test3",  "parent_project_id": null, "manager_id": "01H8Y39CDFYSY86PMB5D4C8YNA"}' http://localhost:8080/projects
curl -X POST -H "Content-Type: application/json" -d '{"ticket_title": "create test plan", "ticket_type_id": 1, "description": "create test plan next version", "priority": 9,  "status_id": 9, "progress": 0, "start_date": "2023-09-01", "due_date": "2023-12-31", "parent_ticket_id": null, "project_id": "01H93DKA54R7HB80098QRQ7WW0", "assignee_id": "01H95VREP370GZ080BBH4EZQ6J"}' http://localhost:8080/tickets
curl -X POST -H "Content-Type: application/json" -d '{"user_id": "01H93DKA54R7HB80098QRQ7WW0", "project_id": "01H93DKA54R7HB80098QRQ7WW0", "role": "9"}' http://localhost:8080/user_projects
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
