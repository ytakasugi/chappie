# chappie

## ユーザー作成

```text
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "admin", "email": "admin@email.com", "password": "PasswordAdmin", "role": "0"}' http://localhost:8080/users
curl -X POST -H "Content-Type: application/json" -d '{"project_name": "51_Integration_Test3", "description": "Integration_Test3", "manager_id": "01H8Y39CDFYSY86PMB5D4C8YNA"}' http://localhost:8080/projects
curl -X POST -H "Content-Type: application/json" -d '{"ticket_title": "create test plan", "description": "create test plan next version", "priority": 9,  "status": 9, "progress": 0, "due_date": "2023-12-31", "project_id": 1, "assignee_id": "01H95VREP370GZ080BBH4EZQ6J"}' http://localhost:8080/tickets
curl -X POST -H "Content-Type: application/json" -d '{"user_id": "01H93DKA54R7HB80098QRQ7WW0", "project_id": 1, "role": "9"}' http://localhost:8080/user_projects
```
