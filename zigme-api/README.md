# zigme-api
This is an axum based rest api for handling:
- Getting/Setting alarm state from redis db.
- Reading most recent events from redis db.
- Sending request to pushover for triggering a remote device alarm.
- (TODO) Sending request to start/stop a local siren alarm. 
- (TODO) Reading zigbee2mqtt device status list (online/offline + current states)