## Texter

This is a simple wrapper around aws-sdk-sns functions to help you publish text/topics.

### For publishing it to topic
1. Create a topic under SNS and subscribe email ids/ contact numbers of people that needs to be notified.
2. Use that topic-arn in the policy document to publish it to topic

#### Policy template

```
  {
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "VisualEditor0",
            "Effect": "Allow",
            "Action": "sns:Publish",
            "Resource": <valid-topic-arn>
        }
    ]
}
```

### For publishing it to contact number
1. No need to create a topic.
2. In the following policy document let the resource field be set to all.

#### Policy template

```
  {
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "VisualEditor0",
            "Effect": "Allow",
            "Action": "sns:Publish",
            "Resource": <valid-topic-arn>
        }
    ]
}
```