import { phoneAlarmState } from "$lib/store";
import { json } from '@sveltejs/kit';

let phoneAlarmStateValue = false;
phoneAlarmState.subscribe(value => {
    value ? phoneAlarmStateValue = true : phoneAlarmStateValue = false;
});

export type AlarmPayload = {
    title: string;
    message: string;
};

// send request to pushover to trigger alarm
async function sendPushoverAlarm(title: string, message: string) {
    const payload = {
        token: import.meta.env.VITE_PUSHOVER_API_TOKEN,
        user: import.meta.env.VITE_PUSHOVER_GROUP_KEY,
        title: title,
        message: message,
        priority: 1,
        sound: "persistent",
    };
    const res = await fetch('https://api.pushover.net/1/messages.json', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
          },
        body: JSON.stringify(payload),
    })
    const json = await res.json()
    return json;
}


// send request to pushover to trigger notification
async function sendPushoverNotification(title: string, message: string) {
    const payload = {
        token: import.meta.env.VITE_PUSHOVER_API_TOKEN,
        user: import.meta.env.VITE_PUSHOVER_GROUP_KEY,
        title: title,
        message: message,
        priority: 0,
    };
    const res = await fetch('https://api.pushover.net/1/messages.json', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
          },
        body: JSON.stringify(payload)
    })
    const json = await res.json()
    return json;
}

/** @type {import('./$types').RequestHandler} */
export async function POST({ request }) {
    const alarmPayload: AlarmPayload = await request.json();
    if (phoneAlarmStateValue) {
        const result = await sendPushoverAlarm(alarmPayload.title, alarmPayload.message);
        return json(result);
    } else {
        const result = await sendPushoverNotification(alarmPayload.title, alarmPayload.message);
        return json(result);
    }
}