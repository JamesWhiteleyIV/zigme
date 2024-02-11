import requests


res =requests.post('http://localhost:5173/alarmtrigger', 
              json={
                "title": "test-title",
                "message": "test-message",
              })
print(res.json())