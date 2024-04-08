import { writable } from 'svelte/store';

export const principal = writable('');
export const identity = writable(null);
export const loggedIn = writable(false);

