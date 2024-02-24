<!-- Main rendered page -->
<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import * as Card from '$lib/components/ui/card';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import { Button } from '$lib/components/ui/button';
	import { Sun, Moon, BellOff, BellRing, ChevronsUpDown } from 'lucide-svelte';
	import { toggleMode } from 'mode-watcher';

	/** @type {import('./$types').PageData} */
	export let data;
	console.log(data);
	let alarmState = data.alarmState;
	let alarmEvents = data.alarmEvents;

	async function phoneAlarmOn() {
		const payload = {
			phone_alarms: true,
			phone_notifications: false
		};
		const res = await fetch('/alarm_state', {
			method: 'PUT',
			body: JSON.stringify(payload)
		});
		alarmState = await res.json();
		console.log(alarmState);
	}

	async function phoneAlarmOff() {
		const payload = {
			phone_alarms: false,
			phone_notifications: true
		};
		const res = await fetch('/alarm_state', {
			method: 'PUT',
			body: JSON.stringify(payload)
		});
		alarmState = await res.json();
		console.log(alarmState);
	}

	async function testAlarmTrigger() {
		const res = await fetch('/alarm_trigger', {
			method: 'POST'
		});
		const data = await res.json();
		console.log(data);
	}
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>
			{#if alarmState.phone_alarms}
				<span style="color: red">ALARM IS ON</span>
			{:else}
				<span style="color: green">ALARM IS OFF</span>
			{/if}
		</Card.Title>
	</Card.Header>
	<Card.Content>
		<Button on:click={phoneAlarmOn} variant="outline">
			<BellRing class="mr-2 h-[1.2rem] w-[1.2rem]" />
			Turn ON
		</Button>

		<Button on:click={phoneAlarmOff} variant="outline">
			<BellOff class="mr-2 h-[1.2rem] w-[1.2rem]" />
			Turn OFF
		</Button>

		<Button on:click={toggleMode} variant="outline" size="icon">
			<Sun
				class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
			/>
			<Moon
				class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
			/>
			<span class="sr-only">Toggle theme</span>
		</Button>
	</Card.Content>
	<Card.Footer>
		<Collapsible.Root class="w-[200px] space-y-2">
			<div class="flex items-center justify-between space-x-4 px-4">
				<h4 class="text-sm font-semibold">Admin Testing</h4>
				<Collapsible.Trigger asChild let:builder>
					<Button builders={[builder]} variant="ghost" size="sm" class="w-12 p-0">
						<ChevronsUpDown class="h-4 w-4" />
					</Button>
				</Collapsible.Trigger>
			</div>
			<Collapsible.Content class="space-y-2">
				<Button on:click={testAlarmTrigger} variant="destructive">Test Alarm Trigger</Button>
			</Collapsible.Content>
		</Collapsible.Root>
	</Card.Footer>
</Card.Root>

<Table.Root>
	<Table.Header>
		<Table.Row>
			<Table.Head class="w-[100px]">Timestamp</Table.Head>
			<Table.Head>Sensor Location</Table.Head>
			<Table.Head>Message</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each alarmEvents as event, i (i)}
			<Table.Row>
				<Table.Cell class="font-medium">{event.timestamp}</Table.Cell>
				<Table.Cell>{event.title}</Table.Cell>
				<Table.Cell>{event.message}</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>
