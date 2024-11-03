# Simple Pub Sub

## Connect as Message Publisher

***Simple Pub Sub*** publishers open a TCP connection on ```127.0.0.1:8060``` to publish messages.
The session must be initialized by the client providing ***project*** and ***topic***. After that, the publisher can send 
messages using the session. These steps are described with the following code snippets using ***netcat***.

```
$ nc 127.0.0.1 8060

>> {
"status":"SessionStarted",
"session_id":"9c833704-1596-446e-840f-b8275e06f49a",
"message":"Session has started"}

$ {"project": "my-project", "topic": "my-topic"}

>> {
"status":"SessionInitialized",
"session_id":"9c833704-1596-446e-840f-b8275e06f49a",
"message":"Initialized session with project my-project and topic my-topic"
}

$ {"data": "my-data"}

>> {
"status":"SessionMessagePublished",
"session_id":"9c833704-1596-446e-840f-b8275e06f49a",
"message":"Successfully published message"
}
```

## Connect as Message Subscriber

## HTTP Endpoints

***Simple Pub Sub*** provides HTTP endpoints for creating topics, subscriptions and messages.
It's also possible to consume messages.
The HTTP endpoints are mainly for development and debugging purpose as well as for a future ***Simple Pub Sub Dashboard***.

### Create a topic

```
curl --location 'http://127.0.0.1:8080/v1/projects/my-project/topics' \
--header 'Content-Type: application/json' \
--data '{
    "topic": "my-topic"
}'
```

### Create a subscription

```
curl --location 'http://127.0.0.1:8080/v1/projects/my-project/topics/my-topic/subscriptions' \
--header 'Content-Type: application/json' \
--data '{
    "subscription": "my-subscription"
}'
```

### Publish a message

```
curl --location 'http://127.0.0.1:8080/v1/projects/my-project/topics/my-topic/messages' \
--header 'Content-Type: application/json' \
--data '{
    "data": "ewogICAgInVzZXJJZCI6ICI2MmRkNDEyNy0wMjY5LTRlNjEtODFiYi1jNDFkZTI0NzEwY2YiLAogICAgInVzZXJuYW1lIjogIlBhdWwgV2llbGFuZCIsCiAgICAic2lnblVwVGltZSI6ICIyMDI0LTExLTAxVDIyOjU3OjI0LjMwMiswMDowMCIKfQ==",
    "attributes": {
        "type": "UserRegisteredEvent"
    }
}'
```

# Architecture Overview

## Message Publisher and Subscriber Flow

<p align="center">
<img src="/drawing/architecture.png" alt=""/>
</p>

https://chesedo.me/blog/manual-dependency-injection-rust/

https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#the-repository-pattern-in-rust