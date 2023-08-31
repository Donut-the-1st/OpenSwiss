<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let name = ''
    let players = ''

    async function regPlayer() {
        await invoke('add_player', { playerName: name })
    }

    async function getPlayers() {
        players = await invoke('get_players')
    }
</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button
        on:click="{regPlayer}"
        on:click="{getPlayers}"
    >
        Add Player
    </button>
    <p>{players}</p>
</div>