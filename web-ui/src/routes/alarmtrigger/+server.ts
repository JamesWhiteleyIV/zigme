import { phoneAlarmState } from "$lib/store";
import { json } from '@sveltejs/kit';

let phoneAlarmStateValue = false;
phoneAlarmState.subscribe(value => {
    value ? phoneAlarmStateValue = true : phoneAlarmStateValue = false;
});

export type AlarmPayload = {
    // type: "phone" | "local-siren";
    title: string;
    message: string;
};

// send request to pushover to trigger alarm
async function sendPushoverAlarm(title: string, message: string) {
    const payload = {
        token: import.meta.env.VITE_PUSHOVER_API_TOKEN,
        user: import.meta.env.VITE_PUSHOVER_GROUP_TOKEN,
        title: title,
        message: message,
        priority: 1,
        sound: "persistent",
    };
    const res = await fetch('https://api.pushover.net/1/messages.json', {
        method: 'POST',
        body: JSON.stringify(payload)
    })
    const json = await res.json()
    console.log(json);
}

// send request to pushover to trigger notification
async function sendPushoverNotification(title: string, message: string) {
    const payload = {
        token: import.meta.env.VITE_PUSHOVER_API_TOKEN,
        user: import.meta.env.VITE_PUSHOVER_GROUP_TOKEN,
        title: title,
        message: message,
        priority: 0,
    };
    const res = await fetch('https://api.pushover.net/1/messages.json', {
        method: 'POST',
        body: JSON.stringify(payload)
    })
    const json = await res.json()
    console.log(json);
}

/** @type {import('./$types').RequestHandler} */
export function POST({ request }) {
	// return new Response(String(random));
    if (phoneAlarmState) {
        return json({ok: "sounding alarm!"});
    } else {
        return json({ok: "notification!"});
    }
}