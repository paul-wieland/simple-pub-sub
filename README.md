# Simple Pub Sub


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

https://chesedo.me/blog/manual-dependency-injection-rust/

https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#the-repository-pattern-in-rust