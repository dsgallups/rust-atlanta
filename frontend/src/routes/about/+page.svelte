<script lang="ts">
	import { goto } from '$app/navigation';

	interface TeamMember {
		name: string;
		role: string;
		bio: string;
		github?: string;
		twitter?: string;
		linkedin?: string;
		avatar?: string;
	}

	interface Milestone {
		date: Date;
		title: string;
		description: string;
		icon: string;
	}

	let teamMembers: TeamMember[] = [
		{
			name: 'Sarah Chen',
			role: 'Lead Organizer',
			bio: 'Rust enthusiast and senior developer. Passionate about building inclusive tech communities.',
			github: 'sarahchen',
			twitter: 'sarahchendev',
			linkedin: 'sarahchen'
		},
		{
			name: 'Alex Rodriguez',
			role: 'Education Lead',
			bio: 'Dedicated to making Rust accessible to everyone. Loves teaching and mentoring.',
			github: 'alexrodriguez',
			linkedin: 'alexrodriguez'
		},
		{
			name: 'Jamie Park',
			role: 'Events Coordinator',
			bio: 'Brings the community together through engaging events and workshops.',
			github: 'jamiepark',
			twitter: 'jamieparktech'
		},
		{
			name: 'Michael Thompson',
			role: 'Technical Advisor',
			bio: 'Rust core contributor with expertise in systems programming and performance.',
			github: 'mthompson',
			linkedin: 'michaelthompson'
		}
	];

	let milestones: Milestone[] = [
		{
			date: new Date('2022-01-15'),
			title: 'Community Founded',
			description: 'Rust Atlanta was born from a small group of passionate developers.',
			icon: '🎉'
		},
		{
			date: new Date('2022-06-20'),
			title: '100 Members',
			description: 'Reached our first major milestone of 100 community members.',
			icon: '🎯'
		},
		{
			date: new Date('2023-02-10'),
			title: 'First Hackathon',
			description: 'Hosted our inaugural 48-hour Rust hackathon with 50+ participants.',
			icon: '💻'
		},
		{
			date: new Date('2023-08-15'),
			title: 'Workshop Series Launch',
			description: 'Started monthly beginner-friendly Rust workshops.',
			icon: '📚'
		},
		{
			date: new Date('2024-01-01'),
			title: '350+ Members',
			description: 'Growing strong with an active and engaged community.',
			icon: '🚀'
		}
	];

	let values = [
		{
			icon: '🤝',
			title: 'Inclusive',
			description: 'We welcome Rustaceans of all skill levels and backgrounds.'
		},
		{
			icon: '📚',
			title: 'Educational',
			description: 'Committed to continuous learning and knowledge sharing.'
		},
		{
			icon: '🌟',
			title: 'Supportive',
			description: 'Foster a helpful environment where everyone can grow.'
		},
		{
			icon: '🔧',
			title: 'Practical',
			description: 'Focus on real-world applications and hands-on experience.'
		}
	];

	function formatDate(date: Date): string {
		return new Intl.DateTimeFormat('en-US', {
			month: 'long',
			year: 'numeric'
		}).format(date);
	}
</script>

<div class="min-h-screen bg-gray-50">
	<!-- Hero Section -->
	<div class="bg-gradient-to-br from-orange-50 to-red-50 border-b border-orange-200">
		<div class="mx-auto max-w-7xl px-4 py-16 sm:px-6 lg:px-8">
			<div class="text-center">
				<h1 class="text-4xl font-bold text-gray-900 sm:text-5xl">
					About <span class="bg-gradient-to-r from-orange-600 to-red-600 bg-clip-text text-transparent">Rust Atlanta</span>
				</h1>
				<p class="mx-auto mt-6 max-w-3xl text-lg text-gray-600">
					We're a vibrant community of Rust developers, learners, and enthusiasts in the Atlanta metropolitan area.
					Whether you're just starting your Rust journey or you're a seasoned systems programmer, you'll find a home here.
				</p>
			</div>
		</div>
	</div>

	<!-- Mission Section -->
	<div class="bg-white py-16">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-12">
				<h2 class="text-3xl font-bold text-gray-900">Our Mission</h2>
				<p class="mx-auto mt-4 max-w-3xl text-lg text-gray-600">
					To grow and nurture the Rust programming community in Atlanta through education,
					collaboration, and building amazing projects together.
				</p>
			</div>

			<div class="grid gap-8 md:grid-cols-2 lg:grid-cols-4">
				{#each values as value}
					<div class="text-center">
						<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-gradient-to-br from-orange-100 to-red-100">
							<span class="text-2xl">{value.icon}</span>
						</div>
						<h3 class="mb-2 text-lg font-semibold text-gray-900">{value.title}</h3>
						<p class="text-gray-600">{value.description}</p>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<!-- Community Stats -->
	<div class="bg-gray-50 py-12">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="rounded-lg bg-gradient-to-r from-orange-600 to-red-600 p-8 text-white">
				<div class="grid gap-8 md:grid-cols-4 text-center">
					<div>
						<div class="text-4xl font-bold">350+</div>
						<div class="mt-1 text-orange-100">Active Members</div>
					</div>
					<div>
						<div class="text-4xl font-bold">120+</div>
						<div class="mt-1 text-orange-100">Events Hosted</div>
					</div>
					<div>
						<div class="text-4xl font-bold">47</div>
						<div class="mt-1 text-orange-100">Open Source Projects</div>
					</div>
					<div>
						<div class="text-4xl font-bold">1.2M+</div>
						<div class="mt-1 text-orange-100">Lines of Rust</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Team Section -->
	<div class="bg-white py-16">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-12">
				<h2 class="text-3xl font-bold text-gray-900">Meet the Organizers</h2>
				<p class="mt-4 text-gray-600">
					The passionate volunteers who keep our community running
				</p>
			</div>

			<div class="grid gap-8 md:grid-cols-2 lg:grid-cols-4">
				{#each teamMembers as member}
					<div class="text-center">
						<div class="mx-auto mb-4 h-32 w-32 rounded-full bg-gradient-to-br from-orange-400 to-red-500"></div>
						<h3 class="text-lg font-semibold text-gray-900">{member.name}</h3>
						<p class="text-sm text-orange-600 mb-2">{member.role}</p>
						<p class="text-sm text-gray-600 mb-4">{member.bio}</p>
						<div class="flex justify-center gap-3">
							{#if member.github}
								<a
									href="https://github.com/{member.github}"
									target="_blank"
									rel="noopener noreferrer"
									class="text-gray-400 hover:text-gray-600"
								>
									<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
										<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
									</svg>
								</a>
							{/if}
							{#if member.twitter}
								<a
									href="https://twitter.com/{member.twitter}"
									target="_blank"
									rel="noopener noreferrer"
									class="text-gray-400 hover:text-gray-600"
								>
									<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
										<path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
									</svg>
								</a>
							{/if}
							{#if member.linkedin}
								<a
									href="https://linkedin.com/in/{member.linkedin}"
									target="_blank"
									rel="noopener noreferrer"
									class="text-gray-400 hover:text-gray-600"
								>
									<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
										<path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/>
									</svg>
								</a>
							{/if}
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<!-- Timeline Section -->
	<div class="bg-gray-50 py-16">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-12">
				<h2 class="text-3xl font-bold text-gray-900">Our Journey</h2>
				<p class="mt-4 text-gray-600">
					Key milestones in the growth of Rust Atlanta
				</p>
			</div>

			<div class="relative">
				<div class="absolute left-1/2 transform -translate-x-1/2 h-full w-0.5 bg-orange-200"></div>
				{#each milestones as milestone, index}
					<div class="relative mb-12 {index % 2 === 0 ? 'md:pr-1/2 md:text-right' : 'md:pl-1/2 md:ml-auto md:w-1/2'}">
						<div class="bg-white rounded-lg p-6 shadow-lg {index % 2 === 0 ? 'md:mr-8' : 'md:ml-8'}">
							<div class="flex items-center gap-3 mb-2 {index % 2 === 0 ? 'md:flex-row-reverse' : ''}">
								<span class="text-2xl">{milestone.icon}</span>
								<time class="text-sm font-medium text-orange-600">
									{formatDate(milestone.date)}
								</time>
							</div>
							<h3 class="text-lg font-semibold text-gray-900 mb-1">{milestone.title}</h3>
							<p class="text-gray-600">{milestone.description}</p>
						</div>
						<div class="absolute top-6 left-1/2 transform -translate-x-1/2 h-4 w-4 rounded-full bg-orange-600 border-4 border-white"></div>
					</div>
				{/each}
			</div>
		</div>
	</div>

	<!-- Join Section -->
	<div id="join" class="bg-white py-16">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="text-center mb-12">
				<h2 class="text-3xl font-bold text-gray-900">Join Our Community</h2>
				<p class="mt-4 text-gray-600">
					Multiple ways to get involved and connect with fellow Rustaceans
				</p>
			</div>

			<div class="grid gap-6 md:grid-cols-3">
				<div class="rounded-lg border border-gray-200 p-6 text-center hover:shadow-lg transition-shadow">
					<div class="mx-auto mb-4 h-12 w-12 text-4xl">💬</div>
					<h3 class="mb-2 text-lg font-semibold text-gray-900">Discord Server</h3>
					<p class="mb-4 text-gray-600">
						Chat with members, ask questions, and share your projects in real-time.
					</p>
					<a
						href="https://discord.gg/rust-atlanta"
						target="_blank"
						rel="noopener noreferrer"
						class="inline-flex items-center gap-2 text-orange-600 hover:text-orange-700 font-medium"
					>
						Join Discord
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
						</svg>
					</a>
				</div>

				<div class="rounded-lg border border-gray-200 p-6 text-center hover:shadow-lg transition-shadow">
					<div class="mx-auto mb-4 h-12 w-12 text-4xl">📅</div>
					<h3 class="mb-2 text-lg font-semibold text-gray-900">Attend Events</h3>
					<p class="mb-4 text-gray-600">
						Join our meetups, workshops, and hackathons to learn and network.
					</p>
					<button
						onclick={() => goto('/events')}
						class="inline-flex items-center gap-2 text-orange-600 hover:text-orange-700 font-medium"
					>
						View Events
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
						</svg>
					</button>
				</div>

				<div class="rounded-lg border border-gray-200 p-6 text-center hover:shadow-lg transition-shadow">
					<div class="mx-auto mb-4 h-12 w-12 text-4xl">🚀</div>
					<h3 class="mb-2 text-lg font-semibold text-gray-900">Contribute</h3>
					<p class="mb-4 text-gray-600">
						Share your knowledge, contribute to projects, or help organize events.
					</p>
					<a
						href="https://github.com/rust-atlanta"
						target="_blank"
						rel="noopener noreferrer"
						class="inline-flex items-center gap-2 text-orange-600 hover:text-orange-700 font-medium"
					>
						GitHub Org
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
						</svg>
					</a>
				</div>
			</div>

			<div class="mt-12 rounded-lg bg-gradient-to-r from-orange-50 to-red-50 p-8 text-center">
				<h3 class="mb-4 text-2xl font-bold text-gray-900">Ready to Start Your Rust Journey?</h3>
				<p class="mx-auto mb-6 max-w-2xl text-gray-600">
					No matter your experience level, we're here to help you learn and grow.
					Join our welcoming community and discover the power of Rust.
				</p>
				<div class="flex flex-col sm:flex-row justify-center gap-4">
					<a
						href="https://discord.gg/rust-atlanta"
						target="_blank"
						rel="noopener noreferrer"
						class="inline-flex items-center justify-center gap-2 rounded-lg bg-gradient-to-r from-orange-600 to-red-600 px-6 py-3 font-medium text-white transition-shadow hover:shadow-lg"
					>
						<svg class="h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
							<path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.865-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"/>
						</svg>
						Join Discord Now
					</a>
					<button
						onclick={() => goto('/resources')}
						class="inline-flex items-center justify-center gap-2 rounded-lg border-2 border-orange-600 bg-white px-6 py-3 font-medium text-orange-600 hover:bg-orange-50"
					>
						Browse Resources
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
						</svg>
					</button>
				</div>
			</div>
		</div>
	</div>

	<!-- Contact Section -->
	<div class="bg-gray-50 py-16">
		<div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
			<div class="text-center">
				<h2 class="text-3xl font-bold text-gray-900 mb-8">Get in Touch</h2>
				<div class="grid gap-6 md:grid-cols-3 max-w-3xl mx-auto">
					<div>
						<div class="text-2xl mb-2">📧</div>
						<h3 class="font-semibold text-gray-900 mb-1">Email</h3>
						<a href="mailto:hello@rust-atlanta.dev" class="text-orange-600 hover:text-orange-700">
							hello@rust-atlanta.dev
						</a>
					</div>
					<div>
						<div class="text-2xl mb-2">🐦</div>
						<h3 class="font-semibold text-gray-900 mb-1">Twitter</h3>
						<a href="https://twitter.com/rustatlanta" target="_blank" rel="noopener noreferrer" class="text-orange-600 hover:text-orange-700">
							@rustatlanta
						</a>
					</div>
					<div>
						<div class="text-2xl mb-2">📍</div>
						<h3 class="font-semibold text-gray-900 mb-1">Location</h3>
						<p class="text-gray-600">Atlanta, GA</p>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
