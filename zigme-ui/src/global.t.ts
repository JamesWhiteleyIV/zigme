type AlarmState = {
	phone_alarms: boolean | null;
	phone_notifications: boolean | null;
	local_siren: boolean | null;
};

type AlarmEvent = {
	title: string;
	message: string;
	timestamp: string;
};

