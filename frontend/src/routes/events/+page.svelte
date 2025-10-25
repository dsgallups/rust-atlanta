<script lang="ts">
	import { goto } from '$app/navigation';

	interface Event {
		id: number;
		title: string;
		description: string;
		date: Date;
		time: string;
		endTime?: string;
		location: string;
		venue?: string;
		type: 'meetup' | 'workshop' | 'hackathon' | 'social' | 'conference';
		capacity: number;
		attendees: number;
		rsvpRequired: boolean;
		isOnline: boolean;
		meetingLink?: string;
		speaker?: {
			name: string;
			bio: string;
			avatar?: string;
		};
		tags: string[];
		isPast: boolean;
	}

	interface RSVPStatus {
		eventId: number;
		status: 'attending' | 'maybe' | 'not-attending';
	}

	// Mock data
	let events = $state<Event[]>([
		{
			id: 1,
			title: 'Rust Beginners Workshop',
			description: 'Join us for a hands-on introduction to Rust programming. We\'ll cover basic syntax, ownership, borrowing, and build a small CLI application together.',
			date: new Date('2024-02-15'),
			time: '6:30 PM',
			endTime: '8:30 PM',
			location: 'Tech Village Atlanta',
			venue: 'Room 204',
			type: 'workshop',
			capacity: 30,
			attendees: 24,
			rsvpRequired: true,
			isOnline: false,
			speaker: {
				name: 'Sarah Chen',
				bio: 'Senior Rust Developer at TechCorp with 5 years of experience'
			},
			tags: ['beginner', 'hands-on', 'cli'],
			isPast: false
		},
		{
			id: 2,
			title: 'Monthly Rust Meetup: Async Programming',
			description: 'Deep dive into async programming in Rust. We\'ll explore tokio, async-std, and best practices for building concurrent applications.',
			date: new Date('2024-02-20'),
			time: '7:00 PM',
			endTime: '9:00 PM',
			location: 'Atlanta Tech Park',
			venue: 'Main Auditorium',
			type: 'meetup',
			capacity: 100,
			attendees: 45,
			rsvpRequired: true,
			isOnline: false,
			speaker: {
				name: 'Alex Rodriguez',
				bio: 'Rust contributor and async runtime expert'
			},
			tags: ['async', 'tokio', 'advanced'],
			isPast: false
		},
		{
			id: 3,
			title: 'WebAssembly with Rust',
			description: 'Learn how to compile Rust to WebAssembly and build high-performance web applications.',
			date: new Date('2024-02-28'),
			time: '6:00 PM',
			endTime: '7:30 PM',
			location: 'Online',
			type: 'workshop',
			capacity: 200,
			attendees: 67,
			rsvpRequired: false,
			isOnline: true,
			meetingLink: 'https://zoom.us/j/123456789',
			speaker: {
				name: 'Jamie Park',
				bio: 'WebAssembly advocate and WASM tooling maintainer'
			},
			tags: ['wasm', 'web', 'frontend'],
			isPast: false
		},
		{
			id: 4,
			title: 'Rust Game Development Hackathon',
			description: 'Build a game in 48 hours using Rust and Bevy engine. Teams of up to 4 people, prizes for top 3 teams!',
			date: new Date('2024-03-02'),
			time: '9:00 AM',
			endTime: '6:00 PM',
			location: 'Innovation Hub',
			venue: 'Co-working Space',
			type: 'hackathon',
			capacity: 60,
			attendees: 42,
			rsvpRequired: true,
			isOnline: false,
			tags: ['gamedev', 'bevy', 'competition'],
			isPast: false
		},
		{
			id: 5,
			title: 'Rust & Coffee Social',
			description: 'Casual morning meetup to discuss Rust projects, share experiences, and network with fellow Rustaceans.',
			date: new Date('2024-03-09'),
			time: '10:00 AM',
			endTime: '12:00 PM',
			location: 'Inman Perk Coffee',
			type: 'social',
			capacity: 25,
			attendees: 18,
			rsvpRequired: false,
			isOnline: false,
			tags: ['networking', 'casual', 'coffee'],
			isPast: false
		},
		{
			id: 6,
			title: 'Building CLI Tools with Rust',
			description: 'Workshop on creating command-line interfaces with clap, structopt, and other popular Rust CLI libraries.',
			date: new Date('2024-01-20'),
			time: '6:30 PM',
			endTime: '8:30 PM',
			location: 'Tech Village Atlanta',
			type: 'workshop',
			capacity: 40,
			attendees: 38,
			rsvpRequired: true,
			isOnline: false,
			tags: ['cli', 'tools', 'practical'],
			isPast: true
		},
		{
			id: 7,
			title: 'Rust Performance Optimization',
			description: 'Learn profiling techniques, optimization strategies, and how to write blazingly fast Rust code.',
			date: new Date('2024-01-15'),
			time: '7:00 PM',
			endTime: '9:00 PM',
			location: 'Online',
			type: 'meetup',
			capacity: 150,
			attendees: 112,
			rsvpRequired: false,
			isOnline: true,
			tags: ['performance', 'optimization', 'advanced'],
			isPast: true
		}
	]);

	let userRSVPs = $state<RSVPStatus[]>([]);
	let selectedType = $state<string>('all');
	let showOnlineOnly = $state(false);
	let showPastEvents = $state(false);
	let viewMode = $state<'list' | 'calendar'>('list');

	let filteredEvents = $derived(
		events.filter(event => {
			if (!showPastEvents && event.isPast) return false;
			if (showPastEvents && !event.isPast) return false;
			if (showOnlineOnly && !event.isOnline) return false;
			if (selectedType !== 'all' && event.type !== selectedType) return false;
			return true;
		}).sort((a, b) => a.date.getTime() - b.date.getTime())
	);

	function formatDate(date: Date): string {
		return new Intl.DateTimeFormat('en-US', {
			weekday: 'long',
			month: 'long',
			day: 'numeric',
			year: 'numeric'
		}).format(date);
	}

	function formatShortDate(date: Date): string {
		return new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric'
		}).format(date);
	}

	function getEventTypeColor(type: string): string {
		switch (type) {
			case 'workshop':
				return 'bg-blue-100 text-blue-800';
			case 'meetup':
				return 'bg-green-100 text-green-800';
			case 'hackathon':
				return 'bg-purple-100 text-purple-800';
			case 'social':
				return 'bg-yellow-100 text-yellow-800';
			case 'conference':
				return 'bg-red-100 text-red-800';
			default:
				return 'bg-gray-100 text-gray-800';
		}
	}

	function getEventTypeIcon(type: string): string {
		switch (type) {
			case 'workshop': return '🛠️';
			case 'meetup': return '👥';
			case 'hackathon': return '💻';
			case 'social': return '☕';
			case 'conference': return '🎤';
			default: return '📅';
		}
	}

	function getRSVPStatus(eventId: number): RSVPStatus | undefined {
		return userRSVPs.find(r => r.eventId === eventId);
	}

	async function handleRSVP(eventId: number, status: 'attending' | 'maybe' | 'not-attending') {
		// Mock API call
		await new Promise(resolve => setTimeout(resolve, 500));

		const existingIndex = userRSVPs.findIndex(r => r.eventId === eventId);
		if (existingIndex >= 0) {
			userRSVPs[existingIndex].status = status;
		} else {
			userRSVPs = [...userRSVPs, { eventId, status }];
		}

		// Update attendee count
		const event = events.find(e => e.id === eventId);
		if (event && status === 'attending') {
			event.attendees = Math.min(event.attendees + 1, event.capacity);
		}
	}

	function getCalendarDays(): (Date | null)[] {
		const today = new Date();
		const firstDay = new Date(today.getFullYear(), today.getMonth(), 1);
		const lastDay = new Date(today.getFullYear(), today.getMonth() + 1, 0);
		const startDate = new Date(firstDay);
		startDate.setDate(startDate.getDate() - firstDay.getDay());

		const days: (Date | null)[] = [];
		const current = new Date(startDate);

		for (let i = 0; i < 42; i++) {
			if (current.getMonth() === today.getMonth()) {
				days.push(new Date(current));
			} else {
				days.push(null);
			}
			current.setDate(current.getDate() + 1);
		}

		return days;
	}

	function getEventsForDate(date: Date): Event[] {
		return events.filter(event =>
			event.date.getDate() === date.getDate() &&
			event.date.getMonth() === date.getMonth() &&
			event.date.getFullYear() === date.getFullYear()
		);
	}
</script>

<div class="min-h-screen bg-gray-50">
	<!-- Header -->
	<div class="bg-white border-b border-gray-200">
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<div class="md:flex md:items-center md:justify-between">
				<div>
					<h1 class="text-3xl font-bold text-gray-900">Events & Meetups</h1>
					<p class="mt-2 text-gray-600">
						Join us for workshops, hackathons, and social gatherings
					</p>
				</div>
				<div class="mt-4 md:mt-0 flex gap-2">
					<button
						onclick={() => viewMode = viewMode === 'list' ? 'calendar' : 'list'}
						class="inline-flex items-center gap-2 rounded-lg border border-gray-300 bg-white px-4 py-2 font-medium text-gray-700 hover:bg-gray-50"
					>
						{#if viewMode === 'list'}
							📅 Calendar View
						{:else}
							📝 List View
						{/if}
					</button>
					<a
						href="/events/submit"
						class="inline-flex items-center gap-2 rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-4 py-2 font-medium text-white transition-shadow hover:shadow-lg"
					>
						<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
						</svg>
						Host Event
					</a>
				</div>
			</div>
		</div>
	</div>

	<!-- Filters -->
	<div class="mx-auto max-w-7xl px-4 py-6 sm:px-6 lg:px-8">
		<div class="flex flex-wrap items-center gap-4">
			<select
				bind:value={selectedType}
				class="rounded-lg border border-gray-300 bg-white px-4 py-2 focus:border-orange-500 focus:outline-none focus:ring-2 focus:ring-orange-500"
			>
				<option value="all">All Events</option>
				<option value="meetup">Meetups</option>
				<option value="workshop">Workshops</option>
				<option value="hackathon">Hackathons</option>
				<option value="social">Social</option>
				<option value="conference">Conferences</option>
			</select>

			<label class="flex items-center gap-2 cursor-pointer">
				<input
					type="checkbox"
					bind:checked={showOnlineOnly}
					class="rounded border-gray-300 text-orange-600 focus:ring-orange-500"
				/>
				<span class="text-gray-700">Online only</span>
			</label>

			<div class="flex rounded-lg border border-gray-300 overflow-hidden">
				<button
					onclick={() => showPastEvents = false}
					class="px-4 py-2 font-medium {!showPastEvents
						? 'bg-orange-600 text-white'
						: 'bg-white text-gray-700 hover:bg-gray-50'}"
				>
					Upcoming
				</button>
				<button
					onclick={() => showPastEvents = true}
					class="px-4 py-2 font-medium {showPastEvents
						? 'bg-orange-600 text-white'
						: 'bg-white text-gray-700 hover:bg-gray-50'}"
				>
					Past
				</button>
			</div>
		</div>
	</div>

	<!-- Events Display -->
	<div class="mx-auto max-w-7xl px-4 pb-12 sm:px-6 lg:px-8">
		{#if viewMode === 'list'}
			<!-- List View -->
			{#if filteredEvents.length === 0}
				<div class="text-center py-12">
					<div class="text-6xl mb-4">📅</div>
					<p class="text-gray-600">No events found matching your criteria</p>
				</div>
			{:else}
				<div class="space-y-6">
					{#each filteredEvents as event}
						<div class="rounded-lg border border-gray-200 bg-white overflow-hidden shadow-sm hover:shadow-lg transition-shadow">
							<div class="md:flex">
								<!-- Date Card -->
								<div class="md:w-32 bg-gradient-to-br from-orange-500 to-red-600 text-white p-6 text-center">
									<div class="text-3xl font-bold">{event.date.getDate()}</div>
									<div class="text-lg">{formatShortDate(event.date).split(' ')[0]}</div>
									<div class="text-sm mt-2">{event.time}</div>
								</div>

								<!-- Event Details -->
								<div class="flex-1 p-6">
									<div class="flex items-start justify-between">
										<div>
											<div class="flex items-center gap-3 mb-2">
												<span class="text-2xl">{getEventTypeIcon(event.type)}</span>
												<span class="rounded-full px-2 py-1 text-xs font-medium {getEventTypeColor(event.type)}">
													{event.type}
												</span>
												{#if event.isOnline}
													<span class="rounded-full bg-blue-100 px-2 py-1 text-xs font-medium text-blue-800">
														🌐 Online
													</span>
												{/if}
											</div>
											<h3 class="text-xl font-semibold text-gray-900 mb-2">{event.title}</h3>
											<p class="text-gray-600 mb-4">{event.description}</p>

											{#if event.speaker}
												<div class="flex items-center gap-3 mb-4">
													<div class="h-10 w-10 rounded-full bg-gradient-to-br from-orange-400 to-red-500"></div>
													<div>
														<div class="font-medium text-gray-900">{event.speaker.name}</div>
														<div class="text-sm text-gray-600">{event.speaker.bio}</div>
													</div>
												</div>
											{/if}

											<div class="flex flex-wrap gap-4 text-sm text-gray-600">
												<div class="flex items-center gap-1">
													<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
														<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
														<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
													</svg>
													{event.location}
													{#if event.venue}
														- {event.venue}
													{/if}
												</div>
												<div class="flex items-center gap-1">
													<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
														<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
													</svg>
													{event.attendees}/{event.capacity} attending
												</div>
											</div>

											<div class="flex flex-wrap gap-1 mt-4">
												{#each event.tags as tag}
													<span class="rounded bg-gray-100 px-2 py-1 text-xs text-gray-700">
														{tag}
													</span>
												{/each}
											</div>
										</div>
									</div>

									<!-- RSVP Section -->
									{#if !event.isPast}
										<div class="mt-6 flex items-center gap-3">
											{#if event.rsvpRequired}
												{@const rsvp = getRSVPStatus(event.id)}
												<div class="flex gap-2">
													<button
														onclick={() => handleRSVP(event.id, 'attending')}
														class="rounded-lg px-4 py-2 font-medium transition-colors {rsvp?.status === 'attending'
															? 'bg-green-600 text-white'
															: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
													>
														✓ Attending
													</button>
													<button
														onclick={() => handleRSVP(event.id, 'maybe')}
														class="rounded-lg px-4 py-2 font-medium transition-colors {rsvp?.status === 'maybe'
															? 'bg-yellow-600 text-white'
															: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
													>
														? Maybe
													</button>
													<button
														onclick={() => handleRSVP(event.id, 'not-attending')}
														class="rounded-lg px-4 py-2 font-medium transition-colors {rsvp?.status === 'not-attending'
															? 'bg-red-600 text-white'
															: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
													>
														× Can't Go
													</button>
												</div>
											{/if}
											{#if event.isOnline && event.meetingLink}
												<a
													href={event.meetingLink}
													target="_blank"
													rel="noopener noreferrer"
													class="inline-flex items-center gap-2 text-orange-600 hover:text-orange-700 font-medium"
												>
													Join Online →
												</a>
											{/if}
										</div>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{:else}
			<!-- Calendar View -->
			<div class="bg-white rounded-lg shadow-lg p-6">
				<div class="mb-6">
					<h2 class="text-xl font-semibold text-gray-900">
						{new Date().toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
					</h2>
				</div>
				<div class="grid grid-cols-7 gap-2">
					<!-- Day headers -->
					{#each ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'] as day}
						<div class="text-center text-sm font-semibold text-gray-600 py-2">
							{day}
						</div>
					{/each}

					<!-- Calendar days -->
					{#each getCalendarDays() as day}
						<div class="border border-gray-200 rounded-lg p-2 min-h-24 {day ? 'bg-white' : 'bg-gray-50'}">
							{#if day}
								<div class="text-sm font-medium text-gray-900 mb-1">
									{day.getDate()}
								</div>
								{@const dayEvents = getEventsForDate(day)}
								{#if dayEvents.length > 0}
									<div class="space-y-1">
										{#each dayEvents.slice(0, 2) as event}
											<div
												class="text-xs p-1 rounded truncate cursor-pointer hover:opacity-80 {getEventTypeColor(event.type)}"
												onclick={() => goto(`/events/${event.id}`)}
											>
												{event.title}
											</div>
										{/each}
										{#if dayEvents.length > 2}
											<div class="text-xs text-gray-600">
												+{dayEvents.length - 2} more
											</div>
										{/if}
									</div>
								{/if}
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>
