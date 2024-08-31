/**
 * This file contains functions for managing application settings.
 * Settings are persisted in <Application Data>/settings.json and are used by both the UI and the backend.
 *
 * TODO: Rewrite this to be an injectable object so we don't need `storeInstance`.
 */

import { createStore } from '@tauri-apps/plugin-store';
import { writable, type Writable } from 'svelte/store';

const store = createStore('settings.json', { autoSave: true });

/**
 * Persisted confirmation that user has confirmed their analytics settings.
 */
export function appAnalyticsConfirmed() {
	return persisted(false, 'appAnalyticsConfirmed');
}

/**
 * Provides a writable store for obtaining or setting the current state of application metrics.
 * The application metrics can be enabled or disabled by setting the value of the store to true or false.
 * @returns A writable store with the appMetricsEnabled config.
 */
export function appMetricsEnabled() {
	return persisted(true, 'appMetricsEnabled');
}

/**
 * Provides a writable store for obtaining or setting the current state of application error reporting.
 * The application error reporting can be enabled or disabled by setting the value of the store to true or false.
 * @returns A writable store with the appErrorReportingEnabled config.
 */
export function appErrorReportingEnabled() {
	return persisted(true, 'appErrorReportingEnabled');
}

/**
 * Provides a writable store for obtaining or setting the current state of non-anonemous application metrics.
 * The setting can be enabled or disabled by setting the value of the store to true or false.
 * @returns A writable store with the appNonAnonMetricsEnabled config.
 */
export function appNonAnonMetricsEnabled() {
	return persisted(false, 'appNonAnonMetricsEnabled');
}

function persisted<T>(initial: T, key: string): Writable<T> & { onDisk: () => Promise<T> } {
	async function setAndPersist(value: T, set: (value: T) => void) {
		const storeInstance = await store;
		await storeInstance.set(key, value);
		await storeInstance.save();

		set(value);
	}

	async function synchronize(set: (value: T) => void): Promise<void> {
		const value = await storeValueWithDefault(initial, key);
		set(value);
	}

	function update() {
		throw 'Not implemented';
	}

	const thisStore = writable<T>(initial, (set) => {
		synchronize(set);
	});

	async function set(value: T) {
		setAndPersist(value, thisStore.set);
	}

	async function onDisk() {
		return await storeValueWithDefault(initial, key);
	}

	const subscribe = thisStore.subscribe;

	return {
		subscribe,
		set,
		update,
		onDisk
	};
}

async function storeValueWithDefault<T>(initial: T, key: string): Promise<T> {
	const storeInstance = await store;
	try {
		await storeInstance.load();
	} catch (e) {
		// If file does not exist, reset it
		storeInstance.reset();
	}
	const stored = (await storeInstance.get(key)) as T;

	if (stored === null) {
		return initial;
	} else {
		return stored;
	}
}
