# Simple Pub Sub

## Connect as Message Publisher

***Simple Pub Sub*** publishers open a TCP connection port ```8060``` to publish messages.
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

***Simple Pub Sub*** subscribers open a TCP connection o port ```8070``` to receive messages.
After the session has started, the client must provide ***project***, ***topic*** and ***subscription*** to receive messages.
The client can then listen for messages to arrive.

```
$ nc 127.0.0.1 8060

>> {
"status":"SubscriberSessionStarted",
"session_id":"aa702172-8b3e-4486-a09b-655e62e33726",
"message":"Session has started"
}

$ {
"project": "my-project", 
"topic": "my-topic", 
"subscription": "my-subscription"
}

>> {
"status":"SubscriberSessionInitialized",
"session_id":"aa702172-8b3e-4486-a09b-655e62e33726",
"message":"Initialized session with project my-project and topic my-topic and subscription my-subscription"
}

>> {
"project":"my-project",
"topic":"my-topic",
"subscription":"my-subscription",
"message_id":"56380ac2-aeff-4f36-9f85-314d8127b784",
"data":"my-data",
"publish_time":"2024-11-03T13:29:28.304022Z",
"attributes":null,
"acknowledged":false
}
```

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