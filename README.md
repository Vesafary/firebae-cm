# firebae-cm
Rust crate for Firebase Cloud Messaging Http V1 

### Information
I recommend to read up on FCM before attempting to use this crate.

| What | Where | 
| ---- | ----- | 
| Basic information on FCM | https://firebase.google.com/docs/cloud-messaging |
| Used data structure | https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages |


### Usage
``` rust
use firebae_cm::{Client, Message, MessageBody, Receiver};
    
async fn send_message() {
    // Setup message
    let receiver = Receiver::topic("subscribers");
    let empty_body = MessageBody::new(receiver);
    let message = Message::new("your_project_id", "your_jwt_token", empty_body);

    // Create client and send message
    let client = Client::new();
    let res: Result<String, firebae_cm::Error> = client.send(message).await;
}

```
This will result in a post request to
`https://fcm.googleapis.com/v1/projects/your_project_id/messages:send`
with the appropriate `Authentication: Bearer your_jwt_token` header
and the following body:
``` json
{
   "message":{
      "topic":"subscribers",
   }
}
```

For more useful examples, see the examples folder.