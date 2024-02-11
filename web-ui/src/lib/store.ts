import { writable } from 'svelte/store';

// critical alert to pushover alarm
export const phoneAlarmState = writable(false);

// TODO: possible future ideas:
// general notification to pushover
// export const phoneNotificationState = writable(false);
// local alarm siren
// export const localAlarmState = writable(false);
