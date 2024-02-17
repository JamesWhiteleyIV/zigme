import requests


# res =requests.post('http://localhost:5173/alarmtrigger', 
#               json={
#                 "title": "test-title",
#                 "message": "test-message",
#               })
# print(res.json())

res =requests.put('http://localhost:3000/alarm_state', json={"phone_alarms": False, "phone_notifications": True, "local_siren": False})
# res =requests.put('http://localhost:3000/alarm_state', json={"phone_alarms": True, "phone_notifications": False, "local_siren": False})
# res =requests.put('http://localhost:3000/alarm_state', json={"phone_alarms": False, "phone_notifications": False, "local_siren": False})
# res =requests.put('http://localhost:3000/alarm_state', json={"phone_alarms": True, "phone_notifications": True, "local_siren": True})
print(res.text)
# res =requests.get('http://localhost:3000/alarm_state')
# print(res.text)
res = requests.post('http://localhost:3000/alarm_trigger', json={"title": "hello", "message": "world"})
print(res.text)