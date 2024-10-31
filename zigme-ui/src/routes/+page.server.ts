// Initial main page fetch for alarm state
/** @type {import('./$types').PageServerLoad} */

// TODO: need to make these API URIs not hardcoded

export async function load({ fetch }) {
	//const alarmStateResponse = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/alarm_state`);
	//const alarmEventsResponse = await fetch(`${import.meta.env.VITE_ZIGME_API_URI}/events`);
	const alarmStateResponse = await fetch("http://zigme-api:3020/alarm_state");
	const alarmEventsResponse = await fetch("http://zigme-api:3020/events");

	const alarmState: AlarmState = await alarmStateResponse.json();
	const alarmEvents: AlarmEvent = await alarmEventsResponse.json();

	const data = { alarmState, alarmEvents };

	// console.log(data);
	return data;
}
