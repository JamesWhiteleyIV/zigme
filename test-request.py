import requests


# res =requests.post('http://localhost:5173/alarmtrigger', 
#               json={
#                 "title": "test-title",
#                 "message": "test-message",
#               })
# print(res.json())

res =requests.put('http://localhost:5173/alarmstate/true')
print(res.json())