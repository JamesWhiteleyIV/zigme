/** @type {import('./$types').PageServerLoad} */
export async function load({ params, fetch }) {
    const res = await fetch("/alarmstate")
	// return {phoneAlarmState: };
    return {};
}