<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import SvelteTable from "svelte-table";


    // define column configs
    const columns = [
        {
            key: "ID",
            title: "ID",
            value: v => v.ID,
        },
        {
            key: "player_name",
            title: "Name",
            value: v => v.player_name,
        }
    ];

    let new_name = ''
    let rows = []


    async function regPlayer() {
        await invoke('add_player', { playerName: new_name })
    }

    async function getPlayers() {
        rows = JSON.parse(await invoke('get_players'))
    }
</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{new_name}" />
    <button
        on:click="{regPlayer}"
        on:click="{getPlayers}"
        on:click="{()=> new_name = null}"
    >
        Add Player
    </button>
    <SvelteTable columns="{columns}" rows="{rows}"></SvelteTable>
</div>