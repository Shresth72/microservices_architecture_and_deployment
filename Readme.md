# Microservices Architecture in nodeJs / rust (rocket.rs)

**Initial Implementation**
-
- Multiple services that communicate through message broker (Apache Kafka / RabbitMQ)
- Nginx to act as reverse proxy for managing request APIs from client
- Dockerized Images for database, services and reverse proxy to make it easily deployable on cloud

**RCP Implementation**
-
- Segregate Responsibilities to follow pure CQRS
    - [Learn about Command and Query Responsibility Segregation](https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs)
- Optimizing services for scalability
- Refactoring codebase and keeping meaningful data close to the services
- Revoke redundant data from services / DB
- Isolate DB instance with special access to perform DBA operations


**Continuous Deployment Setup**
-
- Working...