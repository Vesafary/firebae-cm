# firebae-cm
Rust crate for Firebase Cloud Messaging Http V1 

### Information
I recommend to read up on FCM before attempting to use this crate.

| What | Where | 
| ---- | ----- | 
| Basic information on FCM | https://firebase.google.com/docs/cloud-messaging?authuser=0 |
| Used data structure | https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages |


### Usage
``` rust
use firebae_cm::{
    Client,
    Message,
    MessageBody,
    Receiver,
    Notification,
    AndroidConfig,
};

#[derive(serde::Serialize)]
pub struct MessageData {
    pub custom_field: String,
}

fn main {
    let device_token = "your_fcm_device_token";
    let jwt_token = "your_jwt_token";

    // define data to send via FCM
    let data = MessageData {
        custom_field: "Hello, world!".to_string(),
    };

    let notification = Notification {
        title: Some("Custom title".to_string()),
        body: Some("Custom body".to_string()),
        image: None,
    }

    // set android settings
    let mut android_config = AndroidConfig::new();
    android_config
        .direct_boot_ok(true);
    
    // define the receiver mode (Token, Topic or Contition)
    let receiver = Receiver::Token(device_token);
    
    // create a message to our receiver(s) and set all configuration
    let mut body = MessageBody::new(receiver);
    body
        .data(data)?
        .android(android_config);
    
    // finalize the message with the correct url and authentication 
    let message = Message::new("project_id", token, body);
    
    // create a client and send the data
    let client = Client::new();
    println!("{:?}", client.send(message).await?);
    
    
    
}
```
This will result in a post request to
`https://fcm.googleapis.com/v1/projects/project_id/messages:send`
with the appropriate `Authentication: Bearer your_jwt_token` header
and the following body:
``` json
{
   "message":{
      "token":"your_fcm_device_token",
      "notification":{
        "title":"Custom title",
        "body":"Custom body"
      },
      "data" : {
        "custom_field" : "Hello, world!",
      },
      "android": {
        "direct_boot_ok": true
      }
   }
}
```