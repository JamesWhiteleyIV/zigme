// Initial main page fetch for alarm state
/** @type {import('./$types').PageServerLoad} */

export async function load({ fetch }) {
	const alarmStateResponse = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/alarm_state`);
	const alarmEventsResponse = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/events`);
	const alarmState: AlarmState = await alarmStateResponse.json();
	const alarmEvents: AlarmEvent = await alarmEventsResponse.json();
	const data = { alarmState, alarmEvents };
	console.log(data);
	return data;
}
