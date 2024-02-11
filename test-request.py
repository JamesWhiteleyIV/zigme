import requests


res =requests.post('http://localhost:5173/alarmtrigger', 
              json={
                "test": "ok"
              })
print(res.json())